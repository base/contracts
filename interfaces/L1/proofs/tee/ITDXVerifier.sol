// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ZkCoProcessorType, ZkCoProcessorConfig } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";

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

/// @notice Public journal emitted by the off-chain/ZK TDX DCAP verifier.
/// @param result Overall verification result after quote and collateral validation in the guest.
/// @param tcbStatus Intel TDX TCB status for the platform.
/// @param timestamp Quote timestamp in milliseconds since Unix epoch.
/// @param collateralExpiration Earliest expiration timestamp in seconds across accepted collateral.
/// @param rootCaHash Hash of the Intel root CA used to validate the PCK/collateral signing chains.
/// @param pckCertHash Hash of the PCK leaf certificate that signed the quote attestation key chain.
/// @param tcbInfoHash Hash of the TCB info collateral consumed by the guest.
/// @param qeIdentityHash Hash of the QE identity collateral consumed by the guest.
/// @param publicKey Uncompressed secp256k1 public key: 0x04 || x || y.
/// @param signer Ethereum address derived from publicKey.
/// @param imageHash Multiproof-compatible image hash derived from MRTD and RTMR0-3.
/// @param mrTdHash keccak256 hash of the 48-byte MRTD measurement.
/// @param reportDataPrefix First 32 bytes of TDREPORT.REPORTDATA.
/// @param reportDataSuffix Last 32 bytes of TDREPORT.REPORTDATA, available for app-specific binding.
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
}

/// @title ITDXVerifier
/// @notice Interface for Intel TDX quote verification used by TDX-aware TEE prover registries.
interface ITDXVerifier {
    /// @notice Verifies a ZK proof of Intel TDX DCAP quote verification and returns attested signer metadata.
    /// @param output ABI-encoded TDXVerifierJournal public values from the ZK verifier guest.
    /// @param zkCoprocessor ZK proving system used to generate the proof.
    /// @param proofBytes ZK proof bytes.
    /// @return journal Verified TDX attestation metadata.
    function verify(
        bytes calldata output,
        ZkCoProcessorType zkCoprocessor,
        bytes calldata proofBytes
    )
        external
        returns (TDXVerifierJournal memory journal);

    /// @notice Retrieves the configuration for a specific coprocessor.
    function getZkConfig(ZkCoProcessorType zkCoprocessor) external view returns (ZkCoProcessorConfig memory);

    /// @notice Returns whether a TCB status is accepted by verifier policy.
    function allowedTcbStatuses(TDXTcbStatus status) external view returns (bool);

    /// @notice Updates the address authorized to submit verified proofs.
    function setProofSubmitter(address newProofSubmitter) external;
}
