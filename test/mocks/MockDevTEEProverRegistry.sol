// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

/// @title DevTEEProverRegistry
/// @notice Development version of TEEProverRegistry with bypassed attestation for testing.
/// @dev This contract adds addDevSigner() which bypasses AWS Nitro attestation verification.
///      DO NOT deploy this contract to production networks.
contract DevTEEProverRegistry is TEEProverRegistry {
    constructor(
        INitroEnclaveVerifier nitroVerifier,
        ITDXVerifier tdxVerifier,
        IDisputeGameFactory factory
    )
        TEEProverRegistry(nitroVerifier, tdxVerifier, factory)
    { }

    /// @notice Registers a Nitro signer for testing (bypasses attestation verification).
    /// @dev Only callable by owner. For development/testing use only.
    ///      The imageHash parameter is stored so isValidSigner() can validate against
    ///      the current AggregateVerifier's TEE_IMAGE_HASH.
    /// @param signer The address of the signer to register.
    /// @param imageHash The TEE image hash to associate with this signer.
    function addDevSigner(address signer, bytes32 imageHash) external onlyOwner {
        _registerSigner(signer, imageHash, TEEType.NITRO);
    }

    /// @notice Registers a TDX signer for testing (bypasses attestation verification).
    /// @dev Only callable by owner. For development/testing use only.
    /// @param signer The address of the signer to register.
    /// @param imageHash The TEE image hash to associate with this signer.
    function addDevTDXSigner(address signer, bytes32 imageHash) external onlyOwner {
        _registerSigner(signer, imageHash, TEEType.TDX);
    }
}
