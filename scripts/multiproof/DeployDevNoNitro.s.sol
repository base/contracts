// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { console2 as console } from "lib/forge-std/src/console2.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @title DeployDevNoNitro
/// @notice Development deployment using DevTEEProverRegistry, which bypasses AWS Nitro attestation
///         validation. See scripts/multiproof/README.md for usage. Not for production.
contract DeployDevNoNitro is DeployDevBase {
    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 public constant INIT_BOND = 0.001 ether;

    function _blockInterval() internal pure override returns (uint256) {
        return BLOCK_INTERVAL;
    }

    function _intermediateBlockInterval() internal pure override returns (uint256) {
        return INTERMEDIATE_BLOCK_INTERVAL;
    }

    function _initBond() internal pure override returns (uint256) {
        return INIT_BOND;
    }

    function _outputSuffix() internal pure override returns (string memory) {
        return "-dev-no-nitro.json";
    }

    function _deployTEERegistryImpl() internal override returns (address) {
        return
            address(
                new DevTEEProverRegistry(INitroEnclaveVerifier(address(0)), IDisputeGameFactory(disputeGameFactory))
            );
    }

    function _logHeader() internal view override {
        console.log("=== Deploying Dev Infrastructure (NO NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
    }

    function _printSummary() internal view override {
        console.log("\n=== DEV DEPLOYMENT COMPLETE (NO NITRO) ===");
        console.log("DevTEEProverRegistry:", teeProverRegistryProxy);
        console.log("TEEVerifier:", teeVerifier);
        console.log("DisputeGameFactory:", disputeGameFactory);
        console.log("AnchorStateRegistry (mock):", address(mockAnchorRegistry));
        console.log("DelayedWETH (mock):", mockDelayedWETH);
        console.log("AggregateVerifier:", aggregateVerifier);
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("TEE Image Hash:", vm.toString(cfg.teeImageHash()));
        console.log("Config Hash:", vm.toString(cfg.multiproofConfigHash()));
    }
}
