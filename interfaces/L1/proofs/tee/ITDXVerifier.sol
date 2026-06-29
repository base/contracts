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

/// @notice Public journal emitted by the offchain/ZK TDX DCAP verifier.
/// @param tcbStatus Intel TDX TCB status for the platform.
/// @param timestamp Quote timestamp in milliseconds since Unix epoch.
/// @param collateralExpiration Earliest expiration timestamp in seconds across accepted collateral.
/// @param rootCaHash Hash of the Intel root CA used to validate the PCK/collateral signing chains.
/// @param publicKeyX secp256k1 public key x-coordinate.
/// @param publicKeyY secp256k1 public key y-coordinate.
/// @param imageHash Multiproof-compatible image hash derived from MRTD and RTMR0-3.
/// @param reportDataPrefix First 32 bytes of TDREPORT.REPORTDATA.
struct TDXVerifierJournal {
    TDXTcbStatus tcbStatus;
    uint64 timestamp;
    uint64 collateralExpiration;
    bytes32 rootCaHash;
    bytes32 publicKeyX;
    bytes32 publicKeyY;
    bytes32 imageHash;
    bytes32 reportDataPrefix;
}

/// @title ITDXVerifier
/// @notice Interface for Intel TDX quote verification used by TDX-aware TEE prover registries.
interface ITDXVerifier {
    /// @notice Verifies a ZK proof of Intel TDX DCAP quote verification and returns attested signer metadata.
    /// @param output ABI-encoded TDXVerifierJournal public values from the ZK verifier guest.
    /// @param proofBytes ZK proof bytes.
    /// @return signer Ethereum address derived from the attested public key.
    /// @return imageHash Multiproof-compatible image hash derived from MRTD and RTMR0-3.
    function verify(
        bytes calldata output,
        bytes calldata proofBytes
    )
        external
        view
        returns (address signer, bytes32 imageHash);
}
