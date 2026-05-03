// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Ownable } from "@solady/auth/Ownable.sol";
import { ISP1Verifier } from "lib/sp1-contracts/contracts/src/ISP1Verifier.sol";
import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import {
    ITDXVerifier,
    TDXTcbStatus,
    TDXVerificationResult,
    TDXVerifierJournal,
    ZkCoProcessorType,
    ZkCoProcessorConfig
} from "interfaces/multiproof/tee/ITDXVerifier.sol";

/// @title TDXVerifier
/// @notice Production-shape Intel TDX DCAP verifier for multiproof signer registration.
/// @dev The heavy TDX work is expected to happen in a ZK guest: quote signature verification, PCK chain
///      validation, TCB info validation, QE identity validation, CRL checks, and extraction of TDREPORT fields.
///      This contract verifies the ZK proof and enforces on-chain policy over the verified journal.
contract TDXVerifier is Ownable, ITDXVerifier, ISemver {
    /// @notice Conversion factor from milliseconds to seconds.
    uint256 private constant MS_PER_SECOND = 1000;

    /// @notice Maximum accepted age of a TDX quote, in seconds.
    uint64 public maxTimeDiff;

    /// @notice Address authorized to submit TDX proofs, expected to be the TDX-aware registry.
    address public proofSubmitter;

    /// @notice Hash of the trusted Intel root CA used by the ZK verifier guest.
    bytes32 public rootCaHash;

    /// @notice Configuration mapping for each supported ZK coprocessor type.
    mapping(ZkCoProcessorType => ZkCoProcessorConfig) public zkConfig;

    /// @inheritdoc ITDXVerifier
    mapping(TDXTcbStatus => bool) public allowedTcbStatuses;

    /// @notice Emitted when the trusted Intel root CA hash changes.
    event RootCaHashUpdated(bytes32 indexed rootCaHash);

    /// @notice Emitted when the proof submitter changes.
    event ProofSubmitterChanged(address indexed proofSubmitter);

    /// @notice Emitted when the quote timestamp tolerance changes.
    event MaxTimeDiffUpdated(uint64 maxTimeDiff);

    /// @notice Emitted when a TCB status policy changes.
    event TcbStatusPolicyUpdated(TDXTcbStatus indexed status, bool allowed);

    /// @notice Emitted when ZK configuration changes.
    event ZKConfigurationUpdated(ZkCoProcessorType indexed zkCoprocessor, ZkCoProcessorConfig config);

    /// @notice Emitted after a TDX attestation journal is accepted.
    event AttestationSubmitted(
        address indexed signer,
        bytes32 indexed imageHash,
        TDXTcbStatus indexed tcbStatus,
        bytes32 pckCertHash,
        bytes32 tcbInfoHash,
        bytes32 qeIdentityHash
    );

    /// @notice Thrown when a zero maxTimeDiff is provided.
    error ZeroMaxTimeDiff();

    /// @notice Thrown when a zero address is provided for the proof submitter.
    error ZeroProofSubmitter();

    /// @notice Thrown when a zero root CA hash is provided.
    error ZeroRootCaHash();

    /// @notice Thrown when the caller is not the configured proof submitter.
    error CallerNotProofSubmitter();

    /// @notice Thrown when the ZK coprocessor type is unknown.
    error UnknownZkCoprocessor();

    /// @notice Thrown when the configured ZK verifier address is zero.
    error ZkVerifierNotConfigured(ZkCoProcessorType zkCoprocessor);

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
        ZkCoProcessorType zkCoprocessor,
        ZkCoProcessorConfig memory config,
        TDXTcbStatus[] memory initialAllowedTcbStatuses
    ) {
        _initializeOwner(owner);
        _setMaxTimeDiff(initialMaxTimeDiff);
        _setRootCaHash(initialRootCaHash);
        _setProofSubmitter(initialProofSubmitter);
        _setZkConfiguration(zkCoprocessor, config);

        for (uint256 i = 0; i < initialAllowedTcbStatuses.length; i++) {
            _setTcbStatusAllowed(initialAllowedTcbStatuses[i], true);
        }
    }

    /// @inheritdoc ITDXVerifier
    function getZkConfig(ZkCoProcessorType zkCoprocessor) external view returns (ZkCoProcessorConfig memory) {
        return zkConfig[zkCoprocessor];
    }

    /// @notice Sets the trusted Intel root CA hash.
    function setRootCaHash(bytes32 newRootCaHash) external onlyOwner {
        _setRootCaHash(newRootCaHash);
    }

    /// @notice Sets the proof submitter, expected to be the TDX-aware registry.
    function setProofSubmitter(address newProofSubmitter) external onlyOwner {
        _setProofSubmitter(newProofSubmitter);
    }

    /// @notice Sets maximum allowed quote age, in seconds.
    function setMaxTimeDiff(uint64 newMaxTimeDiff) external onlyOwner {
        _setMaxTimeDiff(newMaxTimeDiff);
    }

    /// @notice Sets whether a TDX TCB status is accepted.
    function setTcbStatusAllowed(TDXTcbStatus status, bool allowed) external onlyOwner {
        _setTcbStatusAllowed(status, allowed);
    }

    /// @notice Configures a ZK verifier/program for a coprocessor.
    function setZkConfiguration(ZkCoProcessorType zkCoprocessor, ZkCoProcessorConfig memory config) external onlyOwner {
        _setZkConfiguration(zkCoprocessor, config);
    }

    /// @inheritdoc ITDXVerifier
    function verify(
        bytes calldata output,
        ZkCoProcessorType zkCoprocessor,
        bytes calldata proofBytes
    )
        external
        returns (TDXVerifierJournal memory journal)
    {
        if (msg.sender != proofSubmitter) revert CallerNotProofSubmitter();

        _verifyZk(zkCoprocessor, output, proofBytes);
        journal = abi.decode(output, (TDXVerifierJournal));
        _verifyJournal(journal);

        emit AttestationSubmitted(
            journal.signer,
            journal.imageHash,
            journal.tcbStatus,
            journal.pckCertHash,
            journal.tcbInfoHash,
            journal.qeIdentityHash
        );
    }

    /// @notice Semantic version.
    /// @custom:semver 0.2.0
    function version() public pure virtual returns (string memory) {
        return "0.2.0";
    }

    function _verifyJournal(TDXVerifierJournal memory journal) internal view {
        if (journal.result != TDXVerificationResult.Success) revert TDXVerificationFailed(journal.result);
        bytes32 expectedRootCaHash = rootCaHash;
        if (journal.rootCaHash != expectedRootCaHash) revert RootCaHashMismatch(expectedRootCaHash, journal.rootCaHash);
        if (!allowedTcbStatuses[journal.tcbStatus]) revert TcbStatusNotAllowed(journal.tcbStatus);
        if (journal.collateralExpiration <= block.timestamp) revert CollateralExpired(journal.collateralExpiration);

        uint64 timestamp = journal.timestamp / uint64(MS_PER_SECOND);
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

    function _verifyZk(
        ZkCoProcessorType zkCoprocessor,
        bytes calldata output,
        bytes calldata proofBytes
    )
        internal
        view
    {
        ZkCoProcessorConfig memory config = zkConfig[zkCoprocessor];
        if (config.zkVerifier == address(0)) revert ZkVerifierNotConfigured(zkCoprocessor);

        if (zkCoprocessor == ZkCoProcessorType.RiscZero) {
            IRiscZeroVerifier(config.zkVerifier).verify(proofBytes, config.verifierId, sha256(output));
        } else if (zkCoprocessor == ZkCoProcessorType.Succinct) {
            ISP1Verifier(config.zkVerifier).verifyProof(config.verifierId, output, proofBytes);
        } else {
            revert UnknownZkCoprocessor();
        }
    }

    function _derivePublicKeyHash(bytes memory publicKey) internal pure returns (bytes32 publicKeyHash) {
        if (publicKey.length != 65 || publicKey[0] != 0x04) revert InvalidPublicKey();
        // Skip the 32-byte length word and the 0x04 uncompressed prefix; hash the 64-byte X||Y.
        assembly {
            publicKeyHash := keccak256(add(publicKey, 0x21), 64)
        }
    }

    function _setRootCaHash(bytes32 newRootCaHash) internal {
        if (newRootCaHash == bytes32(0)) revert ZeroRootCaHash();
        rootCaHash = newRootCaHash;
        emit RootCaHashUpdated(newRootCaHash);
    }

    function _setProofSubmitter(address newProofSubmitter) internal {
        if (newProofSubmitter == address(0)) revert ZeroProofSubmitter();
        proofSubmitter = newProofSubmitter;
        emit ProofSubmitterChanged(newProofSubmitter);
    }

    function _setMaxTimeDiff(uint64 newMaxTimeDiff) internal {
        if (newMaxTimeDiff == 0) revert ZeroMaxTimeDiff();
        maxTimeDiff = newMaxTimeDiff;
        emit MaxTimeDiffUpdated(newMaxTimeDiff);
    }

    function _setTcbStatusAllowed(TDXTcbStatus status, bool allowed) internal {
        allowedTcbStatuses[status] = allowed;
        emit TcbStatusPolicyUpdated(status, allowed);
    }

    function _setZkConfiguration(ZkCoProcessorType zkCoprocessor, ZkCoProcessorConfig memory config) internal {
        zkConfig[zkCoprocessor] = config;
        emit ZKConfigurationUpdated(zkCoprocessor, config);
    }
}
