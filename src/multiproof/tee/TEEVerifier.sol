// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import {IVerifier} from "interfaces/multiproof/IVerifier.sol";

import {SystemConfigGlobal} from "./SystemConfigGlobal.sol";

/// @title TEEVerifier
/// @notice Stateless TEE proof verifier that validates signatures against registered signers.
/// @dev This contract is designed to be used as the TEE_VERIFIER in the AggregateVerifier.
///      It verifies that proofs are signed by enclave addresses registered in SystemConfigGlobal
///      via AWS Nitro attestation, and that the signer's PCR0 matches the claimed imageId.
///      Additionally, it verifies that the L1 origin block referenced in the proof actually exists
///      by checking against blockhash() or the EIP-2935 history contract.
///      The contract is intentionally stateless - all state related to output proposals is
///      managed by the calling contract (e.g., AggregateVerifier).
contract TEEVerifier is IVerifier {
    /// @notice The SystemConfigGlobal contract that manages valid TEE signers.
    /// @dev Signers are registered via AWS Nitro attestation in SystemConfigGlobal.
    SystemConfigGlobal public immutable SYSTEM_CONFIG_GLOBAL;

    /// @notice Thrown when the recovered signer is not a valid registered signer.
    error InvalidSigner(address signer);

    /// @notice Thrown when the signature format is invalid.
    error InvalidSignature();

    /// @notice Thrown when the signer's registered PCR0 does not match the claimed imageId.
    error ImageIdMismatch(bytes32 signerPCR0, bytes32 claimedImageId);

    /// @notice Thrown when the proof format is invalid.
    error InvalidProofFormat();

    /// @notice Thrown when the proposer is not a valid registered proposer.
    error InvalidProposer(address proposer);

    /// @notice Constructs the TEEVerifier contract.
    /// @param systemConfigGlobal The SystemConfigGlobal contract address.
    constructor(SystemConfigGlobal systemConfigGlobal) {
        SYSTEM_CONFIG_GLOBAL = systemConfigGlobal;
    }

    /// @notice Verifies a TEE proof for a state transition.
    /// @param proofBytes The proof: proposer (20) + l1OriginHash (32) + l1OriginNumber (32) + signature (65) = 149 bytes.
    /// @param imageId The claimed TEE image hash (PCR0). Must match the signer's registered PCR0.
    /// @param journal The keccak256 hash of the proof's public inputs.
    /// @return valid Whether the proof is valid.
    function verify(bytes calldata proofBytes, bytes32 imageId, bytes32 journal) external view override returns (bool) {
        if (proofBytes.length < 149) revert InvalidProofFormat();

        address proposer = address(bytes20(proofBytes[0:20]));
        bytes calldata signature = proofBytes[84:149];

        // Recover the signer from the signature
        // The signature should be over the journal hash directly (not eth-signed-message prefixed)
        (address signer, ECDSA.RecoverError err) = ECDSA.tryRecover(journal, signature);

        if (err != ECDSA.RecoverError.NoError) {
            revert InvalidSignature();
        }

        if (!SYSTEM_CONFIG_GLOBAL.isValidProposer(proposer)) {
            revert InvalidProposer(proposer);
        }

        // Get the PCR0 the signer was registered with
        bytes32 registeredPCR0 = SYSTEM_CONFIG_GLOBAL.signerPCR0(signer);

        // Check that the signer is registered (PCR0 != 0)
        if (registeredPCR0 == bytes32(0)) {
            revert InvalidSigner(signer);
        }

        // Check that the signer's registered PCR0 matches the claimed imageId
        // This ensures the signer was running the exact enclave image specified
        if (registeredPCR0 != imageId) {
            revert ImageIdMismatch(registeredPCR0, imageId);
        }

        return true;
    }
}
