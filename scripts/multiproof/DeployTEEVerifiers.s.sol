// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployTEEVerifiers
 * @notice Deploys NitroEnclaveVerifier and TDXVerifier in one broadcast.
 *
 * Usage:
 *
 *   forge script scripts/multiproof/DeployTEEVerifiers.s.sol:DeployTEEVerifiers \
 *     --sig "run(address,address,address,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)" \
 *     <OWNER> <NITRO_RISC0_VERIFIER_ROUTER> <TDX_RISC0_VERIFIER_ROUTER> <SET_BUILDER_IMAGE_ID> \
 *     <NITRO_ROOT_CERT> <NITRO_VERIFIER_ID> \
 *     <NITRO_VERIFIER_PROOF_ID> <TDX_VERIFIER_ID> <INTEL_ROOT_CA_HASH> \
 *     --rpc-url <RPC_URL> --broadcast --private-key <DEPLOYER_KEY>
 *
 * The broadcaster must be the owner because this script calls addVerifyRoute()
 * on the freshly deployed NitroEnclaveVerifier.
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import { RiscZeroSetVerifier, RiscZeroSetVerifierLib } from "lib/risc0-ethereum/contracts/src/RiscZeroSetVerifier.sol";

import { ZkCoProcessorConfig, ZkCoProcessorType } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { NitroEnclaveVerifier } from "src/L1/proofs/tee/NitroEnclaveVerifier.sol";
import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract DeployTEEVerifiers is Script {
    /// @notice Maximum Nitro attestation and TDX quote age accepted by the verifiers.
    uint64 internal constant MAX_TIME_DIFF = 3600;

    struct Input {
        address owner;
        address nitroRisc0VerifierRouter;
        address tdxRisc0VerifierRouter;
        bytes32 setBuilderImageId;
        bytes32 nitroRootCert;
        bytes32 nitroVerifierId;
        bytes32 nitroVerifierProofId;
        bytes32 tdxVerifierId;
        bytes32 intelRootCaHash;
    }

    function run(
        address owner,
        address nitroRisc0VerifierRouter,
        address tdxRisc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 nitroVerifierProofId,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        public
    {
        Input memory input = Input({
            owner: owner,
            nitroRisc0VerifierRouter: nitroRisc0VerifierRouter,
            tdxRisc0VerifierRouter: tdxRisc0VerifierRouter,
            setBuilderImageId: setBuilderImageId,
            nitroRootCert: nitroRootCert,
            nitroVerifierId: nitroVerifierId,
            nitroVerifierProofId: nitroVerifierProofId,
            tdxVerifierId: tdxVerifierId,
            intelRootCaHash: intelRootCaHash
        });

        require(input.owner != address(0), "owner must be non-zero");
        require(input.nitroRisc0VerifierRouter != address(0), "nitroRisc0VerifierRouter must be non-zero");
        require(input.tdxRisc0VerifierRouter != address(0), "tdxRisc0VerifierRouter must be non-zero");
        require(input.setBuilderImageId != bytes32(0), "setBuilderImageId must be non-zero");
        require(input.nitroRootCert != bytes32(0), "nitroRootCert must be non-zero");
        require(input.nitroVerifierId != bytes32(0), "nitroVerifierId must be non-zero");
        require(input.tdxVerifierId != bytes32(0), "tdxVerifierId must be non-zero");
        require(input.intelRootCaHash != bytes32(0), "intelRootCaHash must be non-zero");

        bytes4 setVerifierSelector = RiscZeroSetVerifierLib.selector(input.setBuilderImageId);

        console.log("=== Deploying TEE Verifiers ===");
        console.log("Owner:", input.owner);
        console.log("Nitro RISC Zero Verifier Router:", input.nitroRisc0VerifierRouter);
        console.log("TDX RISC Zero Verifier Router:", input.tdxRisc0VerifierRouter);
        console.log("Set Builder Image ID:", vm.toString(input.setBuilderImageId));
        console.log("Set Verifier Selector:", vm.toString(setVerifierSelector));
        console.log("Nitro Root Cert:", vm.toString(input.nitroRootCert));
        console.log("Nitro Verifier ID:", vm.toString(input.nitroVerifierId));
        console.log("Nitro Verifier Proof ID:", vm.toString(input.nitroVerifierProofId));
        console.log("TDX Verifier ID:", vm.toString(input.tdxVerifierId));
        console.log("Intel Root CA Hash:", vm.toString(input.intelRootCaHash));
        console.log("");
        console.log("NOTE: Nitro proofSubmitter is set to owner as placeholder.");
        console.log("");

        vm.startBroadcast();

        address setVerifier = address(
            new RiscZeroSetVerifier(IRiscZeroVerifier(input.nitroRisc0VerifierRouter), input.setBuilderImageId, "")
        );

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: input.nitroVerifierId, aggregatorId: bytes32(0), zkVerifier: input.nitroRisc0VerifierRouter
        });

        NitroEnclaveVerifier nitroEnclaveVerifierContract = new NitroEnclaveVerifier(
            input.owner,
            MAX_TIME_DIFF,
            new bytes32[](0),
            new uint64[](0),
            input.nitroRootCert,
            input.owner,
            address(0),
            ZkCoProcessorType.RiscZero,
            zkConfig,
            input.nitroVerifierProofId
        );
        address nitroEnclaveVerifier = address(nitroEnclaveVerifierContract);
        nitroEnclaveVerifierContract.addVerifyRoute(ZkCoProcessorType.RiscZero, setVerifierSelector, setVerifier);

        address tdxVerifier = address(
            new TDXVerifier(MAX_TIME_DIFF, input.intelRootCaHash, input.tdxRisc0VerifierRouter, input.tdxVerifierId)
        );

        vm.stopBroadcast();

        console.log("RiscZeroSetVerifier:", setVerifier);
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifier);
        console.log("TDXVerifier:", tdxVerifier);
        console.log("");
        console.log(">>> Use these verifier addresses for the TDX multiproof deployment <<<");

        string memory key = "deployment";
        vm.serializeAddress(key, "RiscZeroSetVerifier", setVerifier);
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifier);
        vm.serializeAddress(key, "TDXVerifier", tdxVerifier);
        vm.serializeAddress(key, "NitroRiscZeroVerifierRouter", input.nitroRisc0VerifierRouter);
        vm.serializeAddress(key, "TDXRiscZeroVerifierRouter", input.tdxRisc0VerifierRouter);
        vm.serializeBytes32(key, "RiscZeroSetBuilderImageId", input.setBuilderImageId);
        vm.serializeString(key, "RiscZeroSetVerifierSelector", vm.toString(setVerifierSelector));
        vm.serializeBytes32(key, "NitroRootCert", input.nitroRootCert);
        vm.serializeBytes32(key, "NitroVerifierId", input.nitroVerifierId);
        vm.serializeBytes32(key, "NitroVerifierProofId", input.nitroVerifierProofId);
        vm.serializeBytes32(key, "TDXVerifierId", input.tdxVerifierId);
        vm.serializeBytes32(key, "IntelRootCaHash", input.intelRootCaHash);
        vm.serializeUint(key, "NitroMaxTimeDiff", MAX_TIME_DIFF);
        string memory json = vm.serializeUint(key, "TDXMaxTimeDiff", MAX_TIME_DIFF);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-tee-verifiers.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
