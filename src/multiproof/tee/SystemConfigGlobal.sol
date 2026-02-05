// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {OwnableManagedUpgradeable} from "@op-enclave/OwnableManagedUpgradeable.sol";
import {NitroValidator} from "@nitro-validator/NitroValidator.sol";
import {LibBytes} from "@nitro-validator/LibBytes.sol";
import {LibCborElement, CborElement, CborDecode} from "@nitro-validator/CborDecode.sol";
import {ICertManager} from "@nitro-validator/ICertManager.sol";

/// @title SystemConfigGlobal
/// @notice Manages TEE signer registration via AWS Nitro attestation.
/// @dev Signers are registered by providing a valid AWS Nitro attestation document.
///      Each signer is associated with the PCR0 (enclave image hash) from their attestation,
///      which allows TEEVerifier to validate that a signer was registered with a specific image.
contract SystemConfigGlobal is OwnableManagedUpgradeable, NitroValidator {
    using LibBytes for bytes;
    using CborDecode for bytes;
    using LibCborElement for CborElement;

    /// @notice Maximum age of an attestation document (60 minutes).
    uint256 public constant MAX_AGE = 60 minutes;

    /// @notice The address of the proposer.
    address public proposer;

    /// @notice Mapping of valid PCR0s (enclave image hashes) attested from AWS Nitro.
    /// @dev Only attestations with a PCR0 in this mapping can register signers.
    mapping(bytes32 => bool) public validPCR0s;

    /// @notice Mapping of signer address to the PCR0 they were registered with.
    /// @dev A non-zero value indicates the signer is valid and was registered with that PCR0.
    ///      This replaces the old validSigners(address => bool) mapping to enable imageID validation.
    mapping(address => bytes32) public signerPCR0;

    /// @notice Emitted when a signer is registered.
    event SignerRegistered(address indexed signer, bytes32 indexed pcr0);

    /// @notice Emitted when a signer is deregistered.
    event SignerDeregistered(address indexed signer);

    /// @notice Emitted when a PCR0 is registered.
    event PCR0Registered(bytes32 indexed pcr0Hash);

    /// @notice Emitted when a PCR0 is deregistered.
    event PCR0Deregistered(bytes32 indexed pcr0Hash);

    /// @notice Semantic version.
    /// @custom:semver 0.1.0
    function version() public pure virtual returns (string memory) {
        return "0.1.0";
    }

    constructor(ICertManager certManager) NitroValidator(certManager) {
        // On test networks, skip auto-initialization to allow manual initialization.
        // On production, disable the implementation by setting dead addresses.
        bool isTestnet = block.chainid == 31337 // Anvil
            || block.chainid == 1337 // Ganache
            || block.chainid == 11155111 // Sepolia
            || block.chainid == 84532; // Base Sepolia
        if (!isTestnet) {
            initialize({_owner: address(0xdEaD), _manager: address(0xdEaD)});
        }
    }

    function initialize(address _owner, address _manager) public initializer {
        __OwnableManaged_init();
        transferOwnership(_owner);
        transferManagement(_manager);
    }

    /// @notice Sets the proposer address.
    function setProposer(address _proposer) external onlyOwner {
        proposer = _proposer;
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
        require(validPCR0s[pcr0Hash], "invalid pcr0 in attestation");

        require(ptrs.timestamp + MAX_AGE > block.timestamp, "attestation too old");

        // The publicKey is encoded in the form specified in section 4.3.6 of ANSI X9.62,
        // which is a 0x04 byte followed by the x and y coordinates of the public key.
        // We ignore the first byte when hashing.
        bytes32 publicKeyHash = attestationTbs.keccak(ptrs.publicKey.start() + 1, ptrs.publicKey.length() - 1);
        address enclaveAddress = address(uint160(uint256(publicKeyHash)));

        // Store the PCR0 hash for this signer (enables imageID validation)
        signerPCR0[enclaveAddress] = pcr0Hash;
        emit SignerRegistered(enclaveAddress, pcr0Hash);
    }

    /// @notice Deregisters a signer.
    /// @param signer The address of the signer to deregister.
    function deregisterSigner(address signer) external onlyOwnerOrManager {
        delete signerPCR0[signer];
        emit SignerDeregistered(signer);
    }

    /// @notice Registers a signer for testing (bypasses attestation verification).
    /// @dev Only callable by owner, only works on test networks.
    ///      Allowed chains: Anvil (31337), Ganache (1337), Sepolia (11155111), Base Sepolia (84532).
    ///      DO NOT deploy this to production networks.
    /// @param signer The address of the signer to register.
    /// @param pcr0Hash The PCR0 hash to associate with this signer.
    function addDevSigner(address signer, bytes32 pcr0Hash) external onlyOwner {
        require(
            block.chainid == 31337 // Anvil
                || block.chainid == 1337 // Ganache
                || block.chainid == 11155111 // Sepolia
                || block.chainid == 84532, // Base Sepolia
            "dev only: test chains only"
        );
        signerPCR0[signer] = pcr0Hash;
        emit SignerRegistered(signer, pcr0Hash);
    }

    /// @notice Checks if an address is a valid signer.
    /// @param signer The address to check.
    /// @return True if the signer is registered, false otherwise.
    function isValidSigner(address signer) external view returns (bool) {
        return signerPCR0[signer] != bytes32(0);
    }
}
