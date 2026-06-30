// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

contract TEEVerifierTest is Test {
    TEEVerifier public verifier;
    address internal teeProverRegistry;

    uint256 internal constant NITRO_SIGNER_PRIVATE_KEY =
        0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    uint256 internal constant TDX_SIGNER_PRIVATE_KEY =
        0x2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;

    bytes32 internal constant NITRO_IMAGE_ID = keccak256("test-nitro-image-id");
    bytes32 internal constant TDX_IMAGE_ID = keccak256("test-tdx-image-id");
    address internal immutable PROPOSER = makeAddr("proposer");

    function setUp() public {
        teeProverRegistry = makeAddr("tee-prover-registry");
        vm.mockCall(teeProverRegistry, abi.encodeWithSignature("isValidProposer(address)", PROPOSER), abi.encode(true));
        _mockSigner(NITRO_SIGNER_PRIVATE_KEY, NITRO_IMAGE_ID, true);
        _mockSigner(TDX_SIGNER_PRIVATE_KEY, TDX_IMAGE_ID, true);

        verifier = new TEEVerifier(
            TEEProverRegistry(teeProverRegistry), IAnchorStateRegistry(makeAddr("anchor-state-registry"))
        );
    }

    function testVerifyValidNitroSignature() public view {
        bytes32 journal = keccak256("test-journal");
        bytes memory proofBytes = _buildProof(PROPOSER, _signature(NITRO_SIGNER_PRIVATE_KEY, journal));
        assertTrue(verifier.verify(proofBytes, NITRO_IMAGE_ID, journal));
    }

    function testVerifyValidTDXSignature() public view {
        bytes32 journal = keccak256("test-journal");
        bytes memory proofBytes = _buildProof(PROPOSER, _signature(TDX_SIGNER_PRIVATE_KEY, journal));
        assertTrue(verifier.verify(proofBytes, TDX_IMAGE_ID, journal));
    }

    function testVerifyFailsWithInvalidSignature() public {
        bytes32 journal = keccak256("test-journal");
        bytes memory invalidSignature = new bytes(65);
        invalidSignature[64] = bytes1(uint8(27)); // Set v to 27

        bytes memory proofBytes = _buildProof(PROPOSER, invalidSignature);

        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(proofBytes, NITRO_IMAGE_ID, journal);
    }

    function testVerifyFailsWithInvalidProposer() public {
        bytes32 journal = keccak256("test-journal");
        bytes memory proofBytes = _buildProof(address(0), _signature(NITRO_SIGNER_PRIVATE_KEY, journal));

        vm.mockCall(
            teeProverRegistry, abi.encodeWithSignature("isValidProposer(address)", address(0)), abi.encode(false)
        );
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(proofBytes, NITRO_IMAGE_ID, journal);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        uint256 unregisteredKey = 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef;
        address unregisteredSigner = vm.addr(unregisteredKey);

        bytes32 journal = keccak256("test-journal");

        bytes memory proofBytes = _buildProof(PROPOSER, _signature(unregisteredKey, journal));

        _mockSigner(unregisteredKey, bytes32(0), false);
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(proofBytes, NITRO_IMAGE_ID, journal);
    }

    function testVerifyFailsWithImageIdMismatch() public {
        bytes32 journal = keccak256("test-journal");

        bytes32 wrongImageId = keccak256("different-image");
        bytes memory proofBytes = _buildProof(PROPOSER, _signature(NITRO_SIGNER_PRIVATE_KEY, journal));
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.ImageIdMismatch.selector, NITRO_IMAGE_ID, wrongImageId));
        verifier.verify(proofBytes, wrongImageId, journal);
    }

    function testVerifyFailsWhenTEETypeIsIncludedInProof() public {
        bytes32 journal = keccak256("test-journal");
        bytes memory proofBytes = abi.encodePacked(PROPOSER, uint8(3), _signature(NITRO_SIGNER_PRIVATE_KEY, journal));

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(proofBytes, NITRO_IMAGE_ID, journal);
    }

    function testVerifyFailsWithInvalidProofFormat() public {
        bytes32 journal = keccak256("test-journal");

        bytes memory shortProof = new bytes(50);

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(shortProof, NITRO_IMAGE_ID, journal);
    }

    function _mockSigner(uint256 privateKey, bytes32 imageId, bool registered) internal {
        address signer = vm.addr(privateKey);
        vm.mockCall(
            teeProverRegistry, abi.encodeWithSignature("isRegisteredSigner(address)", signer), abi.encode(registered)
        );
        vm.mockCall(teeProverRegistry, abi.encodeWithSignature("signerImageHash(address)", signer), abi.encode(imageId));
    }

    function _signature(uint256 privateKey, bytes32 journal) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, journal);
        return abi.encodePacked(r, s, v);
    }

    function _buildProof(address proposer, bytes memory signature) internal pure returns (bytes memory) {
        return abi.encodePacked(proposer, signature);
    }
}
