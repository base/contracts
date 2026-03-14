// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {
    INitroEnclaveVerifier
} from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { EnumerableSetLib } from "@solady-v0.0.245/utils/EnumerableSetLib.sol";

import { TEEProverRegistry } from "src/multiproof/tee/TEEProverRegistry.sol";

/// @title DevTEEProverRegistry
/// @notice Development version of TEEProverRegistry with bypassed attestation for testing.
/// @dev This contract adds addDevSigner() which bypasses AWS Nitro attestation verification.
///      DO NOT deploy this contract to production networks.
contract DevTEEProverRegistry is TEEProverRegistry {
    using EnumerableSetLib for EnumerableSetLib.AddressSet;

    constructor(INitroEnclaveVerifier nitroVerifier) TEEProverRegistry(nitroVerifier) { }

    /// @notice Registers a signer for testing (bypasses attestation verification).
    /// @dev Only callable by owner. For development/testing use only.
    /// @param signer The address of the signer to register.
    /// @param pcr0Hash The PCR0 hash to associate with this signer.
    function addDevSigner(address signer, bytes32 pcr0Hash) external onlyOwner {
        signerPCR0[signer] = pcr0Hash;
        _registeredSigners.add(signer);
        emit SignerRegistered(signer, pcr0Hash);
    }
}
