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
 *     --sig "run(address,bytes32,bytes32)" \
 *     <RISC0_VERIFIER_ROUTER> <TDX_VERIFIER_ID> <INTEL_ROOT_CA_HASH> \
 *     --rpc-url <RPC_URL> --broadcast --private-key <DEPLOYER_KEY>
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract DeployTDXVerifier is Script {
    /// @notice Maximum TDX quote age accepted by TDXVerifier.
    uint64 internal constant TDX_MAX_TIME_DIFF = 3600;

    /// @param risc0VerifierRouter Existing RISC Zero verifier router.
    /// @param tdxVerifierId RISC Zero image ID for the TDX DCAP verifier guest.
    /// @param intelRootCaHash Hash of the trusted Intel root CA consumed by the guest.
    function run(address risc0VerifierRouter, bytes32 tdxVerifierId, bytes32 intelRootCaHash) public {
        require(risc0VerifierRouter != address(0), "risc0VerifierRouter must be non-zero");
        require(tdxVerifierId != bytes32(0), "tdxVerifierId must be non-zero");
        require(intelRootCaHash != bytes32(0), "intelRootCaHash must be non-zero");

        console.log("=== Deploying TDXVerifier ===");
        console.log("RISC Zero Verifier Router:", risc0VerifierRouter);
        console.log("TDX Verifier ID:", vm.toString(tdxVerifierId));
        console.log("Intel Root CA Hash:", vm.toString(intelRootCaHash));
        console.log("Max Time Diff:", TDX_MAX_TIME_DIFF);
        console.log("");

        vm.startBroadcast();

        address tdxVerifier =
            address(new TDXVerifier(TDX_MAX_TIME_DIFF, intelRootCaHash, risc0VerifierRouter, tdxVerifierId));

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
