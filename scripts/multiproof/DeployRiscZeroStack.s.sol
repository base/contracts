// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployRiscZeroStack
 * @notice Deploys the RISC Zero verifier stack and NitroEnclaveVerifier.
 *
 * This script is separated from the main deployment scripts because the RISC Zero
 * set verifier and NitroEnclaveVerifier contracts (via ISP1Verifier) require
 * Solidity ^0.8.20, while the main deployment scripts and their transitive
 * dependencies are pinned to =0.8.15.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * USAGE
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 *   forge script scripts/multiproof/DeployRiscZeroStack.s.sol:DeployRiscZeroStack \
 *     --sig "run(address,bytes32,bytes32,bytes32,address,bytes32)" \
 *     <OWNER> <SET_BUILDER_IMAGE_ID> <NITRO_ROOT_CERT> <NITRO_VERIFIER_ID> \
 *     <NITRO_PROOF_SUBMITTER> <NITRO_VERIFIER_PROOF_ID> \
 *     --broadcast --rpc-url <RPC_URL> --private-key <DEPLOYER_KEY>
 *
 * Outputs:
 *   - RiscZeroGroth16Verifier, RiscZeroSetVerifier, RiscZeroVerifierRouter
 *   - NitroEnclaveVerifier (set `nitroEnclaveVerifier` in deploy config to this address)
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import { IRiscZeroSelectable } from "lib/risc0-ethereum/contracts/src/IRiscZeroSelectable.sol";
import { RiscZeroVerifierRouter } from "lib/risc0-ethereum/contracts/src/RiscZeroVerifierRouter.sol";
import { RiscZeroGroth16Verifier } from "lib/risc0-ethereum/contracts/src/groth16/RiscZeroGroth16Verifier.sol";
import { ControlID } from "lib/risc0-ethereum/contracts/src/groth16/ControlID.sol";
import { RiscZeroSetVerifier } from "lib/risc0-ethereum/contracts/src/RiscZeroSetVerifier.sol";

import {
    NitroEnclaveVerifier,
    ZkCoProcessorType,
    ZkCoProcessorConfig
} from "src/multiproof/tee/NitroEnclaveVerifier.sol";

/// @title DeployRiscZeroStack
/// @notice Deploys the RISC Zero verifier stack + NitroEnclaveVerifier for use with DeployDevWithNitro.
contract DeployRiscZeroStack is Script {
    /// @notice Maximum attestation age accepted by the NitroEnclaveVerifier (1 hour).
    uint64 public constant NITRO_MAX_TIME_DIFF = 3600;

    address public groth16Verifier;
    address public setVerifier;
    address public router;
    address public nitroEnclaveVerifier;

    /// @param owner               Owner for the Router and NitroEnclaveVerifier.
    /// @param setBuilderImageId   RISC Zero set builder image ID (from Boundless deployment).
    /// @param nitroRootCert       SHA-256 hash of the AWS Nitro root certificate.
    /// @param nitroVerifierId     RISC Zero image ID of the attestation verifier guest.
    /// @param nitroProofSubmitter Address authorized to submit proofs (e.g. TEE registrar).
    /// @param nitroVerifierProofId Proof ID for batch verification (bytes32(0) if unused).
    function run(
        address owner,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        address nitroProofSubmitter,
        bytes32 nitroVerifierProofId
    ) public {
        require(owner != address(0), "owner must be non-zero");
        require(setBuilderImageId != bytes32(0), "setBuilderImageId must be non-zero");
        require(nitroRootCert != bytes32(0), "nitroRootCert must be non-zero");
        require(nitroVerifierId != bytes32(0), "nitroVerifierId must be non-zero");
        require(nitroProofSubmitter != address(0), "nitroProofSubmitter must be non-zero");

        console.log("=== Deploying RISC Zero Verifier Stack + NitroEnclaveVerifier ===");
        console.log("Owner:", owner);
        console.log("Set Builder Image ID:", vm.toString(setBuilderImageId));
        console.log("Nitro Root Cert:", vm.toString(nitroRootCert));
        console.log("Nitro Verifier ID:", vm.toString(nitroVerifierId));
        console.log("Nitro Proof Submitter:", nitroProofSubmitter);
        console.log("");

        vm.startBroadcast();

        // ── RISC Zero verifier stack ──────────────────────────────────────

        // 1. Deploy Groth16 verifier (uses version-locked ControlID constants)
        groth16Verifier = address(
            new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID)
        );
        console.log("RiscZeroGroth16Verifier:", groth16Verifier);

        // 2. Deploy set verifier (delegates root verification to Groth16 verifier)
        setVerifier = address(
            new RiscZeroSetVerifier(IRiscZeroVerifier(groth16Verifier), setBuilderImageId, "")
        );
        console.log("RiscZeroSetVerifier:", setVerifier);

        // 3. Deploy router and register both verifiers
        RiscZeroVerifierRouter routerContract = new RiscZeroVerifierRouter(owner);
        router = address(routerContract);
        console.log("RiscZeroVerifierRouter:", router);

        bytes4 groth16Selector = IRiscZeroSelectable(groth16Verifier).SELECTOR();
        bytes4 setVerifierSelector = IRiscZeroSelectable(setVerifier).SELECTOR();
        routerContract.addVerifier(groth16Selector, IRiscZeroVerifier(groth16Verifier));
        routerContract.addVerifier(setVerifierSelector, IRiscZeroVerifier(setVerifier));
        console.log("  Groth16 selector:", vm.toString(groth16Selector));
        console.log("  SetVerifier selector:", vm.toString(setVerifierSelector));

        // ── NitroEnclaveVerifier ──────────────────────────────────────────

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: nitroVerifierId,
            aggregatorId: bytes32(0),
            zkVerifier: router
        });

        // Start with an empty trusted certs array; certs will be auto-cached on first valid proof.
        bytes32[] memory trustedCerts = new bytes32[](0);

        nitroEnclaveVerifier = address(
            new NitroEnclaveVerifier(
                owner,
                NITRO_MAX_TIME_DIFF,
                trustedCerts,
                nitroRootCert,
                nitroProofSubmitter,
                ZkCoProcessorType.RiscZero,
                zkConfig,
                nitroVerifierProofId
            )
        );
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifier);

        vm.stopBroadcast();

        // Print summary
        console.log("");
        console.log("========================================");
        console.log("   RISC ZERO STACK + NITRO DEPLOYED");
        console.log("========================================");
        console.log("RiscZeroGroth16Verifier:", groth16Verifier);
        console.log("RiscZeroSetVerifier:", setVerifier);
        console.log("RiscZeroVerifierRouter:", router);
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifier);
        console.log("");
        console.log(">>> Set nitroEnclaveVerifier in deploy config to:", nitroEnclaveVerifier);
        console.log(">>> Then run DeployDevWithNitro.s.sol <<<");
        console.log("========================================");

        // Write output
        string memory json = string.concat(
            '{"RiscZeroGroth16Verifier":"', vm.toString(groth16Verifier),
            '","RiscZeroSetVerifier":"', vm.toString(setVerifier),
            '","RiscZeroVerifierRouter":"', vm.toString(router),
            '","NitroEnclaveVerifier":"', vm.toString(nitroEnclaveVerifier),
            '"}'
        );
        string memory outPath = string.concat(
            "deployments/", vm.toString(block.chainid), "-risc0-stack.json"
        );
        vm.writeFile(outPath, json);
        console.log("Deployment saved to:", outPath);
    }
}
