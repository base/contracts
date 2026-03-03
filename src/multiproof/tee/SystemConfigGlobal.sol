// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {
    INitroEnclaveVerifier,
    ZkCoProcessorType,
    VerifierJournal,
    VerificationResult,
    Pcr,
    Bytes48
} from "lib/aws-nitro-enclave-attestation/contracts/src/interfaces/INitroEnclaveVerifier.sol";
import { OwnableManagedUpgradeable } from "lib/op-enclave/contracts/src/OwnableManagedUpgradeable.sol";
import { ISemver } from "interfaces/universal/ISemver.sol";

/// @title SystemConfigGlobal
/// @notice Manages TEE signer registration via ZK-verified AWS Nitro attestation.
/// @dev Signers are registered by providing a ZK proof of a valid AWS Nitro attestation document,
///      verified through an external NitroEnclaveVerifier contract (Risc0).
///      Each signer is associated with the PCR0 (enclave image hash) from their attestation,
///      which allows TEEVerifier to validate that a signer was registered with a specific image.
contract SystemConfigGlobal is OwnableManagedUpgradeable, ISemver {
    /// @notice Maximum age of an attestation document (60 minutes), in seconds.
    uint256 public constant MAX_AGE = 60 minutes;

    /// @notice Conversion factor from milliseconds to seconds.
    /// @dev AWS Nitro attestation timestamps are in milliseconds since epoch,
    ///      but block.timestamp is in seconds.
    uint256 private constant MS_PER_SECOND = 1000;

    /// @notice The external NitroEnclaveVerifier contract used for ZK attestation verification.
    INitroEnclaveVerifier public immutable NITRO_VERIFIER;

    /// @notice Mapping of valid PCR0s (enclave image hashes) attested from AWS Nitro.
    /// @dev Only attestations with a PCR0 in this mapping can register signers.
    mapping(bytes32 => bool) public validPCR0s;

    /// @notice Mapping of signer address to the PCR0 they were registered with.
    /// @dev A non-zero value indicates the signer is valid and was registered with that PCR0.
    mapping(address => bytes32) public signerPCR0;

    /// @notice Mapping of whether an address is a valid proposer.
    mapping(address => bool) public isValidProposer;

    /// @notice Emitted when a signer is registered.
    event SignerRegistered(address indexed signer, bytes32 indexed pcr0);

    /// @notice Emitted when a signer is deregistered.
    event SignerDeregistered(address indexed signer);

    /// @notice Emitted when a PCR0 is registered.
    event PCR0Registered(bytes32 indexed pcr0Hash);

    /// @notice Emitted when a PCR0 is deregistered.
    event PCR0Deregistered(bytes32 indexed pcr0Hash);

    /// @notice Emitted when the proposer is set.
    event ProposerSet(address indexed proposer, bool isValid);

    /// @notice Thrown when the PCR0 in the attestation is not registered as valid.
    error InvalidPCR0();

    /// @notice Thrown when the attestation document is too old.
    error AttestationTooOld();

    /// @notice Thrown when the ZK attestation verification fails.
    error AttestationVerificationFailed();

    /// @notice Thrown when PCR0 (index 0) is not found in the attestation's PCR list.
    error PCR0NotFound();

    /// @notice Thrown when the attestation's public key is too short to derive a signer address.
    error InvalidPublicKey();

    constructor(INitroEnclaveVerifier nitroVerifier) {
        NITRO_VERIFIER = nitroVerifier;
        initialize({ initialOwner: address(0xdEaD), initialManager: address(0xdEaD) });
    }

    /// @notice Sets the proposer address.
    /// @param proposer The proposer address.
    /// @param isValid Whether the proposer is valid.
    function setProposer(address proposer, bool isValid) external onlyOwner {
        isValidProposer[proposer] = isValid;
        emit ProposerSet(proposer, isValid);
    }

    /// @notice Registers a PCR0 (enclave image hash) as valid.
    /// @param pcr0 The raw PCR0 bytes from the enclave.
    function registerPCR0(bytes calldata pcr0) external onlyOwner {
        bytes32 pcr0Hash = keccak256(pcr0);
        validPCR0s[pcr0Hash] = true;
        emit PCR0Registered(pcr0Hash);
    }

    /// @notice Deregisters a PCR0 (enclave image hash).
    /// @param pcr0 The raw PCR0 bytes from the enclave.
    function deregisterPCR0(bytes calldata pcr0) external onlyOwner {
        bytes32 pcr0Hash = keccak256(pcr0);
        delete validPCR0s[pcr0Hash];
        emit PCR0Deregistered(pcr0Hash);
    }

    /// @notice Registers a signer using a ZK proof of an AWS Nitro attestation document.
    /// @dev The ZK proof must verify a valid attestation that:
    ///      1. Has a valid AWS Nitro certificate chain (verified offchain via ZK)
    ///      2. Contains a PCR0 that has been pre-registered via registerPCR0
    ///      3. Is less than MAX_AGE old
    /// @param output The ABI-encoded VerifierJournal from the ZK proof.
    /// @param proofBytes The Risc0 ZK proof bytes.
    function registerSigner(bytes calldata output, bytes calldata proofBytes) external onlyOwnerOrManager {
        VerifierJournal memory journal = NITRO_VERIFIER.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        if (journal.result != VerificationResult.Success) revert AttestationVerificationFailed();

        if (journal.timestamp / MS_PER_SECOND + MAX_AGE <= block.timestamp) revert AttestationTooOld();

        bytes32 pcr0Hash = _extractPCR0Hash(journal.pcrs);
        if (!validPCR0s[pcr0Hash]) revert InvalidPCR0();

        // The publicKey is encoded in ANSI X9.62 format: 0x04 || x || y (65 bytes).
        // We skip the first byte (0x04 prefix) when hashing to derive the address.
        bytes memory pubKey = journal.publicKey;
        if (pubKey.length != 65) revert InvalidPublicKey();
        bytes32 publicKeyHash;
        assembly {
            publicKeyHash := keccak256(add(pubKey, 0x21), sub(mload(pubKey), 1))
        }
        address enclaveAddress = address(uint160(uint256(publicKeyHash)));

        signerPCR0[enclaveAddress] = pcr0Hash;
        emit SignerRegistered(enclaveAddress, pcr0Hash);
    }

    /// @notice Deregisters a signer.
    /// @param signer The address of the signer to deregister.
    function deregisterSigner(address signer) external onlyOwnerOrManager {
        delete signerPCR0[signer];
        emit SignerDeregistered(signer);
    }

    /// @notice Checks if an address is a valid signer.
    /// @param signer The address to check.
    /// @return True if the signer is registered, false otherwise.
    function isValidSigner(address signer) external view returns (bool) {
        return signerPCR0[signer] != bytes32(0);
    }

    /// @notice Initializes the contract with owner and manager.
    /// @param initialOwner The initial owner address.
    /// @param initialManager The initial manager address.
    function initialize(address initialOwner, address initialManager) public initializer {
        __OwnableManaged_init();
        transferOwnership(initialOwner);
        transferManagement(initialManager);
    }

    /// @notice Semantic version.
    /// @custom:semver 0.2.0
    function version() public pure virtual returns (string memory) {
        return "0.2.0";
    }

    /// @dev Finds PCR0 (index 0) in the PCR array and returns its keccak256 hash.
    function _extractPCR0Hash(Pcr[] memory pcrs) internal pure returns (bytes32) {
        for (uint256 i = 0; i < pcrs.length; i++) {
            if (pcrs[i].index == 0) {
                Bytes48 memory value = pcrs[i].value;
                return keccak256(abi.encodePacked(value.first, value.second));
            }
        }
        revert PCR0NotFound();
    }
}
