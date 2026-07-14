// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployTDXVerifier
 * @notice Deploys the Solidity TDX policy verifier used by TEEProverRegistry.
 *
 * This script is separated from DeployDevWithTDX.s.sol because TDXVerifier imports
 * verifier interfaces that require Solidity ^0.8.20, while the multiproof stack is
 * pinned to Solidity 0.8.15.
 */

import { Script } from "forge-std/Script.sol";

import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract DeployTDXVerifier is Script {
    /// @param risc0VerifierRouter Existing RISC Zero verifier router.
    /// @param tdxVerifierId RISC Zero image ID for the Confidential Space verifier guest.
    /// @param confidentialSpaceRootCaHash Hash of the trusted Confidential Space root CA consumed by the guest.
    function run(address risc0VerifierRouter, bytes32 tdxVerifierId, bytes32 confidentialSpaceRootCaHash) public {
        vm.startBroadcast();

        address tdxVerifier = address(
            new TDXVerifier(
                1 hours, confidentialSpaceRootCaHash, keccak256("base-tdx-prover"), risc0VerifierRouter, tdxVerifierId
            )
        );

        vm.stopBroadcast();

        string memory json = vm.serializeAddress("deployment", "TDXVerifier", tdxVerifier);
        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-tdx-verifier.json");
        vm.writeJson(json, outPath);
    }
}
