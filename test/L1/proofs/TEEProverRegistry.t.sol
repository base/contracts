// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";

import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

/// @notice Mock AggregateVerifier that returns configurable TEE image hashes.
contract MockAggregateVerifierForRegistry {
    bytes32 public TEE_NITRO_IMAGE_HASH;
    bytes32 public TEE_TDX_IMAGE_HASH;

    constructor(bytes32 imageHash) {
        TEE_NITRO_IMAGE_HASH = imageHash;
        TEE_TDX_IMAGE_HASH = imageHash;
    }
}

/// @notice Mock DisputeGameFactory that returns a fixed game implementation.
contract MockDisputeGameFactoryForRegistry {
    mapping(uint32 => address) internal _impls;

    function setImpl(uint32 gameType, address impl) external {
        _impls[gameType] = impl;
    }

    function gameImpls(GameType gameType) external view returns (IDisputeGame) {
        return IDisputeGame(_impls[GameType.unwrap(gameType)]);
    }
}

/// @notice Tests for TEEProverRegistry and DevTEEProverRegistry contracts.
/// @dev IMPORTANT: This test file uses DevTEEProverRegistry as the implementation because
/// registering signers on the production TEEProverRegistry requires a ZK proof of a valid
/// AWS Nitro attestation, which cannot be generated in a test environment. DevTEEProverRegistry extends
/// TEEProverRegistry with an `addDevSigner` function that bypasses attestation verification,
/// allowing us to test all signer-related functionality. All tests for base TEEProverRegistry
/// functionality (ownership, proposer, etc.) are equally valid since
/// DevTEEProverRegistry inherits from TEEProverRegistry without modifying those functions.
contract TEEProverRegistryTest is Test {
    DevTEEProverRegistry public teeProverRegistry;
    ProxyAdmin public proxyAdmin;
    MockDisputeGameFactoryForRegistry public mockFactory;
    MockAggregateVerifierForRegistry public mockVerifier;

    address public owner;
    address public manager;
    address public unauthorized;

    bytes32 public constant TEST_IMAGE_HASH = keccak256("test-image-hash");
    uint32 public constant TEST_GAME_TYPE = 621;

    // Events must be redeclared here because Solidity 0.8.15 doesn't support
    // referencing events from other contracts via qualified names (requires 0.8.21+)
    event SignerRegistered(address indexed signer);
    event SignerDeregistered(address indexed signer);
    event ProposerSet(address indexed proposer, bool isValid);

    function setUp() public {
        owner = makeAddr("owner");
        manager = makeAddr("manager");
        unauthorized = makeAddr("unauthorized");

        // Deploy mock factory and verifier
        mockVerifier = new MockAggregateVerifierForRegistry(TEST_IMAGE_HASH);
        mockFactory = new MockDisputeGameFactoryForRegistry();
        mockFactory.setImpl(TEST_GAME_TYPE, address(mockVerifier));

        // Deploy implementation (using DevTEEProverRegistry for test flexibility)
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
                TEEProverRegistry.initialize, (owner, manager, new address[](0), GameType.wrap(TEST_GAME_TYPE))
            )
        );

        teeProverRegistry = DevTEEProverRegistry(address(proxy));
    }

    // ============ Initialization Tests ============

    function testInitialization() public view {
        assertEq(teeProverRegistry.owner(), owner);
        assertEq(teeProverRegistry.manager(), manager);
        assertEq(teeProverRegistry.version(), "0.7.0");
    }

    function testInitializationWithProposers() public {
        address proposer1 = makeAddr("proposer1");
        address proposer2 = makeAddr("proposer2");
        address proposer3 = makeAddr("proposer3");
        DevTEEProverRegistry impl2 = new DevTEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(1)), IDisputeGameFactory(address(1))
        );
        ProxyAdmin proxyAdmin2 = new ProxyAdmin(address(this));
        address[] memory proposers = new address[](3);
        proposers[0] = proposer1;
        proposers[1] = proposer2;
        proposers[2] = proposer3;
        TransparentUpgradeableProxy proxy2 = new TransparentUpgradeableProxy(
            address(impl2),
            address(proxyAdmin2),
            abi.encodeCall(TEEProverRegistry.initialize, (owner, manager, proposers, GameType.wrap(0)))
        );
        DevTEEProverRegistry registry2 = DevTEEProverRegistry(address(proxy2));
        assertTrue(registry2.isValidProposer(proposer1));
        assertTrue(registry2.isValidProposer(proposer2));
        assertTrue(registry2.isValidProposer(proposer3));
    }

    function testInitializationWithEmptyProposers() public view {
        // Default setUp uses an empty proposer array — no proposer should be set
        assertFalse(teeProverRegistry.isValidProposer(address(0)));
    }

    // ============ Signer Deregistration Tests ============

    function testDeregisterSignerAsOwner() public {
        address signer = makeAddr("signer");

        // Add signer via DevTEEProverRegistry
        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        // Verify signer is registered
        assertTrue(teeProverRegistry.isValidSigner(signer));

        // Deregister as owner
        vm.expectEmit(true, false, false, false);
        emit SignerDeregistered(signer);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        assertFalse(teeProverRegistry.isValidSigner(signer));
    }

    function testDeregisterSignerAsManager() public {
        address signer = makeAddr("signer");

        // Add signer via DevTEEProverRegistry
        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        assertTrue(teeProverRegistry.isValidSigner(signer));

        vm.prank(manager);
        teeProverRegistry.deregisterSigner(signer);

        assertFalse(teeProverRegistry.isValidSigner(signer));
    }

    function testDeregisterSignerFailsIfUnauthorized() public {
        address signer = makeAddr("signer");

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner or the manager");
        teeProverRegistry.deregisterSigner(signer);
    }

    // ============ Proposer Tests ============

    function testSetProposer() public {
        address newProposer = makeAddr("proposer");

        vm.expectEmit(true, false, false, false);
        emit ProposerSet(newProposer, true);

        vm.prank(owner);
        teeProverRegistry.setProposer(newProposer, true);

        assertTrue(teeProverRegistry.isValidProposer(newProposer));
    }

    function testSetProposerFailsIfNotOwner() public {
        address newProposer = makeAddr("proposer");

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.setProposer(newProposer, true);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.setProposer(newProposer, true);
    }

    // ============ isValidSigner Tests ============

    function testIsValidSignerReturnsFalseForUnregistered() public {
        address unregistered = makeAddr("unregistered");
        assertFalse(teeProverRegistry.isValidSigner(unregistered));
    }

    function testIsValidSignerReturnsTrueForRegistered() public {
        address signer = makeAddr("signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        assertTrue(teeProverRegistry.isValidSigner(signer));
        assertEq(uint8(teeProverRegistry.signerTEEType(signer)), uint8(TEEProverRegistry.TEEType.NITRO));
    }

    function testAddDevTDXSigner() public {
        address signer = makeAddr("dev-tdx-signer");

        vm.prank(owner);
        teeProverRegistry.addDevTDXSigner(signer, TEST_IMAGE_HASH);

        assertTrue(teeProverRegistry.isValidSigner(signer));
        assertEq(uint8(teeProverRegistry.signerTEEType(signer)), uint8(TEEProverRegistry.TEEType.TDX));
    }

    function testRegisteringSameSignerWithDifferentTEETypeFails() public {
        address signer = makeAddr("dev-signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                TEEProverRegistry.SignerTEETypeMismatch.selector,
                signer,
                TEEProverRegistry.TEEType.NITRO,
                TEEProverRegistry.TEEType.TDX
            )
        );
        teeProverRegistry.addDevTDXSigner(signer, TEST_IMAGE_HASH);
    }

    // ============ MAX_AGE Tests ============

    function testMaxAgeConstant() public view {
        assertEq(teeProverRegistry.MAX_AGE(), 60 minutes);
    }

    // ============ Ownership Transfer Tests ============

    function testTransferOwnership() public {
        address newOwner = makeAddr("newOwner");

        vm.prank(owner);
        teeProverRegistry.transferOwnership(newOwner);

        assertEq(teeProverRegistry.owner(), newOwner);
    }

    function testTransferOwnershipFailsIfNotOwner() public {
        address newOwner = makeAddr("newOwner");

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.transferOwnership(newOwner);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.transferOwnership(newOwner);
    }

    function testTransferOwnershipFailsForZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("OwnableManaged: new owner is the zero address");
        teeProverRegistry.transferOwnership(address(0));
    }

    // ============ Management Transfer Tests ============

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

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner or the manager");
        teeProverRegistry.transferManagement(newManager);
    }

    function testTransferManagementFailsForZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("OwnableManaged: new manager is the zero address");
        teeProverRegistry.transferManagement(address(0));
    }

    // ============ Renounce Tests ============

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

    // ============ DevTEEProverRegistry: addDevSigner Tests ============

    function testAddDevSigner() public {
        address signer = makeAddr("dev-signer");

        vm.expectEmit(true, false, false, false);
        emit SignerRegistered(signer);

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        assertTrue(teeProverRegistry.isValidSigner(signer));
    }

    function testAddDevSignerFailsIfNotOwner() public {
        address signer = makeAddr("dev-signer");

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);
    }

    function testAddDevSignerIdempotent() public {
        address signer = makeAddr("dev-signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);
        assertTrue(teeProverRegistry.isValidSigner(signer));

        // Adding again should not revert
        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);
        assertTrue(teeProverRegistry.isValidSigner(signer));
    }

    function testAddMultipleDevSigners() public {
        address signer1 = makeAddr("dev-signer-1");
        address signer2 = makeAddr("dev-signer-2");
        address signer3 = makeAddr("dev-signer-3");

        vm.startPrank(owner);
        teeProverRegistry.addDevSigner(signer1, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer2, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer3, TEST_IMAGE_HASH);
        vm.stopPrank();

        assertTrue(teeProverRegistry.isValidSigner(signer1));
        assertTrue(teeProverRegistry.isValidSigner(signer2));
        assertTrue(teeProverRegistry.isValidSigner(signer3));
    }

    // ============ getRegisteredSigners Tests ============

    function testGetRegisteredSignersEmpty() public view {
        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 0);
    }

    function testGetRegisteredSignersAfterRegister() public {
        address signer = makeAddr("signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 1);
        assertEq(signers[0], signer);
    }

    function testGetRegisteredSignersAfterDeregister() public {
        address signer = makeAddr("signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        assertEq(teeProverRegistry.getRegisteredSigners().length, 1);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 0);
    }

    function testGetRegisteredSignersMultiple() public {
        address signer1 = makeAddr("signer-1");
        address signer2 = makeAddr("signer-2");
        address signer3 = makeAddr("signer-3");

        vm.startPrank(owner);
        teeProverRegistry.addDevSigner(signer1, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer2, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer3, TEST_IMAGE_HASH);
        vm.stopPrank();

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 3);

        // Verify all three are present (order not guaranteed)
        bool foundSigner1;
        bool foundSigner2;
        bool foundSigner3;
        for (uint256 i = 0; i < signers.length; i++) {
            if (signers[i] == signer1) foundSigner1 = true;
            if (signers[i] == signer2) foundSigner2 = true;
            if (signers[i] == signer3) foundSigner3 = true;
        }
        assertTrue(foundSigner1);
        assertTrue(foundSigner2);
        assertTrue(foundSigner3);
    }

    function testGetRegisteredSignersConsistencyAfterMixedOperations() public {
        address signer1 = makeAddr("signer-1");
        address signer2 = makeAddr("signer-2");
        address signer3 = makeAddr("signer-3");

        // Register three signers
        vm.startPrank(owner);
        teeProverRegistry.addDevSigner(signer1, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer2, TEST_IMAGE_HASH);
        teeProverRegistry.addDevSigner(signer3, TEST_IMAGE_HASH);
        vm.stopPrank();

        assertEq(teeProverRegistry.getRegisteredSigners().length, 3);

        // Deregister the middle one
        vm.prank(manager);
        teeProverRegistry.deregisterSigner(signer2);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 2);

        // Mapping and set stay consistent
        for (uint256 i = 0; i < signers.length; i++) {
            assertTrue(teeProverRegistry.isValidSigner(signers[i]));
            assertNotEq(signers[i], signer2);
        }

        // Deregistered signer not in mapping either
        assertFalse(teeProverRegistry.isValidSigner(signer2));
    }

    function testGetRegisteredSignersDeregisterIdempotent() public {
        address signer = makeAddr("signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        // Deregistering again should not revert and set should still be empty
        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        assertEq(teeProverRegistry.getRegisteredSigners().length, 0);
    }
}
