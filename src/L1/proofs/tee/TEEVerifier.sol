// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ECDSA } from "lib/openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";

import { TEEProverRegistry } from "./TEEProverRegistry.sol";
import { Verifier } from "../Verifier.sol";

/// @title TEEVerifier
/// @notice Stateless TEE proof verifier that validates Nitro and TDX signatures against registered signers.
/// @dev This contract is designed to be used as the TEE_VERIFIER in the AggregateVerifier.
///      It verifies one TEE signature at a time against a registered Nitro or TDX signer.
///      The contract is intentionally stateless - all state related to output proposals and
///      L1 origin verification is managed by the calling contract (e.g., AggregateVerifier).
contract TEEVerifier is Verifier, ISemver {
    /// @notice The TEEProverRegistry contract that manages valid TEE signers.
    /// @dev Signers are registered via Nitro or TDX attestation in TEEProverRegistry.
    TEEProverRegistry public immutable TEE_PROVER_REGISTRY;

    /// @notice Size of an ECDSA signature in bytes.
    uint256 internal constant SIGNATURE_SIZE = 65;

    /// @notice Size of a TEE proof: proposer(20) + signature(65).
    uint256 internal constant TEE_PROOF_SIZE = 20 + SIGNATURE_SIZE;

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
    /// @param proofBytes The proof: proposer(20) + signature(65) = 85 bytes.
    /// @param imageId The TEE image hash expected for the recovered signer.
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

        bytes calldata signature = proofBytes[20:TEE_PROOF_SIZE];
        address signer = _recoverSigner(journal, signature);
        _validateSigner(signer, imageId);

        return true;
    }

    function _recoverSigner(bytes32 journal, bytes calldata signature) internal pure returns (address signer) {
        // The signature should be over the journal hash directly (not eth-signed-message prefixed).
        ECDSA.RecoverError err;
        (signer, err) = ECDSA.tryRecover(journal, signature);
        if (err != ECDSA.RecoverError.NoError) revert InvalidSignature();
    }

    function _validateSigner(address signer, bytes32 imageId) internal view {
        // A registered signer always has a non-NONE TEE type, so this single read also
        // serves as the registration check (saves an SLOAD versus calling isRegisteredSigner).
        TEEProverRegistry.TEEType signerTEEType = TEE_PROVER_REGISTRY.signerTEEType(signer);
        if (signerTEEType == TEEProverRegistry.TEEType.NONE) revert InvalidSigner(signer);

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
