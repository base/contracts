// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ICertManager } from "@nitro-validator/ICertManager.sol";

import { SystemConfigGlobal } from "src/multiproof/tee/SystemConfigGlobal.sol";

/// @title DevSystemConfigGlobal
/// @notice Development version of SystemConfigGlobal with bypassed attestation for testing.
/// @dev This contract adds addDevSigner() which bypasses AWS Nitro attestation verification.
///      DO NOT deploy this contract to production networks.
contract DevSystemConfigGlobal is SystemConfigGlobal {
    constructor(ICertManager certManager) SystemConfigGlobal(certManager) { }

    /// @notice Registers a signer for testing (bypasses attestation verification).
    /// @dev Only callable by owner. For development/testing use only.
    /// @param signer The address of the signer to register.
    /// @param pcr0Hash The PCR0 hash to associate with this signer.
    function addDevSigner(address signer, bytes32 pcr0Hash) external onlyOwner {
        signerPCR0[signer] = pcr0Hash;
        emit SignerRegistered(signer, pcr0Hash);
    }
}
