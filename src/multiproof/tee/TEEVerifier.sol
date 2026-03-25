// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";

import { TEEProverRegistry } from "./TEEProverRegistry.sol";
import { Verifier } from "../Verifier.sol";

/// @title TEEVerifier
/// @notice Stateless TEE proof verifier that validates signatures against registered signers.
/// @dev This contract is designed to be used as the TEE_VERIFIER in the AggregateVerifier.
///      It verifies that proofs are signed by enclave addresses registered in TEEProverRegistry
///      via AWS Nitro attestation. PCR0 (enclave image hash) enforcement is handled by
///      AggregateVerifier, which bakes TEE_IMAGE_HASH into the journal that the enclave signs.
///      The contract is intentionally stateless - all state related to output proposals and
///      L1 origin verification is managed by the calling contract (e.g., AggregateVerifier).
contract TEEVerifier is Verifier, ISemver {
    /// @notice The TEEProverRegistry contract that manages valid TEE signers.
    /// @dev Signers are registered via AWS Nitro attestation in TEEProverRegistry.
    TEEProverRegistry public immutable TEE_PROVER_REGISTRY;

    /// @notice Thrown when the recovered signer is not a valid registered signer.
    error InvalidSigner(address signer);

    /// @notice Thrown when the signer's registered image hash does not match the claimed imageId.
    error ImageIdMismatch(bytes32 signerImageHash, bytes32 claimedImageId);

    /// @notice Thrown when the signature format is invalid.
    error InvalidSignature();

    /// @notice Thrown when the proof format is invalid.
    error InvalidProofFormat();

    /// @notice Thrown when the proposer is not a valid registered proposer.
    error InvalidProposer(address proposer);

    /// @notice Constructs the TEEVerifier contract.
    /// @param teeProverRegistry The TEEProverRegistry contract address.
    constructor(
        TEEProverRegistry teeProverRegistry,
        IAnchorStateRegistry anchorStateRegistry
    )
        Verifier(anchorStateRegistry)
    {
        TEE_PROVER_REGISTRY = teeProverRegistry;
    }

    /// @notice Verifies a TEE proof for a state transition.
    /// @param proofBytes The proof: proposer(20) + signature(65) = 85 bytes.
    /// @param imageId The claimed TEE image hash (from the calling AggregateVerifier's TEE_IMAGE_HASH).
    ///        Validated against the signer's registered image hash to prevent cross-game-type attacks.
    /// @param journal The keccak256 hash of the proof's public inputs.
    /// @return valid Whether the proof is valid.
    function verify(
        bytes calldata proofBytes,
        bytes32 imageId,
        bytes32 journal
    )
        external
        view
        override
        notNullified
        returns (bool)
    {
        if (proofBytes.length < 85) revert InvalidProofFormat();

        address proposer = address(bytes20(proofBytes[0:20]));
        bytes calldata signature = proofBytes[20:85];

        // Recover the signer from the signature
        // The signature should be over the journal hash directly (not eth-signed-message prefixed)
        (address signer, ECDSA.RecoverError err) = ECDSA.tryRecover(journal, signature);

        if (err != ECDSA.RecoverError.NoError) {
            revert InvalidSignature();
        }

        if (!TEE_PROVER_REGISTRY.isValidProposer(proposer)) {
            revert InvalidProposer(proposer);
        }

        // Check that the signer is registered
        if (!TEE_PROVER_REGISTRY.isRegisteredSigner(signer)) {
            revert InvalidSigner(signer);
        }

        // Check that the signer's registered image hash matches the calling AggregateVerifier's imageId.
        // This prevents a signer registered under one enclave image from being used in a game
        // that expects a different image (e.g., after an upgrade or across game types).
        bytes32 registeredImageHash = TEE_PROVER_REGISTRY.signerImageHash(signer);
        if (registeredImageHash != imageId) {
            revert ImageIdMismatch(registeredImageHash, imageId);
        }

        return true;
    }

    /// @notice Semantic version.
    /// @custom:semver 0.2.0
    function version() public pure virtual returns (string memory) {
        return "0.2.0";
    }
}
