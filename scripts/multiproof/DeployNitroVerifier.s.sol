// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployNitroVerifier
 * @notice Deploys the RISC Zero set verifier route and NitroEnclaveVerifier used by TEEProverRegistry.
 *
 * This script is separated from the main multiproof deployment scripts because
 * NitroEnclaveVerifier imports verifier interfaces that require Solidity ^0.8.20,
 * while the multiproof stack is pinned to Solidity 0.8.15.
 *
 * Usage:
 *
 *   forge script scripts/multiproof/DeployNitroVerifier.s.sol:DeployNitroVerifier \
 *     --sig "run(address,address,bytes32,bytes32,bytes32,bytes32)" \
 *     <OWNER> <RISC0_VERIFIER_ROUTER> <SET_BUILDER_IMAGE_ID> \
 *     <NITRO_ROOT_CERT> <NITRO_VERIFIER_ID> <NITRO_VERIFIER_PROOF_ID> \
 *     --rpc-url <RPC_URL> --broadcast --private-key <DEPLOYER_KEY>
 *
 * Use bytes32(0) for <NITRO_VERIFIER_PROOF_ID> unless using batched Nitro proofs.
 *
 * The broadcaster must be the owner because this script calls addVerifyRoute()
 * on the freshly deployed NitroEnclaveVerifier.
 *
 * After running DeployDevWithTDX.s.sol, the Nitro verifier's proofSubmitter is
 * updated to the deployed TEEProverRegistry.
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import { RiscZeroSetVerifier, RiscZeroSetVerifierLib } from "lib/risc0-ethereum/contracts/src/RiscZeroSetVerifier.sol";

import { ZkCoProcessorConfig, ZkCoProcessorType } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { NitroEnclaveVerifier } from "src/L1/proofs/tee/NitroEnclaveVerifier.sol";

contract DeployNitroVerifier is Script {
    /// @notice Maximum Nitro attestation age accepted by NitroEnclaveVerifier.
    uint64 internal constant NITRO_MAX_TIME_DIFF = 1 hours;

    /// @param owner Owner for NitroEnclaveVerifier. Must be the broadcaster for route setup.
    /// @param risc0VerifierRouter Existing RISC Zero verifier router.
    /// @param setBuilderImageId RISC Zero set builder image ID.
    /// @param nitroRootCert SHA-256 hash of the AWS Nitro root certificate.
    /// @param nitroVerifierId RISC Zero image ID for the Nitro attestation verifier guest.
    /// @param nitroVerifierProofId Optional verifier proof ID used by Nitro batch verification.
    function run(
        address owner,
        address risc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 nitroVerifierProofId
    )
        public
    {
        require(owner != address(0), "owner must be non-zero");
        require(risc0VerifierRouter != address(0), "risc0VerifierRouter must be non-zero");
        require(setBuilderImageId != bytes32(0), "setBuilderImageId must be non-zero");
        require(nitroRootCert != bytes32(0), "nitroRootCert must be non-zero");
        require(nitroVerifierId != bytes32(0), "nitroVerifierId must be non-zero");

        bytes4 setVerifierSelector = RiscZeroSetVerifierLib.selector(setBuilderImageId);

        console.log("=== Deploying NitroEnclaveVerifier ===");
        console.log("Owner:", owner);
        console.log("RISC Zero Verifier Router:", risc0VerifierRouter);
        console.log("Set Builder Image ID:", vm.toString(setBuilderImageId));
        console.log("Set Verifier Selector:", vm.toString(setVerifierSelector));
        console.log("Nitro Root Cert:", vm.toString(nitroRootCert));
        console.log("Nitro Verifier ID:", vm.toString(nitroVerifierId));
        console.log("Nitro Verifier Proof ID:", vm.toString(nitroVerifierProofId));
        console.log("Max Time Diff:", NITRO_MAX_TIME_DIFF);
        console.log("");
        console.log("NOTE: proofSubmitter is set to owner as placeholder.");
        console.log("      DeployDevWithTDX.s.sol updates it to TEEProverRegistry.");
        console.log("");

        string memory key = "deployment";
        vm.serializeAddress(key, "RiscZeroVerifierRouter", risc0VerifierRouter);
        vm.serializeBytes32(key, "RiscZeroSetBuilderImageId", setBuilderImageId);
        vm.serializeString(key, "RiscZeroSetVerifierSelector", vm.toString(setVerifierSelector));
        vm.serializeBytes32(key, "NitroRootCert", nitroRootCert);
        vm.serializeBytes32(key, "NitroVerifierId", nitroVerifierId);
        vm.serializeBytes32(key, "NitroVerifierProofId", nitroVerifierProofId);

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: nitroVerifierId, aggregatorId: bytes32(0), zkVerifier: risc0VerifierRouter
        });

        vm.startBroadcast();

        address setVerifier =
            address(new RiscZeroSetVerifier(IRiscZeroVerifier(risc0VerifierRouter), setBuilderImageId, ""));
        NitroEnclaveVerifier verifier = new NitroEnclaveVerifier(
            owner,
            NITRO_MAX_TIME_DIFF,
            new bytes32[](0),
            new uint64[](0),
            nitroRootCert,
            owner,
            address(0),
            ZkCoProcessorType.RiscZero,
            zkConfig,
            nitroVerifierProofId
        );
        address nitroEnclaveVerifier = address(verifier);

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, setVerifierSelector, setVerifier);

        vm.stopBroadcast();

        console.log("RiscZeroSetVerifier:", setVerifier);
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifier);
        console.log("");
        console.log(">>> Use this NitroEnclaveVerifier address in the deploy config <<<");

        vm.serializeAddress(key, "RiscZeroSetVerifier", setVerifier);
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifier);
        string memory json = vm.serializeUint(key, "MaxTimeDiff", NITRO_MAX_TIME_DIFF);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-nitro-verifier.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
