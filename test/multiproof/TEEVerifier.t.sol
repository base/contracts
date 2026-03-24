// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";

import { INitroEnclaveVerifier } from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";
import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";
import { GameType } from "src/dispute/lib/Types.sol";

import { MockAnchorStateRegistry } from "scripts/multiproof/mocks/MockAnchorStateRegistry.sol";
import { DevTEEProverRegistry } from "src/multiproof/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/multiproof/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/multiproof/tee/TEEVerifier.sol";

contract TEEVerifierTest is Test {
    TEEVerifier public verifier;
    DevTEEProverRegistry public teeProverRegistry;
    ProxyAdmin public proxyAdmin;
    MockAnchorStateRegistry public anchorStateRegistry;

    // Test signer - we'll derive address from private key
    uint256 internal constant SIGNER_PRIVATE_KEY = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    address internal signerAddress;

    bytes32 internal constant IMAGE_ID = keccak256("test-image-id");
    address internal immutable PROPOSER = makeAddr("proposer");

    address internal owner;

    function setUp() public {
        owner = address(this);

        // Derive signer address from private key
        signerAddress = vm.addr(SIGNER_PRIVATE_KEY);

        // Deploy implementation (NitroEnclaveVerifier not needed for dev signer tests)
        DevTEEProverRegistry impl =
            new DevTEEProverRegistry(INitroEnclaveVerifier(address(0)), IDisputeGameFactory(address(1)));

        // Deploy proxy admin
        proxyAdmin = new ProxyAdmin(address(this));

        // Deploy proxy
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl),
            address(proxyAdmin),
            abi.encodeCall(TEEProverRegistry.initialize, (owner, owner, address(0), GameType.wrap(0)))
        );

        teeProverRegistry = DevTEEProverRegistry(address(proxy));

        // Register the signer
        teeProverRegistry.addDevSigner(signerAddress);

        // Set the proposer as valid
        teeProverRegistry.setProposer(PROPOSER, true);

        // Deploy TEEVerifier
        anchorStateRegistry = new MockAnchorStateRegistry();
        verifier = new TEEVerifier(
            TEEProverRegistry(address(teeProverRegistry)), IAnchorStateRegistry(address(anchorStateRegistry))
        );
    }

    function testVerifyValidSignature() public view {
        // Create a journal hash
        bytes32 journal = keccak256("test-journal");

        // Sign the journal with the signer's private key
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Construct proof: proposer(20) + signature(65) = 85 bytes
        bytes memory proofBytes = abi.encodePacked(PROPOSER, signature);

        // Verify should return true regardless of imageId (enforced via journal hash, not registry)
        bool result = verifier.verify(proofBytes, IMAGE_ID, journal);
        assertTrue(result);
    }

    function testVerifyFailsWithInvalidSignature() public {
        bytes32 journal = keccak256("test-journal");

        // Create an invalid signature (all zeros except v)
        bytes memory invalidSignature = new bytes(65);
        invalidSignature[64] = bytes1(uint8(27)); // Set v to 27

        bytes memory proofBytes = abi.encodePacked(PROPOSER, invalidSignature);

        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithInvalidProposer() public {
        // Create a journal hash
        bytes32 journal = keccak256("test-journal");

        // Sign the journal with the signer's private key
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Construct proof: proposer(20) + signature(65) = 85 bytes
        bytes memory proofBytes = abi.encodePacked(address(0), signature);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        // Use a different private key that's not registered
        uint256 unregisteredKey = 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef;
        address unregisteredSigner = vm.addr(unregisteredKey);

        bytes32 journal = keccak256("test-journal");

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(unregisteredKey, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        bytes memory proofBytes = abi.encodePacked(PROPOSER, signature);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifySucceedsWithAnyImageId() public view {
        // imageId is no longer checked against the registry — it's enforced via journal hash
        bytes32 journal = keccak256("test-journal");

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        bytes memory proofBytes = abi.encodePacked(PROPOSER, signature);

        // Different imageId should still pass — registry doesn't check it
        bool result = verifier.verify(proofBytes, keccak256("different-image"), journal);
        assertTrue(result);
    }

    function testVerifyFailsWithInvalidProofFormat() public {
        bytes32 journal = keccak256("test-journal");

        // Proof too short (less than 85 bytes)
        bytes memory shortProof = new bytes(50);

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(shortProof, IMAGE_ID, journal);
    }

    function testConstants() public view {
        assertEq(address(verifier.TEE_PROVER_REGISTRY()), address(teeProverRegistry));
    }
}
