// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { ITEEProverRegistry } from "interfaces/L1/proofs/tee/ITEEProverRegistry.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

contract TEEVerifierTest is Test {
    TEEVerifier public verifier;
    address internal teeProverRegistry;

    uint256 internal constant NITRO_SIGNER_PRIVATE_KEY =
        0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;

    bytes32 internal constant NITRO_IMAGE_ID = keccak256("test-nitro-image-id");
    bytes32 internal constant JOURNAL = keccak256("test-journal");
    address internal immutable PROPOSER = makeAddr("proposer");

    function setUp() public {
        teeProverRegistry = makeAddr("tee-prover-registry");
        vm.mockCall(teeProverRegistry, abi.encodeCall(ITEEProverRegistry.isValidProposer, (PROPOSER)), abi.encode(true));
        _mockSigner(NITRO_SIGNER_PRIVATE_KEY, NITRO_IMAGE_ID, true);

        verifier = new TEEVerifier(
            TEEProverRegistry(teeProverRegistry), IAnchorStateRegistry(makeAddr("anchor-state-registry"))
        );
    }

    function testVerifyValidNitroSignature() public view {
        bytes memory proofBytes = abi.encodePacked(PROPOSER, _signature(NITRO_SIGNER_PRIVATE_KEY, JOURNAL));
        assertTrue(verifier.verify(proofBytes, NITRO_IMAGE_ID, JOURNAL));
    }

    function testVerifyFailsWithInvalidSignature() public {
        bytes memory invalidSignature = abi.encodePacked(bytes32(0), bytes32(0), uint8(27));
        bytes memory proofBytes = abi.encodePacked(PROPOSER, invalidSignature);

        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(proofBytes, NITRO_IMAGE_ID, JOURNAL);
    }

    function testVerifyFailsWithInvalidProposer() public {
        bytes memory proofBytes = abi.encodePacked(address(0), _signature(NITRO_SIGNER_PRIVATE_KEY, JOURNAL));

        vm.mockCall(
            teeProverRegistry, abi.encodeCall(ITEEProverRegistry.isValidProposer, (address(0))), abi.encode(false)
        );
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(proofBytes, NITRO_IMAGE_ID, JOURNAL);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        uint256 unregisteredKey = 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef;
        address unregisteredSigner = vm.addr(unregisteredKey);

        bytes memory proofBytes = abi.encodePacked(PROPOSER, _signature(unregisteredKey, JOURNAL));

        _mockSigner(unregisteredKey, bytes32(0), false);
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(proofBytes, NITRO_IMAGE_ID, JOURNAL);
    }

    function testVerifyFailsWithImageIdMismatch() public {
        bytes32 wrongImageId = keccak256("different-image");
        bytes memory proofBytes = abi.encodePacked(PROPOSER, _signature(NITRO_SIGNER_PRIVATE_KEY, JOURNAL));
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.ImageIdMismatch.selector, NITRO_IMAGE_ID, wrongImageId));
        verifier.verify(proofBytes, wrongImageId, JOURNAL);
    }

    function testVerifyFailsWhenTEETypeIsIncludedInProof() public {
        bytes memory proofBytes = abi.encodePacked(PROPOSER, uint8(3), _signature(NITRO_SIGNER_PRIVATE_KEY, JOURNAL));

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(proofBytes, NITRO_IMAGE_ID, JOURNAL);
    }

    function _mockSigner(uint256 privateKey, bytes32 imageId, bool registered) internal {
        address signer = vm.addr(privateKey);
        vm.mockCall(
            teeProverRegistry, abi.encodeCall(ITEEProverRegistry.isRegisteredSigner, (signer)), abi.encode(registered)
        );
        vm.mockCall(
            teeProverRegistry, abi.encodeCall(ITEEProverRegistry.signerImageHash, (signer)), abi.encode(imageId)
        );
    }

    function _signature(uint256 privateKey, bytes32 journal) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, journal);
        return abi.encodePacked(r, s, v);
    }
}
