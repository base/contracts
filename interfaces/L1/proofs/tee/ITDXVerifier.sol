// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Result emitted by the Confidential Space TDX verifier guest.
enum TDXVerificationResult {
    Unknown,
    Success,
    TokenMalformed,
    TokenSignatureInvalid,
    RootCaNotTrusted,
    TokenClaimsInvalid,
    TokenExpired,
    TokenNonceMismatch
}

/// @notice Public journal emitted by the offchain/ZK Confidential Space verifier.
/// @param result Overall token verification result.
/// @param issuedAt Token issuance time in seconds since Unix epoch.
/// @param expiration Token expiration time in seconds since Unix epoch.
/// @param rootCaHash Hash of the Google Confidential Space root CA.
/// @param tokenLeafCertHash Hash of the token leaf certificate.
/// @param publicKey Uncompressed secp256k1 public key.
/// @param signer Ethereum address derived from `publicKey`.
/// @param imageHash OCI manifest digest for the prover workload.
/// @param audienceHash Hash of the token audience.
/// @param tokenNonceHash Hash of the signer-bound registrar nonce.
/// @param hardwareModelHash Hash of the token hardware model claim.
/// @param secureBoot Whether Secure Boot was enabled.
/// @param debugDisabled Whether the Confidential Space image was debug-disabled since boot.
/// @param commandOverride Whether the workload command was overridden.
/// @param environmentOverride Whether the workload environment was overridden.
/// @param chainId L1 chain ID bound into the token nonce.
/// @param registryAddress TEE prover registry address bound into the token nonce.
struct TDXVerifierJournal {
    TDXVerificationResult result;
    uint64 issuedAt;
    uint64 expiration;
    bytes32 rootCaHash;
    bytes32 tokenLeafCertHash;
    bytes publicKey;
    address signer;
    bytes32 imageHash;
    bytes32 audienceHash;
    bytes32 tokenNonceHash;
    bytes32 hardwareModelHash;
    bool secureBoot;
    bool debugDisabled;
    bool commandOverride;
    bool environmentOverride;
    uint64 chainId;
    address registryAddress;
}

/// @title ITDXVerifier
/// @notice Interface for Confidential Space TDX verification used by TDX-aware TEE prover registries.
interface ITDXVerifier {
    /// @notice Verifies a ZK proof of a Confidential Space Intel TDX token and returns signer metadata.
    /// @param output ABI-encoded TDXVerifierJournal public values from the ZK verifier guest.
    /// @param proofBytes ZK proof bytes.
    /// @return signer Ethereum address derived from the attested public key.
    /// @return imageHash OCI manifest digest for the prover workload.
    function verify(
        bytes calldata output,
        bytes calldata proofBytes
    )
        external
        view
        returns (address signer, bytes32 imageHash);
}
