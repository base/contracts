// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import {IVerifier} from "../interfaces/IVerifier.sol";

import {SystemConfigGlobal} from "./SystemConfigGlobal.sol";

/// @title TEEVerifier
/// @notice Stateless TEE proof verifier that validates signatures against registered signers.
/// @dev This contract is designed to be used as the TEE_VERIFIER in the AggregateVerifier.
///      It verifies that proofs are signed by enclave addresses registered in SystemConfigGlobal
///      via AWS Nitro attestation, and that the signer's PCR0 matches the claimed imageId.
///      Additionally, it verifies that the L1 origin block referenced in the proof actually exists
///      by checking against blockhash() or the EIP-2935 history contract.
///      The contract is intentionally stateless - all state related to output proposals is
///      managed by the calling contract (e.g., AggregateVerifier).
contract TEEVerifier is IVerifier {
    /// @notice The EIP-2935 blockhash history contract address (deployed post-Pectra).
    /// @dev This contract stores blockhashes for the last ~8192 blocks, extending the
    ///      256-block window of the native blockhash() opcode.
    address public constant EIP2935_CONTRACT = 0x0000F90827F1C53a10cb7A02335B175320002935;

    /// @notice The maximum number of blocks that blockhash() can look back.
    uint256 public constant BLOCKHASH_WINDOW = 256;

    /// @notice The maximum number of blocks that EIP-2935 can look back (~8192).
    uint256 public constant EIP2935_WINDOW = 8191;

    /// @notice The SystemConfigGlobal contract that manages valid TEE signers.
    /// @dev Signers are registered via AWS Nitro attestation in SystemConfigGlobal.
    SystemConfigGlobal public immutable SYSTEM_CONFIG_GLOBAL;

    /// @notice Thrown when the recovered signer is not a valid registered signer.
    error InvalidSigner(address signer);

    /// @notice Thrown when the signature format is invalid.
    error InvalidSignature();

    /// @notice Thrown when the signer's registered PCR0 does not match the claimed imageId.
    error ImageIdMismatch(bytes32 signerPCR0, bytes32 claimedImageId);

    /// @notice Thrown when the L1 origin block is too old to verify.
    error L1OriginTooOld(uint256 l1OriginNumber, uint256 currentBlock);

    /// @notice Thrown when the L1 origin block number is in the future.
    error L1OriginInFuture(uint256 l1OriginNumber, uint256 currentBlock);

    /// @notice Thrown when the L1 origin hash doesn't match the actual blockhash.
    error L1OriginHashMismatch(bytes32 claimed, bytes32 actual);

    /// @notice Thrown when the proof format is invalid.
    error InvalidProofFormat();

    /// @notice Constructs the TEEVerifier contract.
    /// @param systemConfigGlobal The SystemConfigGlobal contract address.
    constructor(SystemConfigGlobal systemConfigGlobal) {
        SYSTEM_CONFIG_GLOBAL = systemConfigGlobal;
    }

    /// @notice Verifies a TEE proof for a state transition.
    /// @param proofBytes The proof: l1OriginHash (32) + l1OriginNumber (32) + signature (65) = 129 bytes.
    /// @param imageId The claimed TEE image hash (PCR0). Must match the signer's registered PCR0.
    /// @param journal The keccak256 hash of the proof's public inputs.
    /// @return valid Whether the proof is valid.
    function verify(bytes calldata proofBytes, bytes32 imageId, bytes32 journal) external view override returns (bool) {
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
        bytes32 registeredPCR0 = SYSTEM_CONFIG_GLOBAL.signerPCR0(signer);

        // Check that the signer is registered (PCR0 != 0)
        if (registeredPCR0 == bytes32(0)) {
            revert InvalidSigner(signer);
        }

        // Check that the signer's registered PCR0 matches the claimed imageId
        // This ensures the signer was running the exact enclave image specified
        if (registeredPCR0 != imageId) {
            revert ImageIdMismatch(registeredPCR0, imageId);
        }

        return true;
    }

    /// @notice Verifies that the claimed L1 origin hash matches the actual blockhash.
    /// @param l1OriginHash The L1 block hash claimed in the proof.
    /// @param l1OriginNumber The L1 block number claimed in the proof.
    function _verifyL1Origin(bytes32 l1OriginHash, uint256 l1OriginNumber) private view {
        // Check for future block
        if (l1OriginNumber >= block.number) {
            revert L1OriginInFuture(l1OriginNumber, block.number);
        }

        bytes32 actualHash;
        uint256 blockAge = block.number - l1OriginNumber;

        // Prefer blockhash() over EIP-2935 when possible since it's cheaper (no external call).
        if (blockAge <= BLOCKHASH_WINDOW) {
            actualHash = blockhash(l1OriginNumber);
        } else if (blockAge <= EIP2935_WINDOW) {
            // EIP-2935 expects raw calldata: exactly 32 bytes containing the block number.
            // Using a Solidity interface would add a 4-byte function selector, causing a revert.
            // We use a low-level staticcall with raw 32-byte calldata instead.
            (bool success, bytes memory result) = EIP2935_CONTRACT.staticcall(abi.encode(l1OriginNumber));
            if (!success || result.length != 32) {
                revert L1OriginTooOld(l1OriginNumber, block.number);
            }
            actualHash = abi.decode(result, (bytes32));
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
