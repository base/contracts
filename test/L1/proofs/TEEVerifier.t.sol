// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

contract TEEVerifierTest is Test {
    TEEVerifier internal verifier;
    address internal immutable teeProverRegistry = makeAddr("tee-prover-registry");

    bytes32 internal constant NITRO_IMAGE_ID = keccak256("test-nitro-image-id");
    bytes32 internal constant JOURNAL = keccak256("test-journal");
    address internal immutable PROPOSER = makeAddr("proposer");

    function setUp() public {
        vm.mockCall(teeProverRegistry, abi.encodeWithSignature("isValidProposer(address)", PROPOSER), abi.encode(true));
        vm.mockCall(
            teeProverRegistry, abi.encodeWithSignature("isRegisteredSigner(address)", vm.addr(1)), abi.encode(true)
        );
        vm.mockCall(
            teeProverRegistry,
            abi.encodeWithSignature("signerImageHash(address)", vm.addr(1)),
            abi.encode(NITRO_IMAGE_ID)
        );

        verifier = new TEEVerifier(TEEProverRegistry(teeProverRegistry), IAnchorStateRegistry(address(0)));
    }

    function testVerifyValidNitroSignature() public view {
        assertTrue(verifier.verify(_proof(1), NITRO_IMAGE_ID, JOURNAL));
    }

    function testVerifyFailsWithInvalidSignature() public {
        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(abi.encodePacked(PROPOSER, new bytes(65)), NITRO_IMAGE_ID, JOURNAL);
    }

    function testVerifyFailsWithInvalidProposer() public {
        vm.mockCall(
            teeProverRegistry, abi.encodeWithSignature("isValidProposer(address)", address(0)), abi.encode(false)
        );
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(abi.encodePacked(address(0), new bytes(65)), NITRO_IMAGE_ID, JOURNAL);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        address unregisteredSigner = vm.addr(2);

        vm.mockCall(
            teeProverRegistry,
            abi.encodeWithSignature("isRegisteredSigner(address)", unregisteredSigner),
            abi.encode(false)
        );
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(_proof(2), NITRO_IMAGE_ID, JOURNAL);
    }

    function testVerifyFailsWithImageIdMismatch() public {
        bytes32 wrongImageId = keccak256("different-image");
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.ImageIdMismatch.selector, NITRO_IMAGE_ID, wrongImageId));
        verifier.verify(_proof(1), wrongImageId, JOURNAL);
    }

    function testVerifyFailsWhenTEETypeIsIncludedInProof() public {
        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(new bytes(86), NITRO_IMAGE_ID, JOURNAL);
    }

    function _proof(uint256 privateKey) internal view returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, JOURNAL);
        return abi.encodePacked(PROPOSER, r, s, v);
    }
}
