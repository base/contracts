// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ISP1Verifier } from "src/dispute/zk/ISP1Verifier.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";
import { Verifier } from "../Verifier.sol";

/// @title ZkVerifier
/// @notice Verifies ZK proofs via a Succinct SP1 verifier gateway.
/// @dev Adapts the SP1 verifier interface to the IVerifier interface used by AggregateVerifier.
///      The AggregateVerifier passes (proofBytes, imageId=ZK_IMAGE_HASH, journal=keccak256(publicInputs)).
///      This contract re-encodes the journal as SP1 public values and forwards the proof to the
///      SP1 verifier gateway, using imageId as the program verification key.
contract ZkVerifier is Verifier {
    /// @notice The SP1 verifier gateway contract.
    ISP1Verifier public immutable SP1_VERIFIER;

    /// @param sp1Verifier The deployed SP1 verifier gateway address.
    /// @param anchorStateRegistry The anchor state registry for nullification checks.
    constructor(ISP1Verifier sp1Verifier, IAnchorStateRegistry anchorStateRegistry) Verifier(anchorStateRegistry) {
        SP1_VERIFIER = sp1Verifier;
    }

    /// @notice Verifies a ZK proof via the SP1 verifier gateway.
    /// @param proofBytes The SP1 proof bytes (first 4 bytes must match the verifier hash).
    /// @param imageId The SP1 program verification key (ZK_IMAGE_HASH from AggregateVerifier).
    /// @param journal The keccak256 hash of the proof's public inputs.
    /// @return True if the proof is valid.
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
        SP1_VERIFIER.verifyProof(imageId, abi.encodePacked(journal), proofBytes);
        return true;
    }

    /// @custom:semver 0.1.0
    function version() public pure virtual returns (string memory) {
        return "0.1.0";
    }
}
