// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Statuses that a TDX quote/collateral verifier may emit.
/// @dev Unknown is index 0 so uninitialized values fail closed.
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

/// @notice Public journal emitted by the offchain/ZK TDX DCAP verifier.
/// @param result Overall verification result after quote and collateral validation in the guest.
/// @param tcbStatus Intel TDX TCB status for the platform.
/// @param timestamp Quote timestamp in milliseconds since Unix epoch.
/// @param collateralExpiration Earliest expiration timestamp in seconds across accepted collateral.
/// @param rootCaHash Hash of the Intel root CA used to validate the PCK/collateral signing chains.
/// @param publicKey Uncompressed secp256k1 public key: 0x04 || x || y.
/// @param signer Ethereum address derived from publicKey.
/// @param imageHash Multiproof-compatible image hash derived from MRTD and RTMR0-3.
/// @param reportDataPrefix First 32 bytes of TDREPORT.REPORTDATA.
/// @param reportDataSuffix Last 32 bytes of TDREPORT.REPORTDATA, available for app-specific binding.
struct TDXVerifierJournal {
    TDXVerificationResult result;
    TDXTcbStatus tcbStatus;
    uint64 timestamp;
    uint64 collateralExpiration;
    bytes32 rootCaHash;
    bytes publicKey;
    address signer;
    bytes32 imageHash;
    bytes32 reportDataPrefix;
    bytes32 reportDataSuffix;
}

/// @title ITDXVerifier
/// @notice Interface for Intel TDX quote verification used by TDX-aware TEE prover registries.
interface ITDXVerifier {
    /// @notice Address authorized to submit verified proofs.
    function proofSubmitter() external view returns (address);

    /// @notice Verifies a ZK proof of Intel TDX DCAP quote verification and returns attested signer metadata.
    /// @param output ABI-encoded TDXVerifierJournal public values from the ZK verifier guest.
    /// @param proofBytes ZK proof bytes.
    /// @return journal Verified TDX attestation metadata.
    function verify(
        bytes calldata output,
        bytes calldata proofBytes
    )
        external
        view
        returns (TDXVerifierJournal memory journal);
}
