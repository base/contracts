// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";

// Optimism
import {AnchorStateRegistry} from "optimism/src/dispute/AnchorStateRegistry.sol";
import {DelayedWETH} from "optimism/src/dispute/DelayedWETH.sol";
import {DisputeGameFactory} from "optimism/src/dispute/DisputeGameFactory.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {IDelayedWETH} from "optimism/interfaces/dispute/IDelayedWETH.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "optimism/interfaces/dispute/IDisputeGameFactory.sol";
import {ISystemConfig} from "optimism/interfaces/L1/ISystemConfig.sol";
import {Claim, GameStatus, GameType, Hash, Proposal, Timestamp} from "optimism/src/dispute/lib/Types.sol";

// OpenZeppelin
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

import {AggregateVerifier} from "src/AggregateVerifier.sol";
import {IVerifier} from "src/interfaces/IVerifier.sol";

import {MockSystemConfig} from "src/mocks/MockSystemConfig.sol";
import {MockVerifier} from "src/mocks/MockVerifier.sol";

contract BaseTest is Test {
    // Constants
    GameType public constant AGGREGATE_VERIFIER_GAME_TYPE = GameType.wrap(621);
    uint256 public constant L2_CHAIN_ID = 8453;

    // MUST HAVE: BLOCK_INTERVAL % INTERMEDIATE_BLOCK_INTERVAL == 0
    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 10;

    uint256 public constant INIT_BOND = 1 ether;
    uint256 public constant DELAYED_WETH_DELAY = 1 days;
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
    DelayedWETH public delayedWETH;

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
        // Deploy the delayed WETH
        DelayedWETH _delayedWETH = new DelayedWETH(DELAYED_WETH_DELAY);
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

        // Deploy proxy for delayed WETH
        TransparentUpgradeableProxy delayedWETHProxy =
            new TransparentUpgradeableProxy(address(_delayedWETH), address(proxyAdmin), "");
        delayedWETH = DelayedWETH(payable(address(delayedWETHProxy)));

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
        delayedWETH.initialize(ISystemConfig(address(systemConfig)));
    }

    function _deployAndSetAggregateVerifier() internal {
        // Deploy the dispute game relay implementation
        AggregateVerifier aggregateVerifierImpl = new AggregateVerifier(
            AGGREGATE_VERIFIER_GAME_TYPE,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            ZK_IMAGE_HASH,
            CONFIG_HASH,
            L2_CHAIN_ID,
            BLOCK_INTERVAL,
            INTERMEDIATE_BLOCK_INTERVAL
        );

        // Set the implementation for the aggregate verifier
        factory.setImplementation(AGGREGATE_VERIFIER_GAME_TYPE, IDisputeGame(address(aggregateVerifierImpl)));

        // Set the bond amount for the aggregate verifier
        factory.setInitBond(AGGREGATE_VERIFIER_GAME_TYPE, INIT_BOND);
    }

    // Helper function to create a game via factory
    function _createAggregateVerifierGame(
        address creator,
        Claim rootClaim,
        uint256 l2BlockNumber,
        uint32 parentIndex,
        bytes memory proof
    ) internal returns (AggregateVerifier game) {
        bytes memory intermediateRoots =
            abi.encodePacked(_generateIntermediateRootsExceptLast(l2BlockNumber), rootClaim.raw());
        bytes memory extraData = abi.encodePacked(uint256(l2BlockNumber), uint32(parentIndex), intermediateRoots);

        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        return AggregateVerifier(
            address(factory.create{value: INIT_BOND}(AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, extraData, proof))
        );
    }

    function _provideProof(AggregateVerifier game, address prover, bytes memory proofBytes) internal {
        vm.prank(prover);
        game.verifyProposalProof(proofBytes);
    }

    /// @notice Generates a properly formatted proof for testing.
    /// @dev The proof format is: l1OriginHash (32) + l1OriginNumber (32) + additional data.
    ///      Since MockVerifier always returns true, we just need the correct structure.
    /// @param salt A salt to make proofs unique.
    /// @param proofType The type of proof to generate.
    /// @return proof The formatted proof bytes.
    function _generateProof(bytes memory salt, AggregateVerifier.ProofType proofType)
        internal
        view
        returns (bytes memory)
    {
        // Use the previous block hash as l1OriginHash
        bytes32 l1OriginHash = blockhash(block.number - 1);
        // Use the previous block number as l1OriginNumber
        uint256 l1OriginNumber = block.number - 1;
        // Add some padding/signature data (65 bytes minimum for a signature)
        bytes memory signature = abi.encodePacked(salt, bytes32(0), bytes32(0), uint8(27));

        return abi.encodePacked(uint8(proofType), l1OriginHash, l1OriginNumber, signature);
    }

    function _generateIntermediateRootsExceptLast(uint256 l2BlockNumber) internal pure returns (bytes memory) {
        bytes memory intermediateRoots;
        uint256 startingL2BlockNumber = l2BlockNumber - BLOCK_INTERVAL;
        for (uint256 i = 1; i < BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL; i++) {
            intermediateRoots = abi.encodePacked(
                intermediateRoots, keccak256(abi.encode(startingL2BlockNumber + INTERMEDIATE_BLOCK_INTERVAL * i))
            );
        }
        return intermediateRoots;
    }
}
