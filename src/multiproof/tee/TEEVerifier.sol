// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {IVerifier} from "../interfaces/IVerifier.sol";
import {SystemConfigGlobal} from "./SystemConfigGlobal.sol";

/// @title IEIP2935
/// @notice Interface for the EIP-2935 blockhash history contract.
interface IEIP2935 {
    function get(uint256 blockNumber) external view returns (bytes32);
}

/// @title TEEVerifier
/// @notice Stateless TEE proof verifier that validates signatures against registered signers.
/// @dev This contract is designed to be used as the TEE_VERIFIER in the AggregateVerifier.
///      It verifies that proofs are signed by enclave addresses registered in SystemConfigGlobal
///      via AWS Nitro attestation, and that the signer's PCR0 matches the claimed imageID.
///      Additionally, it verifies that the L1 origin block referenced in the proof actually exists
///      by checking against blockhash() or the EIP-2935 history contract.
///      The contract is intentionally stateless - all state related to output proposals is
///      managed by the calling contract (e.g., AggregateVerifier).
contract TEEVerifier is IVerifier {
    /// @notice The SystemConfigGlobal contract that manages valid TEE signers.
    /// @dev Signers are registered via AWS Nitro attestation in SystemConfigGlobal.
    SystemConfigGlobal public immutable systemConfigGlobal;

    /// @notice The EIP-2935 blockhash history contract address (deployed post-Pectra).
    /// @dev This contract stores blockhashes for the last ~8192 blocks, extending the
    ///      256-block window of the native blockhash() opcode.
    address public constant EIP2935_CONTRACT = 0x0F792be4B0c0cb4DAE440Ef133E90C0eCD48CCCC;

    /// @notice The maximum number of blocks that blockhash() can look back.
    uint256 public constant BLOCKHASH_WINDOW = 256;

    /// @notice The maximum number of blocks that EIP-2935 can look back (~8192).
    uint256 public constant EIP2935_WINDOW = 8191;

    /// @notice Emitted when a proof is successfully verified.
    /// @param signer The address that signed the proof.
    /// @param imageID The image ID (PCR0 hash) that was validated.
    /// @param journal The journal hash that was signed.
    event ProofVerified(address indexed signer, bytes32 indexed imageID, bytes32 indexed journal);

    /// @notice Thrown when the recovered signer is not a valid registered signer.
    error InvalidSigner(address signer);

    /// @notice Thrown when the signature format is invalid.
    error InvalidSignature();

    /// @notice Thrown when the signer's registered PCR0 does not match the claimed imageID.
    error ImageIDMismatch(bytes32 signerPCR0, bytes32 claimedImageID);

    /// @notice Thrown when the L1 origin block is too old to verify.
    error L1OriginTooOld(uint256 l1OriginNumber, uint256 currentBlock);

    /// @notice Thrown when the L1 origin hash doesn't match the actual blockhash.
    error L1OriginHashMismatch(bytes32 claimed, bytes32 actual);

    /// @notice Thrown when the proof format is invalid.
    error InvalidProofFormat();

    /// @notice Constructs the TEEVerifier contract.
    /// @param _systemConfigGlobal The SystemConfigGlobal contract address.
    constructor(SystemConfigGlobal _systemConfigGlobal) {
        systemConfigGlobal = _systemConfigGlobal;
    }

    /// @notice Verifies a TEE proof for a state transition.
    /// @param proofBytes The proof: l1OriginHash (32) + l1OriginNumber (32) + signature (65) = 129 bytes.
    /// @param imageID The claimed TEE image hash (PCR0). Must match the signer's registered PCR0.
    /// @param journal The keccak256 hash of the proof's public inputs.
    /// @return valid Whether the proof is valid.
    function verify(bytes calldata proofBytes, bytes32 imageID, bytes32 journal) external view override returns (bool) {
        if (proofBytes.length < 129) revert InvalidProofFormat();

        bytes32 l1OriginHash = bytes32(proofBytes[0:32]);
        uint256 l1OriginNumber = uint256(bytes32(proofBytes[32:64]));
        bytes calldata signature = proofBytes[64:129];

        // Verify claimed L1 origin hash matches actual blockhash
        _verifyL1Origin(l1OriginHash, l1OriginNumber);

        // Recover the signer from the signature
        // The signature should be over the journal hash directly (not eth-signed-message prefixed)
        (address signer, ECDSA.RecoverError err) = ECDSA.tryRecover(journal, signature);

        if (err != ECDSA.RecoverError.NoError) {
            revert InvalidSignature();
        }

        // Get the PCR0 the signer was registered with
        bytes32 registeredPCR0 = systemConfigGlobal.signerPCR0(signer);

        // Check that the signer is registered (PCR0 != 0)
        if (registeredPCR0 == bytes32(0)) {
            revert InvalidSigner(signer);
        }

        // Check that the signer's registered PCR0 matches the claimed imageID
        // This ensures the signer was running the exact enclave image specified
        if (registeredPCR0 != imageID) {
            revert ImageIDMismatch(registeredPCR0, imageID);
        }

        return true;
    }

    /// @notice Verifies that the claimed L1 origin hash matches the actual blockhash.
    /// @param l1OriginHash The L1 block hash claimed in the proof.
    /// @param l1OriginNumber The L1 block number claimed in the proof.
    function _verifyL1Origin(bytes32 l1OriginHash, uint256 l1OriginNumber) internal view {
        bytes32 actualHash;

        if (block.number > l1OriginNumber && block.number - l1OriginNumber <= BLOCKHASH_WINDOW) {
            actualHash = blockhash(l1OriginNumber);
        } else if (block.number > l1OriginNumber && block.number - l1OriginNumber <= EIP2935_WINDOW) {
            actualHash = IEIP2935(EIP2935_CONTRACT).get(l1OriginNumber);
        } else {
            revert L1OriginTooOld(l1OriginNumber, block.number);
        }

        if (actualHash == bytes32(0)) {
            revert L1OriginTooOld(l1OriginNumber, block.number);
        }

        if (actualHash != l1OriginHash) {
            revert L1OriginHashMismatch(l1OriginHash, actualHash);
        }
    }
}
