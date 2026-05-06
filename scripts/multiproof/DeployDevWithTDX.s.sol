// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/**
 * @title DeployDevWithTDX
 * @notice Development deployment using the TDX signer-registration path.
 *
 * This deploys the same local multiproof testing infrastructure as the existing
 * dev scripts, but configures TEEProverRegistry for TDX signer registration. Deploy
 * TDXVerifier first with DeployTDXVerifier.s.sol and pass its address to run().
 * The default run(address) entrypoint configures DEFAULT_TDX_REGISTRATION_MANAGER
 * as the registry manager so it can submit TDX signer registrations.
 */

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
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
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

import { MinimalProxyAdmin } from "./mocks/MinimalProxyAdmin.sol";
import { MockAnchorStateRegistry } from "./mocks/MockAnchorStateRegistry.sol";
import { MockDelayedWETH } from "./mocks/MockDelayedWETH.sol";

contract DeployDevWithTDX is Script {
    uint256 public constant INIT_BOND = 0.00001 ether;
    address public constant DEFAULT_TDX_REGISTRATION_MANAGER = 0x93900CB7eCdB5994352b19DfD8a900Cd4fa437B7;

    DeployConfig public constant cfg =
        DeployConfig(address(uint160(uint256(keccak256(abi.encode("optimism.deployconfig"))))));

    address public nitroEnclaveVerifierAddr;
    address public tdxVerifierAddr;
    address public tdxRegistrationManager;
    address public teeProverRegistryProxy;
    address public teeVerifier;
    address public disputeGameFactory;
    IAnchorStateRegistry public mockAnchorRegistry;
    address public mockDelayedWETH;
    address public aggregateVerifier;
    Hash public startingAnchorRoot;
    uint256 public startingAnchorBlockNumber;

    function setUp() public {
        DeployUtils.etchLabelAndAllowCheatcodes({ _etchTo: address(cfg), _cname: "DeployConfig" });
        cfg.read(Config.deployConfigPath());
    }

    function run(address tdxVerifier) public {
        run(tdxVerifier, DEFAULT_TDX_REGISTRATION_MANAGER);
    }

    function run(address tdxVerifier, address registrationManager) public {
        run(tdxVerifier, registrationManager, cfg.multiproofGenesisOutputRoot(), cfg.multiproofGenesisBlockNumber());
    }

    function run(
        address tdxVerifier,
        address registrationManager,
        bytes32 asrStartingOutputRoot,
        uint256 asrStartingBlockNumber
    )
        public
    {
        nitroEnclaveVerifierAddr = cfg.nitroEnclaveVerifier();
        require(tdxVerifier != address(0), "tdxVerifier must be non-zero");
        require(nitroEnclaveVerifierAddr != address(0), "nitroEnclaveVerifier must be set in config");
        require(registrationManager != address(0), "registrationManager must be non-zero");
        require(asrStartingOutputRoot != bytes32(0), "asrStartingOutputRoot must be non-zero");
        tdxVerifierAddr = tdxVerifier;
        tdxRegistrationManager = registrationManager;
        startingAnchorRoot = Hash.wrap(asrStartingOutputRoot);
        startingAnchorBlockNumber = asrStartingBlockNumber;

        GameType gameType = GameType.wrap(uint32(cfg.multiproofGameType()));

        console.log("=== Deploying Dev Infrastructure (WITH TDX) ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.finalSystemOwner());
        console.log("TEE Proposer:", cfg.teeProposer());
        console.log("TEE Challenger:", cfg.teeChallenger());
        console.log("Game Type:", cfg.multiproofGameType());
        console.log("NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("TDXVerifier:", tdxVerifierAddr);
        console.log("TDX Registration Manager:", tdxRegistrationManager);
        console.log("ASR Starting Output Root:", vm.toString(startingAnchorRoot.raw()));
        console.log("ASR Starting L2 Block:", startingAnchorBlockNumber);
        console.log("");
        console.log("NOTE: TDXVerifier owner must be the broadcaster/finalSystemOwner.");

        vm.startBroadcast();

        _deployInfrastructure(gameType);
        _deployTDXContracts(gameType);
        _deployAggregateVerifier(gameType);

        vm.stopBroadcast();

        _printSummary();
        _writeOutput();
    }

    function _deployTDXContracts(GameType gameType) internal {
        address owner = cfg.finalSystemOwner();
        address registryImpl = address(
            new TEEProverRegistry(
                INitroEnclaveVerifier(nitroEnclaveVerifierAddr),
                ITDXVerifier(tdxVerifierAddr),
                IDisputeGameFactory(disputeGameFactory)
            )
        );

        address[] memory initialProposers = new address[](2);
        initialProposers[0] = cfg.teeProposer();
        initialProposers[1] = cfg.teeChallenger();

        Proxy registryProxy = new Proxy(msg.sender);
        registryProxy.upgradeToAndCall(
            registryImpl,
            abi.encodeCall(TEEProverRegistry.initialize, (owner, tdxRegistrationManager, initialProposers, gameType))
        );
        registryProxy.changeAdmin(address(0xdead));
        teeProverRegistryProxy = address(registryProxy);

        ITDXVerifier(tdxVerifierAddr).setProofSubmitter(teeProverRegistryProxy);

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
        asr.initialize(disputeGameFactory, startingAnchorRoot, startingAnchorBlockNumber, gameType);
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
                cfg.multiproofBlockInterval(),
                cfg.multiproofIntermediateBlockInterval()
            )
        );

        DisputeGameFactory factory = DisputeGameFactory(disputeGameFactory);
        factory.setImplementation(gameType, IDisputeGame(aggregateVerifier), "");
        factory.setInitBond(gameType, INIT_BOND);
    }

    function _printSummary() internal view {
        console.log("\n========================================");
        console.log("      DEV DEPLOYMENT COMPLETE (TDX)");
        console.log("========================================");
        console.log("\nTDX Contracts:");
        console.log("  NitroEnclaveVerifier:", nitroEnclaveVerifierAddr);
        console.log("  TDXVerifier:", tdxVerifierAddr);
        console.log("  TEEProverRegistry:", teeProverRegistryProxy);
        console.log("  TDX Registration Manager:", tdxRegistrationManager);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory:", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", address(mockAnchorRegistry));
        console.log("  ASR Starting Output Root:", vm.toString(startingAnchorRoot.raw()));
        console.log("  ASR Starting L2 Block:", startingAnchorBlockNumber);
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", cfg.multiproofGameType());
        console.log("  TEE Image Hash:", vm.toString(cfg.teeImageHash()));
        console.log("  Config Hash:", vm.toString(cfg.multiproofConfigHash()));
        console.log("========================================");
        console.log("\n>>> NEXT STEP: Register one Nitro signer and one TDX signer <<<");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerSigner(bytes,bytes)" <NITRO_OUTPUT> <NITRO_PROOF_BYTES>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n  cast send", teeProverRegistryProxy);
        console.log('    "registerTDXSigner(bytes,bytes)" <TDX_OUTPUT> <PROOF_BYTES>');
        console.log("    --private-key <OWNER_OR_MANAGER_KEY> --rpc-url <RPC>");
        console.log("\n========================================\n");
    }

    function _writeOutput() internal {
        string memory key = "deployment";
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifierAddr);
        vm.serializeAddress(key, "TDXVerifier", tdxVerifierAddr);
        vm.serializeAddress(key, "TDXRegistrationManager", tdxRegistrationManager);
        vm.serializeAddress(key, "TEEProverRegistry", teeProverRegistryProxy);
        vm.serializeAddress(key, "TEEVerifier", teeVerifier);
        vm.serializeAddress(key, "DisputeGameFactory", disputeGameFactory);
        vm.serializeAddress(key, "AnchorStateRegistry", address(mockAnchorRegistry));
        vm.serializeBytes32(key, "ASRStartingOutputRoot", startingAnchorRoot.raw());
        vm.serializeUint(key, "ASRStartingBlockNumber", startingAnchorBlockNumber);
        vm.serializeAddress(key, "DelayedWETH", mockDelayedWETH);
        string memory json = vm.serializeAddress(key, "AggregateVerifier", aggregateVerifier);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-dev-with-tdx.json");
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }
}
