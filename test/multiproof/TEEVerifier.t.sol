// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {Test} from "forge-std/Test.sol";

import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import {ICertManager} from "lib/nitro-validator/src/ICertManager.sol";

import {DevSystemConfigGlobal} from "src/multiproof/mocks/MockDevSystemConfigGlobal.sol";
import {SystemConfigGlobal} from "src/multiproof/tee/SystemConfigGlobal.sol";
import {TEEVerifier} from "src/multiproof/tee/TEEVerifier.sol";

import {MockCertManager} from "src/multiproof/mocks/MockCertManager.sol";

contract TEEVerifierTest is Test {
    TEEVerifier public verifier;
    DevSystemConfigGlobal public systemConfigGlobal;
    MockCertManager public certManager;
    ProxyAdmin public proxyAdmin;

    // Test signer - we'll derive address from private key
    uint256 internal constant SIGNER_PRIVATE_KEY = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    address internal signerAddress;

    bytes32 internal constant PCR0_HASH = keccak256("test-pcr0");
    bytes32 internal constant IMAGE_ID = PCR0_HASH; // imageId must match PCR0 hash
    address internal immutable PROPOSER = makeAddr("proposer");

    address internal owner;

    function setUp() public {
        owner = address(this);

        // Derive signer address from private key
        signerAddress = vm.addr(SIGNER_PRIVATE_KEY);

        // Deploy mock cert manager
        certManager = new MockCertManager();

        // Deploy implementation
        DevSystemConfigGlobal impl = new DevSystemConfigGlobal(ICertManager(address(certManager)));

        // Deploy proxy admin
        proxyAdmin = new ProxyAdmin();

        // Deploy proxy
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl), address(proxyAdmin), abi.encodeCall(SystemConfigGlobal.initialize, (owner, owner))
        );

        systemConfigGlobal = DevSystemConfigGlobal(address(proxy));

        // Register the signer with PCR0 hash
        systemConfigGlobal.addDevSigner(signerAddress, PCR0_HASH);

        // Set the proposer as valid
        systemConfigGlobal.setProposer(PROPOSER, true);

        // Deploy TEEVerifier
        verifier = new TEEVerifier(SystemConfigGlobal(address(systemConfigGlobal)));
    }

    function testVerifyValidSignature() public view {
        // Create a journal hash
        bytes32 journal = keccak256("test-journal");

        // Get current block info for L1 origin
        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        // Sign the journal with the signer's private key
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Construct proof: proposer (20) + l1OriginHash (32) + l1OriginNumber (32) + signature (65)
        bytes memory proofBytes = abi.encodePacked(PROPOSER, l1OriginHash, l1OriginNumber, signature);

        // Verify should return true
        bool result = verifier.verify(proofBytes, IMAGE_ID, journal);
        assertTrue(result);
    }

    function testVerifyFailsWithInvalidSignature() public {
        bytes32 journal = keccak256("test-journal");

        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        // Create an invalid signature (all zeros except v)
        bytes memory invalidSignature = new bytes(65);
        invalidSignature[64] = bytes1(uint8(27)); // Set v to 27

        bytes memory proofBytes = abi.encodePacked(PROPOSER, l1OriginHash, l1OriginNumber, invalidSignature);

        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithInvalidProposer() public {
        // Create a journal hash
        bytes32 journal = keccak256("test-journal");

        // Get current block info for L1 origin
        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        // Sign the journal with the signer's private key
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Construct proof: proposer (20) + l1OriginHash (32) + l1OriginNumber (32) + signature (65)
        bytes memory proofBytes = abi.encodePacked(address(0), l1OriginHash, l1OriginNumber, signature);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        // Use a different private key that's not registered
        uint256 unregisteredKey = 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef;
        address unregisteredSigner = vm.addr(unregisteredKey);

        bytes32 journal = keccak256("test-journal");

        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(unregisteredKey, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        bytes memory proofBytes = abi.encodePacked(PROPOSER, l1OriginHash, l1OriginNumber, signature);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithImageIdMismatch() public {
        bytes32 journal = keccak256("test-journal");

        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        bytes memory proofBytes = abi.encodePacked(PROPOSER, l1OriginHash, l1OriginNumber, signature);

        // Use a different imageId that doesn't match the registered PCR0
        bytes32 wrongImageId = keccak256("wrong-image-id");

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.ImageIdMismatch.selector, PCR0_HASH, wrongImageId));
        verifier.verify(proofBytes, wrongImageId, journal);
    }

    function testVerifyFailsWithInvalidProofFormat() public {
        bytes32 journal = keccak256("test-journal");

        // Proof too short (less than 129 bytes)
        bytes memory shortProof = new bytes(100);

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(shortProof, IMAGE_ID, journal);
    }

    function testConstants() public view {
        assertEq(address(verifier.SYSTEM_CONFIG_GLOBAL()), address(systemConfigGlobal));
    }
}
