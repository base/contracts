// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { console2 as console } from "lib/forge-std/src/console2.sol";

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
    uint256 public constant BLOCK_INTERVAL = 600;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 30;
    uint256 public constant INIT_BOND = 0.00001 ether;

    address public nitroEnclaveVerifierAddr;
    address public tdxVerifierAddr;

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

    function _logHeader() internal view override {
        console.log("=== Deploying Dev Infrastructure (WITH NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("TDXVerifier:", tdxVerifierAddr);
        console.log("");
        console.log("NOTE: Using REAL TEEProverRegistry - ZK attestation proof REQUIRED.");
    }

    function _printSummary() internal view override {
        console.log("\n========================================");
        console.log("   DEV DEPLOYMENT COMPLETE (WITH NITRO)");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("  TDXVerifier:", tdxVerifierAddr);
        console.log("  TEEProverRegistry:", teeProverRegistryProxy);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", address(mockAnchorRegistry));
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", cfg.multiproofGameType());
        console.log("  Nitro Image Hash:", vm.toString(cfg.teeNitroImageHash()));
        console.log("  TDX Image Hash:", vm.toString(cfg.teeTdxImageHash()));
        console.log("  Config Hash:", vm.toString(cfg.multiproofConfigHash()));
        console.log("========================================");
        console.log("\n>>> NEXT STEP: Register one Nitro signer and one TDX signer <<<");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerSigner(bytes,bytes)" <NITRO_OUTPUT> <NITRO_PROOF_BYTES>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerTDXSigner(bytes,bytes)" <TDX_OUTPUT> <TDX_PROOF_BYTES>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n========================================\n");
    }
}
