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
    IRiscZeroVerifier public immutable riscZeroVerifier;

    /// @notice RISC Zero image ID for the TDX DCAP verifier guest.
    bytes32 public immutable verifierId;

    error ZeroInput();
    error RootCaHashMismatch(bytes32 expected, bytes32 actual);
    error TcbStatusNotAllowed(TDXTcbStatus status);
    error CollateralExpired(uint64 collateralExpiration);
    error InvalidTimestamp(uint64 timestampSeconds, uint256 currentTimestamp);
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
        riscZeroVerifier = IRiscZeroVerifier(initialRiscZeroVerifier);
        verifierId = initialVerifierId;
    }

    /// @inheritdoc ITDXVerifier
    function verify(
        bytes calldata output,
        bytes calldata proofBytes
    )
        external
        view
        returns (address signer, bytes32 imageHash)
    {
        riscZeroVerifier.verify(proofBytes, verifierId, sha256(output));
        TDXVerifierJournal memory journal = abi.decode(output, (TDXVerifierJournal));

        if (journal.rootCaHash != rootCaHash) revert RootCaHashMismatch(rootCaHash, journal.rootCaHash);
        if (journal.tcbStatus != TDXTcbStatus.UpToDate && journal.tcbStatus != TDXTcbStatus.SwHardeningNeeded) {
            revert TcbStatusNotAllowed(journal.tcbStatus);
        }
        if (journal.collateralExpiration <= block.timestamp) revert CollateralExpired(journal.collateralExpiration);

        uint64 timestamp = journal.timestamp / 1000;
        if (timestamp + maxTimeDiff <= block.timestamp || timestamp >= block.timestamp) {
            revert InvalidTimestamp(timestamp, block.timestamp);
        }

        bytes32 publicKeyHash = keccak256(abi.encodePacked(journal.publicKeyX, journal.publicKeyY));
        if (journal.reportDataPrefix != publicKeyHash) {
            revert ReportDataMismatch(publicKeyHash, journal.reportDataPrefix);
        }

        return (address(uint160(uint256(publicKeyHash))), journal.imageHash);
    }

    /// @notice Semantic version.
    /// @custom:semver 0.3.0
    function version() public pure virtual returns (string memory) {
        return "0.3.0";
    }
}
