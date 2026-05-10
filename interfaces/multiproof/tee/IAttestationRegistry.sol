// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/// @title IAttestationRegistry
/// @notice Registry interface for tracking verified AWS Nitro Enclave attestations on Base L2
/// @dev Complements INitroEnclaveVerifier by providing a queryable record of past verifications
interface IAttestationRegistry {
    /// @notice Emitted when a new attestation record is registered
    /// @param imageId The enclave image ID that was verified
    /// @param submitter Address that submitted the attestation
    /// @param timestamp Block timestamp of registration
    event AttestationRegistered(bytes32 indexed imageId, address indexed submitter, uint256 timestamp);

    /// @notice Emitted when an attestation record is revoked
    /// @param imageId The enclave image ID that was revoked
    /// @param revokedBy Address that triggered the revocation
    event AttestationRevoked(bytes32 indexed imageId, address indexed revokedBy);

    /// @notice Registers a verified attestation on-chain
    /// @param imageId The verified enclave image ID
    /// @param proofHash Hash of the original proof for auditability
    function registerAttestation(bytes32 imageId, bytes32 proofHash) external;

    /// @notice Revokes a previously registered attestation
    /// @param imageId The enclave image ID to revoke
    function revokeAttestation(bytes32 imageId) external;

    /// @notice Returns registration details for a given imageId
    /// @param imageId The enclave image ID to query
    /// @return submitter Address that registered the attestation
    /// @return timestamp Block time of registration
    /// @return revoked Whether this attestation has been revoked
    function getAttestation(bytes32 imageId) external view returns (
        address submitter,
        uint256 timestamp,
        bool revoked
    );

    /// @notice Returns total number of registered attestations
    function attestationCount() external view returns (uint256);
}
