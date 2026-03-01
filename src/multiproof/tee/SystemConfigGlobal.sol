// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { LibCborElement, CborElement, CborDecode } from "lib/nitro-validator/src/CborDecode.sol";
import { ICertManager } from "lib/nitro-validator/src/ICertManager.sol";
import { LibBytes } from "lib/nitro-validator/src/LibBytes.sol";
import { NitroValidator } from "lib/nitro-validator/src//NitroValidator.sol";
import { OwnableManagedUpgradeable } from "lib/op-enclave/contracts/src/OwnableManagedUpgradeable.sol";

/// @title SystemConfigGlobal
/// @notice Manages TEE signer registration via AWS Nitro attestation.
/// @dev Signers are registered by providing a valid AWS Nitro attestation document.
///      Each signer is associated with the PCR0 (enclave image hash) from their attestation,
///      which allows TEEVerifier to validate that a signer was registered with a specific image.
contract SystemConfigGlobal is OwnableManagedUpgradeable, NitroValidator {
    using LibBytes for bytes;
    using CborDecode for bytes;
    using LibCborElement for CborElement;

    /// @notice Maximum age of an attestation document (60 minutes), in seconds.
    uint256 public constant MAX_AGE = 60 minutes;

    /// @notice Conversion factor from milliseconds to seconds.
    /// @dev AWS Nitro attestation timestamps are in milliseconds since epoch,
    ///      but block.timestamp is in seconds.
    uint256 private constant MS_PER_SECOND = 1000;

    /// @notice Mapping of valid PCR0s (enclave image hashes) attested from AWS Nitro.
    /// @dev Only attestations with a PCR0 in this mapping can register signers.
    mapping(bytes32 => bool) public validPCR0s;

    /// @notice Mapping of signer address to the PCR0 they were registered with.
    /// @dev A non-zero value indicates the signer is valid and was registered with that PCR0.
    ///      This replaces the old validSigners(address => bool) mapping to enable imageId validation.
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

    constructor(ICertManager certManager) NitroValidator(certManager) {
        // Always disable the implementation contract by setting dead addresses.
        // Proxies will call initialize() to set the real owner/manager.
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

    /// @notice Registers a signer using an AWS Nitro attestation document.
    /// @dev The attestation must:
    ///      1. Be signed by a valid AWS Nitro certificate chain
    ///      2. Contain a PCR0 that has been pre-registered via registerPCR0
    ///      3. Be less than MAX_AGE old
    /// @param attestationTbs The TBS (to-be-signed) portion of the attestation document.
    /// @param signature The signature over the attestation.
    function registerSigner(bytes calldata attestationTbs, bytes calldata signature) external onlyOwnerOrManager {
        Ptrs memory ptrs = validateAttestation(attestationTbs, signature);
        bytes32 pcr0Hash = attestationTbs.keccak(ptrs.pcrs[0]);
        if (!validPCR0s[pcr0Hash]) revert InvalidPCR0();
        // Convert attestation timestamp from milliseconds to seconds before comparing
        if (ptrs.timestamp / MS_PER_SECOND + MAX_AGE <= block.timestamp) revert AttestationTooOld();

        // The publicKey is encoded in the form specified in section 4.3.6 of ANSI X9.62,
        // which is a 0x04 byte followed by the x and y coordinates of the public key.
        // We ignore the first byte when hashing.
        bytes32 publicKeyHash = attestationTbs.keccak(ptrs.publicKey.start() + 1, ptrs.publicKey.length() - 1);
        address enclaveAddress = address(uint160(uint256(publicKeyHash)));

        // Store the PCR0 hash for this signer (enables imageId validation)
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
    /// @custom:semver 0.1.0
    function version() public pure virtual returns (string memory) {
        return "0.1.0";
    }
}
