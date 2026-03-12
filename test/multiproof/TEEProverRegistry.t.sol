// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";

import {
    INitroEnclaveVerifier
} from "lib/aws-nitro-enclave-attestation/contracts/src/interfaces/INitroEnclaveVerifier.sol";

import { DevTEEProverRegistry } from "src/multiproof/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/multiproof/tee/TEEProverRegistry.sol";

/// @notice Tests for TEEProverRegistry and DevTEEProverRegistry contracts.
/// @dev IMPORTANT: This test file uses DevTEEProverRegistry as the implementation because
/// registering signers on the production TEEProverRegistry requires a ZK proof of a valid
/// AWS Nitro attestation, which cannot be generated in a test environment. DevTEEProverRegistry extends
/// TEEProverRegistry with an `addDevSigner` function that bypasses attestation verification,
/// allowing us to test all signer-related functionality. All tests for base TEEProverRegistry
/// functionality (PCR0 management, ownership, proposer, etc.) are equally valid since
/// DevTEEProverRegistry inherits from TEEProverRegistry without modifying those functions.
contract TEEProverRegistryTest is Test {
    DevTEEProverRegistry public teeProverRegistry;
    ProxyAdmin public proxyAdmin;

    address public owner;
    address public manager;
    address public unauthorized;

    bytes public constant TEST_PCR0 = hex"abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    bytes32 public pcr0Hash;

    // Events must be redeclared here because Solidity 0.8.15 doesn't support
    // referencing events from other contracts via qualified names (requires 0.8.21+)
    event SignerRegistered(address indexed signer, bytes32 indexed pcr0);
    event SignerDeregistered(address indexed signer);
    event PCR0Registered(bytes32 indexed pcr0Hash);
    event PCR0Deregistered(bytes32 indexed pcr0Hash);
    event ProposerSet(address indexed proposer, bool isValid);

    function setUp() public {
        owner = makeAddr("owner");
        manager = makeAddr("manager");
        unauthorized = makeAddr("unauthorized");

        pcr0Hash = keccak256(TEST_PCR0);

        // Deploy implementation (using DevTEEProverRegistry for test flexibility)
        // NitroEnclaveVerifier is not needed since tests use addDevSigner(), so pass address(0).
        DevTEEProverRegistry impl = new DevTEEProverRegistry(INitroEnclaveVerifier(address(0)));

        // Deploy proxy admin
        proxyAdmin = new ProxyAdmin(address(this));

        // Deploy proxy
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl), address(proxyAdmin), abi.encodeCall(TEEProverRegistry.initialize, (owner, manager))
        );

        teeProverRegistry = DevTEEProverRegistry(address(proxy));
    }

    // ============ Initialization Tests ============

    function testInitialization() public view {
        assertEq(teeProverRegistry.owner(), owner);
        assertEq(teeProverRegistry.manager(), manager);
        assertEq(teeProverRegistry.version(), "0.2.0");
    }

    // ============ PCR0 Registration Tests ============

    function testRegisterPCR0() public {
        vm.expectEmit(true, false, false, false);
        emit PCR0Registered(pcr0Hash);

        vm.prank(owner);
        teeProverRegistry.registerPCR0(TEST_PCR0);

        assertTrue(teeProverRegistry.validPCR0s(pcr0Hash));
    }

    function testRegisterPCR0FailsIfNotOwner() public {
        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.registerPCR0(TEST_PCR0);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.registerPCR0(TEST_PCR0);
    }

    function testDeregisterPCR0() public {
        // First register
        vm.prank(owner);
        teeProverRegistry.registerPCR0(TEST_PCR0);
        assertTrue(teeProverRegistry.validPCR0s(pcr0Hash));

        // Then deregister
        vm.expectEmit(true, false, false, false);
        emit PCR0Deregistered(pcr0Hash);

        vm.prank(owner);
        teeProverRegistry.deregisterPCR0(TEST_PCR0);

        assertFalse(teeProverRegistry.validPCR0s(pcr0Hash));
    }

    function testDeregisterPCR0FailsIfNotOwner() public {
        vm.prank(owner);
        teeProverRegistry.registerPCR0(TEST_PCR0);

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.deregisterPCR0(TEST_PCR0);
    }

    // ============ Signer Deregistration Tests ============

    function testDeregisterSignerAsOwner() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("signer-pcr0");

        // Add signer via DevTEEProverRegistry
        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, signerPcr0);

        // Verify signer is registered
        assertTrue(teeProverRegistry.isValidSigner(signer));

        // Deregister as owner
        vm.expectEmit(true, false, false, false);
        emit SignerDeregistered(signer);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        assertFalse(teeProverRegistry.isValidSigner(signer));
        assertEq(teeProverRegistry.signerPCR0(signer), bytes32(0));
    }

    function testDeregisterSignerAsManager() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("signer-pcr0");

        // Add signer via DevTEEProverRegistry
        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, signerPcr0);

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
        bytes32 signerPcr0 = keccak256("signer-pcr0");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, signerPcr0);

        assertTrue(teeProverRegistry.isValidSigner(signer));
    }

    // ============ signerPCR0 Tests ============

    function testSignerPCR0ReturnsZeroForUnregistered() public {
        address unregistered = makeAddr("unregistered");
        assertEq(teeProverRegistry.signerPCR0(unregistered), bytes32(0));
    }

    function testSignerPCR0ReturnsCorrectValue() public {
        address signer = makeAddr("signer");
        bytes32 expectedPcr0 = keccak256("signer-pcr0");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, expectedPcr0);

        assertEq(teeProverRegistry.signerPCR0(signer), expectedPcr0);
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
        bytes32 devPcr0Hash = keccak256("dev-pcr0");

        vm.expectEmit(true, true, false, false);
        emit SignerRegistered(signer, devPcr0Hash);

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, devPcr0Hash);

        assertTrue(teeProverRegistry.isValidSigner(signer));
        assertEq(teeProverRegistry.signerPCR0(signer), devPcr0Hash);
    }

    function testAddDevSignerFailsIfNotOwner() public {
        address signer = makeAddr("dev-signer");
        bytes32 devPcr0Hash = keccak256("dev-pcr0");

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.addDevSigner(signer, devPcr0Hash);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        teeProverRegistry.addDevSigner(signer, devPcr0Hash);
    }

    function testAddDevSignerCanOverwriteExisting() public {
        address signer = makeAddr("dev-signer");
        bytes32 firstPcr0 = keccak256("first-pcr0");
        bytes32 secondPcr0 = keccak256("second-pcr0");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, firstPcr0);
        assertEq(teeProverRegistry.signerPCR0(signer), firstPcr0);

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, secondPcr0);
        assertEq(teeProverRegistry.signerPCR0(signer), secondPcr0);
    }

    function testAddDevSignerWithZeroPcr0() public {
        address signer = makeAddr("dev-signer");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, bytes32(0));

        // Signer should not be valid because PCR0 is zero
        assertFalse(teeProverRegistry.isValidSigner(signer));
        assertEq(teeProverRegistry.signerPCR0(signer), bytes32(0));
    }

    function testAddMultipleDevSigners() public {
        address signer1 = makeAddr("dev-signer-1");
        address signer2 = makeAddr("dev-signer-2");
        address signer3 = makeAddr("dev-signer-3");

        bytes32 pcr0Hash1 = keccak256("pcr0-1");
        bytes32 pcr0Hash2 = keccak256("pcr0-2");
        bytes32 pcr0Hash3 = keccak256("pcr0-3");

        vm.startPrank(owner);
        teeProverRegistry.addDevSigner(signer1, pcr0Hash1);
        teeProverRegistry.addDevSigner(signer2, pcr0Hash2);
        teeProverRegistry.addDevSigner(signer3, pcr0Hash3);
        vm.stopPrank();

        assertTrue(teeProverRegistry.isValidSigner(signer1));
        assertTrue(teeProverRegistry.isValidSigner(signer2));
        assertTrue(teeProverRegistry.isValidSigner(signer3));

        assertEq(teeProverRegistry.signerPCR0(signer1), pcr0Hash1);
        assertEq(teeProverRegistry.signerPCR0(signer2), pcr0Hash2);
        assertEq(teeProverRegistry.signerPCR0(signer3), pcr0Hash3);
    }

    // ============ getRegisteredSigners Tests ============

    function testGetRegisteredSignersEmpty() public view {
        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 0);
    }

    function testGetRegisteredSignersAfterRegister() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("pcr0");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, signerPcr0);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 1);
        assertEq(signers[0], signer);
    }

    function testGetRegisteredSignersAfterDeregister() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("pcr0");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, signerPcr0);

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

        bytes32 sharedPcr0 = keccak256("pcr0");

        vm.startPrank(owner);
        teeProverRegistry.addDevSigner(signer1, sharedPcr0);
        teeProverRegistry.addDevSigner(signer2, sharedPcr0);
        teeProverRegistry.addDevSigner(signer3, sharedPcr0);
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

        bytes32 sharedPcr0 = keccak256("pcr0");

        // Register three signers
        vm.startPrank(owner);
        teeProverRegistry.addDevSigner(signer1, sharedPcr0);
        teeProverRegistry.addDevSigner(signer2, sharedPcr0);
        teeProverRegistry.addDevSigner(signer3, sharedPcr0);
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
        assertEq(teeProverRegistry.signerPCR0(signer2), bytes32(0));
    }

    function testGetRegisteredSignersDeregisterIdempotent() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("pcr0");

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, signerPcr0);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        // Deregistering again should not revert and set should still be empty
        vm.prank(owner);
        teeProverRegistry.deregisterSigner(signer);

        assertEq(teeProverRegistry.getRegisteredSigners().length, 0);
    }
}
