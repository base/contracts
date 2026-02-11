// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {Test} from "forge-std/Test.sol";

import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import {ICertManager} from "@nitro-validator/ICertManager.sol";

import {DevSystemConfigGlobal} from "src/tee/DevSystemConfigGlobal.sol";
import {SystemConfigGlobal} from "src/tee/SystemConfigGlobal.sol";

import {MockCertManager} from "src/mocks/MockCertManager.sol";

/// @notice Tests for SystemConfigGlobal and DevSystemConfigGlobal contracts.
/// @dev IMPORTANT: This test file uses DevSystemConfigGlobal as the implementation because
/// registering signers on the production SystemConfigGlobal requires valid AWS Nitro attestation
/// documents, which cannot be generated in a test environment. DevSystemConfigGlobal extends
/// SystemConfigGlobal with an `addDevSigner` function that bypasses attestation verification,
/// allowing us to test all signer-related functionality. All tests for base SystemConfigGlobal
/// functionality (PCR0 management, ownership, proposer, etc.) are equally valid since
/// DevSystemConfigGlobal inherits from SystemConfigGlobal without modifying those functions.
contract SystemConfigGlobalTest is Test {
    DevSystemConfigGlobal public systemConfigGlobal;
    MockCertManager public certManager;
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

        // Deploy mock cert manager
        certManager = new MockCertManager();

        // Deploy implementation (using DevSystemConfigGlobal for test flexibility)
        DevSystemConfigGlobal impl = new DevSystemConfigGlobal(ICertManager(address(certManager)));

        // Deploy proxy admin
        proxyAdmin = new ProxyAdmin();

        // Deploy proxy
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl), address(proxyAdmin), abi.encodeCall(SystemConfigGlobal.initialize, (owner, manager))
        );

        systemConfigGlobal = DevSystemConfigGlobal(address(proxy));
    }

    // ============ Initialization Tests ============

    function testInitialization() public view {
        assertEq(systemConfigGlobal.owner(), owner);
        assertEq(systemConfigGlobal.manager(), manager);
        assertEq(systemConfigGlobal.version(), "0.1.0");
    }

    // ============ PCR0 Registration Tests ============

    function testRegisterPCR0() public {
        vm.expectEmit(true, false, false, false);
        emit PCR0Registered(pcr0Hash);

        vm.prank(owner);
        systemConfigGlobal.registerPCR0(TEST_PCR0);

        assertTrue(systemConfigGlobal.validPCR0s(pcr0Hash));
    }

    function testRegisterPCR0FailsIfNotOwner() public {
        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.registerPCR0(TEST_PCR0);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.registerPCR0(TEST_PCR0);
    }

    function testDeregisterPCR0() public {
        // First register
        vm.prank(owner);
        systemConfigGlobal.registerPCR0(TEST_PCR0);
        assertTrue(systemConfigGlobal.validPCR0s(pcr0Hash));

        // Then deregister
        vm.expectEmit(true, false, false, false);
        emit PCR0Deregistered(pcr0Hash);

        vm.prank(owner);
        systemConfigGlobal.deregisterPCR0(TEST_PCR0);

        assertFalse(systemConfigGlobal.validPCR0s(pcr0Hash));
    }

    function testDeregisterPCR0FailsIfNotOwner() public {
        vm.prank(owner);
        systemConfigGlobal.registerPCR0(TEST_PCR0);

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.deregisterPCR0(TEST_PCR0);
    }

    // ============ Signer Deregistration Tests ============

    function testDeregisterSignerAsOwner() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("signer-pcr0");

        // Add signer via DevSystemConfigGlobal
        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, signerPcr0);

        // Verify signer is registered
        assertTrue(systemConfigGlobal.isValidSigner(signer));

        // Deregister as owner
        vm.expectEmit(true, false, false, false);
        emit SignerDeregistered(signer);

        vm.prank(owner);
        systemConfigGlobal.deregisterSigner(signer);

        assertFalse(systemConfigGlobal.isValidSigner(signer));
        assertEq(systemConfigGlobal.signerPCR0(signer), bytes32(0));
    }

    function testDeregisterSignerAsManager() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("signer-pcr0");

        // Add signer via DevSystemConfigGlobal
        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, signerPcr0);

        assertTrue(systemConfigGlobal.isValidSigner(signer));

        vm.prank(manager);
        systemConfigGlobal.deregisterSigner(signer);

        assertFalse(systemConfigGlobal.isValidSigner(signer));
    }

    function testDeregisterSignerFailsIfUnauthorized() public {
        address signer = makeAddr("signer");

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner or the manager");
        systemConfigGlobal.deregisterSigner(signer);
    }

    // ============ Proposer Tests ============

    function testSetProposer() public {
        address newProposer = makeAddr("proposer");

        vm.expectEmit(true, false, false, false);
        emit ProposerSet(newProposer, true);

        vm.prank(owner);
        systemConfigGlobal.setProposer(newProposer, true);

        assertTrue(systemConfigGlobal.isValidProposer(newProposer));
    }

    function testSetProposerFailsIfNotOwner() public {
        address newProposer = makeAddr("proposer");

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.setProposer(newProposer, true);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.setProposer(newProposer, true);
    }

    // ============ isValidSigner Tests ============

    function testIsValidSignerReturnsFalseForUnregistered() public {
        address unregistered = makeAddr("unregistered");
        assertFalse(systemConfigGlobal.isValidSigner(unregistered));
    }

    function testIsValidSignerReturnsTrueForRegistered() public {
        address signer = makeAddr("signer");
        bytes32 signerPcr0 = keccak256("signer-pcr0");

        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, signerPcr0);

        assertTrue(systemConfigGlobal.isValidSigner(signer));
    }

    // ============ signerPCR0 Tests ============

    function testSignerPCR0ReturnsZeroForUnregistered() public {
        address unregistered = makeAddr("unregistered");
        assertEq(systemConfigGlobal.signerPCR0(unregistered), bytes32(0));
    }

    function testSignerPCR0ReturnsCorrectValue() public {
        address signer = makeAddr("signer");
        bytes32 expectedPcr0 = keccak256("signer-pcr0");

        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, expectedPcr0);

        assertEq(systemConfigGlobal.signerPCR0(signer), expectedPcr0);
    }

    // ============ MAX_AGE Tests ============

    function testMaxAgeConstant() public view {
        assertEq(systemConfigGlobal.MAX_AGE(), 60 minutes);
    }

    // ============ Ownership Transfer Tests ============

    function testTransferOwnership() public {
        address newOwner = makeAddr("newOwner");

        vm.prank(owner);
        systemConfigGlobal.transferOwnership(newOwner);

        assertEq(systemConfigGlobal.owner(), newOwner);
    }

    function testTransferOwnershipFailsIfNotOwner() public {
        address newOwner = makeAddr("newOwner");

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.transferOwnership(newOwner);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.transferOwnership(newOwner);
    }

    function testTransferOwnershipFailsForZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("OwnableManaged: new owner is the zero address");
        systemConfigGlobal.transferOwnership(address(0));
    }

    // ============ Management Transfer Tests ============

    function testTransferManagementAsOwner() public {
        address newManager = makeAddr("newManager");

        vm.prank(owner);
        systemConfigGlobal.transferManagement(newManager);

        assertEq(systemConfigGlobal.manager(), newManager);
    }

    function testTransferManagementAsManager() public {
        address newManager = makeAddr("newManager");

        vm.prank(manager);
        systemConfigGlobal.transferManagement(newManager);

        assertEq(systemConfigGlobal.manager(), newManager);
    }

    function testTransferManagementFailsIfUnauthorized() public {
        address newManager = makeAddr("newManager");

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner or the manager");
        systemConfigGlobal.transferManagement(newManager);
    }

    function testTransferManagementFailsForZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("OwnableManaged: new manager is the zero address");
        systemConfigGlobal.transferManagement(address(0));
    }

    // ============ Renounce Tests ============

    function testRenounceOwnership() public {
        vm.prank(owner);
        systemConfigGlobal.renounceOwnership();

        assertEq(systemConfigGlobal.owner(), address(0));
    }

    function testRenounceManagementAsOwner() public {
        vm.prank(owner);
        systemConfigGlobal.renounceManagement();

        assertEq(systemConfigGlobal.manager(), address(0));
    }

    function testRenounceManagementAsManager() public {
        vm.prank(manager);
        systemConfigGlobal.renounceManagement();

        assertEq(systemConfigGlobal.manager(), address(0));
    }

    // ============ DevSystemConfigGlobal: addDevSigner Tests ============

    function testAddDevSigner() public {
        address signer = makeAddr("dev-signer");
        bytes32 devPcr0Hash = keccak256("dev-pcr0");

        vm.expectEmit(true, true, false, false);
        emit SignerRegistered(signer, devPcr0Hash);

        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, devPcr0Hash);

        assertTrue(systemConfigGlobal.isValidSigner(signer));
        assertEq(systemConfigGlobal.signerPCR0(signer), devPcr0Hash);
    }

    function testAddDevSignerFailsIfNotOwner() public {
        address signer = makeAddr("dev-signer");
        bytes32 devPcr0Hash = keccak256("dev-pcr0");

        vm.prank(manager);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.addDevSigner(signer, devPcr0Hash);

        vm.prank(unauthorized);
        vm.expectRevert("OwnableManaged: caller is not the owner");
        systemConfigGlobal.addDevSigner(signer, devPcr0Hash);
    }

    function testAddDevSignerCanOverwriteExisting() public {
        address signer = makeAddr("dev-signer");
        bytes32 firstPcr0 = keccak256("first-pcr0");
        bytes32 secondPcr0 = keccak256("second-pcr0");

        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, firstPcr0);
        assertEq(systemConfigGlobal.signerPCR0(signer), firstPcr0);

        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, secondPcr0);
        assertEq(systemConfigGlobal.signerPCR0(signer), secondPcr0);
    }

    function testAddDevSignerWithZeroPcr0() public {
        address signer = makeAddr("dev-signer");

        vm.prank(owner);
        systemConfigGlobal.addDevSigner(signer, bytes32(0));

        // Signer should not be valid because PCR0 is zero
        assertFalse(systemConfigGlobal.isValidSigner(signer));
        assertEq(systemConfigGlobal.signerPCR0(signer), bytes32(0));
    }

    function testAddMultipleDevSigners() public {
        address signer1 = makeAddr("dev-signer-1");
        address signer2 = makeAddr("dev-signer-2");
        address signer3 = makeAddr("dev-signer-3");

        bytes32 pcr0Hash1 = keccak256("pcr0-1");
        bytes32 pcr0Hash2 = keccak256("pcr0-2");
        bytes32 pcr0Hash3 = keccak256("pcr0-3");

        vm.startPrank(owner);
        systemConfigGlobal.addDevSigner(signer1, pcr0Hash1);
        systemConfigGlobal.addDevSigner(signer2, pcr0Hash2);
        systemConfigGlobal.addDevSigner(signer3, pcr0Hash3);
        vm.stopPrank();

        assertTrue(systemConfigGlobal.isValidSigner(signer1));
        assertTrue(systemConfigGlobal.isValidSigner(signer2));
        assertTrue(systemConfigGlobal.isValidSigner(signer3));

        assertEq(systemConfigGlobal.signerPCR0(signer1), pcr0Hash1);
        assertEq(systemConfigGlobal.signerPCR0(signer2), pcr0Hash2);
        assertEq(systemConfigGlobal.signerPCR0(signer3), pcr0Hash3);
    }
}
