// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Intel TDX TCB status reduced to the statuses this contract's policy needs.
/// @dev Unknown is index 0 so uninitialized values fail closed.
enum TDXTcbStatus {
    Unknown,
    UpToDate,
    SwHardeningNeeded,
    ConfigurationNeeded,
    ConfigurationAndSwHardeningNeeded,
    OutOfDate,
    OutOfDateConfigurationNeeded,
    Revoked
}

/// @notice Result emitted by the TDX verifier guest.
enum TDXVerificationResult {
    Unknown,
    Success,
    InvalidQuote,
    QuoteSignatureInvalid,
    RootCaNotTrusted,
    PckCertChainInvalid,
    TcbInfoInvalid,
    QeIdentityInvalid,
    TcbStatusNotAllowed,
    CollateralExpired,
    InvalidTimestamp,
    ReportDataMismatch
}

/// @notice Public journal emitted by the offchain/ZK TDX DCAP verifier.
/// @param result Overall quote and collateral verification result.
/// @param tcbStatus Intel TDX TCB status for the platform.
/// @param timestamp Quote timestamp in milliseconds since Unix epoch.
/// @param collateralExpiration Earliest expiration timestamp in seconds across accepted collateral.
/// @param rootCaHash Hash of the Intel root CA used to validate the PCK/collateral signing chains.
/// @param pckCertHash Hash of the PCK leaf certificate.
/// @param tcbInfoHash Hash of the TCB info collateral.
/// @param qeIdentityHash Hash of the QE identity collateral.
/// @param publicKey Uncompressed secp256k1 public key.
/// @param signer Ethereum address derived from `publicKey`.
/// @param imageHash CI-derived OCI manifest digest for the prover workload.
/// @param mrTdHash Keccak256 hash of the MRTD measurement for diagnostics.
/// @param reportDataPrefix First 32 bytes of TDREPORT.REPORTDATA.
/// @param reportDataSuffix Last 32 bytes of TDREPORT.REPORTDATA.
/// @param tdAttributes TD attributes; the debug bit must be unset.
/// @param chainId L1 chain ID bound into report data.
/// @param registryAddress TEE prover registry address bound into report data.
struct TDXVerifierJournal {
    TDXVerificationResult result;
    TDXTcbStatus tcbStatus;
    uint64 timestamp;
    uint64 collateralExpiration;
    bytes32 rootCaHash;
    bytes32 pckCertHash;
    bytes32 tcbInfoHash;
    bytes32 qeIdentityHash;
    bytes publicKey;
    address signer;
    bytes32 imageHash;
    bytes32 mrTdHash;
    bytes32 reportDataPrefix;
    bytes32 reportDataSuffix;
    uint64 tdAttributes;
    uint64 chainId;
    address registryAddress;
}

/// @title ITDXVerifier
/// @notice Interface for Intel TDX quote verification used by TDX-aware TEE prover registries.
interface ITDXVerifier {
    /// @notice Verifies a ZK proof of Intel TDX DCAP quote verification and returns attested signer metadata.
    /// @param output ABI-encoded TDXVerifierJournal public values from the ZK verifier guest.
    /// @param proofBytes ZK proof bytes.
    /// @return signer Ethereum address derived from the attested public key.
    /// @return imageHash CI-derived OCI manifest digest for the prover workload.
    function verify(
        bytes calldata output,
        bytes calldata proofBytes
    )
        external
        view
        returns (address signer, bytes32 imageHash);
}
