// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import { ITDXVerifier, TDXTcbStatus, TDXVerifierJournal } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

/// @title TDXVerifier
/// @notice Production-shape Intel TDX DCAP verifier for multiproof signer registration.
/// @dev The heavy TDX work is expected to happen in a ZK guest: quote signature verification, PCK chain
///      validation, TCB info validation, QE identity validation, CRL checks, and extraction of TDREPORT fields.
///      This contract verifies the ZK proof and enforces onchain policy over the verified journal.
contract TDXVerifier is ITDXVerifier, ISemver {
    /// @notice Maximum accepted age of a TDX quote, in seconds.
    uint64 public immutable maxTimeDiff;

    /// @notice Hash of the trusted Intel root CA used by the ZK verifier guest.
    bytes32 public immutable rootCaHash;

    /// @notice RISC Zero verifier router.
    address public immutable riscZeroVerifier;

    /// @notice RISC Zero image ID for the TDX DCAP verifier guest.
    bytes32 public immutable verifierId;

    /// @notice Thrown when a zero constructor input is provided.
    error ZeroInput();

    /// @notice Thrown when the TDX verifier guest did not report success.
    error TDXVerificationFailed();

    /// @notice Thrown when the journal root does not match the trusted Intel root.
    error RootCaHashMismatch(bytes32 expected, bytes32 actual);

    /// @notice Thrown when the journal's TCB status is not allowed.
    error TcbStatusNotAllowed(TDXTcbStatus status);

    /// @notice Thrown when collateral consumed by the ZK guest is expired.
    error CollateralExpired(uint64 collateralExpiration);

    /// @notice Thrown when the quote timestamp is outside policy.
    error InvalidTimestamp(uint64 timestampSeconds, uint256 currentTimestamp);

    /// @notice Thrown when the public key is not an uncompressed secp256k1 public key.
    error InvalidPublicKey();

    /// @notice Thrown when TDREPORT.REPORTDATA does not bind the supplied public key.
    error ReportDataMismatch(bytes32 expected, bytes32 actual);

    constructor(
        uint64 initialMaxTimeDiff,
        bytes32 initialRootCaHash,
        address initialRiscZeroVerifier,
        bytes32 initialVerifierId
    ) {
        if (
            initialMaxTimeDiff == 0 || initialRootCaHash == bytes32(0) || initialRiscZeroVerifier == address(0)
                || initialVerifierId == bytes32(0)
        ) revert ZeroInput();
        maxTimeDiff = initialMaxTimeDiff;
        rootCaHash = initialRootCaHash;
        riscZeroVerifier = initialRiscZeroVerifier;
        verifierId = initialVerifierId;
    }

    /// @inheritdoc ITDXVerifier
    function verify(
        bytes calldata output,
        bytes calldata proofBytes
    )
        external
        view
        returns (TDXVerifierJournal memory journal)
    {
        IRiscZeroVerifier(riscZeroVerifier).verify(proofBytes, verifierId, sha256(output));
        journal = abi.decode(output, (TDXVerifierJournal));

        if (!journal.success) revert TDXVerificationFailed();
        if (journal.rootCaHash != rootCaHash) {
            revert RootCaHashMismatch(rootCaHash, journal.rootCaHash);
        }
        if (journal.tcbStatus != TDXTcbStatus.UpToDate && journal.tcbStatus != TDXTcbStatus.SwHardeningNeeded) {
            revert TcbStatusNotAllowed(journal.tcbStatus);
        }
        if (journal.collateralExpiration <= block.timestamp) revert CollateralExpired(journal.collateralExpiration);

        uint64 timestamp = journal.timestamp / 1000;
        if (timestamp + maxTimeDiff <= block.timestamp || timestamp >= block.timestamp) {
            revert InvalidTimestamp(timestamp, block.timestamp);
        }

        bytes memory publicKey = journal.publicKey;
        if (publicKey.length != 65 || publicKey[0] != 0x04) revert InvalidPublicKey();
        bytes32 publicKeyHash;
        assembly {
            publicKeyHash := keccak256(add(publicKey, 0x21), 64)
        }
        if (journal.reportDataPrefix != publicKeyHash) {
            revert ReportDataMismatch(publicKeyHash, journal.reportDataPrefix);
        }

        journal.signer = address(uint160(uint256(publicKeyHash)));
    }

    /// @notice Semantic version.
    /// @custom:semver 0.3.0
    function version() public pure virtual returns (string memory) {
        return "0.3.0";
    }
}
