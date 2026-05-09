// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployTEEVerifiers
 * @notice Deploys NitroEnclaveVerifier and TDXVerifier in one broadcast.
 *
 * Usage:
 *
 *   forge script scripts/multiproof/DeployTEEVerifiers.s.sol:DeployTEEVerifiers \
 *     --sig "run(address,address,bytes32,bytes32,bytes32,bytes32,bytes32)" \
 *     <OWNER> <RISC0_VERIFIER_ROUTER> <SET_BUILDER_IMAGE_ID> \
 *     <NITRO_ROOT_CERT> <NITRO_VERIFIER_ID> \
 *     <TDX_VERIFIER_ID> <INTEL_ROOT_CA_HASH> \
 *     --rpc-url <RPC_URL> --broadcast --private-key <DEPLOYER_KEY>
 *
 * If using batched Nitro proofs, use the overload that also supplies
 * <NITRO_VERIFIER_PROOF_ID>:
 *
 *   --sig "run(address,address,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)"
 *
 * If Nitro and TDX use different RISC Zero verifier routers, use:
 *
 *   --sig "run(address,address,address,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)"
 *
 * The broadcaster must be the owner because this script calls addVerifyRoute()
 * on the freshly deployed NitroEnclaveVerifier. Use the longest overload when
 * Nitro and TDX proofs use different RISC Zero verifier router contracts.
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import { RiscZeroSetVerifier, RiscZeroSetVerifierLib } from "lib/risc0-ethereum/contracts/src/RiscZeroSetVerifier.sol";

import { ZkCoProcessorConfig, ZkCoProcessorType } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { TDXTcbStatus } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { NitroEnclaveVerifier } from "src/L1/proofs/tee/NitroEnclaveVerifier.sol";
import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract DeployTEEVerifiers is Script {
    /// @notice Maximum Nitro attestation age accepted by NitroEnclaveVerifier.
    uint64 internal constant NITRO_MAX_TIME_DIFF = 3600;

    /// @notice Maximum TDX quote age accepted by TDXVerifier.
    uint64 internal constant TDX_MAX_TIME_DIFF = 3600;

    address public setVerifier;
    address public nitroEnclaveVerifier;
    address public tdxVerifier;

    function run(
        address owner,
        address risc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        public
    {
        run(
            owner,
            risc0VerifierRouter,
            setBuilderImageId,
            nitroRootCert,
            nitroVerifierId,
            bytes32(0),
            tdxVerifierId,
            intelRootCaHash
        );
    }

    function run(
        address owner,
        address nitroRisc0VerifierRouter,
        address tdxRisc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        public
    {
        run(
            owner,
            nitroRisc0VerifierRouter,
            tdxRisc0VerifierRouter,
            setBuilderImageId,
            nitroRootCert,
            nitroVerifierId,
            bytes32(0),
            tdxVerifierId,
            intelRootCaHash
        );
    }

    function run(
        address owner,
        address risc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 nitroVerifierProofId,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        public
    {
        run(
            owner,
            risc0VerifierRouter,
            risc0VerifierRouter,
            setBuilderImageId,
            nitroRootCert,
            nitroVerifierId,
            nitroVerifierProofId,
            tdxVerifierId,
            intelRootCaHash
        );
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
        bytes4 setVerifierSelector = _validateInputs(
            owner,
            nitroRisc0VerifierRouter,
            tdxRisc0VerifierRouter,
            setBuilderImageId,
            nitroRootCert,
            nitroVerifierId,
            tdxVerifierId,
            intelRootCaHash
        );

        console.log("=== Deploying TEE Verifiers ===");
        console.log("Owner:", owner);
        console.log("Nitro RISC Zero Verifier Router:", nitroRisc0VerifierRouter);
        console.log("TDX RISC Zero Verifier Router:", tdxRisc0VerifierRouter);
        console.log("Set Builder Image ID:", vm.toString(setBuilderImageId));
        console.log("Set Verifier Selector:", vm.toString(setVerifierSelector));
        console.log("Nitro Root Cert:", vm.toString(nitroRootCert));
        console.log("Nitro Verifier ID:", vm.toString(nitroVerifierId));
        console.log("Nitro Verifier Proof ID:", vm.toString(nitroVerifierProofId));
        console.log("TDX Verifier ID:", vm.toString(tdxVerifierId));
        console.log("Intel Root CA Hash:", vm.toString(intelRootCaHash));
        console.log("");
        console.log("NOTE: proofSubmitter is set to owner as placeholder on both verifiers.");
        console.log("      DeployDevWithTDX.s.sol updates both to TEEProverRegistry.");
        console.log("");

        vm.startBroadcast();

        (setVerifier, nitroEnclaveVerifier) = _deployNitroVerifier(
            owner, nitroRisc0VerifierRouter, setBuilderImageId, nitroRootCert, nitroVerifierId, nitroVerifierProofId
        );
        tdxVerifier = _deployTDXVerifier(owner, tdxRisc0VerifierRouter, tdxVerifierId, intelRootCaHash);

        vm.stopBroadcast();

        console.log("RiscZeroSetVerifier:", setVerifier);
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifier);
        console.log("TDXVerifier:", tdxVerifier);
        console.log("");
        console.log(">>> Use these verifier addresses for the TDX multiproof deployment <<<");

        _writeOutput(
            setVerifier,
            nitroEnclaveVerifier,
            tdxVerifier,
            nitroRisc0VerifierRouter,
            tdxRisc0VerifierRouter,
            setBuilderImageId,
            setVerifierSelector,
            nitroRootCert,
            nitroVerifierId,
            nitroVerifierProofId,
            tdxVerifierId,
            intelRootCaHash
        );
    }

    function _deployNitroVerifier(
        address owner,
        address risc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 nitroVerifierProofId
    )
        internal
        returns (address deployedSetVerifier, address deployedNitroVerifier)
    {
        deployedSetVerifier =
            address(new RiscZeroSetVerifier(IRiscZeroVerifier(risc0VerifierRouter), setBuilderImageId, ""));

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: nitroVerifierId, aggregatorId: bytes32(0), zkVerifier: risc0VerifierRouter
        });

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
        deployedNitroVerifier = address(verifier);

        bytes4 setVerifierSelector = RiscZeroSetVerifierLib.selector(setBuilderImageId);
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, setVerifierSelector, deployedSetVerifier);
    }

    function _deployTDXVerifier(
        address owner,
        address risc0VerifierRouter,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        internal
        returns (address deployedTDXVerifier)
    {
        TDXTcbStatus[] memory allowedStatuses = new TDXTcbStatus[](2);
        allowedStatuses[0] = TDXTcbStatus.UpToDate;
        allowedStatuses[1] = TDXTcbStatus.SwHardeningNeeded;

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: tdxVerifierId, aggregatorId: bytes32(0), zkVerifier: risc0VerifierRouter
        });

        deployedTDXVerifier = address(
            new TDXVerifier(
                owner, TDX_MAX_TIME_DIFF, intelRootCaHash, owner, ZkCoProcessorType.RiscZero, zkConfig, allowedStatuses
            )
        );
    }

    function _validateInputs(
        address owner,
        address nitroRisc0VerifierRouter,
        address tdxRisc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        internal
        pure
        returns (bytes4 setVerifierSelector)
    {
        require(owner != address(0), "owner must be non-zero");
        require(nitroRisc0VerifierRouter != address(0), "nitroRisc0VerifierRouter must be non-zero");
        require(tdxRisc0VerifierRouter != address(0), "tdxRisc0VerifierRouter must be non-zero");
        require(setBuilderImageId != bytes32(0), "setBuilderImageId must be non-zero");
        require(nitroRootCert != bytes32(0), "nitroRootCert must be non-zero");
        require(nitroVerifierId != bytes32(0), "nitroVerifierId must be non-zero");
        require(tdxVerifierId != bytes32(0), "tdxVerifierId must be non-zero");
        require(intelRootCaHash != bytes32(0), "intelRootCaHash must be non-zero");

        setVerifierSelector = RiscZeroSetVerifierLib.selector(setBuilderImageId);
    }

    function _writeOutput(
        address deployedSetVerifier,
        address deployedNitroVerifier,
        address deployedTDXVerifier,
        address nitroRisc0VerifierRouter,
        address tdxRisc0VerifierRouter,
        bytes32 setBuilderImageId,
        bytes4 setVerifierSelector,
        bytes32 nitroRootCert,
        bytes32 nitroVerifierId,
        bytes32 nitroVerifierProofId,
        bytes32 tdxVerifierId,
        bytes32 intelRootCaHash
    )
        internal
    {
        string memory key = "deployment";
        vm.serializeAddress(key, "RiscZeroSetVerifier", deployedSetVerifier);
        vm.serializeAddress(key, "NitroEnclaveVerifier", deployedNitroVerifier);
        vm.serializeAddress(key, "TDXVerifier", deployedTDXVerifier);
        vm.serializeAddress(key, "RiscZeroVerifierRouter", tdxRisc0VerifierRouter);
        vm.serializeAddress(key, "NitroRiscZeroVerifierRouter", nitroRisc0VerifierRouter);
        vm.serializeAddress(key, "TDXRiscZeroVerifierRouter", tdxRisc0VerifierRouter);
        vm.serializeBytes32(key, "RiscZeroSetBuilderImageId", setBuilderImageId);
        vm.serializeString(key, "RiscZeroSetVerifierSelector", vm.toString(setVerifierSelector));
        vm.serializeBytes32(key, "NitroRootCert", nitroRootCert);
        vm.serializeBytes32(key, "NitroVerifierId", nitroVerifierId);
        vm.serializeBytes32(key, "NitroVerifierProofId", nitroVerifierProofId);
        vm.serializeBytes32(key, "TDXVerifierId", tdxVerifierId);
        vm.serializeBytes32(key, "IntelRootCaHash", intelRootCaHash);
        vm.serializeUint(key, "NitroMaxTimeDiff", NITRO_MAX_TIME_DIFF);
        string memory json = vm.serializeUint(key, "TDXMaxTimeDiff", TDX_MAX_TIME_DIFF);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-tee-verifiers.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
