// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { INitroEnclaveVerifier } from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/multiproof/tee/ITDXVerifier.sol";
import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";

import { TEEProverRegistry } from "src/multiproof/tee/TEEProverRegistry.sol";

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

    /// @notice Registers a signer for testing (bypasses attestation verification).
    /// @dev Only callable by owner. For development/testing use only.
    ///      The imageHash parameter is stored so isValidSigner() can validate against
    ///      the current AggregateVerifier's TEE_IMAGE_HASH.
    /// @param signer The address of the signer to register.
    /// @param imageHash The TEE image hash to associate with this signer.
    function addDevSigner(address signer, bytes32 imageHash) external onlyOwner {
        _registerSigner(signer, imageHash);
    }
}
