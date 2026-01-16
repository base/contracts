// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import "src/AggregateVerifier.sol";

import {IVerifier} from "src/interfaces/IVerifier.sol";

// Mocks
import {MockVerifier} from "src/mocks/MockVerifier.sol";
import {MockSystemConfig} from "src/mocks/MockSystemConfig.sol";

// Optimism
import {IDisputeGame, DisputeGameFactory} from "optimism/src/dispute/DisputeGameFactory.sol";
import {GameType, Claim} from "optimism/src/dispute/lib/Types.sol";
import {
    ISystemConfig,
    IDisputeGameFactory,
    Hash,
    Proposal,
    AnchorStateRegistry
} from "optimism/src/dispute/AnchorStateRegistry.sol";

// OpenZeppelin
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract BaseTest is Test {
    // Constants
    GameType public constant AGGREGATE_VERIFIER_GAME_TYPE = GameType.wrap(621);
    uint256 public constant L2_CHAIN_ID = 8453;
    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INIT_BOND = 1 ether;
    // Finality delay handled by the AggregateVerifier
    uint256 public constant FINALITY_DELAY = 0 days;

    uint256 public currentL2BlockNumber = 0;

    address public immutable TEE_PROVER = makeAddr("tee-prover");
    address public immutable ZK_PROVER = makeAddr("zk-prover");
    address public immutable ATTACKER = makeAddr("attacker");

    bytes32 public immutable TEE_IMAGE_HASH = keccak256("tee-image");
    bytes32 public immutable ZK_IMAGE_HASH = keccak256("zk-image");
    bytes32 public immutable CONFIG_HASH = keccak256("config");

    ProxyAdmin public proxyAdmin;
    MockSystemConfig public systemConfig;

    DisputeGameFactory public factory;
    AnchorStateRegistry public anchorStateRegistry;

    MockVerifier public teeVerifier;
    MockVerifier public zkVerifier;

    function setUp() public virtual {
        _deployContractsAndProxies();
        _initializeProxies();

        // Deploy the implementations
        _deployAndSetAggregateVerifier();

        anchorStateRegistry.setRespectedGameType(AGGREGATE_VERIFIER_GAME_TYPE);

        // Set the timestamp to after the retirement timestamp
        vm.warp(block.timestamp + 1);
    }

    function _deployContractsAndProxies() internal {
        // Deploy the system config
        systemConfig = new MockSystemConfig();

        // Deploy the relay anchor state registry
        AnchorStateRegistry _anchorStateRegistry = new AnchorStateRegistry(FINALITY_DELAY);
        // Deploy the dispute game factory
        DisputeGameFactory _factory = new DisputeGameFactory();

        // Deploy proxy admin
        proxyAdmin = new ProxyAdmin();

        // Deploy proxy for anchor state registry
        TransparentUpgradeableProxy anchorStateRegistryProxy =
            new TransparentUpgradeableProxy(address(_anchorStateRegistry), address(proxyAdmin), "");
        anchorStateRegistry = AnchorStateRegistry(address(anchorStateRegistryProxy));

        // Deploy proxy for factory
        TransparentUpgradeableProxy factoryProxy =
            new TransparentUpgradeableProxy(address(_factory), address(proxyAdmin), "");
        factory = DisputeGameFactory(address(factoryProxy));

        // Deploy the verifiers
        teeVerifier = new MockVerifier();
        zkVerifier = new MockVerifier();
    }

    function _initializeProxies() internal {
        // Initialize the proxies
        anchorStateRegistry.initialize(
            ISystemConfig(address(systemConfig)),
            IDisputeGameFactory(address(factory)),
            Proposal({
                root: Hash.wrap(keccak256(abi.encode(currentL2BlockNumber))), l2SequenceNumber: currentL2BlockNumber
            }),
            GameType.wrap(0)
        );
        factory.initialize(address(this));
    }

    function _deployAndSetAggregateVerifier() internal {
        // Deploy the dispute game relay implementation
        AggregateVerifier aggregateVerifierImpl = new AggregateVerifier(
            AGGREGATE_VERIFIER_GAME_TYPE,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            ZK_IMAGE_HASH,
            CONFIG_HASH,
            TEE_PROVER,
            L2_CHAIN_ID,
            BLOCK_INTERVAL
        );

        // Set the implementation for the aggregate verifier
        factory.setImplementation(AGGREGATE_VERIFIER_GAME_TYPE, IDisputeGame(address(aggregateVerifierImpl)));

        // Set the bond amount for the aggregate verifier
        factory.setInitBond(AGGREGATE_VERIFIER_GAME_TYPE, INIT_BOND);
    }

    // Helper function to create a game via factory
    function _createAggregateVerifierGame(address creator, Claim rootClaim, uint256 l2BlockNumber, uint32 parentIndex)
        internal
        returns (AggregateVerifier game)
    {
        bytes memory extraData = abi.encodePacked(uint256(l2BlockNumber), uint32(parentIndex));

        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        return AggregateVerifier(
            address(factory.create{value: INIT_BOND}(AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, extraData))
        );
    }

    function _provideProof(AggregateVerifier game, address prover, bool isTeeProof, bytes memory proof) internal {
        vm.prank(prover);
        if (isTeeProof) {
            game.verifyProof(proof, AggregateVerifier.ProofType.TEE);
        } else {
            game.verifyProof(proof, AggregateVerifier.ProofType.ZK);
        }
    }
}
