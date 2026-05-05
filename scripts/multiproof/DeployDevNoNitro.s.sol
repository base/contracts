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
 * This script deploys infrastructure using DevTEEProverRegistry, which BYPASSES
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
 *   cast send $TEE_PROVER_REGISTRY \
 *     "addDevSigner(address,bytes32)" $SIGNER_ADDRESS $TEE_IMAGE_HASH \
 *     --private-key $OWNER_KEY --rpc-url $RPC_URL
 *
 * No attestation, PCR0 registration, or certificate validation required.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * COMPARISON WITH DeployDevWithNitro
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * | Feature                    | DeployDevNoNitro       | DeployDevWithNitro     |
 * |----------------------------|------------------------|------------------------|
 * | TEEProverRegistry          | DevTEEProverRegistry   | TEEProverRegistry      |
 * | Signer registration        | addDevSigner()         | registerSigner()       |
 * | Requires Nitro enclave     | No                     | Yes                    |
 * | Validates attestation (ZK) | No                     | Yes                    |
 * | PCR0 pre-registration      | No                     | Yes                    |
 * | Attestation freshness      | N/A                    | < 60 minutes           |
 *
 * Both scripts use mocks for AnchorStateRegistry, DelayedWETH, and ZK Verifier.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 */

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { Proxy } from "src/universal/Proxy.sol";
import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { DisputeGameFactory } from "src/L1/proofs/DisputeGameFactory.sol";
import { GameType, Hash } from "src/libraries/bridge/Types.sol";

import { DeployConfig } from "scripts/deploy/DeployConfig.s.sol";
import { Config } from "scripts/libraries/Config.sol";
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";
import { MockVerifier } from "test/mocks/MockVerifier.sol";
import { DevTEEProverRegistry } from "src/L1/proofs/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

import { MinimalProxyAdmin } from "./mocks/MinimalProxyAdmin.sol";
import { MockAnchorStateRegistry } from "./mocks/MockAnchorStateRegistry.sol";
import { MockDelayedWETH } from "./mocks/MockDelayedWETH.sol";

/// @title DeployDevNoNitro
/// @notice Development deployment WITHOUT AWS Nitro attestation validation.
/// @dev Uses DevTEEProverRegistry which allows addDevSigner() to bypass attestation.
contract DeployDevNoNitro is Script {
    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 public constant INIT_BOND = 0.001 ether;

    DeployConfig public constant cfg =
        DeployConfig(address(uint160(uint256(keccak256(abi.encode("optimism.deployconfig"))))));

    address public teeProverRegistryProxy;
    address public teeVerifier;
    address public disputeGameFactory;
    IAnchorStateRegistry public mockAnchorRegistry;
    address public mockDelayedWETH;
    address public aggregateVerifier;

    function setUp() public {
        DeployUtils.etchLabelAndAllowCheatcodes({ _etchTo: address(cfg), _cname: "DeployConfig" });
        cfg.read(Config.deployConfigPath());
    }

    function run() public {
        GameType gameType = GameType.wrap(uint32(cfg.multiproofGameType()));

        console.log("=== Deploying Dev Infrastructure (NO NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("");
        console.log("NOTE: Using DevTEEProverRegistry - NO attestation required.");

        vm.startBroadcast();

        _deployInfrastructure(gameType);
        _deployTEEContracts(gameType);
        _deployAggregateVerifier(gameType);

        vm.stopBroadcast();

        _printSummary();
        _writeOutput();
    }

    function _deployTEEContracts(GameType gameType) internal {
        address owner = cfg.finalSystemOwner();
        address teeRegistryImpl = address(
            new DevTEEProverRegistry(INitroEnclaveVerifier(address(0)), IDisputeGameFactory(disputeGameFactory))
        );
        address[] memory initialProposers = new address[](2);
        initialProposers[0] = cfg.teeProposer();
        initialProposers[1] = cfg.teeChallenger();
        Proxy teeProxy = new Proxy(msg.sender);
        teeProxy.upgradeToAndCall(
            teeRegistryImpl, abi.encodeCall(TEEProverRegistry.initialize, (owner, owner, initialProposers, gameType))
        );
        teeProxy.changeAdmin(address(0xdead));
        teeProverRegistryProxy = address(teeProxy);

        teeVerifier = address(new TEEVerifier(TEEProverRegistry(teeProverRegistryProxy), mockAnchorRegistry));
    }

    function _deployInfrastructure(GameType gameType) internal {
        address factoryImpl = address(new DisputeGameFactory());
        MinimalProxyAdmin proxyAdmin = new MinimalProxyAdmin(cfg.finalSystemOwner());

        Proxy proxy = new Proxy(msg.sender);
        proxy.upgradeTo(factoryImpl);
        proxy.changeAdmin(address(proxyAdmin));
        DisputeGameFactory(address(proxy)).initialize(cfg.finalSystemOwner());
        disputeGameFactory = address(proxy);

        MockAnchorStateRegistry asr = new MockAnchorStateRegistry();
        mockAnchorRegistry = IAnchorStateRegistry(address(asr));
        asr.initialize(
            disputeGameFactory,
            Hash.wrap(cfg.multiproofGenesisOutputRoot()),
            cfg.multiproofGenesisBlockNumber(),
            gameType
        );
    }

    function _deployAggregateVerifier(GameType gameType) internal {
        address zkVerifier = address(new MockVerifier(mockAnchorRegistry));
        mockDelayedWETH = address(new MockDelayedWETH());

        AggregateVerifier.ZkHashes memory zkHashes =
            AggregateVerifier.ZkHashes({ rangeHash: cfg.zkRangeHash(), aggregateHash: cfg.zkAggregationHash() });

        aggregateVerifier = address(
            new AggregateVerifier(
                gameType,
                mockAnchorRegistry,
                IDelayedWETH(payable(mockDelayedWETH)),
                IVerifier(teeVerifier),
                IVerifier(zkVerifier),
                cfg.teeImageHash(),
                zkHashes,
                cfg.multiproofConfigHash(),
                cfg.l2ChainID(),
                BLOCK_INTERVAL,
                INTERMEDIATE_BLOCK_INTERVAL
            )
        );

        DisputeGameFactory factory = DisputeGameFactory(disputeGameFactory);
        factory.setImplementation(gameType, IDisputeGame(aggregateVerifier), "");
        factory.setInitBond(gameType, INIT_BOND);
    }

    function _printSummary() internal view {
        console.log("\n========================================");
        console.log("    DEV DEPLOYMENT COMPLETE (NO NITRO)");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  DevTEEProverRegistry:", teeProverRegistryProxy);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", address(mockAnchorRegistry));
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", cfg.multiproofGameType());
        console.log("  TEE Image Hash:", vm.toString(cfg.teeImageHash()));
        console.log("  Config Hash:", vm.toString(cfg.multiproofConfigHash()));
        console.log("========================================");
        console.log("\n>>> NEXT STEP - Register dev signer (NO ATTESTATION NEEDED) <<<");
        console.log("\ncast send", teeProverRegistryProxy);
        console.log('  "addDevSigner(address,bytes32)" <SIGNER_ADDRESS>');
        console.log(" ", vm.toString(cfg.teeImageHash()));
        console.log("  --private-key <OWNER_KEY> --rpc-url <RPC>");
        console.log("\n========================================\n");
    }

    function _writeOutput() internal {
        string memory key = "deployment";
        vm.serializeAddress(key, "TEEProverRegistry", teeProverRegistryProxy);
        vm.serializeAddress(key, "TEEVerifier", teeVerifier);
        vm.serializeAddress(key, "DisputeGameFactory", disputeGameFactory);
        vm.serializeAddress(key, "AnchorStateRegistry", address(mockAnchorRegistry));
        vm.serializeAddress(key, "DelayedWETH", mockDelayedWETH);
        string memory json = vm.serializeAddress(key, "AggregateVerifier", aggregateVerifier);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-dev-no-nitro.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
