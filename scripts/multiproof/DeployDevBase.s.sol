// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Proxy } from "src/universal/Proxy.sol";
import { Script } from "lib/forge-std/src/Script.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
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

import { MockAnchorStateRegistry } from "./mocks/MockAnchorStateRegistry.sol";
import { MockDelayedWETH } from "./mocks/MockDelayedWETH.sol";

abstract contract DeployDevBase is Script {
    DeployConfig internal constant cfg =
        DeployConfig(address(uint160(uint256(keccak256(abi.encode("optimism.deployconfig"))))));

    address internal teeProverRegistryProxy;
    address internal teeVerifier;
    address internal disputeGameFactory;
    IAnchorStateRegistry internal mockAnchorRegistry;
    address internal mockDelayedWETH;
    address internal aggregateVerifier;

    function setUp() public {
        DeployUtils.etchLabelAndAllowCheatcodes({ _etchTo: address(cfg), _cname: "DeployConfig" });
        cfg.read(Config.deployConfigPath());
    }

    function run() public {
        run(cfg.multiproofGenesisOutputRoot(), cfg.multiproofGenesisBlockNumber());
    }

    function run(bytes32 asrStartingOutputRoot, uint256 asrStartingBlockNumber) public {
        require(asrStartingOutputRoot != bytes32(0), "asrStartingOutputRoot must be non-zero");
        Hash startingAnchorRoot = Hash.wrap(asrStartingOutputRoot);

        GameType gameType = GameType.wrap(uint32(cfg.multiproofGameType()));

        _preflight();

        vm.startBroadcast();

        _deployInfrastructure(gameType, startingAnchorRoot, asrStartingBlockNumber);
        _deployTEEContracts(gameType);
        _deployAggregateVerifier(gameType);

        vm.stopBroadcast();

        _writeOutput(startingAnchorRoot, asrStartingBlockNumber);
    }

    function _deployInfrastructure(
        GameType gameType,
        Hash startingAnchorRoot,
        uint256 startingAnchorBlockNumber
    )
        internal
    {
        Proxy proxy = new Proxy(msg.sender);
        proxy.upgradeTo(address(new DisputeGameFactory()));
        DisputeGameFactory(address(proxy)).initialize(msg.sender);
        proxy.changeAdmin(address(0xdead));
        disputeGameFactory = address(proxy);

        MockAnchorStateRegistry asr = new MockAnchorStateRegistry();
        mockAnchorRegistry = IAnchorStateRegistry(address(asr));
        asr.initialize(disputeGameFactory, startingAnchorRoot, startingAnchorBlockNumber, gameType);
    }

    function _deployTEEContracts(GameType gameType) internal {
        address[] memory initialProposers = new address[](2);
        initialProposers[0] = cfg.teeProposer();
        initialProposers[1] = cfg.teeChallenger();

        Proxy teeProxy = new Proxy(msg.sender);
        teeProxy.upgradeToAndCall(
            _deployTEERegistryImpl(),
            abi.encodeCall(
                TEEProverRegistry.initialize,
                (cfg.finalSystemOwner(), _teeRegistrationManager(), initialProposers, gameType)
            )
        );
        teeProxy.changeAdmin(address(0xdead));
        teeProverRegistryProxy = address(teeProxy);

        _afterTEERegistryDeploy();

        teeVerifier = address(new TEEVerifier(TEEProverRegistry(teeProverRegistryProxy), mockAnchorRegistry));
    }

    function _deployAggregateVerifier(GameType gameType) internal {
        mockDelayedWETH = address(new MockDelayedWETH());

        aggregateVerifier = address(
            new AggregateVerifier(
                gameType,
                mockAnchorRegistry,
                IDelayedWETH(payable(mockDelayedWETH)),
                IVerifier(teeVerifier),
                IVerifier(address(new MockVerifier(mockAnchorRegistry))),
                cfg.teeNitroImageHash(),
                cfg.teeTdxImageHash(),
                AggregateVerifier.ZkHashes({ rangeHash: cfg.zkRangeHash(), aggregateHash: cfg.zkAggregationHash() }),
                cfg.multiproofConfigHash(),
                cfg.l2ChainId(),
                cfg.multiproofBlockInterval(),
                cfg.multiproofIntermediateBlockInterval()
            )
        );

        DisputeGameFactory(disputeGameFactory).setImplementation(gameType, IDisputeGame(aggregateVerifier));
        DisputeGameFactory(disputeGameFactory).setInitBond(gameType, _initBond());
        DisputeGameFactory(disputeGameFactory).transferOwnership(cfg.finalSystemOwner());
    }

    function _writeOutput(Hash startingAnchorRoot, uint256 startingAnchorBlockNumber) internal {
        string memory key = "deployment";
        vm.serializeAddress(key, "TEEProverRegistry", teeProverRegistryProxy);
        vm.serializeAddress(key, "TEEVerifier", teeVerifier);
        _serializeExtra(key);
        vm.serializeAddress(key, "DisputeGameFactory", disputeGameFactory);
        vm.serializeAddress(key, "AnchorStateRegistry", address(mockAnchorRegistry));
        vm.serializeBytes32(key, "ASRStartingOutputRoot", startingAnchorRoot.raw());
        vm.serializeUint(key, "ASRStartingBlockNumber", startingAnchorBlockNumber);
        vm.serializeAddress(key, "DelayedWETH", mockDelayedWETH);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), _outputSuffix());
        vm.writeJson(vm.serializeAddress(key, "AggregateVerifier", aggregateVerifier), outPath);
        console.log("Deployment saved to:", outPath);
    }

    function _initBond() internal pure virtual returns (uint256) {
        return 0.00001 ether;
    }

    function _outputSuffix() internal pure virtual returns (string memory);
    function _deployTEERegistryImpl() internal virtual returns (address);
    function _preflight() internal view virtual;
    function _serializeExtra(string memory key) internal virtual;

    function _teeRegistrationManager() internal view virtual returns (address) {
        return cfg.finalSystemOwner();
    }

    function _afterTEERegistryDeploy() internal virtual { }
}
