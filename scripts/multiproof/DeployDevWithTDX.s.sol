// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { console2 as console } from "lib/forge-std/src/console2.sol";

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @notice Development deployment using the TDX signer-registration path.
contract DeployDevWithTDX is DeployDevBase {
    uint256 public constant INIT_BOND = 0.00001 ether;

    address public nitroEnclaveVerifierAddr;
    address public tdxVerifierAddr;
    address public tdxRegistrationManager;

    function run(
        address tdxVerifier,
        address registrationManager,
        bytes32 asrStartingOutputRoot,
        uint256 asrStartingBlockNumber
    )
        public
    {
        run(cfg.nitroEnclaveVerifier(), tdxVerifier, registrationManager, asrStartingOutputRoot, asrStartingBlockNumber);
    }

    function run(
        address nitroEnclaveVerifier,
        address tdxVerifier,
        address registrationManager,
        bytes32 asrStartingOutputRoot,
        uint256 asrStartingBlockNumber
    )
        public
    {
        nitroEnclaveVerifierAddr = nitroEnclaveVerifier;
        tdxVerifierAddr = tdxVerifier;
        tdxRegistrationManager = registrationManager;
        run(asrStartingOutputRoot, asrStartingBlockNumber);
    }

    function _blockInterval() internal view override returns (uint256) {
        return cfg.multiproofBlockInterval();
    }

    function _intermediateBlockInterval() internal view override returns (uint256) {
        return cfg.multiproofIntermediateBlockInterval();
    }

    function _initBond() internal pure override returns (uint256) {
        return INIT_BOND;
    }

    function _outputSuffix() internal pure override returns (string memory) {
        return "-dev-with-tdx.json";
    }

    function _preflight() internal view override {
        require(tdxVerifierAddr != address(0), "tdxVerifier must be non-zero");
        require(nitroEnclaveVerifierAddr != address(0), "nitroEnclaveVerifier must be non-zero");
        require(tdxRegistrationManager != address(0), "registrationManager must be non-zero");
    }

    function _deployTEERegistryImpl() internal override returns (address) {
        return address(
            new DevTEEProverRegistry(
                INitroEnclaveVerifier(nitroEnclaveVerifierAddr),
                ITDXVerifier(tdxVerifierAddr),
                IDisputeGameFactory(disputeGameFactory)
            )
        );
    }

    function _teeRegistrationManager(address) internal view override returns (address) {
        return tdxRegistrationManager;
    }

    function _afterTEERegistryDeploy() internal override {
        INitroEnclaveVerifier(nitroEnclaveVerifierAddr).setProofSubmitter(teeProverRegistryProxy);
    }

    function _serializeExtra(string memory key) internal override {
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifierAddr);
        vm.serializeAddress(key, "TDXVerifier", tdxVerifierAddr);
        vm.serializeAddress(key, "TDXRegistrationManager", tdxRegistrationManager);
    }

    function _logHeader() internal view override {
        console.log("=== Deploying Dev Infrastructure (WITH TDX) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("TDXVerifier:", tdxVerifierAddr);
        console.log("TDX Registration Manager:", tdxRegistrationManager);
        console.log("ASR Starting Output Root:", vm.toString(startingAnchorRoot.raw()));
        console.log("ASR Starting L2 Block:", startingAnchorBlockNumber);
        console.log("");
        console.log("NOTE: NitroEnclaveVerifier owner must be the broadcaster/finalSystemOwner.");
    }

    function _printSummary() internal view override {
        console.log("\n========================================");
        console.log("      DEV DEPLOYMENT COMPLETE (TDX)");
        console.log("========================================");
        console.log("\nTDX Contracts:");
        console.log("  NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("  TDXVerifier:", tdxVerifierAddr);
        console.log("  TEEProverRegistry:", teeProverRegistryProxy);
        console.log("  TDX Registration Manager:", tdxRegistrationManager);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", address(mockAnchorRegistry));
        console.log("  ASR Starting Output Root:", vm.toString(startingAnchorRoot.raw()));
        console.log("  ASR Starting L2 Block:", startingAnchorBlockNumber);
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", cfg.multiproofGameType());
        console.log("  Nitro Image Hash:", vm.toString(cfg.teeNitroImageHash()));
        console.log("  TDX Image Hash:", vm.toString(cfg.teeTdxImageHash()));
        console.log("  Config Hash:", vm.toString(cfg.multiproofConfigHash()));
        console.log("========================================");
    }
}
