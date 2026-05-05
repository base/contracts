// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";

import { TEEProverRegistry } from "./TEEProverRegistry.sol";
import { Verifier } from "../Verifier.sol";

/// @title TEEVerifier
/// @notice Stateless TEE proof verifier that validates Nitro and TDX signatures against registered signers.
/// @dev This contract is designed to be used as the TEE_VERIFIER in the AggregateVerifier.
///      It verifies that proofs are signed by a Nitro signer and a TDX signer registered in
///      TEEProverRegistry. PCR0 / TDX image hash enforcement is handled by
///      AggregateVerifier, which bakes TEE_IMAGE_HASH into the journal that the enclave signs.
///      The contract is intentionally stateless - all state related to output proposals and
///      L1 origin verification is managed by the calling contract (e.g., AggregateVerifier).
contract TEEVerifier is Verifier, ISemver {
    /// @notice The TEEProverRegistry contract that manages valid TEE signers.
    /// @dev Signers are registered via Nitro or TDX attestation in TEEProverRegistry.
    TEEProverRegistry public immutable TEE_PROVER_REGISTRY;

    /// @notice Size of an ECDSA signature in bytes.
    uint256 internal constant SIGNATURE_SIZE = 65;

    /// @notice Size of a TEE proof: proposer(20) + nitro signature(65) + tdx signature(65).
    uint256 internal constant TEE_PROOF_SIZE = 20 + SIGNATURE_SIZE * 2;

    /// @notice Thrown when a recovered signer is not a valid registered signer.
    error InvalidSigner(address signer);

    /// @notice Thrown when the signer's registered image hash does not match the claimed imageId.
    error ImageIdMismatch(bytes32 signerImageHash, bytes32 claimedImageId);

    /// @notice Thrown when the signature format is invalid.
    error InvalidSignature();

    /// @notice Thrown when the proof format is invalid.
    error InvalidProofFormat();

    /// @notice Thrown when the proposer is not a valid registered proposer.
    error InvalidProposer(address proposer);

    /// @notice Thrown when both signatures recover to the same signer.
    error DuplicateSigner(address signer);

    /// @notice Thrown when neither signature came from a Nitro-registered signer.
    error MissingNitroSignature();

    /// @notice Thrown when neither signature came from a TDX-registered signer.
    error MissingTDXSignature();

    /// @notice Constructs the TEEVerifier contract.
    /// @param teeProverRegistry The TEEProverRegistry contract address.
    constructor(
        TEEProverRegistry teeProverRegistry,
        IAnchorStateRegistry anchorStateRegistry
    )
        Verifier(anchorStateRegistry)
    {
        TEE_PROVER_REGISTRY = teeProverRegistry;
    }

    /// @notice Verifies a TEE proof for a state transition.
    /// @param proofBytes The proof: proposer(20) + two signatures(65 each) = 150 bytes.
    ///        One signature must recover to a Nitro signer and the other to a TDX signer.
    /// @param imageId The claimed TEE image hash (from the calling AggregateVerifier's TEE_IMAGE_HASH).
    ///        Validated against each signer's registered image hash to prevent cross-game-type attacks.
    /// @param journal The keccak256 hash of the proof's public inputs.
    /// @return valid Whether the proof is valid.
    function verify(
        bytes calldata proofBytes,
        bytes32 imageId,
        bytes32 journal
    )
        external
        view
        override
        notNullified
        returns (bool)
    {
        if (proofBytes.length != TEE_PROOF_SIZE) revert InvalidProofFormat();

        address proposer = address(bytes20(proofBytes[0:20]));
        if (!TEE_PROVER_REGISTRY.isValidProposer(proposer)) {
            revert InvalidProposer(proposer);
        }

        bytes calldata firstSignature = proofBytes[20:20 + SIGNATURE_SIZE];
        bytes calldata secondSignature = proofBytes[20 + SIGNATURE_SIZE:TEE_PROOF_SIZE];

        address firstSigner = _recoverSigner(journal, firstSignature);
        address secondSigner = _recoverSigner(journal, secondSignature);

        TEEProverRegistry.TEEType firstTEEType = _validateSigner(firstSigner, imageId);
        TEEProverRegistry.TEEType secondTEEType = _validateSigner(secondSigner, imageId);

        if (firstSigner == secondSigner) revert DuplicateSigner(firstSigner);

        if (firstTEEType != TEEProverRegistry.TEEType.NITRO && secondTEEType != TEEProverRegistry.TEEType.NITRO) {
            revert MissingNitroSignature();
        }
        if (firstTEEType != TEEProverRegistry.TEEType.TDX && secondTEEType != TEEProverRegistry.TEEType.TDX) {
            revert MissingTDXSignature();
        }

        return true;
    }

    function _recoverSigner(bytes32 journal, bytes calldata signature) internal pure returns (address signer) {
        // The signature should be over the journal hash directly (not eth-signed-message prefixed).
        ECDSA.RecoverError err;
        (signer, err) = ECDSA.tryRecover(journal, signature);
        if (err != ECDSA.RecoverError.NoError) revert InvalidSignature();
    }

    function _validateSigner(address signer, bytes32 imageId)
        internal
        view
        returns (TEEProverRegistry.TEEType teeType)
    {
        // A registered signer always has a non-NONE TEE type, so this single read also
        // serves as the registration check (saves an SLOAD versus calling isRegisteredSigner).
        teeType = TEE_PROVER_REGISTRY.signerTEEType(signer);
        if (teeType == TEEProverRegistry.TEEType.NONE) revert InvalidSigner(signer);

        // Prevents a signer registered under one enclave image from being used in a game
        // that expects a different image (e.g., after an upgrade or across game types).
        bytes32 registeredImageHash = TEE_PROVER_REGISTRY.signerImageHash(signer);
        if (registeredImageHash != imageId) {
            revert ImageIdMismatch(registeredImageHash, imageId);
        }
    }

    /// @notice Semantic version.
    /// @custom:semver 0.3.0
    function version() public pure virtual returns (string memory) {
        return "0.3.0";
    }
}
