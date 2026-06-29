// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Ownable } from "lib/solady/src/auth/Ownable.sol";
import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import {
    ITDXVerifier,
    TDXTcbStatus,
    TDXVerificationResult,
    TDXVerifierJournal
} from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

/// @title TDXVerifier
/// @notice Production-shape Intel TDX DCAP verifier for multiproof signer registration.
/// @dev The heavy TDX work is expected to happen in a ZK guest: quote signature verification, PCK chain
///      validation, TCB info validation, QE identity validation, CRL checks, and extraction of TDREPORT fields.
///      This contract verifies the ZK proof and enforces onchain policy over the verified journal.
contract TDXVerifier is Ownable, ITDXVerifier, ISemver {
    /// @notice Maximum accepted age of a TDX quote, in seconds.
    uint64 public immutable maxTimeDiff;

    /// @notice Address authorized to submit TDX proofs, expected to be the TDX-aware registry.
    address public proofSubmitter;

    /// @notice Hash of the trusted Intel root CA used by the ZK verifier guest.
    bytes32 public immutable rootCaHash;

    /// @notice RISC Zero verifier router.
    address public immutable riscZeroVerifier;

    /// @notice RISC Zero image ID for the TDX DCAP verifier guest.
    bytes32 public immutable verifierId;

    /// @notice Emitted when the proof submitter changes.
    event ProofSubmitterChanged(address indexed proofSubmitter);

    /// @notice Thrown when a zero maxTimeDiff is provided.
    error ZeroMaxTimeDiff();

    /// @notice Thrown when a zero address is provided for the proof submitter.
    error ZeroProofSubmitter();

    /// @notice Thrown when a zero root CA hash is provided.
    error ZeroRootCaHash();

    /// @notice Thrown when the caller is not the configured proof submitter.
    error CallerNotProofSubmitter();

    /// @notice Thrown when a zero RISC Zero verifier address is provided.
    error ZeroRiscZeroVerifier();

    /// @notice Thrown when a zero verifier ID is provided.
    error ZeroVerifierId();

    /// @notice Thrown when the TDX verifier guest did not report success.
    error TDXVerificationFailed(TDXVerificationResult result);

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

    /// @notice Thrown when the journal signer does not match the supplied public key.
    error SignerMismatch(address expected, address actual);

    /// @notice Thrown when TDREPORT.REPORTDATA does not bind the supplied public key.
    error ReportDataMismatch(bytes32 expected, bytes32 actual);

    constructor(
        address owner,
        uint64 initialMaxTimeDiff,
        bytes32 initialRootCaHash,
        address initialProofSubmitter,
        address initialRiscZeroVerifier,
        bytes32 initialVerifierId
    ) {
        _initializeOwner(owner);
        if (initialMaxTimeDiff == 0) revert ZeroMaxTimeDiff();
        if (initialRootCaHash == bytes32(0)) revert ZeroRootCaHash();
        if (initialRiscZeroVerifier == address(0)) revert ZeroRiscZeroVerifier();
        if (initialVerifierId == bytes32(0)) revert ZeroVerifierId();
        maxTimeDiff = initialMaxTimeDiff;
        rootCaHash = initialRootCaHash;
        riscZeroVerifier = initialRiscZeroVerifier;
        verifierId = initialVerifierId;
        _setProofSubmitter(initialProofSubmitter);
    }

    /// @notice Sets the proof submitter, expected to be the TDX-aware registry.
    function setProofSubmitter(address newProofSubmitter) external onlyOwner {
        _setProofSubmitter(newProofSubmitter);
    }

    /// @inheritdoc ITDXVerifier
    function allowedTcbStatuses(TDXTcbStatus status) public pure returns (bool) {
        return status == TDXTcbStatus.UpToDate || status == TDXTcbStatus.SwHardeningNeeded;
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
        if (msg.sender != proofSubmitter) revert CallerNotProofSubmitter();

        IRiscZeroVerifier(riscZeroVerifier).verify(proofBytes, verifierId, sha256(output));
        journal = abi.decode(output, (TDXVerifierJournal));
        _verifyJournal(journal);
    }

    /// @notice Semantic version.
    /// @custom:semver 0.3.0
    function version() public pure virtual returns (string memory) {
        return "0.3.0";
    }

    function _verifyJournal(TDXVerifierJournal memory journal) internal view {
        if (journal.result != TDXVerificationResult.Success) revert TDXVerificationFailed(journal.result);
        if (journal.rootCaHash != rootCaHash) {
            revert RootCaHashMismatch(rootCaHash, journal.rootCaHash);
        }
        if (!allowedTcbStatuses(journal.tcbStatus)) revert TcbStatusNotAllowed(journal.tcbStatus);
        if (journal.collateralExpiration <= block.timestamp) revert CollateralExpired(journal.collateralExpiration);

        uint64 timestamp = journal.timestamp / 1000;
        if (timestamp + maxTimeDiff <= block.timestamp || timestamp >= block.timestamp) {
            revert InvalidTimestamp(timestamp, block.timestamp);
        }

        bytes32 publicKeyHash = _derivePublicKeyHash(journal.publicKey);
        address signer = address(uint160(uint256(publicKeyHash)));
        if (journal.signer != signer) revert SignerMismatch(signer, journal.signer);
        if (journal.reportDataPrefix != publicKeyHash) {
            revert ReportDataMismatch(publicKeyHash, journal.reportDataPrefix);
        }
    }

    function _derivePublicKeyHash(bytes memory publicKey) internal pure returns (bytes32 publicKeyHash) {
        if (publicKey.length != 65 || publicKey[0] != 0x04) revert InvalidPublicKey();
        // Skip the 32-byte length word and the 0x04 uncompressed prefix; hash the 64-byte X||Y.
        assembly {
            publicKeyHash := keccak256(add(publicKey, 0x21), 64)
        }
    }

    function _setProofSubmitter(address newProofSubmitter) internal {
        if (newProofSubmitter == address(0)) revert ZeroProofSubmitter();
        proofSubmitter = newProofSubmitter;
        emit ProofSubmitterChanged(newProofSubmitter);
    }
}
