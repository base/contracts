// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

/// @title DevTEEProverRegistry
/// @notice Test/development registry that can register signers without Nitro attestation verification.
/// @dev DO NOT deploy this contract to production networks.
contract DevTEEProverRegistry is TEEProverRegistry {
    constructor(
        INitroEnclaveVerifier nitroVerifier,
        ITDXVerifier tdxVerifier,
        IDisputeGameFactory factory
    )
        TEEProverRegistry(nitroVerifier, tdxVerifier, factory)
    { }

    function addDevSigner(address signer, bytes32 imageHash, TEEType teeType) external onlyOwner {
        _registerSigner(signer, imageHash, teeType);
    }
}
