// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/**
 * @title DeployDevWithNitro
 * @notice Development deployment WITH AWS Nitro attestation validation.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 *                            DEPLOYMENT TYPE: DEV (WITH NITRO)
 * ══════════════════════════════════════════════════════════════════════════════════
 *
 * This script deploys infrastructure using the REAL TEEProverRegistry, which
 * REQUIRES a ZK proof of a valid AWS Nitro attestation for signer registration.
 * You cannot use addDevSigner() - you must go through the full registerSigner() flow.
 *
 * PREREQUISITES:
 *   1. Deploy the RISC Zero verifier stack AND NitroEnclaveVerifier using
 *      DeployRiscZeroStack.s.sol (required because NitroEnclaveVerifier and its
 *      dependencies need Solidity ^0.8.20, while this script is pinned to =0.8.15).
 *   2. Set `nitroEnclaveVerifier` in the deploy config to the deployed address.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * SIGNER REGISTRATION FLOW
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * After deployment, register a signer by generating a RISC Zero ZK proof of a
 * valid AWS Nitro attestation document and calling:
 *
 *   cast send $TEE_PROVER_REGISTRY \
 *     "registerSigner(bytes,bytes)" $ZK_OUTPUT $ZK_PROOF_BYTES \
 *     --private-key $OWNER_OR_MANAGER_KEY --rpc-url $RPC_URL
 *
 * IMPORTANT: The attestation is only valid for 60 minutes! Generate the proof
 * and submit the transaction within that window.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 */

import { console2 as console } from "lib/forge-std/src/console2.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

import { DeployDevBase } from "./DeployDevBase.s.sol";

contract DeployDevWithNitro is DeployDevBase {
    uint256 public constant BLOCK_INTERVAL = 600;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 30;
    uint256 public constant INIT_BOND = 0.00001 ether;

    address public nitroEnclaveVerifierAddr;

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
        require(
            nitroEnclaveVerifierAddr != address(0),
            "nitroEnclaveVerifier must be set in config (deploy via DeployRiscZeroStack.s.sol first)"
        );
    }

    function _deployTEERegistryImpl() internal override returns (address) {
        return address(
            new TEEProverRegistry(
                INitroEnclaveVerifier(nitroEnclaveVerifierAddr), IDisputeGameFactory(disputeGameFactory)
            )
        );
    }

    function _serializeExtra(string memory key) internal override {
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifierAddr);
    }

    function _logHeader() internal view override {
        console.log("=== Deploying Dev Infrastructure (WITH NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("");
        console.log("NOTE: Using REAL TEEProverRegistry - ZK attestation proof REQUIRED.");
    }

    function _printSummary() internal view override {
        console.log("\n========================================");
        console.log("   DEV DEPLOYMENT COMPLETE (WITH NITRO)");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("  TEEProverRegistry:", teeProverRegistryProxy);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", address(mockAnchorRegistry));
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", cfg.multiproofGameType());
        console.log("  TEE Image Hash:", vm.toString(cfg.teeImageHash()));
        console.log("  Config Hash:", vm.toString(cfg.multiproofConfigHash()));
        console.log("========================================");
        console.log("\n>>> NEXT STEP: Register signer with ZK attestation proof <<<");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerSigner(bytes,bytes)" <ZK_OUTPUT> <ZK_PROOF_BYTES>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n========================================\n");
    }
}
