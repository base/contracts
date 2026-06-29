// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @title DeployDevWithNitro
/// @notice Development deployment WITH AWS Nitro attestation validation. Uses the real
///         TEEProverRegistry, so signer registration requires a ZK proof of a valid AWS
///         Nitro attestation (no addDevSigner bypass).
contract DeployDevWithNitro is DeployDevBase {
    address public nitroEnclaveVerifierAddr;
    address public tdxVerifierAddr;

    function _outputSuffix() internal pure override returns (string memory) {
        return "-dev-with-nitro.json";
    }

    function _preflight() internal override {
        nitroEnclaveVerifierAddr = cfg.nitroEnclaveVerifier();
        tdxVerifierAddr = cfg.tdxVerifier();
        require(
            nitroEnclaveVerifierAddr != address(0),
            "nitroEnclaveVerifier must be set in config (deploy via DeployNitroVerifier.s.sol first)"
        );
        require(tdxVerifierAddr != address(0), "tdxVerifier must be set in config");
    }

    function _deployTEERegistryImpl() internal override returns (address) {
        return address(
            new TEEProverRegistry(
                INitroEnclaveVerifier(nitroEnclaveVerifierAddr),
                ITDXVerifier(tdxVerifierAddr),
                IDisputeGameFactory(disputeGameFactory)
            )
        );
    }

    function _serializeExtra(string memory key) internal override {
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifierAddr);
        vm.serializeAddress(key, "TDXVerifier", tdxVerifierAddr);
    }
}
