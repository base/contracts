// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title DeployNitroVerifier
 * @notice Deploys the RISC Zero set verifier route and NitroEnclaveVerifier used by TEEProverRegistry.
 *
 * This script is separated from the main multiproof deployment scripts because
 * NitroEnclaveVerifier imports verifier interfaces that require Solidity ^0.8.20,
 * while the multiproof stack is pinned to Solidity 0.8.15.
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import { RiscZeroSetVerifier, RiscZeroSetVerifierLib } from "lib/risc0-ethereum/contracts/src/RiscZeroSetVerifier.sol";

import { ZkCoProcessorConfig, ZkCoProcessorType } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { NitroEnclaveVerifier } from "src/L1/proofs/tee/NitroEnclaveVerifier.sol";

contract DeployNitroVerifier is Script {
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

        ZkCoProcessorConfig memory zkConfig = ZkCoProcessorConfig({
            verifierId: nitroVerifierId, aggregatorId: bytes32(0), zkVerifier: risc0VerifierRouter
        });

        vm.startBroadcast();

        address setVerifier =
            address(new RiscZeroSetVerifier(IRiscZeroVerifier(risc0VerifierRouter), setBuilderImageId, ""));
        NitroEnclaveVerifier verifier = new NitroEnclaveVerifier(
            owner,
            1 hours,
            new bytes32[](0),
            new uint64[](0),
            nitroRootCert,
            owner,
            address(0),
            ZkCoProcessorType.RiscZero,
            zkConfig,
            nitroVerifierProofId
        );

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, setVerifierSelector, setVerifier);

        vm.stopBroadcast();

        console.log("RiscZeroSetVerifier:", setVerifier);
        console.log("NitroEnclaveVerifier:", address(verifier));

        string memory key = "deployment";
        vm.serializeAddress(key, "RiscZeroSetVerifier", setVerifier);
        string memory json = vm.serializeAddress(key, "NitroEnclaveVerifier", address(verifier));

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-nitro-verifier.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
