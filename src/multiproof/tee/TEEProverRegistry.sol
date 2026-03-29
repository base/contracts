// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {
    INitroEnclaveVerifier,
    ZkCoProcessorType,
    VerifierJournal,
    VerificationResult,
    Pcr,
    Bytes48
} from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { OwnableManagedUpgradeable } from "lib/op-enclave/contracts/src/OwnableManagedUpgradeable.sol";
import { ISemver } from "interfaces/universal/ISemver.sol";
import { EnumerableSetLib } from "@solady-v0.0.245/utils/EnumerableSetLib.sol";
import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";
import { GameType } from "src/dispute/lib/Types.sol";

/// @title TEEProverRegistry
/// @notice Manages TEE signer registration via ZK-verified AWS Nitro attestation.
/// @dev Signers are registered by providing a ZK proof of a valid AWS Nitro attestation document,
///      verified through an external NitroEnclaveVerifier contract (Risc0).
///      At registration time, the attestation's PCR0 is checked against the TEE_IMAGE_HASH from
///      the current AggregateVerifier implementation in the DisputeGameFactory, ensuring only
///      signers from the expected enclave image can register.
contract TEEProverRegistry is OwnableManagedUpgradeable, ISemver {
    using EnumerableSetLib for EnumerableSetLib.AddressSet;
    /// @notice Maximum age of an attestation document (60 minutes), in seconds.
    uint256 public constant MAX_AGE = 60 minutes;

    /// @notice Conversion factor from milliseconds to seconds.
    /// @dev AWS Nitro attestation timestamps are in milliseconds since epoch,
    ///      but block.timestamp is in seconds.
    uint256 private constant MS_PER_SECOND = 1000;

    /// @notice The external NitroEnclaveVerifier contract used for ZK attestation verification.
    INitroEnclaveVerifier public immutable NITRO_VERIFIER;

    /// @notice The DisputeGameFactory used to look up the current AggregateVerifier and its TEE_IMAGE_HASH.
    IDisputeGameFactory public immutable DISPUTE_GAME_FACTORY;

    /// @notice The game type used to look up the AggregateVerifier in the factory.
    /// @dev Owner-settable to support game type migrations (e.g., moving to a permissioned type).
    GameType public gameType;

    /// @notice Mapping of whether a signer address is registered.
    mapping(address => bool) public isRegisteredSigner;

    /// @notice Mapping of signer address to the PCR0 image hash they were registered with.
    /// @dev Used as defense-in-depth: isValidSigner checks this against the current
    ///      AggregateVerifier's TEE_IMAGE_HASH, so signers automatically become invalid
    ///      when the AggregateVerifier upgrades to a new image hash.
    mapping(address => bytes32) public signerImageHash;

    /// @notice Mapping of whether an address is a valid proposer.
    mapping(address => bool) public isValidProposer;

    /// @notice Enumerable set of all currently registered signer addresses.
    /// @dev Kept in sync with `isRegisteredSigner`: add on register, remove on deregister.
    ///      Enables O(1) on-chain enumeration via `getRegisteredSigners()`.
    EnumerableSetLib.AddressSet internal _registeredSigners;

    /// @notice Emitted when a signer is registered.
    event SignerRegistered(address indexed signer);

    /// @notice Emitted when a signer is deregistered.
    event SignerDeregistered(address indexed signer);

    /// @notice Emitted when the proposer is set.
    event ProposerSet(address indexed proposer, bool isValid);

    /// @notice Emitted when the game type is updated.
    event GameTypeUpdated(GameType gameType);

    /// @notice Thrown when the attestation document is too old.
    error AttestationTooOld();

    /// @notice Thrown when the ZK attestation verification fails.
    error AttestationVerificationFailed();

    /// @notice Thrown when the attestation's public key is too short to derive a signer address.
    error InvalidPublicKey();

    /// @notice Thrown when PCR0 (index 0) is not found in the attestation's PCR list.
    error PCR0NotFound();

    /// @notice Thrown when the attestation's PCR0 does not match the expected TEE_IMAGE_HASH.
    error PCR0Mismatch(bytes32 expected, bytes32 actual);

    /// @notice Thrown when the dispute game factory is not configured.
    error DisputeGameFactoryNotSet();

    /// @notice Thrown when reading TEE_IMAGE_HASH from the AggregateVerifier fails.
    error ImageHashReadFailed();

    /// @notice Thrown when setting a game type whose AggregateVerifier has no TEE_IMAGE_HASH.
    error InvalidGameType();

    constructor(INitroEnclaveVerifier nitroVerifier, IDisputeGameFactory factory) {
        if (address(factory) == address(0)) revert DisputeGameFactoryNotSet();
        NITRO_VERIFIER = nitroVerifier;
        DISPUTE_GAME_FACTORY = factory;
        initialize({
            initialOwner: address(0xdEaD),
            initialManager: address(0xdEaD),
            initialProposers: new address[](0),
            gameType_: GameType.wrap(0)
        });
    }

    /// @notice Sets the proposer address.
    /// @param proposer The proposer address.
    /// @param isValid Whether the proposer is valid.
    function setProposer(address proposer, bool isValid) external onlyOwner {
        isValidProposer[proposer] = isValid;
        emit ProposerSet(proposer, isValid);
    }

    /// @notice Updates the game type used to look up the AggregateVerifier.
    /// @dev Validates that the new game type has an AggregateVerifier with a non-zero TEE_IMAGE_HASH.
    /// @param gameType_ The new game type ID.
    function setGameType(GameType gameType_) external onlyOwner {
        // Validate the new game type points to a valid AggregateVerifier with a TEE_IMAGE_HASH
        GameType oldGameType = gameType;
        gameType = gameType_;
        bytes32 imageHash = _getExpectedImageHash();
        if (imageHash == bytes32(0)) {
            gameType = oldGameType;
            revert InvalidGameType();
        }
        emit GameTypeUpdated(gameType_);
    }

    /// @notice Registers a signer using a ZK proof of an AWS Nitro attestation document.
    /// @dev The ZK proof must verify a valid attestation that:
    ///      1. Has a valid AWS Nitro certificate chain (verified offchain via ZK)
    ///      2. Has a PCR0 matching the TEE_IMAGE_HASH from the current AggregateVerifier
    ///      3. Is less than MAX_AGE old
    /// @param output The ABI-encoded VerifierJournal from the ZK proof.
    /// @param proofBytes The Risc0 ZK proof bytes.
    function registerSigner(bytes calldata output, bytes calldata proofBytes) external onlyOwnerOrManager {
        VerifierJournal memory journal = NITRO_VERIFIER.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        if (journal.result != VerificationResult.Success) revert AttestationVerificationFailed();

        if (journal.timestamp / MS_PER_SECOND + MAX_AGE <= block.timestamp) revert AttestationTooOld();

        // Validate PCR0 against the current AggregateVerifier's TEE_IMAGE_HASH
        bytes32 pcr0Hash = _extractPCR0Hash(journal.pcrs);
        bytes32 expectedImageHash = _getExpectedImageHash();
        if (pcr0Hash != expectedImageHash) revert PCR0Mismatch(expectedImageHash, pcr0Hash);

        // The publicKey is encoded in ANSI X9.62 format: 0x04 || x || y (65 bytes).
        // We skip the first byte (0x04 prefix) when hashing to derive the address.
        bytes memory pubKey = journal.publicKey;
        if (pubKey.length != 65) revert InvalidPublicKey();
        bytes32 publicKeyHash;
        assembly {
            publicKeyHash := keccak256(add(pubKey, 0x21), sub(mload(pubKey), 1))
        }
        address enclaveAddress = address(uint160(uint256(publicKeyHash)));

        isRegisteredSigner[enclaveAddress] = true;
        signerImageHash[enclaveAddress] = expectedImageHash;
        _registeredSigners.add(enclaveAddress);
        emit SignerRegistered(enclaveAddress);
    }

    /// @notice Deregisters a signer.
    /// @param signer The address of the signer to deregister.
    function deregisterSigner(address signer) external onlyOwnerOrManager {
        delete isRegisteredSigner[signer];
        delete signerImageHash[signer];
        _registeredSigners.remove(signer);
        emit SignerDeregistered(signer);
    }

    /// @notice Checks if an address is a valid signer.
    /// @dev Defense-in-depth: checks both that the signer is registered AND that their
    ///      registered image hash matches the current AggregateVerifier's TEE_IMAGE_HASH.
    ///      This ensures signers automatically become invalid when the AggregateVerifier upgrades.
    /// @param signer The address to check.
    /// @return True if the signer is registered with the current image hash, false otherwise.
    function isValidSigner(address signer) external view returns (bool) {
        return isRegisteredSigner[signer] && signerImageHash[signer] == _getExpectedImageHash();
    }

    /// @notice Returns all currently registered signer addresses.
    /// @dev Reads directly from the on-chain enumerable set — no event scanning required.
    ///      The order of addresses in the returned array is not guaranteed.
    /// @return An array of all registered signer addresses.
    function getRegisteredSigners() external view returns (address[] memory) {
        return _registeredSigners.values();
    }

    /// @notice Returns the expected TEE image hash from the current AggregateVerifier.
    /// @return The TEE_IMAGE_HASH from the AggregateVerifier registered in the factory.
    function getExpectedImageHash() external view returns (bytes32) {
        return _getExpectedImageHash();
    }

    /// @notice Initializes the contract with owner, manager, proposers, and game type.
    /// @param initialOwner The initial owner address.
    /// @param initialManager The initial manager address.
    /// @param initialProposers Array of initial proposer addresses (zero addresses are skipped).
    /// @param gameType_ The game type for the AggregateVerifier.
    function initialize(
        address initialOwner,
        address initialManager,
        address[] memory initialProposers,
        GameType gameType_
    )
        public
        initializer
    {
        __OwnableManaged_init();
        transferOwnership(initialOwner);
        transferManagement(initialManager);
        gameType = gameType_;
        for (uint256 i = 0; i < initialProposers.length; i++) {
            if (initialProposers[i] != address(0)) {
                isValidProposer[initialProposers[i]] = true;
                emit ProposerSet(initialProposers[i], true);
            }
        }
    }

    /// @notice Semantic version.
    /// @custom:semver 0.4.0
    function version() public pure virtual returns (string memory) {
        return "0.4.0";
    }

    /// @dev Reads TEE_IMAGE_HASH from the AggregateVerifier registered in the factory.
    function _getExpectedImageHash() internal view returns (bytes32) {
        address impl = address(DISPUTE_GAME_FACTORY.gameImpls(gameType));
        // AggregateVerifier.TEE_IMAGE_HASH() selector
        (bool success, bytes memory data) = impl.staticcall(abi.encodeWithSignature("TEE_IMAGE_HASH()"));
        if (!success || data.length != 32) revert ImageHashReadFailed();
        return abi.decode(data, (bytes32));
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
