// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";

import { INitroEnclaveVerifier } from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/multiproof/tee/ITDXVerifier.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";
import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";
import { GameType } from "src/dispute/lib/Types.sol";

import { IDisputeGame } from "interfaces/dispute/IDisputeGame.sol";
import { MockAnchorStateRegistry } from "scripts/multiproof/mocks/MockAnchorStateRegistry.sol";
import { DevTEEProverRegistry } from "src/multiproof/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/multiproof/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/multiproof/tee/TEEVerifier.sol";

/// @notice Mock AggregateVerifier that returns a configurable TEE_IMAGE_HASH.
contract MockAggregateVerifierForVerifier {
    bytes32 public TEE_IMAGE_HASH;

    constructor(bytes32 imageHash) {
        TEE_IMAGE_HASH = imageHash;
    }
}

/// @notice Mock DisputeGameFactory that returns a fixed game implementation.
contract MockDisputeGameFactoryForVerifier {
    mapping(uint32 => address) internal _impls;

    function setImpl(uint32 gameType_, address impl) external {
        _impls[gameType_] = impl;
    }

    function gameImpls(GameType gameType_) external view returns (IDisputeGame) {
        return IDisputeGame(_impls[GameType.unwrap(gameType_)]);
    }
}

contract TEEVerifierTest is Test {
    TEEVerifier public verifier;
    DevTEEProverRegistry public teeProverRegistry;
    ProxyAdmin public proxyAdmin;
    MockAnchorStateRegistry public anchorStateRegistry;

    uint256 internal constant NITRO_SIGNER_PRIVATE_KEY =
        0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    uint256 internal constant TDX_SIGNER_PRIVATE_KEY =
        0x2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    address internal nitroSignerAddress;
    address internal tdxSignerAddress;

    bytes32 internal constant IMAGE_ID = keccak256("test-image-id");
    uint32 internal constant TEST_GAME_TYPE = 621;
    address internal immutable PROPOSER = makeAddr("proposer");

    address internal owner;

    function setUp() public {
        owner = address(this);

        nitroSignerAddress = vm.addr(NITRO_SIGNER_PRIVATE_KEY);
        tdxSignerAddress = vm.addr(TDX_SIGNER_PRIVATE_KEY);

        // Deploy mock factory and verifier
        MockAggregateVerifierForVerifier mockVerifier = new MockAggregateVerifierForVerifier(IMAGE_ID);
        MockDisputeGameFactoryForVerifier mockFactory = new MockDisputeGameFactoryForVerifier();
        mockFactory.setImpl(TEST_GAME_TYPE, address(mockVerifier));

        // Deploy implementation (NitroEnclaveVerifier not needed for dev signer tests)
        DevTEEProverRegistry impl = new DevTEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(1)), IDisputeGameFactory(address(mockFactory))
        );

        // Deploy proxy admin
        proxyAdmin = new ProxyAdmin(address(this));

        // Deploy proxy
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl),
            address(proxyAdmin),
            abi.encodeCall(
                TEEProverRegistry.initialize, (owner, owner, new address[](0), GameType.wrap(TEST_GAME_TYPE))
            )
        );

        teeProverRegistry = DevTEEProverRegistry(address(proxy));

        // Register one Nitro signer and one TDX signer with the correct image hash.
        teeProverRegistry.addDevSigner(nitroSignerAddress, IMAGE_ID);
        teeProverRegistry.addDevTDXSigner(tdxSignerAddress, IMAGE_ID);

        // Set the proposer as valid
        teeProverRegistry.setProposer(PROPOSER, true);

        // Deploy TEEVerifier
        anchorStateRegistry = new MockAnchorStateRegistry();
        verifier = new TEEVerifier(
            TEEProverRegistry(address(teeProverRegistry)), IAnchorStateRegistry(address(anchorStateRegistry))
        );
    }

    function testVerifyValidSignatures() public view {
        bytes32 journal = keccak256("test-journal");
        bytes memory proofBytes = _proofBytes(journal);
        bool result = verifier.verify(proofBytes, IMAGE_ID, journal);
        assertTrue(result);
    }

    function testVerifyFailsWithInvalidSignature() public {
        bytes32 journal = keccak256("test-journal");
        bytes memory invalidSignature = new bytes(65);
        invalidSignature[64] = bytes1(uint8(27)); // Set v to 27

        bytes memory proofBytes = _buildProof(PROPOSER, invalidSignature, _signature(TDX_SIGNER_PRIVATE_KEY, journal));

        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithInvalidProposer() public {
        bytes32 journal = keccak256("test-journal");
        bytes memory proofBytes = _buildProof(
            address(0), _signature(NITRO_SIGNER_PRIVATE_KEY, journal), _signature(TDX_SIGNER_PRIVATE_KEY, journal)
        );

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        // Use a different private key that's not registered
        uint256 unregisteredKey = 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef;
        address unregisteredSigner = vm.addr(unregisteredKey);

        bytes32 journal = keccak256("test-journal");

        bytes memory proofBytes =
            _buildProof(PROPOSER, _signature(unregisteredKey, journal), _signature(TDX_SIGNER_PRIVATE_KEY, journal));

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithImageIdMismatch() public {
        bytes32 journal = keccak256("test-journal");
        bytes memory proofBytes = _proofBytes(journal);

        // Different imageId should fail — signer was registered with IMAGE_ID
        bytes32 wrongImageId = keccak256("different-image");
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.ImageIdMismatch.selector, IMAGE_ID, wrongImageId));
        verifier.verify(proofBytes, wrongImageId, journal);
    }

    function testVerifyFailsWithDuplicateSigner() public {
        bytes32 journal = keccak256("test-journal");
        bytes memory nitroSignature = _signature(NITRO_SIGNER_PRIVATE_KEY, journal);
        bytes memory proofBytes = _buildProof(PROPOSER, nitroSignature, nitroSignature);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.DuplicateSigner.selector, nitroSignerAddress));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithoutNitroSignature() public {
        bytes32 journal = keccak256("test-journal");
        uint256 secondTDXKey = 0x3234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;

        vm.prank(owner);
        teeProverRegistry.addDevTDXSigner(vm.addr(secondTDXKey), IMAGE_ID);

        bytes memory proofBytes =
            _buildProof(PROPOSER, _signature(TDX_SIGNER_PRIVATE_KEY, journal), _signature(secondTDXKey, journal));

        vm.expectRevert(TEEVerifier.MissingNitroSignature.selector);
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithoutTDXSignature() public {
        bytes32 journal = keccak256("test-journal");
        uint256 secondNitroKey = 0x4234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;

        vm.prank(owner);
        teeProverRegistry.addDevSigner(vm.addr(secondNitroKey), IMAGE_ID);

        bytes memory proofBytes =
            _buildProof(PROPOSER, _signature(NITRO_SIGNER_PRIVATE_KEY, journal), _signature(secondNitroKey, journal));

        vm.expectRevert(TEEVerifier.MissingTDXSignature.selector);
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithInvalidProofFormat() public {
        bytes32 journal = keccak256("test-journal");

        // Proof too short (less than proposer + two signatures).
        bytes memory shortProof = new bytes(50);

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(shortProof, IMAGE_ID, journal);
    }

    function testConstants() public view {
        assertEq(address(verifier.TEE_PROVER_REGISTRY()), address(teeProverRegistry));
    }

    function _proofBytes(bytes32 journal) internal view returns (bytes memory) {
        return _buildProof(
            PROPOSER, _signature(NITRO_SIGNER_PRIVATE_KEY, journal), _signature(TDX_SIGNER_PRIVATE_KEY, journal)
        );
    }

    function _signature(uint256 privateKey, bytes32 journal) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, journal);
        return abi.encodePacked(r, s, v);
    }

    function _buildProof(
        address proposer,
        bytes memory firstSignature,
        bytes memory secondSignature
    )
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(proposer, firstSignature, secondSignature);
    }
}
