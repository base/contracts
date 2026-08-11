// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { NitroValidator } from "lib/nitro-validator/src/NitroValidator.sol";

import { EnumerableSetLib } from "src/vendor/EnumerableSetLib.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

/// @title DevTEEProverRegistry
/// @notice Test/development registry that can register signers without Nitro attestation verification.
/// @dev DO NOT deploy this contract to production networks.
contract DevTEEProverRegistry is TEEProverRegistry {
    using EnumerableSetLib for EnumerableSetLib.AddressSet;

    constructor(NitroValidator nitroValidator, IDisputeGameFactory factory)
        TEEProverRegistry(nitroValidator, factory)
    { }

    /// @notice Registers a signer and image hash without attestation verification.
    /// @dev Only callable by owner. For development/testing use only.
    /// @param signer The address of the signer to register.
    /// @param imageHash The TEE image hash to associate with this signer.
    function addDevSigner(address signer, bytes32 imageHash) external onlyOwner {
        isRegisteredSigner[signer] = true;
        signerImageHash[signer] = imageHash;
        _registeredSigners.add(signer);
        emit SignerRegistered(signer);
    }
}
