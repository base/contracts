// SPDX-License-Identifier: Apache2.0
pragma solidity ^0.8.0;

import { Ownable } from "@solady/auth/Ownable.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {
    INitroEnclaveVerifier,
    ZkCoProcessorType,
    ZkCoProcessorConfig,
    VerifierJournal,
    BatchVerifierJournal,
    VerificationResult
} from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import { ISP1Verifier } from "lib/sp1-contracts/contracts/src/ISP1Verifier.sol";

/**
 * @title NitroEnclaveVerifier
 * @dev Implementation contract for AWS Nitro Enclave attestation verification using zero-knowledge proofs
 * @dev Custom version of Automata's NitroEnclaveVerifier contract at
 * https://github.com/automata-network/aws-nitro-enclave-attestation/tree/26c90565cb009e6539643a0956f9502a12ade672
 *
 * Differences from the upstream Automata contract:
 * - Verification of ZK proofs is restricted to an authorized proof submitter address
 * - All privileged actions emit events for monitoring
 * - Removes verification-with-explicit-program-ID and Pico logic
 *
 * This contract provides on-chain verification of AWS Nitro Enclave attestation reports by validating
 * zero-knowledge proofs generated off-chain. It supports both single and batch verification modes
 * and can work with multiple ZK proof systems (RISC Zero and Succinct SP1).
 *
 * Key features:
 * - Certificate chain management with automatic caching of newly discovered certificates
 * - Timestamp validation with configurable time tolerance
 * - Certificate revocation capabilities for compromised intermediate certificates
 * - Gas-efficient batch verification for multiple attestations
 * - Support for both RISC Zero and SP1 proving systems
 *
 * Security considerations:
 * - Only the contract owner can manage certificates and configurations
 * - Root certificate is immutable once set (requires owner to change)
 * - Intermediate certificates are automatically cached but can be revoked
 * - Timestamp validation prevents replay attacks within the configured time window
 */
contract NitroEnclaveVerifier is Ownable, INitroEnclaveVerifier {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// @dev Sentinel address to indicate a route has been permanently frozen
    address private constant FROZEN = address(0xdead);

    /// @dev Address that can submit proofs
    address public proofSubmitter;

    /// @dev Configuration mapping for each supported ZK coprocessor type
    mapping(ZkCoProcessorType => ZkCoProcessorConfig) public zkConfig;

    /// @dev Mapping of trusted intermediate certificate hashes (excludes root certificate)
    mapping(bytes32 trustedCertHash => bool) public trustedIntermediateCerts;

    /// @dev Maximum allowed time difference in seconds for attestation timestamp validation
    uint64 public maxTimeDiff;

    /// @dev Hash of the trusted AWS Nitro Enclave root certificate
    bytes32 public rootCert;

    /// @dev Set of all supported verifier program IDs per coprocessor
    mapping(ZkCoProcessorType => EnumerableSet.Bytes32Set) private _verifierIdSet;

    /// @dev Set of all supported aggregator program IDs per coprocessor
    mapping(ZkCoProcessorType => EnumerableSet.Bytes32Set) private _aggregatorIdSet;

    /// @dev Route-specific verifier overrides (selector -> verifier address)
    mapping(ZkCoProcessorType => mapping(bytes4 selector => address zkVerifier)) private _zkVerifierRoutes;

    /// @dev Mapping from verifierId to its corresponding verifierProofId representation
    mapping(ZkCoProcessorType => mapping(bytes32 verifierId => bytes32 verifierProofId)) private _verifierProofIds;

    // ============ Custom Errors ============

    /// @dev Error thrown when an unsupported or unknown ZK coprocessor type is used
    error Unknown_Zk_Coprocessor();

    /// @dev Error thrown when attempting to remove the currently active (latest) program ID
    error CannotRemoveLatestProgramId(ZkCoProcessorType zkCoProcessor, bytes32 identifier);

    /// @dev Error thrown when a ZK route has been permanently frozen
    error ZkRouteFrozen(ZkCoProcessorType zkCoProcessor, bytes4 selector);

    /// @dev Error thrown when no ZK verifier is configured for the coprocessor
    error ZkVerifierNotConfigured(ZkCoProcessorType zkCoProcessor);

    /// @dev Thrown when a caller other than the authorized proof submitter calls verify or batchVerify
    error CallerNotProofSubmitter();

    /// @dev Thrown when a certificate hash is not found in the trusted intermediate certificates set
    error CertificateNotFound(bytes32 certHash);

    /// @dev Thrown when a program ID argument is bytes32(0)
    error ZeroProgramId();

    /// @dev Thrown when attempting to set a program ID that is already the latest
    error ProgramIdAlreadyLatest(ZkCoProcessorType zkCoProcessor, bytes32 identifier);

    /// @dev Thrown when attempting to remove or operate on a program ID that does not exist in the set
    error ProgramIdNotFound(ZkCoProcessorType zkCoProcessor, bytes32 identifier);

    /// @dev Thrown when a zero address is provided where a verifier address is required
    error ZeroVerifierAddress();

    /// @dev Thrown when a zero address is provided for the proof submitter
    error ZeroProofSubmitter();

    /// @dev Thrown when the batch journal's verifier VK does not match the expected verifier proof ID
    error VerifierVkMismatch(bytes32 expected, bytes32 actual);

    /// @dev Thrown when the first certificate in a chain does not match the stored root certificate
    error RootCertMismatch(bytes32 expected, bytes32 actual);

    /// @dev Thrown when calling verifyWithProgramId or batchVerifyWithProgramId, which are intentionally disabled
    error NotImplemented();

    /// @dev Error thrown when a zero maxTimeDiff is provided
    error ZeroMaxTimeDiff();

    // ============ Events ============

    /// @dev Emitted when a new verifier program ID is added/updated
    event VerifierIdUpdated(ZkCoProcessorType indexed zkCoProcessor, bytes32 indexed newId, bytes32 newProofId);

    /// @dev Emitted when a new aggregator program ID is added/updated
    event AggregatorIdUpdated(ZkCoProcessorType indexed zkCoProcessor, bytes32 indexed newId);

    /// @dev Emitted when a program ID is removed from the supported set
    event ProgramIdRemoved(ZkCoProcessorType indexed zkCoProcessor, bytes32 indexed programId, bool isAggregator);

    /// @dev Emitted when a route-specific verifier is added
    event ZkRouteAdded(ZkCoProcessorType indexed zkCoProcessor, bytes4 indexed selector, address verifier);

    /// @dev Emitted when a route is permanently frozen
    event ZkRouteWasFrozen(ZkCoProcessorType indexed zkCoProcessor, bytes4 indexed selector);

    /// @dev Emitted when the proof of attestation has been successfully verified
    event AttestationSubmitted(VerificationResult result, ZkCoProcessorType zkCoProcessor, bytes output);

    /// @dev Emitted when a batched proof has been successfully verified; encodedBatched = abi.encode(VerifierJournal[])
    event BatchAttestationSubmitted(bytes32 verifierId, ZkCoProcessorType zkCoProcessor, bytes encodedBatch);

    /// @dev Event emitted when the proof submitter address is changed
    event ProofSubmitterChanged(address newProofSubmitter);

    /// @dev Event emitted when the root certificate is changed
    event RootCertChanged(bytes32 newRootCert);

    /// @dev Event emitted when the ZK configuration is updated
    event ZKConfigurationUpdated(ZkCoProcessorType zkCoProcessor, ZkCoProcessorConfig config, bytes32 verifierProofId);

    /// @dev Event emitted when a certificate is revoked
    event CertRevoked(bytes32 certHash);

    /// @dev Event emitted when the maximum time difference is updated
    event MaxTimeDiffUpdated(uint64 newMaxTimeDiff);

    /**
     * @dev Initializes the contract with owner, time tolerance and initial trusted certificates
     * @param _owner Address to be set as the contract owner
     * @param _maxTimeDiff Maximum time difference in seconds for timestamp validation
     * @param _initializeTrustedCerts Array of initial trusted intermediate certificate hashes
     *
     * Sets the provided address as the contract owner and initializes the trusted certificate set.
     * The root certificate must be set separately after deployment.
     */
    constructor(address _owner, uint64 _maxTimeDiff, bytes32[] memory _initializeTrustedCerts) {
        if (_maxTimeDiff == 0) revert ZeroMaxTimeDiff();
        maxTimeDiff = _maxTimeDiff;
        for (uint256 i = 0; i < _initializeTrustedCerts.length; i++) {
            trustedIntermediateCerts[_initializeTrustedCerts[i]] = true;
        }
        _initializeOwner(_owner);
    }

    // ============ Query Functions ============

    /**
     * @dev Retrieves the configuration for a specific coprocessor
     * @param _zkCoProcessor Type of ZK coprocessor (RiscZero or Succinct)
     * @return ZkCoProcessorConfig Configuration parameters including program IDs and verifier address
     */
    function getZkConfig(ZkCoProcessorType _zkCoProcessor) external view returns (ZkCoProcessorConfig memory) {
        return zkConfig[_zkCoProcessor];
    }

    /**
     * @dev Returns all supported verifier program IDs for a coprocessor
     * @param _zkCoProcessor Type of ZK coprocessor
     * @return Array of all supported verifier program IDs
     */
    function getVerifierIds(ZkCoProcessorType _zkCoProcessor) external view returns (bytes32[] memory) {
        return _verifierIdSet[_zkCoProcessor].values();
    }

    /**
     * @dev Returns all supported aggregator program IDs for a coprocessor
     * @param _zkCoProcessor Type of ZK coprocessor
     * @return Array of all supported aggregator program IDs
     */
    function getAggregatorIds(ZkCoProcessorType _zkCoProcessor) external view returns (bytes32[] memory) {
        return _aggregatorIdSet[_zkCoProcessor].values();
    }

    /**
     * @dev Checks if a verifier program ID is in the supported set
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _verifierId Verifier program ID to check
     * @return True if the ID is supported
     */
    function isVerifierIdSupported(ZkCoProcessorType _zkCoProcessor, bytes32 _verifierId) external view returns (bool) {
        return _verifierIdSet[_zkCoProcessor].contains(_verifierId);
    }

    /**
     * @dev Checks if an aggregator program ID is in the supported set
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _aggregatorId Aggregator program ID to check
     * @return True if the ID is supported
     */
    function isAggregatorIdSupported(
        ZkCoProcessorType _zkCoProcessor,
        bytes32 _aggregatorId
    )
        external
        view
        returns (bool)
    {
        return _aggregatorIdSet[_zkCoProcessor].contains(_aggregatorId);
    }

    /**
     * @dev Gets the verifier address for a specific route
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _selector Proof selector
     * @return Verifier address (route-specific or default fallback)
     */
    function getZkVerifier(ZkCoProcessorType _zkCoProcessor, bytes4 _selector) external view returns (address) {
        address verifier = _zkVerifierRoutes[_zkCoProcessor][_selector];

        if (verifier == FROZEN) {
            revert ZkRouteFrozen(_zkCoProcessor, _selector);
        }

        if (verifier == address(0)) {
            return zkConfig[_zkCoProcessor].zkVerifier;
        }

        return verifier;
    }

    /**
     * @dev Returns the verifierProofId for a given verifierId
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _verifierId The verifier program ID
     * @return The corresponding verifierProofId
     */
    function getVerifierProofId(ZkCoProcessorType _zkCoProcessor, bytes32 _verifierId) external view returns (bytes32) {
        return _verifierProofIds[_zkCoProcessor][_verifierId];
    }

    /**
     * @dev Checks the prefix length of trusted certificates in each provided certificate chain for reports
     * @param _report_certs Array of certificate chains, each containing certificate hashes
     * @return Array indicating the prefix length of trusted certificates in each chain
     *
     * For each certificate chain:
     * 1. Validates that the first certificate matches the stored root certificate
     * 2. Counts consecutive trusted certificates starting from the root
     * 3. Stops counting when an untrusted certificate is encountered
     *
     * This function is used to pre-validate certificate chains before generating proofs,
     * helping to optimize the proving process by determining trusted certificate lengths.
     * Usually called from off-chain
     */
    function checkTrustedIntermediateCerts(bytes32[][] calldata _report_certs) public view returns (uint8[] memory) {
        uint8[] memory results = new uint8[](_report_certs.length);
        bytes32 rootCertHash = rootCert;
        for (uint256 i = 0; i < _report_certs.length; i++) {
            bytes32[] calldata certs = _report_certs[i];
            uint8 trustedCertPrefixLen = 1;
            if (certs[0] != rootCertHash) {
                revert RootCertMismatch(rootCertHash, certs[0]);
            }
            for (uint256 j = 1; j < certs.length; j++) {
                if (!trustedIntermediateCerts[certs[j]]) {
                    break;
                }
                trustedCertPrefixLen += 1;
            }
            results[i] = trustedCertPrefixLen;
        }
        return results;
    }

    // ============ Admin Functions ============

    /**
     * @dev Sets the trusted root certificate hash
     * @param _rootCert Hash of the AWS Nitro Enclave root certificate
     *
     * Requirements:
     * - Only callable by contract owner
     *
     * The root certificate serves as the trust anchor for all certificate chain validations.
     * This should be set to the hash of AWS's root certificate for Nitro Enclaves.
     */
    function setRootCert(bytes32 _rootCert) external onlyOwner {
        rootCert = _rootCert;
        emit RootCertChanged(_rootCert);
    }

    /**
     * @dev Updates the maximum allowed time difference for attestation timestamp validation
     * @param _maxTimeDiff New maximum time difference in seconds
     *
     * Requirements:
     * - Only callable by contract owner
     * - Must be greater than zero
     */
    function setMaxTimeDiff(uint64 _maxTimeDiff) external onlyOwner {
        if (_maxTimeDiff == 0) revert ZeroMaxTimeDiff();
        maxTimeDiff = _maxTimeDiff;
        emit MaxTimeDiffUpdated(_maxTimeDiff);
    }

    /**
     * @dev Configures zero-knowledge verification parameters for a specific coprocessor
     * @param _zkCoProcessor Type of ZK coprocessor (RiscZero or Succinct)
     * @param _config Configuration parameters including program IDs and verifier address
     * @param _verifierProofId The verifierProofId corresponding to the verifierId in config
     *
     * Requirements:
     * - Only callable by contract owner
     *
     * This function sets up the necessary parameters for ZK proof verification:
     * - verifierId: Program ID for single attestation verification
     * - aggregatorId: Program ID for batch/aggregated verification
     * - zkVerifier: Address of the deployed ZK verifier contract
     *
     * Note: Program IDs are automatically added to the supported version sets
     * The verifierProofId is stored in a separate mapping (verifierId => verifierProofId)
     */
    function setZkConfiguration(
        ZkCoProcessorType _zkCoProcessor,
        ZkCoProcessorConfig memory _config,
        bytes32 _verifierProofId
    )
        external
        onlyOwner
    {
        zkConfig[_zkCoProcessor] = _config;

        // Auto-add program IDs to the version sets and store verifierProofId mapping
        if (_config.verifierId != bytes32(0)) {
            _verifierIdSet[_zkCoProcessor].add(_config.verifierId);
            _verifierProofIds[_zkCoProcessor][_config.verifierId] = _verifierProofId;
        }
        if (_config.aggregatorId != bytes32(0)) {
            _aggregatorIdSet[_zkCoProcessor].add(_config.aggregatorId);
        }
        emit ZKConfigurationUpdated(_zkCoProcessor, _config, _verifierProofId);
    }

    /**
     * @dev Revokes a trusted intermediate certificate
     * @param _certHash Hash of the certificate to revoke
     *
     * Requirements:
     * - Only callable by contract owner
     * - Certificate must exist in the trusted intermediate certificates set
     *
     * This function allows the owner to revoke compromised intermediate certificates
     * without affecting the root certificate or other trusted certificates.
     */
    function revokeCert(bytes32 _certHash) external onlyOwner {
        if (!trustedIntermediateCerts[_certHash]) {
            revert CertificateNotFound(_certHash);
        }
        delete trustedIntermediateCerts[_certHash];
        emit CertRevoked(_certHash);
    }

    /**
     * @dev Updates the verifier program ID, adding the new version to the supported set
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _newVerifierId New verifier program ID to set as latest
     * @param _newVerifierProofId New verifier proof ID (stored in mapping, used in batch verification)
     */
    function updateVerifierId(
        ZkCoProcessorType _zkCoProcessor,
        bytes32 _newVerifierId,
        bytes32 _newVerifierProofId
    )
        external
        onlyOwner
    {
        if (_newVerifierId == bytes32(0)) revert ZeroProgramId();
        if (zkConfig[_zkCoProcessor].verifierId == _newVerifierId) {
            revert ProgramIdAlreadyLatest(_zkCoProcessor, _newVerifierId);
        }

        zkConfig[_zkCoProcessor].verifierId = _newVerifierId;
        _verifierIdSet[_zkCoProcessor].add(_newVerifierId);
        _verifierProofIds[_zkCoProcessor][_newVerifierId] = _newVerifierProofId;

        emit VerifierIdUpdated(_zkCoProcessor, _newVerifierId, _newVerifierProofId);
    }

    /**
     * @dev Updates the aggregator program ID, adding the new version to the supported set
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _newAggregatorId New aggregator program ID to set as latest
     */
    function updateAggregatorId(ZkCoProcessorType _zkCoProcessor, bytes32 _newAggregatorId) external onlyOwner {
        if (_newAggregatorId == bytes32(0)) revert ZeroProgramId();
        if (zkConfig[_zkCoProcessor].aggregatorId == _newAggregatorId) {
            revert ProgramIdAlreadyLatest(_zkCoProcessor, _newAggregatorId);
        }

        zkConfig[_zkCoProcessor].aggregatorId = _newAggregatorId;
        _aggregatorIdSet[_zkCoProcessor].add(_newAggregatorId);

        emit AggregatorIdUpdated(_zkCoProcessor, _newAggregatorId);
    }

    /**
     * @dev Removes a verifier program ID from the supported set
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _verifierId Verifier program ID to remove
     */
    function removeVerifierId(ZkCoProcessorType _zkCoProcessor, bytes32 _verifierId) external onlyOwner {
        if (!_verifierIdSet[_zkCoProcessor].contains(_verifierId)) {
            revert ProgramIdNotFound(_zkCoProcessor, _verifierId);
        }

        // Cannot remove the latest verifier ID - must update to a new one first
        if (zkConfig[_zkCoProcessor].verifierId == _verifierId) {
            revert CannotRemoveLatestProgramId(_zkCoProcessor, _verifierId);
        }

        _verifierIdSet[_zkCoProcessor].remove(_verifierId);
        delete _verifierProofIds[_zkCoProcessor][_verifierId];
        emit ProgramIdRemoved(_zkCoProcessor, _verifierId, false);
    }

    /**
     * @dev Removes an aggregator program ID from the supported set
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _aggregatorId Aggregator program ID to remove
     */
    function removeAggregatorId(ZkCoProcessorType _zkCoProcessor, bytes32 _aggregatorId) external onlyOwner {
        if (!_aggregatorIdSet[_zkCoProcessor].contains(_aggregatorId)) {
            revert ProgramIdNotFound(_zkCoProcessor, _aggregatorId);
        }

        // Cannot remove the latest aggregator ID - must update to a new one first
        if (zkConfig[_zkCoProcessor].aggregatorId == _aggregatorId) {
            revert CannotRemoveLatestProgramId(_zkCoProcessor, _aggregatorId);
        }

        _aggregatorIdSet[_zkCoProcessor].remove(_aggregatorId);
        emit ProgramIdRemoved(_zkCoProcessor, _aggregatorId, true);
    }

    /**
     * @dev Adds a route-specific verifier override
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _selector Proof selector (first 4 bytes of proof data)
     * @param _verifier Address of the verifier contract for this route
     */
    function addVerifyRoute(ZkCoProcessorType _zkCoProcessor, bytes4 _selector, address _verifier) external onlyOwner {
        if (_verifier == address(0)) revert ZeroVerifierAddress();

        if (_zkVerifierRoutes[_zkCoProcessor][_selector] == FROZEN) {
            revert ZkRouteFrozen(_zkCoProcessor, _selector);
        }

        _zkVerifierRoutes[_zkCoProcessor][_selector] = _verifier;
        emit ZkRouteAdded(_zkCoProcessor, _selector, _verifier);
    }

    /**
     * @dev Permanently freezes a verification route
     * @param _zkCoProcessor Type of ZK coprocessor
     * @param _selector Proof selector to freeze
     *
     * WARNING: This action is IRREVERSIBLE
     */
    function freezeVerifyRoute(ZkCoProcessorType _zkCoProcessor, bytes4 _selector) external onlyOwner {
        address currentVerifier = _zkVerifierRoutes[_zkCoProcessor][_selector];

        if (currentVerifier == FROZEN) {
            revert ZkRouteFrozen(_zkCoProcessor, _selector);
        }

        _zkVerifierRoutes[_zkCoProcessor][_selector] = FROZEN;
        emit ZkRouteWasFrozen(_zkCoProcessor, _selector);
    }

    /**
     * @dev Sets the proof submitter address
     * @param _proofSubmitter The address of the proof submitter
     *
     * Requirements:
     * - Only callable by contract owner
     * - Address must not be zero
     */
    function setProofSubmitter(address _proofSubmitter) external onlyOwner {
        if (_proofSubmitter == address(0)) revert ZeroProofSubmitter();
        proofSubmitter = _proofSubmitter;
        emit ProofSubmitterChanged(_proofSubmitter);
    }

    // ============ Verification Functions ============

    /**
     * @dev Verifies a single attestation report using zero-knowledge proof
     * @param output Encoded VerifierJournal containing the verification result
     * @param zkCoprocessor Type of ZK coprocessor used to generate the proof
     * @param proofBytes Zero-knowledge proof data for the attestation
     * @return journal VerifierJournal containing the verification result and extracted data
     *
     * This function performs end-to-end verification of a single attestation:
     * 1. Retrieves the single verification program ID from configuration
     * 2. Verifies the zero-knowledge proof using the specified coprocessor
     * 3. Decodes the verification journal from the output
     * 4. Validates the journal through comprehensive checks
     * 5. Returns the final verification result
     *
     * The returned journal contains all extracted attestation data including:
     * - Verification status and any error conditions
     * - Certificate chain information and trust levels
     * - User data, nonce, and public key from the attestation
     * - Platform Configuration Registers (PCRs) for integrity measurement
     * - Module ID and timestamp information
     */
    function verify(
        bytes calldata output,
        ZkCoProcessorType zkCoprocessor,
        bytes calldata proofBytes
    )
        external
        returns (VerifierJournal memory journal)
    {
        if (msg.sender != proofSubmitter) revert CallerNotProofSubmitter();
        bytes32 programId = zkConfig[zkCoprocessor].verifierId;
        _verifyZk(zkCoprocessor, programId, output, proofBytes);
        journal = abi.decode(output, (VerifierJournal));
        journal = _verifyJournal(journal);
        emit AttestationSubmitted(journal.result, zkCoprocessor, output);
    }

    /**
     * @dev Verifies multiple attestation reports in a single batch operation
     * @param output Encoded BatchVerifierJournal containing aggregated verification results
     * @param zkCoprocessor Type of ZK coprocessor used to generate the proof
     * @param proofBytes Zero-knowledge proof data for batch verification
     * @return results Array of VerifierJournal results, one for each attestation in the batch
     *
     * This function provides gas-efficient batch verification by:
     * 1. Using the aggregator program ID for ZK proof verification
     * 2. Validating the batch verifier key matches the expected value
     * 3. Processing each individual attestation through standard validation
     * 4. Returning comprehensive results for all attestations
     *
     * Batch verification is recommended when processing multiple attestations
     * as it significantly reduces gas costs compared to individual verifications.
     */
    function batchVerify(
        bytes calldata output,
        ZkCoProcessorType zkCoprocessor,
        bytes calldata proofBytes
    )
        external
        returns (VerifierJournal[] memory results)
    {
        if (msg.sender != proofSubmitter) revert CallerNotProofSubmitter();
        bytes32 aggregatorId = zkConfig[zkCoprocessor].aggregatorId;
        bytes32 verifierId = zkConfig[zkCoprocessor].verifierId;
        bytes32 verifierProofId = _verifierProofIds[zkCoprocessor][verifierId];

        _verifyZk(zkCoprocessor, aggregatorId, output, proofBytes);
        BatchVerifierJournal memory batchJournal = abi.decode(output, (BatchVerifierJournal));
        if (batchJournal.verifierVk != verifierProofId) {
            revert VerifierVkMismatch(verifierProofId, batchJournal.verifierVk);
        }
        uint256 n = batchJournal.outputs.length;
        results = new VerifierJournal[](n);
        for (uint256 i = 0; i < n; i++) {
            results[i] = _verifyJournal(batchJournal.outputs[i]);
        }
        emit BatchAttestationSubmitted(verifierId, zkCoprocessor, abi.encode(results));
    }

    // ============ Internal Functions ============

    /**
     * @dev Internal function to cache newly discovered trusted certificates
     * @param journal Verification journal containing certificate chain information
     *
     * This function automatically adds any certificates beyond the trusted length
     * to the trusted intermediate certificates set. This optimizes future verifications
     * by expanding the known trusted certificate set based on successful verifications.
     */
    function _cacheNewCert(VerifierJournal memory journal) internal {
        for (uint256 i = journal.trustedCertsPrefixLen; i < journal.certs.length; i++) {
            bytes32 certHash = journal.certs[i];
            trustedIntermediateCerts[certHash] = true;
        }
    }

    /**
     * @dev Internal function to verify and validate a journal entry
     * @param journal Verification journal to validate
     * @return Updated journal with final verification result
     *
     * This function performs comprehensive validation:
     * 1. Checks if the initial ZK verification was successful
     * 2. Validates the root certificate matches the trusted root
     * 3. Ensures all trusted certificates are still valid (not revoked)
     * 4. Validates the attestation timestamp is within acceptable range
     * 5. Caches newly discovered certificates for future use
     *
     * The timestamp validation converts milliseconds to seconds and checks:
     * - Attestation is not too old (timestamp + maxTimeDiff >= block.timestamp)
     * - Attestation is not from the future (timestamp <= block.timestamp)
     */
    function _verifyJournal(VerifierJournal memory journal) internal returns (VerifierJournal memory) {
        if (journal.result != VerificationResult.Success) {
            return journal;
        }
        if (journal.trustedCertsPrefixLen == 0) {
            journal.result = VerificationResult.RootCertNotTrusted;
            return journal;
        }
        // Check every trusted certificate to ensure none have been revoked
        for (uint256 i = 0; i < journal.trustedCertsPrefixLen; i++) {
            bytes32 certHash = journal.certs[i];
            if (i == 0) {
                if (certHash != rootCert) {
                    journal.result = VerificationResult.RootCertNotTrusted;
                    return journal;
                }
                continue;
            }
            if (!trustedIntermediateCerts[certHash]) {
                journal.result = VerificationResult.IntermediateCertsNotTrusted;
                return journal;
            }
        }
        uint64 timestamp = journal.timestamp / 1000;
        if (timestamp + maxTimeDiff < block.timestamp || timestamp > block.timestamp) {
            journal.result = VerificationResult.InvalidTimestamp;
            return journal;
        }
        _cacheNewCert(journal);
        return journal;
    }

    /**
     * @dev Internal function to verify zero-knowledge proofs using the appropriate coprocessor
     * @param zkCoprocessor Type of ZK coprocessor (RiscZero or Succinct)
     * @param programId Program identifier for the verification program
     * @param output Encoded output data to verify
     * @param proofBytes Zero-knowledge proof data
     */
    function _verifyZk(
        ZkCoProcessorType zkCoprocessor,
        bytes32 programId,
        bytes calldata output,
        bytes calldata proofBytes
    )
        internal
        view
    {
        // Resolve the verifier address (route-specific or default)
        address verifier = _resolveZkVerifier(zkCoprocessor, proofBytes);

        if (zkCoprocessor == ZkCoProcessorType.RiscZero) {
            IRiscZeroVerifier(verifier).verify(proofBytes, programId, sha256(output));
        } else if (zkCoprocessor == ZkCoProcessorType.Succinct) {
            ISP1Verifier(verifier).verifyProof(programId, output, proofBytes);
        } else {
            revert Unknown_Zk_Coprocessor();
        }
    }

    /**
     * @dev Internal function to resolve the ZK verifier address based on route configuration
     * @param zkCoprocessor Type of ZK coprocessor
     * @param proofBytes Proof data (selector extracted from first 4 bytes)
     * @return Resolved verifier address
     */
    function _resolveZkVerifier(
        ZkCoProcessorType zkCoprocessor,
        bytes calldata proofBytes
    )
        internal
        view
        returns (address)
    {
        bytes4 selector = bytes4(proofBytes[0:4]);
        address verifier = _zkVerifierRoutes[zkCoprocessor][selector];

        // Check if route is frozen
        if (verifier == FROZEN) {
            revert ZkRouteFrozen(zkCoprocessor, selector);
        }

        // Fall back to default verifier if no route-specific one configured
        if (verifier == address(0)) {
            verifier = zkConfig[zkCoprocessor].zkVerifier;
        }

        // Ensure verifier is configured
        if (verifier == address(0)) {
            revert ZkVerifierNotConfigured(zkCoprocessor);
        }

        return verifier;
    }
}
