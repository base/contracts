// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @title DeployDevNoNitro
/// @notice Development deployment using DevTEEProverRegistry, which bypasses AWS Nitro attestation
///         validation. See scripts/multiproof/README.md for usage. Not for production.
contract DeployDevNoNitro is DeployDevBase {
    function _initBond() internal pure override returns (uint256) {
        return 0.001 ether;
    }

    function _outputSuffix() internal pure override returns (string memory) {
        return "-dev-no-nitro.json";
    }

    function _preflight() internal view override {
        require(cfg.tdxVerifier() != address(0), "tdxVerifier must be set in config");
    }

    function _deployTEERegistryImpl() internal override returns (address) {
        return address(
            new DevTEEProverRegistry(
                INitroEnclaveVerifier(address(0)),
                ITDXVerifier(cfg.tdxVerifier()),
                IDisputeGameFactory(disputeGameFactory)
            )
        );
    }

    function _serializeExtra(string memory key) internal override {
        vm.serializeAddress(key, "TDXVerifier", cfg.tdxVerifier());
    }
}
