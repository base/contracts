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
 * This script deploys infrastructure using the REAL TEEProverRegistry, which
 * REQUIRES a ZK proof of a valid AWS Nitro attestation for signer registration.
 * You cannot use addDevSigner() - you must go through the full registerSigner() flow.
 *
 * PREREQUISITES:
 *   1. Deploy the RISC Zero verifier stack AND NitroEnclaveVerifier using
 *      DeployRiscZeroStack.s.sol (required because NitroEnclaveVerifier and its
 *      dependencies need Solidity ^0.8.20, while this script is pinned to =0.8.15).
 *   2. Set `nitroEnclaveVerifier` in the deploy config to the deployed address.
 *
 * ─────────────────────────────────────────────────────────────────────────────────
 * SIGNER REGISTRATION FLOW
 * ─────────────────────────────────────────────────────────────────────────────────
 *
 * After deployment, register a signer by generating a RISC Zero ZK proof of a
 * valid AWS Nitro attestation document and calling:
 *
 *   cast send $TEE_PROVER_REGISTRY \
 *     "registerSigner(bytes,bytes)" $ZK_OUTPUT $ZK_PROOF_BYTES \
 *     --private-key $OWNER_OR_MANAGER_KEY --rpc-url $RPC_URL
 *
 * IMPORTANT: The attestation is only valid for 60 minutes! Generate the proof
 * and submit the transaction within that window.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 */

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { Proxy } from "src/universal/Proxy.sol";
import { Script } from "lib/forge-std/src/Script.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";
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
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

import { MinimalProxyAdmin } from "./mocks/MinimalProxyAdmin.sol";
import { MockAnchorStateRegistry } from "./mocks/MockAnchorStateRegistry.sol";
import { MockDelayedWETH } from "./mocks/MockDelayedWETH.sol";

/// @title DeployDevWithNitro
/// @notice Development deployment WITH AWS Nitro attestation validation.
/// @dev Uses real TEEProverRegistry which requires registerSigner() with valid attestation.
///      NitroEnclaveVerifier must be pre-deployed via DeployRiscZeroStack.s.sol.
contract DeployDevWithNitro is Script {
    uint256 public constant BLOCK_INTERVAL = 600;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 30;
    uint256 public constant INIT_BOND = 0.00001 ether;

    DeployConfig public constant cfg =
        DeployConfig(address(uint160(uint256(keccak256(abi.encode("optimism.deployconfig"))))));

    address public nitroEnclaveVerifierAddr;
    address public tdxVerifierAddr;
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

        // NitroEnclaveVerifier must be pre-deployed (via DeployRiscZeroStack.s.sol)
        nitroEnclaveVerifierAddr = cfg.nitroEnclaveVerifier();
        tdxVerifierAddr = cfg.tdxVerifier();
        require(
            nitroEnclaveVerifierAddr != address(0),
            "nitroEnclaveVerifier must be set in config (deploy via DeployRiscZeroStack.s.sol first)"
        );
        require(tdxVerifierAddr != address(0), "tdxVerifier must be set in config");

        console.log("=== Deploying Dev Infrastructure (WITH NITRO) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("TDXVerifier:", tdxVerifierAddr);
        console.log("");
        console.log("NOTE: Using REAL TEEProverRegistry - ZK attestation proof REQUIRED.");

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
            new TEEProverRegistry(
                INitroEnclaveVerifier(nitroEnclaveVerifierAddr),
                ITDXVerifier(tdxVerifierAddr),
                IDisputeGameFactory(disputeGameFactory)
            )
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
                AggregateVerifier.TeeHashes(cfg.teeNitroImageHash(), cfg.teeTdxImageHash()),
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
        console.log("   DEV DEPLOYMENT COMPLETE (WITH NITRO)");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("  TDXVerifier:", tdxVerifierAddr);
        console.log("  TEEProverRegistry:", teeProverRegistryProxy);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", address(mockAnchorRegistry));
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", cfg.multiproofGameType());
        console.log("  Nitro Image Hash:", vm.toString(cfg.teeNitroImageHash()));
        console.log("  TDX Image Hash:", vm.toString(cfg.teeTdxImageHash()));
        console.log("  Config Hash:", vm.toString(cfg.multiproofConfigHash()));
        console.log("========================================");
        console.log("\n>>> NEXT STEP: Register one Nitro signer and one TDX signer <<<");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerSigner(bytes,bytes)" <NITRO_OUTPUT> <NITRO_PROOF_BYTES>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerTDXSigner(bytes,bytes)" <TDX_OUTPUT> <TDX_PROOF_BYTES>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n========================================\n");
    }

    function _writeOutput() internal {
        string memory key = "deployment";
        vm.serializeAddress(key, "TEEProverRegistry", teeProverRegistryProxy);
        vm.serializeAddress(key, "TEEVerifier", teeVerifier);
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifierAddr);
        vm.serializeAddress(key, "TDXVerifier", tdxVerifierAddr);
        vm.serializeAddress(key, "DisputeGameFactory", disputeGameFactory);
        vm.serializeAddress(key, "AnchorStateRegistry", address(mockAnchorRegistry));
        vm.serializeAddress(key, "DelayedWETH", mockDelayedWETH);
        string memory json = vm.serializeAddress(key, "AggregateVerifier", aggregateVerifier);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-dev-with-nitro.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
