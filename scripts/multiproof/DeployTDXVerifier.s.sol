// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployTDXVerifier
 * @notice Deploys the Solidity TDX policy verifier used by TEEProverRegistry.
 *
 * This script is separated from DeployDevWithTDX.s.sol because TDXVerifier imports
 * verifier interfaces that require Solidity ^0.8.20, while the multiproof stack is
 * pinned to Solidity 0.8.15.
 *
 * Usage:
 *
 *   forge script scripts/multiproof/DeployTDXVerifier.s.sol:DeployTDXVerifier \
 *     --sig "run(address,address,bytes32,bytes32)" \
 *     <OWNER> <RISC0_VERIFIER_ROUTER> <TDX_VERIFIER_ID> <INTEL_ROOT_CA_HASH> \
 *     --rpc-url <RPC_URL> --broadcast --private-key <DEPLOYER_KEY>
 *
 * After running DeployDevWithTDX.s.sol, the TDX verifier's proofSubmitter is
 * updated to the deployed TEEProverRegistry.
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { ZkCoProcessorConfig, ZkCoProcessorType } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { TDXTcbStatus } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract DeployTDXVerifier is Script {
    /// @notice Maximum TDX quote age accepted by TDXVerifier.
    uint64 internal constant TDX_MAX_TIME_DIFF = 3600;

    /// @param owner Owner for TDXVerifier.
    /// @param risc0VerifierRouter Existing RISC Zero verifier router.
    /// @param tdxVerifierId RISC Zero image ID for the TDX DCAP verifier guest.
    /// @param intelRootCaHash Hash of the trusted Intel root CA consumed by the guest.
    function run(address owner, address risc0VerifierRouter, bytes32 tdxVerifierId, bytes32 intelRootCaHash) public {
        require(owner != address(0), "owner must be non-zero");
        require(risc0VerifierRouter != address(0), "risc0VerifierRouter must be non-zero");
        require(tdxVerifierId != bytes32(0), "tdxVerifierId must be non-zero");
        require(intelRootCaHash != bytes32(0), "intelRootCaHash must be non-zero");

        TDXTcbStatus[] memory allowedStatuses = new TDXTcbStatus[](2);
        allowedStatuses[0] = TDXTcbStatus.UpToDate;
        allowedStatuses[1] = TDXTcbStatus.SwHardeningNeeded;

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: tdxVerifierId, aggregatorId: bytes32(0), zkVerifier: risc0VerifierRouter
        });

        console.log("=== Deploying TDXVerifier ===");
        console.log("Owner:", owner);
        console.log("RISC Zero Verifier Router:", risc0VerifierRouter);
        console.log("TDX Verifier ID:", vm.toString(tdxVerifierId));
        console.log("Intel Root CA Hash:", vm.toString(intelRootCaHash));
        console.log("Max Time Diff:", TDX_MAX_TIME_DIFF);
        console.log("");

        vm.startBroadcast();

        address tdxVerifier = address(
            new TDXVerifier(
                owner, TDX_MAX_TIME_DIFF, intelRootCaHash, owner, ZkCoProcessorType.RiscZero, zkConfig, allowedStatuses
            )
        );

        vm.stopBroadcast();

        console.log("TDXVerifier:", tdxVerifier);
        console.log("");
        console.log(">>> Use this address as the DeployDevWithTDX.s.sol argument <<<");

        _writeOutput(tdxVerifier, risc0VerifierRouter, tdxVerifierId, intelRootCaHash);
    }

    function _writeOutput(
        address tdxVerifier,
        address risc0VerifierRouter,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        internal
    {
        string memory key = "deployment";
        vm.serializeAddress(key, "TDXVerifier", tdxVerifier);
        vm.serializeAddress(key, "RiscZeroVerifierRouter", risc0VerifierRouter);
        vm.serializeBytes32(key, "TDXVerifierId", tdxVerifierId);
        vm.serializeBytes32(key, "IntelRootCaHash", intelRootCaHash);
        string memory json = vm.serializeUint(key, "MaxTimeDiff", TDX_MAX_TIME_DIFF);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-tdx-verifier.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
