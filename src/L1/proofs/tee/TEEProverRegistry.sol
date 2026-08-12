// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CborDecode, CborElement, LibCborElement } from "lib/nitro-validator/src/CborDecode.sol";

import { EnumerableSetLib } from "src/vendor/EnumerableSetLib.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroValidator } from "interfaces/L1/proofs/tee/INitroValidator.sol";
import { ISemver } from "interfaces/universal/ISemver.sol";
import { OwnableManagedUpgradeable } from "src/universal/OwnableManagedUpgradeable.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

/// @title TEEProverRegistry
/// @notice Manages TEE signer registration via hinted AWS Nitro attestation validation.
/// @dev Signers are registered by providing an AWS Nitro attestation and P-384 verification hints.
///      The attestation's certificate chain must already be cached in the NitroValidator's CertManager.
///      Registration is PCR0-agnostic: any enclave with a valid attestation can register,
///      enabling pre-registration before hardforks. PCR0 enforcement happens at proof-submission
///      time in TEEVerifier, which checks signerImageHash against the AggregateVerifier's
///      TEE_IMAGE_HASH.
contract TEEProverRegistry is OwnableManagedUpgradeable, ISemver {
    using CborDecode for bytes;
    using EnumerableSetLib for EnumerableSetLib.AddressSet;
    using LibCborElement for CborElement;

    /// @notice Maximum age of an attestation document (60 minutes), in seconds.
    uint256 public constant MAX_AGE = 60 minutes;

    /// @notice Conversion factor from milliseconds to seconds.
    /// @dev AWS Nitro attestation timestamps are in milliseconds since epoch,
    ///      but block.timestamp is in seconds.
    uint256 private constant MS_PER_SECOND = 1000;

    /// @notice The hash of the all-zero PCR0 emitted by debug-mode Nitro enclaves.
    bytes32 private constant DEBUG_MODE_PCR0_HASH = 0xc980e59163ce244bb4bb6211f48c7b46f88a4f40943e84eb99bdc41e129bd293;

    /// @notice The external NitroValidator contract used for hinted attestation validation.
    INitroValidator public immutable NITRO_VALIDATOR;

    /// @notice The DisputeGameFactory used to look up the current AggregateVerifier and its TEE_IMAGE_HASH.
    IDisputeGameFactory public immutable DISPUTE_GAME_FACTORY;

    /// @notice The game type used to look up the AggregateVerifier in the factory.
    /// @dev Owner-settable to support game type migrations.
    GameType public gameType;

    /// @notice Mapping of whether a signer address is registered.
    mapping(address => bool) public isRegisteredSigner;

    /// @notice Mapping of signer address to the PCR0 image hash from their attestation.
    /// @dev Stored at registration time from the validated attestation document.
    ///      TEEVerifier checks this against the AggregateVerifier's TEE_IMAGE_HASH at
    ///      proof-submission time, so signers automatically become unusable when the
    ///      AggregateVerifier upgrades to a new image hash. isValidSigner also uses
    ///      this for off-chain pre-submission checks.
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

    /// @notice Thrown when the attestation document is dated in the future.
    error AttestationFromFuture();

    /// @notice Thrown when the attestation's public key is not an uncompressed secp256k1 key.
    error InvalidPublicKey();

    /// @notice Thrown when PCR0 (index 0) is not found in the attestation's PCR list.
    error PCR0NotFound();

    /// @notice Thrown when PCR0 is not a 48-byte SHA-384 measurement.
    error InvalidPCR0();

    /// @notice Thrown when the dispute game factory is not configured.
    error DisputeGameFactoryNotSet();

    /// @notice Thrown when reading TEE_IMAGE_HASH from the AggregateVerifier fails.
    error ImageHashReadFailed();

    /// @notice Thrown when setting a game type whose AggregateVerifier has no TEE_IMAGE_HASH.
    error InvalidGameType();

    constructor(INitroValidator nitroValidator, IDisputeGameFactory factory) {
        if (address(factory) == address(0)) revert DisputeGameFactoryNotSet();
        NITRO_VALIDATOR = nitroValidator;
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

    /// @notice Registers a signer using a hinted AWS Nitro attestation document.
    /// @dev The NitroValidator must verify an attestation that:
    ///      1. Has a valid, pre-cached AWS Nitro certificate chain
    ///      2. Is less than MAX_AGE old
    ///      Registration is PCR0-agnostic: any enclave with a valid attestation can register.
    ///      This enables pre-registration of new-PCR0 enclaves before a hardfork, eliminating
    ///      proof-generation delay when the on-chain TEE_IMAGE_HASH rotates. The TEEVerifier
    ///      enforces PCR0 correctness at proof-submission time by checking signerImageHash
    ///      against the AggregateVerifier's TEE_IMAGE_HASH, so pre-registered enclaves cannot
    ///      produce accepted proofs until the hardfork activates.
    /// @param attestationTbs The COSE Sign1 to-be-signed bytes.
    /// @param signature The 96-byte P-384 attestation signature.
    /// @param hints Offchain inverse hints for the attestation signature.
    function registerSigner(
        bytes calldata attestationTbs,
        bytes calldata signature,
        bytes calldata hints
    )
        external
        onlyOwnerOrManager
    {
        // The validator returns offsets into the TBS; copy it once for the memory-only CBOR helpers below.
        bytes memory tbs = attestationTbs;
        INitroValidator.Ptrs memory ptrs = NITRO_VALIDATOR.validateAttestationWithHints(tbs, signature, hints);

        uint256 attestationTimestamp = ptrs.timestamp / MS_PER_SECOND;
        if (attestationTimestamp + MAX_AGE <= block.timestamp) revert AttestationTooOld();
        if (attestationTimestamp >= block.timestamp) revert AttestationFromFuture();

        // Extract the attestation's PCR0 and store it for TEEVerifier to check at
        // proof-submission time. No comparison against the current TEE_IMAGE_HASH
        // here — the registry accepts any valid attestation.
        if (ptrs.pcrs.length == 0 || ptrs.pcrs[0].isNull()) revert PCR0NotFound();
        if (ptrs.pcrs[0].length() != 48) revert InvalidPCR0();
        bytes32 pcr0Hash = tbs.keccak(ptrs.pcrs[0]);
        if (pcr0Hash == DEBUG_MODE_PCR0_HASH) revert InvalidPCR0();

        // The publicKey is encoded in ANSI X9.62 format: 0x04 || x || y (65 bytes).
        // We skip the first byte (0x04 prefix) when hashing to derive the address.
        bytes memory pubKey = tbs.slice(ptrs.publicKey);
        if (pubKey.length != 65 || pubKey[0] != 0x04) revert InvalidPublicKey();
        bytes32 publicKeyHash;
        assembly {
            // Length is hardcoded to 64 to skip the 0x04 prefix and hash only the x and y coordinates
            publicKeyHash := keccak256(add(pubKey, 0x21), 64)
        }
        address enclaveAddress = address(uint160(uint256(publicKeyHash)));

        isRegisteredSigner[enclaveAddress] = true;
        signerImageHash[enclaveAddress] = pcr0Hash;
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
    /// @custom:semver 0.6.0
    function version() public pure virtual returns (string memory) {
        return "0.6.0";
    }

    /// @dev Reads TEE_IMAGE_HASH from the AggregateVerifier registered in the factory.
    function _getExpectedImageHash() internal view returns (bytes32) {
        address impl = address(DISPUTE_GAME_FACTORY.gameImpls(gameType));
        // AggregateVerifier.TEE_IMAGE_HASH() selector
        (bool success, bytes memory data) = impl.staticcall(abi.encodeWithSignature("TEE_IMAGE_HASH()"));
        if (!success || data.length != 32) revert ImageHashReadFailed();
        return abi.decode(data, (bytes32));
    }
}
