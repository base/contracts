// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/**
 * @title DeployDevWithNitro
 * @notice Development deployment WITH AWS Nitro attestation validation.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 *                            DEPLOYMENT TYPE: DEV (WITH NITRO)
 * ══════════════════════════════════════════════════════════════════════════════════
 *
 * This script deploys infrastructure using the REAL SystemConfigGlobal, which
 * REQUIRES a ZK proof of a valid AWS Nitro attestation for signer registration.
 * You cannot use addDevSigner() - you must go through the full registerSigner() flow.
 * A pre-deployed NitroEnclaveVerifier contract address must be provided in the config.
 *
 * USE THIS SCRIPT WHEN:
 * - Testing the full Nitro attestation flow end-to-end
 * - You have access to a real AWS Nitro enclave
 * - You want to validate the production registration process
 *
 * DO NOT USE THIS SCRIPT IF:
 * - You don't have a Nitro enclave available
 * - You just want quick local testing (use DeployDevNoNitro instead)
 *
 * NOTE: This still uses mocks for AnchorStateRegistry, DelayedWETH, and ZK Verifier,
 *       so it's not a full production deployment - just production-like for the
 *       Nitro attestation flow.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * STEP 1: Register the PCR0 (enclave image hash) - OWNER ONLY
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * The PCR0 is the raw 48-byte hash of the enclave image. You must register it
 * before any signers using that image can be registered.
 *
 *   # Get raw PCR0 bytes from the enclave (48 bytes, hex-encoded)
 *   PCR0_RAW="0x<48_bytes_hex>"
 *
 *   # Register it (only owner can do this)
 *   cast send $SYSTEM_CONFIG_GLOBAL "registerPCR0(bytes)" $PCR0_RAW \
 *     --private-key $OWNER_KEY --rpc-url $RPC_URL
 *
 * Note: The teeImageHash in deploy-config is keccak256(PCR0_RAW), not the raw bytes.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * STEP 2: Generate ZK proof of the attestation and register the signer
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * Generate a Risc0 ZK proof of the Nitro attestation offchain, then call:
 *
 *   cast send $SYSTEM_CONFIG_GLOBAL \
 *     "registerSigner(bytes,bytes)" $ZK_OUTPUT $ZK_PROOF_BYTES \
 *     --private-key $OWNER_OR_MANAGER_KEY --rpc-url $RPC_URL
 *
 * IMPORTANT: The attestation is only valid for 60 minutes! Generate the proof
 * and submit the transaction within that window.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * VERIFICATION
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * After registration, verify the signer is registered:
 *
 *   # Check if signer is valid
 *   cast call $SYSTEM_CONFIG_GLOBAL "isValidSigner(address)(bool)" $SIGNER_ADDRESS
 *
 *   # Get the PCR0 hash associated with the signer
 *   cast call $SYSTEM_CONFIG_GLOBAL "signerPCR0(address)(bytes32)" $SIGNER_ADDRESS
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * COMPARISON WITH DeployDevNoNitro
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * | Feature                    | DeployDevNoNitro      | DeployDevWithNitro    |
 * |----------------------------|----------------------|------------------------|
 * | SystemConfigGlobal         | DevSystemConfigGlobal | SystemConfigGlobal    |
 * | Signer registration        | addDevSigner()        | registerSigner()      |
 * | Requires Nitro enclave     | No                    | Yes                   |
 * | Validates attestation (ZK) | No                    | Yes                   |
 * | PCR0 pre-registration      | No                    | Yes                   |
 * | Attestation freshness      | N/A                   | < 60 minutes          |
 *
 * Both scripts use mocks for AnchorStateRegistry, DelayedWETH, and ZK Verifier.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 */

import {
    INitroEnclaveVerifier
} from "lib/aws-nitro-enclave-attestation/contracts/src/interfaces/INitroEnclaveVerifier.sol";
import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/dispute/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/dispute/IDisputeGame.sol";
import { DisputeGameFactory } from "src/dispute/DisputeGameFactory.sol";
import { GameType, Hash } from "src/dispute/lib/Types.sol";

import { DeployConfig } from "scripts/deploy/DeployConfig.s.sol";
import { Config } from "scripts/libraries/Config.sol";
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";

import { AggregateVerifier } from "src/multiproof/AggregateVerifier.sol";
import { IVerifier } from "interfaces/multiproof/IVerifier.sol";
import { MockVerifier } from "src/multiproof/mocks/MockVerifier.sol";
import { SystemConfigGlobal } from "src/multiproof/tee/SystemConfigGlobal.sol";
import { TEEVerifier } from "src/multiproof/tee/TEEVerifier.sol";

import { MinimalProxyAdmin } from "./mocks/MinimalProxyAdmin.sol";
import { MockAnchorStateRegistry } from "./mocks/MockAnchorStateRegistry.sol";
import { MockDelayedWETH } from "./mocks/MockDelayedWETH.sol";

/// @title DeployDevWithNitro
/// @notice Development deployment WITH AWS Nitro attestation validation.
/// @dev Uses real SystemConfigGlobal which requires registerSigner() with valid attestation.
contract DeployDevWithNitro is Script {
    /// @notice Constant from Optimism's Constants.sol - the storage slot for proxy admin.
    bytes32 internal constant PROXY_OWNER_ADDRESS = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;

    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 public constant PROOF_THRESHOLD = 1;
    uint256 public constant INIT_BOND = 0.001 ether;

    DeployConfig public constant cfg =
        DeployConfig(address(uint160(uint256(keccak256(abi.encode("optimism.deployconfig"))))));

    // Deployed addresses
    address public systemConfigGlobalProxy;
    address public teeVerifier;
    address public disputeGameFactory;
    address public mockAnchorRegistry;
    address public mockDelayedWETH;
    address public aggregateVerifier;

    function setUp() public {
        DeployUtils.etchLabelAndAllowCheatcodes({ _etchTo: address(cfg), _cname: "DeployConfig" });
        cfg.read(Config.deployConfigPath());
    }

    function run() public {
        GameType gameType = GameType.wrap(uint32(cfg.multiproofGameType()));

        console.log("=== Deploying Dev Infrastructure (WITH NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("");
        console.log("NOTE: Using REAL SystemConfigGlobal - ZK attestation proof REQUIRED.");
        console.log("NitroEnclaveVerifier:", cfg.nitroEnclaveVerifier());

        vm.startBroadcast();

        _deployTEEContracts(cfg.finalSystemOwner(), cfg.nitroEnclaveVerifier());
        _registerProposer(cfg.teeProposer());
        _deployInfrastructure(gameType);
        _deployAggregateVerifier(gameType);

        vm.stopBroadcast();

        _printSummary();
        _writeOutput();
    }

    function _deployTEEContracts(address owner, address _nitroEnclaveVerifier) internal {
        address scgImpl = address(new SystemConfigGlobal(INitroEnclaveVerifier(_nitroEnclaveVerifier)));
        console.log("NitroEnclaveVerifier (external):", _nitroEnclaveVerifier);
        systemConfigGlobalProxy = address(
            new TransparentUpgradeableProxy(
                scgImpl, address(0xdead), abi.encodeCall(SystemConfigGlobal.initialize, (owner, owner))
            )
        );
        console.log("SystemConfigGlobal:", systemConfigGlobalProxy);

        teeVerifier = address(new TEEVerifier(SystemConfigGlobal(systemConfigGlobalProxy)));
        console.log("TEEVerifier:", teeVerifier);
    }

    function _registerProposer(address teeProposer) internal {
        SystemConfigGlobal(systemConfigGlobalProxy).setProposer(teeProposer, true);
        console.log("Registered TEE proposer:", teeProposer);
    }

    function _deployInfrastructure(GameType gameType) internal {
        address factoryImpl = address(new DisputeGameFactory());
        MinimalProxyAdmin proxyAdmin = new MinimalProxyAdmin(cfg.finalSystemOwner());

        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(factoryImpl, address(proxyAdmin), "");

        vm.store(address(proxy), PROXY_OWNER_ADDRESS, bytes32(uint256(uint160(address(proxyAdmin)))));
        DisputeGameFactory(address(proxy)).initialize(cfg.finalSystemOwner());

        disputeGameFactory = address(proxy);
        console.log("DisputeGameFactory:", disputeGameFactory);

        MockAnchorStateRegistry asr = new MockAnchorStateRegistry();
        mockAnchorRegistry = address(asr);
        asr.initialize(
            disputeGameFactory,
            Hash.wrap(cfg.multiproofGenesisOutputRoot()),
            cfg.multiproofGenesisBlockNumber(),
            gameType
        );
        console.log("AnchorStateRegistry (mock):", mockAnchorRegistry);
    }

    function _deployAggregateVerifier(GameType gameType) internal {
        address zkVerifier = address(new MockVerifier());
        console.log("MockVerifier (ZK):", zkVerifier);

        mockDelayedWETH = address(new MockDelayedWETH());
        console.log("MockDelayedWETH:", mockDelayedWETH);

        aggregateVerifier = address(
            new AggregateVerifier(
                gameType,
                IAnchorStateRegistry(mockAnchorRegistry),
                IDelayedWETH(payable(mockDelayedWETH)),
                IVerifier(teeVerifier),
                IVerifier(zkVerifier),
                cfg.teeImageHash(),
                bytes32(0),
                cfg.multiproofConfigHash(),
                8453,
                BLOCK_INTERVAL,
                INTERMEDIATE_BLOCK_INTERVAL,
                PROOF_THRESHOLD
            )
        );
        console.log("AggregateVerifier:", aggregateVerifier);

        DisputeGameFactory(disputeGameFactory).setImplementation(gameType, IDisputeGame(aggregateVerifier));
        DisputeGameFactory(disputeGameFactory).setInitBond(gameType, INIT_BOND);
        console.log("Registered AggregateVerifier with factory");
    }

    function _printSummary() internal view {
        console.log("\n========================================");
        console.log("   DEV DEPLOYMENT COMPLETE (WITH NITRO)");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  NitroEnclaveVerifier (external):", cfg.nitroEnclaveVerifier());
        console.log("  SystemConfigGlobal:", systemConfigGlobalProxy);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", mockAnchorRegistry);
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", cfg.multiproofGameType());
        console.log("  TEE Image Hash:", vm.toString(cfg.teeImageHash()));
        console.log("  Config Hash:", vm.toString(cfg.multiproofConfigHash()));
        console.log("========================================");
        console.log("\n>>> NEXT STEPS (ZK ATTESTATION PROOF REQUIRED) <<<");
        console.log("\n1. Register the PCR0 (raw 48-byte enclave image hash):");
        console.log("   cast send", systemConfigGlobalProxy);
        console.log('     "registerPCR0(bytes)" <PCR0_RAW_BYTES>');
        console.log("     --private-key <OWNER_KEY> --rpc-url <RPC>");
        console.log("\n2. Generate a Risc0 ZK proof of the Nitro attestation offchain.");
        console.log("\n3. Register signer with ZK proof:");
        console.log("   cast send", systemConfigGlobalProxy);
        console.log('     "registerSigner(bytes,bytes)" <ZK_OUTPUT> <ZK_PROOF_BYTES>');
        console.log("     --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\nSee the comments at the top of this file for full details.");
        console.log("========================================\n");
    }

    function _writeOutput() internal {
        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-dev-with-nitro.json");
        string memory output = string.concat(
            '{"SystemConfigGlobal":"',
            vm.toString(systemConfigGlobalProxy),
            '","TEEVerifier":"',
            vm.toString(teeVerifier),
            '","DisputeGameFactory":"',
            vm.toString(disputeGameFactory),
            '","AnchorStateRegistry":"',
            vm.toString(mockAnchorRegistry),
            '","DelayedWETH":"',
            vm.toString(mockDelayedWETH),
            '","AggregateVerifier":"',
            vm.toString(aggregateVerifier),
            '"}'
        );
        vm.writeFile(outPath, output);
        console.log("Deployment saved to:", outPath);
    }
}
