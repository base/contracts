// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { console2 as console } from "lib/forge-std/src/console2.sol";
import { NitroValidator } from "lib/nitro-validator/src/NitroValidator.sol";

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @title DeployDevWithNitro
/// @notice Development deployment WITH AWS Nitro attestation validation. Uses the real
///         TEEProverRegistry, so signer registration requires a hinted AWS Nitro attestation
///         (no addDevSigner bypass).
/// @dev Prerequisite: deploy the hinted validator stack via DeployNitroValidatorStack.s.sol,
///      then set `nitroValidator` in the deploy config. AWS Nitro attestations are only valid
///      for 60 minutes, and the certificate chain must be cached before registerSigner() is called.
contract DeployDevWithNitro is DeployDevBase {
    uint256 public constant BLOCK_INTERVAL = 600;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 30;
    uint256 public constant INIT_BOND = 0.00001 ether;

    address public nitroEnclaveVerifierAddr;
    address public nitroValidatorAddr;

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
        super._preflight();
        nitroEnclaveVerifierAddr = cfg.nitroEnclaveVerifier();
        nitroValidatorAddr = cfg.nitroValidator();
        require(
            nitroValidatorAddr != address(0),
            "nitroValidator must be set in config (deploy via DeployNitroValidatorStack.s.sol first)"
        );
    }

    function _deployTEERegistryImpl() internal override returns (address) {
        return address(
            new TEEProverRegistry({
                nitroValidator: NitroValidator(nitroValidatorAddr), factory: IDisputeGameFactory(disputeGameFactory)
            })
        );
    }

    function _serializeExtra(string memory key) internal override {
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifierAddr);
        vm.serializeAddress(key, "NitroValidator", nitroValidatorAddr);
    }

    function _logHeader() internal view override {
        console.log("=== Deploying Dev Infrastructure (WITH NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("NitroValidator:", nitroValidatorAddr);
        console.log("NitroEnclaveVerifier (rollback):", nitroEnclaveVerifierAddr);
        console.log("");
        console.log("NOTE: Using REAL TEEProverRegistry - hinted attestation REQUIRED.");
    }

    function _printSummary() internal view override {
        console.log("\n========================================");
        console.log("   DEV DEPLOYMENT COMPLETE (WITH NITRO)");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  NitroValidator:", nitroValidatorAddr);
        console.log("  NitroEnclaveVerifier (rollback):", nitroEnclaveVerifierAddr);
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
        console.log("\n>>> NEXT STEP: Cache certificates, then register the signer <<<");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerSigner(bytes,bytes,bytes)" <ATTESTATION_TBS> <SIGNATURE> <HINTS>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n========================================\n");
    }
}
