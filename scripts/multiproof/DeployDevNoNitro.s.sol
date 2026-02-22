// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/**
 * @title DeployDevNoNitro
 * @notice Development deployment WITHOUT AWS Nitro attestation validation.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 *                              DEPLOYMENT TYPE: DEV (NO NITRO)
 * ══════════════════════════════════════════════════════════════════════════════════
 *
 * This script deploys infrastructure using DevSystemConfigGlobal, which BYPASSES
 * AWS Nitro attestation validation. Signers can be registered with a simple call
 * to addDevSigner() without needing a real Nitro enclave or attestation document.
 *
 * USE THIS SCRIPT WHEN:
 * - Running local development or testing
 * - You don't have access to an AWS Nitro enclave
 * - You want to quickly test the prover without attestation overhead
 *
 * DO NOT USE THIS SCRIPT FOR:
 * - Production deployments
 * - Security testing of the attestation flow
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * SIGNER REGISTRATION (SIMPLIFIED)
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * After deployment, register a signer with a single call:
 *
 *   cast send $SYSTEM_CONFIG_GLOBAL \
 *     "addDevSigner(address,bytes32)" $SIGNER_ADDRESS $TEE_IMAGE_HASH \
 *     --private-key $OWNER_KEY --rpc-url $RPC_URL
 *
 * No attestation, PCR0 registration, or certificate validation required.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * COMPARISON WITH DeployDevWithNitro
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * | Feature                    | DeployDevNoNitro      | DeployDevWithNitro    |
 * |----------------------------|----------------------|------------------------|
 * | SystemConfigGlobal         | DevSystemConfigGlobal | SystemConfigGlobal    |
 * | Signer registration        | addDevSigner()        | registerSigner()      |
 * | Requires Nitro enclave     | No                    | Yes                   |
 * | Validates AWS cert chain   | No                    | Yes                   |
 * | PCR0 pre-registration      | No                    | Yes                   |
 * | Attestation freshness      | N/A                   | < 60 minutes          |
 *
 * Both scripts use mocks for AnchorStateRegistry, DelayedWETH, and ZK Verifier.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 */

import {CertManager} from "@nitro-validator/CertManager.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {console2 as console} from "forge-std/console2.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {IDelayedWETH} from "optimism/interfaces/dispute/IDelayedWETH.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {DisputeGameFactory} from "optimism/src/dispute/DisputeGameFactory.sol";
import {GameType, Hash} from "optimism/src/dispute/lib/Types.sol";

import {AggregateVerifier} from "../src/AggregateVerifier.sol";
import {IVerifier} from "../src/interfaces/IVerifier.sol";
import {MockVerifier} from "../src/mocks/MockVerifier.sol";
import {DevSystemConfigGlobal} from "../src/tee/DevSystemConfigGlobal.sol";
import {SystemConfigGlobal} from "../src/tee/SystemConfigGlobal.sol";
import {TEEVerifier} from "../src/tee/TEEVerifier.sol";

import {MinimalProxyAdmin} from "./mocks/MinimalProxyAdmin.sol";
import {MockAnchorStateRegistry} from "./mocks/MockAnchorStateRegistry.sol";
import {MockDelayedWETH} from "./mocks/MockDelayedWETH.sol";

/// @title DeployDevNoNitro
/// @notice Development deployment WITHOUT AWS Nitro attestation validation.
/// @dev Uses DevSystemConfigGlobal which allows addDevSigner() to bypass attestation.
contract DeployDevNoNitro is Script {
    using stdJson for string;

    /// @notice Constant from Optimism's Constants.sol - the storage slot for proxy admin.
    bytes32 internal constant PROXY_OWNER_ADDRESS = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;

    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 public constant INIT_BOND = 0.001 ether;

    /// @notice Config struct to reduce stack variables.
    struct DeployConfig {
        address owner;
        bytes32 teeImageHash;
        address teeProposer;
        GameType gameType;
        uint256 gameTypeRaw;
        bytes32 genesisOutputRoot;
        uint256 genesisBlockNumber;
        bytes32 configHash;
    }

    // Deployed addresses
    address public certManager;
    address public systemConfigGlobalProxy;
    address public teeVerifier;
    address public disputeGameFactory;
    address public mockAnchorRegistry;
    address public mockDelayedWETH;
    address public aggregateVerifier;

    function run() public {
        DeployConfig memory cfg = _loadConfig();

        console.log("=== Deploying Dev Infrastructure (NO NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.owner);
        console.log("TEE Proposer:", cfg.teeProposer);
        console.log("Game Type:", cfg.gameTypeRaw);
        console.log("");
        console.log("NOTE: Using DevSystemConfigGlobal - NO attestation required.");

        vm.startBroadcast();

        _deployTEEContracts(cfg.owner);
        _registerProposer(cfg.teeProposer);
        _deployInfrastructure(cfg);
        _deployAggregateVerifier(cfg);

        vm.stopBroadcast();

        _printSummary(cfg.teeImageHash, cfg.gameTypeRaw, cfg.configHash);
        _writeOutput();
    }

    function _loadConfig() internal view returns (DeployConfig memory cfg) {
        string memory configPath = vm.envOr("DEPLOY_CONFIG_PATH", string("deploy-config/sepolia-no-nitro.json"));
        string memory config = vm.readFile(configPath);

        cfg.owner = config.readAddress(".finalSystemOwner");
        cfg.teeImageHash = config.readBytes32(".teeImageHash");
        cfg.teeProposer = config.readAddressOr(".teeProposer", cfg.owner);
        cfg.gameTypeRaw = config.readUintOr(".gameType", 621);
        cfg.gameType = GameType.wrap(uint32(cfg.gameTypeRaw));
        cfg.genesisOutputRoot = config.readBytes32Or(".genesisOutputRoot", bytes32(uint256(1)));
        cfg.genesisBlockNumber = config.readUintOr(".genesisBlockNumber", 0);
        cfg.configHash = config.readBytes32Or(".configHash", bytes32(0));
    }

    function _deployTEEContracts(address owner) internal {
        // 1. CertManager (not used in dev mode, but deployed for interface compatibility)
        certManager = address(new CertManager());
        console.log("CertManager:", certManager);

        // 2. DevSystemConfigGlobal - allows addDevSigner() to bypass attestation
        address scgImpl = address(new DevSystemConfigGlobal(CertManager(certManager)));
        systemConfigGlobalProxy = address(
            new TransparentUpgradeableProxy(
                scgImpl,
                address(0xdead), // Non-upgradeable for testing
                abi.encodeCall(SystemConfigGlobal.initialize, (owner, owner))
            )
        );
        console.log("DevSystemConfigGlobal:", systemConfigGlobalProxy);

        // 3. TEEVerifier
        teeVerifier = address(new TEEVerifier(SystemConfigGlobal(systemConfigGlobalProxy)));
        console.log("TEEVerifier:", teeVerifier);
    }

    function _registerProposer(address teeProposer) internal {
        SystemConfigGlobal(systemConfigGlobalProxy).setProposer(teeProposer, true);
        console.log("Registered TEE proposer:", teeProposer);
    }

    function _deployInfrastructure(DeployConfig memory cfg) internal {
        // 4. REAL DisputeGameFactory (behind proxy)
        address factoryImpl = address(new DisputeGameFactory());
        MinimalProxyAdmin proxyAdmin = new MinimalProxyAdmin(cfg.owner);

        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            factoryImpl,
            address(proxyAdmin),
            ""
        );

        vm.store(address(proxy), PROXY_OWNER_ADDRESS, bytes32(uint256(uint160(address(proxyAdmin)))));
        DisputeGameFactory(address(proxy)).initialize(cfg.owner);

        disputeGameFactory = address(proxy);
        console.log("DisputeGameFactory:", disputeGameFactory);

        // 5. Mock AnchorStateRegistry
        MockAnchorStateRegistry asr = new MockAnchorStateRegistry();
        mockAnchorRegistry = address(asr);
        asr.initialize(disputeGameFactory, Hash.wrap(cfg.genesisOutputRoot), cfg.genesisBlockNumber, cfg.gameType);
        console.log("AnchorStateRegistry (mock):", mockAnchorRegistry);
    }

    function _deployAggregateVerifier(DeployConfig memory cfg) internal {
        // 6. Mock ZK Verifier
        address zkVerifier = address(new MockVerifier());
        console.log("MockVerifier (ZK):", zkVerifier);

        // 6.5. Mock DelayedWETH for bond handling
        mockDelayedWETH = address(new MockDelayedWETH());
        console.log("MockDelayedWETH:", mockDelayedWETH);

        // 7. AggregateVerifier
        aggregateVerifier = address(
            new AggregateVerifier(
                cfg.gameType,
                IAnchorStateRegistry(mockAnchorRegistry),
                IDelayedWETH(payable(mockDelayedWETH)),
                IVerifier(teeVerifier),
                IVerifier(zkVerifier),
                cfg.teeImageHash,
                bytes32(0), // zkImageHash (unused)
                cfg.configHash,
                8453, // l2ChainId (Base mainnet)
                BLOCK_INTERVAL,
                INTERMEDIATE_BLOCK_INTERVAL
            )
        );
        console.log("AggregateVerifier:", aggregateVerifier);

        // 8. Register AggregateVerifier with the factory
        DisputeGameFactory(disputeGameFactory).setImplementation(cfg.gameType, IDisputeGame(aggregateVerifier));
        DisputeGameFactory(disputeGameFactory).setInitBond(cfg.gameType, INIT_BOND);
        console.log("Registered AggregateVerifier with factory");
    }

    function _printSummary(bytes32 teeImageHash, uint256 gameType, bytes32 configHash) internal view {
        console.log("\n========================================");
        console.log("    DEV DEPLOYMENT COMPLETE (NO NITRO)");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  CertManager:", certManager);
        console.log("  DevSystemConfigGlobal:", systemConfigGlobalProxy);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", mockAnchorRegistry);
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", gameType);
        console.log("  TEE Image Hash:", vm.toString(teeImageHash));
        console.log("  Config Hash:", vm.toString(configHash));
        console.log("========================================");
        console.log("\n>>> NEXT STEP - Register dev signer (NO ATTESTATION NEEDED) <<<");
        console.log("\ncast send", systemConfigGlobalProxy);
        console.log('  "addDevSigner(address,bytes32)" <SIGNER_ADDRESS>');
        console.log(" ", vm.toString(teeImageHash));
        console.log("  --private-key <OWNER_KEY> --rpc-url <RPC>");
        console.log("\n========================================\n");
    }

    function _writeOutput() internal {
        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-dev-no-nitro.json");
        string memory output = string.concat(
            '{"CertManager":"',
            vm.toString(certManager),
            '","SystemConfigGlobal":"',
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
