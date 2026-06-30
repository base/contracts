// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Proxy } from "src/universal/Proxy.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

bytes32 constant TEST_IMAGE_HASH = keccak256("test-image-hash");

/// @notice Mock AggregateVerifier that returns TEE image hashes.
contract MockAggregateVerifierForRegistry {
    bytes32 public constant TEE_NITRO_IMAGE_HASH = TEST_IMAGE_HASH;
    bytes32 public constant TEE_TDX_IMAGE_HASH = TEST_IMAGE_HASH;
}

/// @dev Uses DevTEEProverRegistry because production signer registration requires a Nitro attestation proof.
contract TEEProverRegistryTest is Test {
    DevTEEProverRegistry public teeProverRegistry;

    address public owner;
    address public manager;

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

        teeProverRegistry = _deployRegistry(new address[](0));
    }

    function _deployRegistry(address[] memory proposers) internal returns (DevTEEProverRegistry) {
        MockAggregateVerifierForRegistry verifier = new MockAggregateVerifierForRegistry();
        address factory = makeAddr("factory");
        vm.etch(factory, hex"00");
        vm.mockCall(
            factory, abi.encodeCall(IDisputeGameFactory.gameImpls, (TEST_GAME_TYPE)), abi.encode(address(verifier))
        );

        DevTEEProverRegistry impl = new DevTEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(1)), IDisputeGameFactory(factory)
        );

        address proxyAdmin = makeAddr("proxy-admin");
        Proxy proxy = new Proxy(proxyAdmin);
        vm.prank(proxyAdmin);
        proxy.upgradeToAndCall(
            address(impl), abi.encodeCall(TEEProverRegistry.initialize, (owner, manager, proposers, TEST_GAME_TYPE))
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
        assertEq(teeProverRegistry.version(), "0.7.0");
    }

    function testInitializationWithProposers() public {
        address[] memory proposers = new address[](3);
        proposers[0] = makeAddr("proposer1");
        proposers[1] = makeAddr("proposer2");
        proposers[2] = makeAddr("proposer3");

        DevTEEProverRegistry registry2 = _deployRegistry(proposers);
        assertTrue(registry2.isValidProposer(proposers[0]));
        assertTrue(registry2.isValidProposer(proposers[1]));
        assertTrue(registry2.isValidProposer(proposers[2]));
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

        _expectNotOwnerOrManagerRevert(makeAddr("unauthorized"));
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

        _expectNotOwnerRevert(makeAddr("unauthorized"));
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
        assertTrue(teeProverRegistry.signerTEEType(signer) == TEEProverRegistry.TEEType.NITRO);
    }

    function testAddDevTDXSigner() public {
        address signer = makeAddr("dev-tdx-signer");

        vm.prank(owner);
        teeProverRegistry.addDevTDXSigner(signer, TEST_IMAGE_HASH);

        assertTrue(teeProverRegistry.isValidSigner(signer));
        assertTrue(teeProverRegistry.signerTEEType(signer) == TEEProverRegistry.TEEType.TDX);
    }

    function testRegisteringSameSignerWithDifferentTEETypeOverwritesTEEType() public {
        address signer = makeAddr("dev-signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        vm.prank(owner);
        teeProverRegistry.addDevTDXSigner(signer, TEST_IMAGE_HASH);

        assertTrue(teeProverRegistry.isRegisteredSigner(signer));
        assertTrue(teeProverRegistry.signerTEEType(signer) == TEEProverRegistry.TEEType.TDX);
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

        _expectNotOwnerRevert(makeAddr("unauthorized"));
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);
    }

    function testAddDevSignerIdempotent() public {
        address signer = makeAddr("dev-signer");

        _addDevSigner(signer);
        _addDevSigner(signer);

        assertTrue(teeProverRegistry.isValidSigner(signer));
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
}
