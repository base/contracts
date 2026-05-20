// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import {
    TransparentUpgradeableProxy
} from "lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";

import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
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

/// @dev Uses DevTEEProverRegistry because production signer registration requires a Nitro attestation proof.
contract TEEProverRegistryTest is Test {
    DevTEEProverRegistry public teeProverRegistry;

    address public owner;
    address public manager;
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

        DevTEEProverRegistry impl =
            new DevTEEProverRegistry(INitroEnclaveVerifier(address(0)), IDisputeGameFactory(address(factory)));

        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl),
            makeAddr("proxy-admin"),
            abi.encodeCall(TEEProverRegistry.initialize, (owner, manager, proposers, gameType))
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
        assertEq(teeProverRegistry.version(), "0.5.0");
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
}
