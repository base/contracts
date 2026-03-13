// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

interface IVerifier {
    /// @notice Verifies a proof.
    /// @param proofBytes The proof.
    /// @param imageId The image ID.
    /// @param journal The journal.
    /// @return valid Whether the proof is valid.
    function verify(bytes calldata proofBytes, bytes32 imageId, bytes32 journal) external view returns (bool);

    /// @notice Nullifies the prover to prevent further proof verification.
    /// @dev Should only occur if a soundness issue is found.
    /// @dev Should only be callable by a proper dispute game.
    function nullify() external;
}
