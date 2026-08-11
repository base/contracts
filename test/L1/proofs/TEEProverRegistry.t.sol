// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CborDecode, CborElement, LibCborElement } from "lib/nitro-validator/src/CborDecode.sol";
import { ICertManager } from "lib/nitro-validator/src/ICertManager.sol";
import { IP384Verifier } from "lib/nitro-validator/src/IP384Verifier.sol";
import { NitroValidator } from "lib/nitro-validator/src/NitroValidator.sol";
import { Sha2Ext } from "lib/nitro-validator/src/Sha2Ext.sol";
import { Test } from "lib/forge-std/src/Test.sol";

import { Proxy } from "src/universal/Proxy.sol";

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";

import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { MockNitroValidator } from "test/mocks/MockNitroValidator.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

contract MockAggregateVerifierForRegistry {
    bytes32 public immutable TEE_IMAGE_HASH;

    constructor(bytes32 imageHash) {
        TEE_IMAGE_HASH = imageHash;
    }
}

contract MockDisputeGameFactoryForRegistry {
    IDisputeGame internal immutable _impl;

    constructor(address impl) {
        _impl = IDisputeGame(impl);
    }

    function gameImpls(GameType) external view returns (IDisputeGame) {
        return _impl;
    }
}

contract NitroValidatorParseHarnessForRegistry is NitroValidator {
    constructor(ICertManager certManager, IP384Verifier p384Verifier) NitroValidator(certManager, p384Verifier) { }

    function parseAttestation(bytes memory attestationTbs) external pure returns (Ptrs memory) {
        return _parseAttestation(attestationTbs);
    }
}

/// @dev Uses DevTEEProverRegistry because production signer registration requires a Nitro attestation proof.
contract TEEProverRegistryTest is Test {
    DevTEEProverRegistry public teeProverRegistry;

    address public owner;
    address public manager;
    address public proxyAdmin;
    address public unauthorized;

    bytes32 public constant TEST_IMAGE_HASH = keccak256("test-image-hash");
    GameType public constant TEST_GAME_TYPE = GameType.wrap(621);
    string internal constant NOT_OWNER = "OwnableManaged: caller is not the owner";
    string internal constant NOT_OWNER_OR_MANAGER = "OwnableManaged: caller is not the owner or the manager";

    // Events must be redeclared here because Solidity 0.8.15 doesn't support
    // referencing events from other contracts via qualified names (requires 0.8.21+)
    event SignerRegistered(address indexed signer);
    event SignerDeregistered(address indexed signer);
    event ProposerSet(address indexed proposer, bool isValid);

    function setUp() public {
        owner = makeAddr("owner");
        manager = makeAddr("manager");
        unauthorized = makeAddr("unauthorized");

        teeProverRegistry = _deployRegistry(new address[](0), TEST_GAME_TYPE);
    }

    function _deployRegistry(address[] memory proposers, GameType gameType) internal returns (DevTEEProverRegistry) {
        MockAggregateVerifierForRegistry verifier = new MockAggregateVerifierForRegistry(TEST_IMAGE_HASH);
        MockDisputeGameFactoryForRegistry factory = new MockDisputeGameFactoryForRegistry(address(verifier));

        DevTEEProverRegistry impl = new DevTEEProverRegistry({
            nitroValidator: NitroValidator(address(0)), factory: IDisputeGameFactory(address(factory))
        });

        proxyAdmin = makeAddr("proxy-admin");
        Proxy proxy = new Proxy(proxyAdmin);
        vm.prank(proxyAdmin);
        proxy.upgradeToAndCall(
            address(impl), abi.encodeCall(TEEProverRegistry.initialize, (owner, manager, proposers, gameType))
        );

        return DevTEEProverRegistry(address(proxy));
    }

    function _addDevSigner(address signer) internal {
        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);
    }

    function _addDevSigners(address signer1, address signer2, address signer3) internal {
        vm.startPrank(owner);
        teeProverRegistry.addDevSigner(signer1, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer2, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer3, TEST_IMAGE_HASH);
        vm.stopPrank();
    }

    function _expectNotOwnerRevert(address caller) internal {
        vm.prank(caller);
        vm.expectRevert(bytes(NOT_OWNER));
    }

    function _expectNotOwnerOrManagerRevert(address caller) internal {
        vm.prank(caller);
        vm.expectRevert(bytes(NOT_OWNER_OR_MANAGER));
    }

    function _assertContains(address[] memory values, address expected) internal {
        for (uint256 i = 0; i < values.length; i++) {
            if (values[i] == expected) return;
        }

        fail();
    }

    function testInitialization() public view {
        assertEq(teeProverRegistry.owner(), owner);
        assertEq(teeProverRegistry.manager(), manager);
        assertEq(teeProverRegistry.version(), "0.6.0");
    }

    function testInitializationWithProposers() public {
        address proposer1 = makeAddr("proposer1");
        address proposer2 = makeAddr("proposer2");
        address proposer3 = makeAddr("proposer3");
        address[] memory proposers = new address[](3);
        proposers[0] = proposer1;
        proposers[1] = proposer2;
        proposers[2] = proposer3;

        DevTEEProverRegistry registry2 = _deployRegistry(proposers, TEST_GAME_TYPE);
        assertTrue(registry2.isValidProposer(proposer1));
        assertTrue(registry2.isValidProposer(proposer2));
        assertTrue(registry2.isValidProposer(proposer3));
    }

    function testInitializationWithEmptyProposers() public view {
        assertFalse(teeProverRegistry.isValidProposer(address(0)));
    }

    function testDeregisterSignerAsOwner() public {
        address signer = makeAddr("signer");

        _addDevSigner(signer);

        vm.expectEmit(true, false, false, false, address(teeProverRegistry));
        emit SignerDeregistered(signer);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        assertFalse(teeProverRegistry.isValidSigner(signer));
    }

    function testDeregisterSignerAsManager() public {
        address signer = makeAddr("signer");

        _addDevSigner(signer);

        vm.prank(manager);
        teeProverRegistry.deregisterSigner(signer);

        assertFalse(teeProverRegistry.isValidSigner(signer));
    }

    function testDeregisterSignerFailsIfUnauthorized() public {
        address signer = makeAddr("signer");

        _expectNotOwnerOrManagerRevert(unauthorized);
        teeProverRegistry.deregisterSigner(signer);
    }

    function testSetProposer() public {
        address newProposer = makeAddr("proposer");

        vm.expectEmit(true, false, false, false, address(teeProverRegistry));
        emit ProposerSet(newProposer, true);

        vm.prank(owner);
        teeProverRegistry.setProposer(newProposer, true);

        assertTrue(teeProverRegistry.isValidProposer(newProposer));
    }

    function testSetProposerFailsIfNotOwner() public {
        address newProposer = makeAddr("proposer");

        _expectNotOwnerRevert(manager);
        teeProverRegistry.setProposer(newProposer, true);

        _expectNotOwnerRevert(unauthorized);
        teeProverRegistry.setProposer(newProposer, true);
    }

    function testIsValidSignerReturnsFalseForUnregistered() public {
        address unregistered = makeAddr("unregistered");
        assertFalse(teeProverRegistry.isValidSigner(unregistered));
    }

    function testIsValidSignerReturnsTrueForRegistered() public {
        address signer = makeAddr("signer");

        _addDevSigner(signer);

        assertTrue(teeProverRegistry.isValidSigner(signer));
    }

    function testMaxAgeConstant() public view {
        assertEq(teeProverRegistry.MAX_AGE(), 60 minutes);
    }

    function testTransferOwnership() public {
        address newOwner = makeAddr("newOwner");

        vm.prank(owner);
        teeProverRegistry.transferOwnership(newOwner);

        assertEq(teeProverRegistry.owner(), newOwner);
    }

    function testTransferOwnershipFailsIfNotOwner() public {
        address newOwner = makeAddr("newOwner");

        _expectNotOwnerRevert(manager);
        teeProverRegistry.transferOwnership(newOwner);

        _expectNotOwnerRevert(unauthorized);
        teeProverRegistry.transferOwnership(newOwner);
    }

    function testTransferOwnershipFailsForZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("OwnableManaged: new owner is the zero address");
        teeProverRegistry.transferOwnership(address(0));
    }

    function testTransferManagementAsOwner() public {
        address newManager = makeAddr("newManager");

        vm.prank(owner);
        teeProverRegistry.transferManagement(newManager);

        assertEq(teeProverRegistry.manager(), newManager);
    }

    function testTransferManagementAsManager() public {
        address newManager = makeAddr("newManager");

        vm.prank(manager);
        teeProverRegistry.transferManagement(newManager);

        assertEq(teeProverRegistry.manager(), newManager);
    }

    function testTransferManagementFailsIfUnauthorized() public {
        address newManager = makeAddr("newManager");

        _expectNotOwnerOrManagerRevert(unauthorized);
        teeProverRegistry.transferManagement(newManager);
    }

    function testTransferManagementFailsForZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("OwnableManaged: new manager is the zero address");
        teeProverRegistry.transferManagement(address(0));
    }

    function testRenounceOwnership() public {
        vm.prank(owner);
        teeProverRegistry.renounceOwnership();

        assertEq(teeProverRegistry.owner(), address(0));
    }

    function testRenounceManagementAsOwner() public {
        vm.prank(owner);
        teeProverRegistry.renounceManagement();

        assertEq(teeProverRegistry.manager(), address(0));
    }

    function testRenounceManagementAsManager() public {
        vm.prank(manager);
        teeProverRegistry.renounceManagement();

        assertEq(teeProverRegistry.manager(), address(0));
    }

    function testAddDevSigner() public {
        address signer = makeAddr("dev-signer");

        vm.expectEmit(true, false, false, false, address(teeProverRegistry));
        emit SignerRegistered(signer);

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        assertTrue(teeProverRegistry.isValidSigner(signer));
    }

    function testAddDevSignerFailsIfNotOwner() public {
        address signer = makeAddr("dev-signer");

        _expectNotOwnerRevert(manager);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        _expectNotOwnerRevert(unauthorized);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);
    }

    function testAddDevSignerIdempotent() public {
        address signer = makeAddr("dev-signer");

        _addDevSigner(signer);
        _addDevSigner(signer);

        assertTrue(teeProverRegistry.isValidSigner(signer));
    }

    function testAddMultipleDevSigners() public {
        address signer1 = makeAddr("dev-signer-1");
        address signer2 = makeAddr("dev-signer-2");
        address signer3 = makeAddr("dev-signer-3");

        _addDevSigners(signer1, signer2, signer3);

        assertTrue(teeProverRegistry.isValidSigner(signer1));
        assertTrue(teeProverRegistry.isValidSigner(signer2));
        assertTrue(teeProverRegistry.isValidSigner(signer3));
    }

    function testGetRegisteredSignersEmpty() public view {
        assertEq(teeProverRegistry.getRegisteredSigners().length, 0);
    }

    function testGetRegisteredSignersAfterRegister() public {
        address signer = makeAddr("signer");

        _addDevSigner(signer);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 1);
        assertEq(signers[0], signer);
    }

    function testGetRegisteredSignersAfterDeregister() public {
        address signer = makeAddr("signer");

        _addDevSigner(signer);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        assertEq(teeProverRegistry.getRegisteredSigners().length, 0);
    }

    function testGetRegisteredSignersMultiple() public {
        address signer1 = makeAddr("signer-1");
        address signer2 = makeAddr("signer-2");
        address signer3 = makeAddr("signer-3");

        _addDevSigners(signer1, signer2, signer3);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 3);

        _assertContains(signers, signer1);
        _assertContains(signers, signer2);
        _assertContains(signers, signer3);
    }

    function testGetRegisteredSignersConsistencyAfterMixedOperations() public {
        address signer1 = makeAddr("signer-1");
        address signer2 = makeAddr("signer-2");
        address signer3 = makeAddr("signer-3");

        _addDevSigners(signer1, signer2, signer3);

        vm.prank(manager);
        teeProverRegistry.deregisterSigner(signer2);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 2);
        _assertContains(signers, signer1);
        _assertContains(signers, signer3);
        assertFalse(teeProverRegistry.isValidSigner(signer2));
    }

    function testGetRegisteredSignersDeregisterIdempotent() public {
        address signer = makeAddr("signer");

        _addDevSigner(signer);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        assertEq(teeProverRegistry.getRegisteredSigners().length, 0);
    }

    function test_upgrade_preservesStorage_succeeds() public {
        address signer = makeAddr("upgrade-signer");
        address proposer = makeAddr("upgrade-proposer");
        _addDevSigner(signer);
        vm.prank(owner);
        teeProverRegistry.setProposer(proposer, true);

        MockNitroValidator nitroValidator = new MockNitroValidator();
        TEEProverRegistry newImpl = new TEEProverRegistry({
            nitroValidator: NitroValidator(address(nitroValidator)), factory: teeProverRegistry.DISPUTE_GAME_FACTORY()
        });

        vm.prank(proxyAdmin);
        Proxy(payable(address(teeProverRegistry))).upgradeTo(address(newImpl));

        TEEProverRegistry upgraded = TEEProverRegistry(address(teeProverRegistry));
        assertEq(upgraded.owner(), owner);
        assertEq(upgraded.manager(), manager);
        assertEq(GameType.unwrap(upgraded.gameType()), GameType.unwrap(TEST_GAME_TYPE));
        assertTrue(upgraded.isValidProposer(proposer));
        assertTrue(upgraded.isRegisteredSigner(signer));
        assertEq(upgraded.signerImageHash(signer), TEST_IMAGE_HASH);
        assertEq(upgraded.getRegisteredSigners().length, 1);
        assertEq(upgraded.getRegisteredSigners()[0], signer);
        assertEq(address(upgraded.NITRO_VALIDATOR()), address(nitroValidator));
    }
}

contract TEEProverRegistry_RegisterSigner_Test is Test {
    using CborDecode for bytes;

    uint256 internal constant EIP_7825_GAS_CAP = 16_777_216;
    uint256 internal constant REAL_ATTESTATION_TIMESTAMP = 1_767_472_867;
    uint256 internal constant SIGNER_PRIVATE_KEY = 0x1234567890abcdef;
    GameType internal constant TEST_GAME_TYPE = GameType.wrap(621);
    bytes internal constant SIGNATURE = hex"0102";
    bytes internal constant HINTS = hex"0304";

    struct Fixture {
        bytes tbs;
        bytes signature;
        bytes hints;
        bytes leafCert;
        uint64 leafNotAfter;
    }

    TEEProverRegistry internal teeProverRegistry;
    MockNitroValidator internal mockNitroValidator;
    address internal owner;
    address internal manager;
    address internal signer;
    bytes32 internal pcr0Hash;
    bytes internal attestationTbs;
    ICertManager internal certManager;
    IP384Verifier internal p384Verifier;
    NitroValidator internal nitroValidator;
    NitroValidatorParseHarnessForRegistry internal parser;

    event SignerRegistered(address indexed signer);

    function setUp() public {
        owner = makeAddr("owner");
        manager = makeAddr("manager");
        signer = vm.addr(SIGNER_PRIVATE_KEY);

        bytes memory pcr0 = new bytes(48);
        for (uint256 i = 0; i < pcr0.length; i++) {
            pcr0[i] = bytes1(uint8(i + 1));
        }
        pcr0Hash = keccak256(pcr0);

        bytes memory publicKey =
            hex"04f973a0b87062c389d125d8199e803b832b6ac6bf7867a4f6cd87506060fc4c584b4a0a3f26c988c54c236b224c48bb605b265949e65c098ecd87a581ca10e25d";
        attestationTbs = abi.encodePacked(pcr0, publicKey);

        mockNitroValidator = new MockNitroValidator();
        MockAggregateVerifierForRegistry verifier = new MockAggregateVerifierForRegistry(pcr0Hash);
        MockDisputeGameFactoryForRegistry factory = new MockDisputeGameFactoryForRegistry(address(verifier));
        TEEProverRegistry impl = new TEEProverRegistry({
            nitroValidator: NitroValidator(address(mockNitroValidator)), factory: IDisputeGameFactory(address(factory))
        });

        address proxyAdmin = makeAddr("proxy-admin");
        Proxy proxy = new Proxy(proxyAdmin);
        vm.prank(proxyAdmin);
        proxy.upgradeToAndCall(
            address(impl),
            abi.encodeCall(TEEProverRegistry.initialize, (owner, manager, new address[](0), TEST_GAME_TYPE))
        );
        teeProverRegistry = TEEProverRegistry(address(proxy));

        vm.warp(10_000);
    }

    function test_registerSigner_owner_succeeds() public {
        _mockValidation(_validPtrs(uint64((block.timestamp - 1) * 1000)));

        vm.expectEmit(true, false, false, false, address(teeProverRegistry));
        emit SignerRegistered(signer);
        vm.prank(owner);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);

        assertTrue(teeProverRegistry.isRegisteredSigner(signer));
        assertTrue(teeProverRegistry.isValidSigner(signer));
        assertEq(teeProverRegistry.signerImageHash(signer), pcr0Hash);
        assertEq(teeProverRegistry.getRegisteredSigners().length, 1);
        assertEq(teeProverRegistry.getRegisteredSigners()[0], signer);
    }

    function test_registerSigner_manager_succeeds() public {
        _mockValidation(_validPtrs(uint64((block.timestamp - 1) * 1000)));

        vm.prank(manager);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);

        assertTrue(teeProverRegistry.isRegisteredSigner(signer));
    }

    function test_registerSigner_unauthorized_reverts() public {
        vm.prank(makeAddr("unauthorized"));
        vm.expectRevert("OwnableManaged: caller is not the owner or the manager");
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_validatorFailure_reverts() public {
        bytes memory callData = _validatorCall();
        vm.mockCallRevert(
            address(mockNitroValidator), callData, abi.encodeWithSignature("Error(string)", "unused inverse hints")
        );

        vm.prank(owner);
        vm.expectRevert("unused inverse hints");
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_attestationAtMaxAge_reverts() public {
        _mockValidation(_validPtrs(uint64((block.timestamp - teeProverRegistry.MAX_AGE()) * 1000)));

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.AttestationTooOld.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_attestationOneSecondInsideMaxAge_succeeds() public {
        _mockValidation(_validPtrs(uint64((block.timestamp - teeProverRegistry.MAX_AGE() + 1) * 1000)));

        vm.prank(owner);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);

        assertTrue(teeProverRegistry.isRegisteredSigner(signer));
    }

    function test_registerSigner_currentTimestamp_reverts() public {
        _mockValidation(_validPtrs(uint64(block.timestamp * 1000)));

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.AttestationFromFuture.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_futureTimestamp_reverts() public {
        _mockValidation(_validPtrs(uint64((block.timestamp + 1) * 1000)));

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.AttestationFromFuture.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_missingPCR0_reverts() public {
        NitroValidator.Ptrs memory ptrs = _validPtrs(uint64((block.timestamp - 1) * 1000));
        ptrs.pcrs[0] = LibCborElement.toCborElement(0xf6, 0, 0);
        _mockValidation(ptrs);

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.PCR0NotFound.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_debugModePCR0_reverts() public {
        for (uint256 i = 0; i < 48; i++) {
            attestationTbs[i] = 0;
        }
        _mockValidation(_validPtrs(uint64((block.timestamp - 1) * 1000)));

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.InvalidPCR0.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function testFuzz_registerSigner_invalidPCR0Length_reverts(uint8 pcr0Length) public {
        vm.assume(pcr0Length != 48);
        NitroValidator.Ptrs memory ptrs = _validPtrs(uint64((block.timestamp - 1) * 1000));
        ptrs.pcrs[0] = LibCborElement.toCborElement(0x40, 0, pcr0Length);
        _mockValidation(ptrs);

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.InvalidPCR0.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_invalidPublicKeyLength_reverts() public {
        NitroValidator.Ptrs memory ptrs = _validPtrs(uint64((block.timestamp - 1) * 1000));
        ptrs.publicKey = LibCborElement.toCborElement(0x40, 48, 64);
        _mockValidation(ptrs);

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.InvalidPublicKey.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function test_registerSigner_invalidPublicKeyPrefix_reverts() public {
        attestationTbs[48] = 0x02;
        _mockValidation(_validPtrs(uint64((block.timestamp - 1) * 1000)));

        vm.prank(owner);
        vm.expectRevert(TEEProverRegistry.InvalidPublicKey.selector);
        teeProverRegistry.registerSigner(attestationTbs, SIGNATURE, HINTS);
    }

    function _validPtrs(uint64 timestamp) internal pure returns (NitroValidator.Ptrs memory ptrs) {
        ptrs.timestamp = timestamp;
        ptrs.pcrs = new CborElement[](32);
        ptrs.pcrs[0] = LibCborElement.toCborElement(0x40, 0, 48);
        ptrs.cabundle = new CborElement[](0);
        ptrs.publicKey = LibCborElement.toCborElement(0x40, 48, 65);
    }

    function _validatorCall() internal view returns (bytes memory) {
        return abi.encodeCall(NitroValidator.validateAttestationWithHints, (attestationTbs, SIGNATURE, HINTS));
    }

    function _mockValidation(NitroValidator.Ptrs memory ptrs) internal {
        vm.mockCall(address(mockNitroValidator), _validatorCall(), abi.encode(ptrs));
    }

    function _setUpRealAttestation() internal {
        vm.warp(REAL_ATTESTATION_TIMESTAMP + 1);
        (p384Verifier, certManager, nitroValidator) = _deployValidatorStack();
        parser = new NitroValidatorParseHarnessForRegistry(certManager, p384Verifier);
        teeProverRegistry = _deployRegistry(nitroValidator);
    }

    function test_registerSigner_realAttestationWithoutPublicKey_reverts() public {
        _setUpRealAttestation();
        Fixture memory fixture = _prepareFixture();
        bytes memory callData =
            abi.encodeCall(TEEProverRegistry.registerSigner, (fixture.tbs, fixture.signature, fixture.hints));

        uint256 gasBefore = gasleft();
        (bool success, bytes memory returndata) = address(teeProverRegistry).call(callData);
        uint256 executionGas = gasBefore - gasleft();

        assertFalse(success);
        assertEq(returndata, abi.encodeWithSelector(TEEProverRegistry.InvalidPublicKey.selector));
        assertLt(executionGas + 21_000 + _calldataGas(callData), EIP_7825_GAS_CAP);
    }

    function test_registerSigner_invalidHints_reverts() public {
        _setUpRealAttestation();
        Fixture memory fixture = _prepareFixture();
        fixture.hints[0] = bytes1(uint8(fixture.hints[0]) ^ 1);

        vm.expectRevert("bad inverse hint");
        teeProverRegistry.registerSigner(fixture.tbs, fixture.signature, fixture.hints);
    }

    function test_registerSigner_truncatedHints_reverts() public {
        _setUpRealAttestation();
        Fixture memory fixture = _prepareFixture();
        bytes memory truncatedHints = fixture.hints;
        assembly {
            mstore(truncatedHints, sub(mload(truncatedHints), 1))
        }

        vm.expectRevert("inverse hint underflow");
        teeProverRegistry.registerSigner(fixture.tbs, fixture.signature, truncatedHints);
    }

    function test_registerSigner_surplusHints_reverts() public {
        _setUpRealAttestation();
        Fixture memory fixture = _prepareFixture();
        fixture.hints = abi.encodePacked(fixture.hints, bytes1(0));

        vm.expectRevert("unused inverse hints");
        teeProverRegistry.registerSigner(fixture.tbs, fixture.signature, fixture.hints);
    }

    function test_registerSigner_uncachedCertificateChain_reverts() public {
        _setUpRealAttestation();
        Fixture memory fixture = _prepareFixture();
        (,, NitroValidator uncachedValidator) = _deployValidatorStack();
        TEEProverRegistry uncachedRegistry = _deployRegistry(uncachedValidator);

        vm.expectRevert("inverse hint underflow");
        uncachedRegistry.registerSigner(fixture.tbs, fixture.signature, fixture.hints);
    }

    function test_registerSigner_revokedLeafCertificate_reverts() public {
        _setUpRealAttestation();
        Fixture memory fixture = _prepareFixture();
        certManager.revokeCert(certManager.computeCertId(fixture.leafCert));

        vm.expectRevert("cert revoked");
        teeProverRegistry.registerSigner(fixture.tbs, fixture.signature, fixture.hints);
    }

    function test_registerSigner_expiredLeafCertificate_reverts() public {
        _setUpRealAttestation();
        Fixture memory fixture = _prepareFixture();
        vm.warp(uint256(fixture.leafNotAfter) + 1);

        vm.expectRevert("cert expired");
        teeProverRegistry.registerSigner(fixture.tbs, fixture.signature, fixture.hints);
    }

    function _prepareFixture() internal returns (Fixture memory fixture_) {
        bytes memory attestation = _loadRealAttestation();
        (fixture_.tbs, fixture_.signature) = nitroValidator.decodeAttestationTbs(attestation);
        NitroValidator.Ptrs memory ptrs = parser.parseAttestation(fixture_.tbs);

        bytes32 parentHash;
        for (uint256 i = 0; i < ptrs.cabundle.length; i++) {
            bytes memory cert = fixture_.tbs.slice(ptrs.cabundle[i]);
            bytes memory certHints;
            if (parentHash != bytes32(0)) {
                certHints = _collectCertHints(cert, certManager.loadVerified(parentHash).pubKey);
            }
            parentHash = certManager.verifyCACertWithHints(cert, parentHash, certHints);
        }

        fixture_.leafCert = fixture_.tbs.slice(ptrs.cert);
        bytes memory leafHints = _collectCertHints(fixture_.leafCert, certManager.loadVerified(parentHash).pubKey);
        ICertManager.VerifiedCert memory leaf =
            certManager.verifyClientCertWithHints(fixture_.leafCert, parentHash, leafHints);
        fixture_.leafNotAfter = leaf.notAfter;
        fixture_.hints = _collectVerifyHints(
            Sha2Ext.sha384(fixture_.tbs, 0, fixture_.tbs.length), fixture_.signature, leaf.pubKey
        );
    }

    function _collectCertHints(bytes memory cert, bytes memory parentPubKey) internal returns (bytes memory) {
        string[] memory command = new string[](7);
        command[0] = "node";
        command[1] = "lib/nitro-validator/tools/p384_hints.js";
        command[2] = "cert";
        command[3] = "--cert";
        command[4] = vm.toString(cert);
        command[5] = "--pubkey";
        command[6] = vm.toString(parentPubKey);
        return vm.ffi(command);
    }

    function _collectVerifyHints(
        bytes memory hash,
        bytes memory signature,
        bytes memory pubKey
    )
        internal
        returns (bytes memory)
    {
        string[] memory command = new string[](9);
        command[0] = "node";
        command[1] = "lib/nitro-validator/tools/p384_hints.js";
        command[2] = "verify";
        command[3] = "--hash";
        command[4] = vm.toString(hash);
        command[5] = "--signature";
        command[6] = vm.toString(signature);
        command[7] = "--pubkey";
        command[8] = vm.toString(pubKey);
        return vm.ffi(command);
    }

    function _loadRealAttestation() internal returns (bytes memory) {
        string[] memory command = new string[](3);
        command[0] = "node";
        command[1] = "-e";
        command[2] = string.concat(
            "process.chdir('lib/nitro-validator');",
            "const t=require(process.cwd()+'/tools/nitro_attestation_input.js');",
            "const a=t.repairMissingPublicKeyBytes(t.realFixture());",
            "process.stdout.write('0x'+a.toString('hex'));"
        );
        return vm.ffi(command);
    }

    function _deployValidatorStack()
        internal
        returns (IP384Verifier p384Verifier_, ICertManager certManager_, NitroValidator nitroValidator_)
    {
        address deployScript = _deployCode("DeployNitroValidatorStack", "");
        (bool success, bytes memory returndata) =
            deployScript.call(abi.encodeWithSignature("run(address,address)", address(this), address(this)));
        require(success, "stack deployment failed");
        (address p384VerifierAddr, address certManagerAddr, address nitroValidatorAddr) =
            abi.decode(returndata, (address, address, address));
        p384Verifier_ = IP384Verifier(p384VerifierAddr);
        certManager_ = ICertManager(certManagerAddr);
        nitroValidator_ = NitroValidator(nitroValidatorAddr);
    }

    function _deployRegistry(NitroValidator nitroValidator_) internal returns (TEEProverRegistry registry_) {
        MockAggregateVerifierForRegistry verifier = new MockAggregateVerifierForRegistry(bytes32(uint256(1)));
        MockDisputeGameFactoryForRegistry factory = new MockDisputeGameFactoryForRegistry(address(verifier));
        TEEProverRegistry impl =
            new TEEProverRegistry({ nitroValidator: nitroValidator_, factory: IDisputeGameFactory(address(factory)) });

        address proxyAdmin = makeAddr("real-attestation-proxy-admin");
        Proxy proxy = new Proxy(proxyAdmin);
        vm.prank(proxyAdmin);
        proxy.upgradeToAndCall(
            address(impl),
            abi.encodeCall(
                TEEProverRegistry.initialize, (address(this), address(this), new address[](0), TEST_GAME_TYPE)
            )
        );
        registry_ = TEEProverRegistry(address(proxy));
    }

    function _deployCode(string memory artifact, bytes memory args) internal returns (address deployed_) {
        string[] memory command = new string[](4);
        command[0] = "forge";
        command[1] = "inspect";
        command[2] = artifact;
        command[3] = "bytecode";
        bytes memory creationCode = abi.encodePacked(vm.ffi(command), args);
        assembly {
            deployed_ := create(0, add(creationCode, 0x20), mload(creationCode))
        }
        require(deployed_ != address(0), "deployment failed");
    }

    function _calldataGas(bytes memory data) internal pure returns (uint256 gas_) {
        for (uint256 i = 0; i < data.length; i++) {
            gas_ += data[i] == bytes1(0) ? 4 : 16;
        }
    }
}
