// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
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

abstract contract DeployDevBase is Script {
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

        _preflight();
        _logHeader();

        vm.startBroadcast();

        _deployInfrastructure(gameType);
        _deployTEEContracts(gameType);
        _deployAggregateVerifier(gameType);

        vm.stopBroadcast();

        _printSummary();
        _writeOutput();
    }

    function _deployInfrastructure(GameType gameType) internal {
        address owner = cfg.finalSystemOwner();
        address factoryImpl = address(new DisputeGameFactory());
        MinimalProxyAdmin proxyAdmin = new MinimalProxyAdmin(owner);

        Proxy proxy = new Proxy(msg.sender);
        proxy.upgradeTo(factoryImpl);
        proxy.changeAdmin(address(proxyAdmin));
        DisputeGameFactory(address(proxy)).initialize(owner);
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

    function _deployTEEContracts(GameType gameType) internal {
        address owner = cfg.finalSystemOwner();
        address teeRegistryImpl = _deployTEERegistryImpl();

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
                cfg.l2ChainId(),
                _blockInterval(),
                _intermediateBlockInterval()
            )
        );

        DisputeGameFactory factory = DisputeGameFactory(disputeGameFactory);
        factory.setImplementation(gameType, IDisputeGame(aggregateVerifier), "");
        factory.setInitBond(gameType, _initBond());
    }

    function _writeOutput() internal {
        string memory key = "deployment";
        vm.serializeAddress(key, "TEEProverRegistry", teeProverRegistryProxy);
        vm.serializeAddress(key, "TEEVerifier", teeVerifier);
        _serializeExtra(key);
        vm.serializeAddress(key, "DisputeGameFactory", disputeGameFactory);
        vm.serializeAddress(key, "AnchorStateRegistry", address(mockAnchorRegistry));
        vm.serializeAddress(key, "DelayedWETH", mockDelayedWETH);
        string memory json = vm.serializeAddress(key, "AggregateVerifier", aggregateVerifier);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), _outputSuffix());
        vm.writeJson(json, outPath);
        console.log("Deployment saved to:", outPath);
    }

    function _blockInterval() internal pure virtual returns (uint256);
    function _intermediateBlockInterval() internal pure virtual returns (uint256);
    function _initBond() internal pure virtual returns (uint256);
    function _outputSuffix() internal pure virtual returns (string memory);
    function _deployTEERegistryImpl() internal virtual returns (address);
    function _logHeader() internal view virtual;
    function _printSummary() internal view virtual;

    function _preflight() internal virtual { }
    function _serializeExtra(string memory key) internal virtual { }
}
