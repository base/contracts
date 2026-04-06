// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployRiscZeroStack
 * @notice Deploys a RiscZeroSetVerifier and NitroEnclaveVerifier that work with
 *         an existing RISC Zero verifier router (e.g. the Boundless-deployed
 *         Router on Sepolia).
 *
 * This script is separated from the main deployment scripts because the
 * RiscZeroSetVerifier and NitroEnclaveVerifier contracts (via ISP1Verifier)
 * require Solidity ^0.8.20, while the main deployment scripts and their
 * transitive dependencies are pinned to =0.8.15.
 *
 * The RISC Zero Groth16 verifier and Router are NOT deployed by this script.
 * They are assumed to already exist on-chain (typically deployed by the
 * Boundless marketplace). Pass the router address as a parameter.
 *
 * A local RiscZeroSetVerifier is deployed that delegates root seal verification
 * to the existing Router. This is necessary because the Boundless-deployed
 * SetVerifier may have an outdated inner Groth16 verifier. By routing through
 * the Router, root seals are dispatched to the correct Groth16 verifier
 * regardless of version.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * USAGE
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 *   forge script scripts/multiproof/DeployRiscZeroStack.s.sol:DeployRiscZeroStack \
 *     --sig "run(address,address,bytes32,bytes32,bytes32)" \
 *     <OWNER> <RISC0_VERIFIER_ROUTER> <SET_BUILDER_IMAGE_ID> \
 *     <NITRO_ROOT_CERT> <NITRO_VERIFIER_ID> \
 *     --broadcast --rpc-url <RPC_URL> --private-key <DEPLOYER_KEY>
 *
 * NOTE: The deployer MUST be the same address as OWNER, since the script calls
 *       addVerifyRoute() on the NitroEnclaveVerifier (onlyOwner).
 *
 * Outputs:
 *   - RiscZeroSetVerifier (delegates to existing Router for root verification)
 *   - NitroEnclaveVerifier with route wired to the local SetVerifier
 *
 * POST-DEPLOY:
 *   After deploying TEEProverRegistry via DeployDevWithNitro.s.sol, update the
 *   proofSubmitter on NitroEnclaveVerifier to the TEEProverRegistry address:
 *
 *     cast send <NITRO_ENCLAVE_VERIFIER> "setProofSubmitter(address)" <TEE_PROVER_REGISTRY> \
 *       --rpc-url <RPC_URL> --private-key <OWNER_KEY>
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import { RiscZeroSetVerifier, RiscZeroSetVerifierLib } from "lib/risc0-ethereum/contracts/src/RiscZeroSetVerifier.sol";

import {
    NitroEnclaveVerifier,
    ZkCoProcessorType,
    ZkCoProcessorConfig
} from "src/multiproof/tee/NitroEnclaveVerifier.sol";

/// @title DeployRiscZeroStack
/// @notice Deploys RiscZeroSetVerifier + NitroEnclaveVerifier using an existing Router.
contract DeployRiscZeroStack is Script {
    /// @notice Maximum attestation age accepted by the NitroEnclaveVerifier (1 hour).
    uint64 public constant NITRO_MAX_TIME_DIFF = 3600;

    address public setVerifier;
    address public nitroEnclaveVerifier;

    /// @param owner                Owner for the NitroEnclaveVerifier (must equal msg.sender).
    /// @param risc0VerifierRouter  Address of an existing RISC Zero verifier router
    ///                             (e.g. Boundless-deployed Router).
    /// @param setBuilderImageId    RISC Zero set builder image ID (from Boundless deployment).
    /// @param nitroRootCert        SHA-256 hash of the AWS Nitro root certificate.
    /// @param nitroVerifierId      RISC Zero image ID of the attestation verifier guest.
    function run(
        address owner,
        address risc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId
    )
        public
    {
        require(owner != address(0), "owner must be non-zero");
        require(risc0VerifierRouter != address(0), "risc0VerifierRouter must be non-zero");
        require(setBuilderImageId != bytes32(0), "setBuilderImageId must be non-zero");
        require(nitroRootCert != bytes32(0), "nitroRootCert must be non-zero");
        require(nitroVerifierId != bytes32(0), "nitroVerifierId must be non-zero");

        bytes4 setVerifierSelector = RiscZeroSetVerifierLib.selector(setBuilderImageId);

        console.log("=== Deploying RiscZeroSetVerifier + NitroEnclaveVerifier ===");
        console.log("Owner:", owner);
        console.log("RISC Zero Verifier Router:", risc0VerifierRouter);
        console.log("Set Builder Image ID:", vm.toString(setBuilderImageId));
        console.log("Set Verifier Selector:", vm.toString(setVerifierSelector));
        console.log("Nitro Root Cert:", vm.toString(nitroRootCert));
        console.log("Nitro Verifier ID:", vm.toString(nitroVerifierId));
        console.log("");
        console.log("NOTE: proofSubmitter is set to owner as placeholder.");
        console.log("      Update it to TEEProverRegistry after deploying via setProofSubmitter().");
        console.log("");

        vm.startBroadcast();

        // ── RiscZeroSetVerifier
        // ──────────────────────────────────────────────
        //
        // Deploy a SetVerifier whose inner VERIFIER is the Router. This ensures
        // root seals are dispatched to the correct Groth16 verifier regardless
        // of version, avoiding selector mismatches when the Boundless provers
        // upgrade to newer Groth16 ControlIDs.
        setVerifier = address(new RiscZeroSetVerifier(IRiscZeroVerifier(risc0VerifierRouter), setBuilderImageId, ""));
        console.log("RiscZeroSetVerifier:", setVerifier);
        console.log("  SELECTOR:", vm.toString(setVerifierSelector));

        // ── NitroEnclaveVerifier
        // ─────────────────────────────────────────────

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: nitroVerifierId, aggregatorId: bytes32(0), zkVerifier: risc0VerifierRouter
        });

        // Start with empty trusted certs and expiries arrays; certs will be auto-cached on first valid proof.
        bytes32[] memory trustedCerts = new bytes32[](0);
        uint64[] memory trustedCertExpiries = new uint64[](0);

        // Use owner as placeholder proofSubmitter; must be updated to TEEProverRegistry
        // address after deployment via setProofSubmitter().
        NitroEnclaveVerifier nev = new NitroEnclaveVerifier(
            owner,
            NITRO_MAX_TIME_DIFF,
            trustedCerts,
            trustedCertExpiries,
            nitroRootCert,
            owner,
            ZkCoProcessorType.RiscZero,
            zkConfig,
            bytes32(0)
        );
        nitroEnclaveVerifier = address(nev);
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifier);

        // ── Wire up the per-selector route
        // ───────────────────────────────────
        //
        // Set inclusion proofs from Boundless carry the SetVerifier selector as
        // their first 4 bytes. Route that selector to our local SetVerifier so
        // root seals go through the Router (which has the correct Groth16).
        nev.addVerifyRoute(ZkCoProcessorType.RiscZero, setVerifierSelector, setVerifier);
        console.log("  Route added: selector", vm.toString(setVerifierSelector), "->", setVerifier);

        vm.stopBroadcast();

        // Print summary
        console.log("");
        console.log("========================================");
        console.log("   RISC ZERO STACK + NITRO DEPLOYED");
        console.log("========================================");
        console.log("RiscZeroSetVerifier:", setVerifier);
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifier);
        console.log("RISC Zero Router (external):", risc0VerifierRouter);
        console.log("");
        console.log(">>> Set nitroEnclaveVerifier in deploy config to:", nitroEnclaveVerifier);
        console.log(">>> Then run DeployDevWithNitro.s.sol <<<");
        console.log(">>> Then call setProofSubmitter(TEEProverRegistry) on NitroEnclaveVerifier <<<");
        console.log("========================================");

        // Write output
        string memory json = string.concat(
            '{"RiscZeroSetVerifier":"',
            vm.toString(setVerifier),
            '","NitroEnclaveVerifier":"',
            vm.toString(nitroEnclaveVerifier),
            '","RiscZeroVerifierRouter":"',
            vm.toString(risc0VerifierRouter),
            '"}'
        );
        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-risc0-stack.json");
        vm.writeFile(outPath, json);
        console.log("Deployment saved to:", outPath);
    }
}
