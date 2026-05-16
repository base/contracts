// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "lib/forge-std/src/Test.sol";

// Optimism
import { AnchorStateRegistry } from "src/L1/proofs/AnchorStateRegistry.sol";
import { DelayedWETH } from "src/L1/proofs/DelayedWETH.sol";
import { DisputeGameFactory } from "src/L1/proofs/DisputeGameFactory.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { GameType, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

// OpenZeppelin
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";
import {
    TransparentUpgradeableProxy
} from "lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";

import { MockVerifier } from "test/mocks/MockVerifier.sol";

import { LibClone } from "lib/solady/src/utils/LibClone.sol";

contract BaseTest is Test {
    using LibClone for address;

    GameType public constant AGGREGATE_VERIFIER_GAME_TYPE = GameType.wrap(621);
    uint256 public constant L2_CHAIN_ID = 8453;

    // AggregateVerifier expects evenly spaced intermediate roots.
    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 public constant INTERMEDIATE_ROOTS_COUNT = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL;

    uint256 public constant INIT_BOND = 1 ether;
    uint256 public constant DELAYED_WETH_DELAY = 1 days;
    // Finality delay handled by the AggregateVerifier
    uint256 public constant FINALITY_DELAY = 0 days;

    uint256 public currentL2BlockNumber;

    address public immutable TEE_PROVER = makeAddr("tee-prover");
    address public immutable ZK_PROVER = makeAddr("zk-prover");
    address public immutable ATTACKER = makeAddr("attacker");

    bytes32 public immutable TEE_IMAGE_HASH = keccak256("tee-image");
    bytes32 public immutable ZK_RANGE_HASH = keccak256("zk-range");
    bytes32 public immutable ZK_AGGREGATE_HASH = keccak256("zk-aggregate");
    bytes32 public immutable CONFIG_HASH = keccak256("config");

    ProxyAdmin public proxyAdmin;
    ISystemConfig public systemConfig;

    DisputeGameFactory public factory;
    AnchorStateRegistry public anchorStateRegistry;
    DelayedWETH public delayedWETH;

    MockVerifier public teeVerifier;
    MockVerifier public zkVerifier;

    function setUp() public virtual {
        _deployContractsAndProxies();
        _initializeProxies();

        _deployAndSetAggregateVerifier();

        anchorStateRegistry.setRespectedGameType(AGGREGATE_VERIFIER_GAME_TYPE);

        // Games created at or before the registry's retirement timestamp are invalid.
        vm.warp(block.timestamp + 1);
    }

    function _deployContractsAndProxies() internal {
        systemConfig = ISystemConfig(makeAddr("system-config"));
        vm.mockCall(address(systemConfig), abi.encodeCall(ISystemConfig.guardian, ()), abi.encode(address(this)));
        vm.mockCall(address(systemConfig), abi.encodeCall(ISystemConfig.paused, ()), abi.encode(false));

        AnchorStateRegistry _anchorStateRegistry = new AnchorStateRegistry(FINALITY_DELAY);
        DelayedWETH _delayedWETH = new DelayedWETH(DELAYED_WETH_DELAY);
        DisputeGameFactory _factory = new DisputeGameFactory();

        proxyAdmin = new ProxyAdmin(address(this));

        TransparentUpgradeableProxy anchorStateRegistryProxy =
            new TransparentUpgradeableProxy(address(_anchorStateRegistry), address(proxyAdmin), "");
        anchorStateRegistry = AnchorStateRegistry(address(anchorStateRegistryProxy));

        TransparentUpgradeableProxy factoryProxy =
            new TransparentUpgradeableProxy(address(_factory), address(proxyAdmin), "");
        factory = DisputeGameFactory(address(factoryProxy));

        TransparentUpgradeableProxy delayedWETHProxy =
            new TransparentUpgradeableProxy(address(_delayedWETH), address(proxyAdmin), "");
        delayedWETH = DelayedWETH(payable(address(delayedWETHProxy)));

        teeVerifier = new MockVerifier(IAnchorStateRegistry(address(anchorStateRegistry)));
        zkVerifier = new MockVerifier(IAnchorStateRegistry(address(anchorStateRegistry)));
    }

    function _initializeProxies() internal {
        anchorStateRegistry.initialize(
            systemConfig,
            IDisputeGameFactory(address(factory)),
            Proposal({
                root: Hash.wrap(keccak256(abi.encode(currentL2BlockNumber))), l2SequenceNumber: currentL2BlockNumber
            }),
            GameType.wrap(0)
        );
        factory.initialize(address(this));
        delayedWETH.initialize(systemConfig);
    }

    function _deployAndSetAggregateVerifier() internal {
        AggregateVerifier aggregateVerifierImpl = new AggregateVerifier(
            AGGREGATE_VERIFIER_GAME_TYPE,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            AggregateVerifier.ZkHashes(ZK_RANGE_HASH, ZK_AGGREGATE_HASH),
            CONFIG_HASH,
            L2_CHAIN_ID,
            BLOCK_INTERVAL,
            INTERMEDIATE_BLOCK_INTERVAL
        );

        factory.setImplementation(AGGREGATE_VERIFIER_GAME_TYPE, IDisputeGame(address(aggregateVerifierImpl)));
        factory.setInitBond(AGGREGATE_VERIFIER_GAME_TYPE, INIT_BOND);
    }

    function _createAggregateVerifierGame(
        address creator,
        Claim rootClaim,
        uint256 l2BlockNumber,
        address parentAddress,
        bytes memory proof
    )
        internal
        returns (AggregateVerifier game)
    {
        bytes memory extraData = _aggregateVerifierExtraData(rootClaim, l2BlockNumber, parentAddress);

        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        return AggregateVerifier(
            address(
                factory.createWithInitData{ value: INIT_BOND }(
                    AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, extraData, proof
                )
            )
        );
    }

    /// @notice Clones the `AggregateVerifier` implementation with the same CWIA layout as `DisputeGameFactory`,
    ///         initializes it, but does not register the game in the factory (no `_finalizeGameCreation`).
    function _deployAggregateVerifierCloneWithoutFactoryRegistration(
        address creator,
        Claim rootClaim,
        uint256 l2BlockNumber,
        address parentAddress,
        bytes memory proof
    )
        internal
        returns (AggregateVerifier game)
    {
        IDisputeGame impl = factory.gameImpls(AGGREGATE_VERIFIER_GAME_TYPE);
        bytes memory extraData = _aggregateVerifierExtraData(rootClaim, l2BlockNumber, parentAddress);
        bytes32 l1Head = blockhash(block.number - 1);
        address clone = address(impl).clone(abi.encodePacked(creator, rootClaim, l1Head, extraData));
        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        AggregateVerifier(payable(clone)).initializeWithInitData{ value: INIT_BOND }(proof);
        return AggregateVerifier(payable(clone));
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
    function _generateProof(
        bytes memory salt,
        AggregateVerifier.ProofType proofType
    )
        internal
        view
        returns (bytes memory)
    {
        bytes32 l1OriginHash = blockhash(block.number - 1);
        uint256 l1OriginNumber = block.number - 1;
        bytes memory signature = abi.encodePacked(salt, bytes32(0), bytes32(0), uint8(27));

        return abi.encodePacked(uint8(proofType), l1OriginHash, l1OriginNumber, signature);
    }

    function _aggregateVerifierExtraData(
        Claim rootClaim,
        uint256 l2BlockNumber,
        address parentAddress
    )
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(
            uint256(l2BlockNumber), parentAddress, _generateIntermediateRootsExceptLast(l2BlockNumber), rootClaim.raw()
        );
    }

    function _generateIntermediateRootsExceptLast(uint256 l2BlockNumber) internal pure returns (bytes memory) {
        bytes memory intermediateRoots = new bytes((INTERMEDIATE_ROOTS_COUNT - 1) * 32);
        uint256 startingL2BlockNumber = l2BlockNumber - BLOCK_INTERVAL;
        for (uint256 i = 1; i < INTERMEDIATE_ROOTS_COUNT; i++) {
            bytes32 root = keccak256(abi.encode(startingL2BlockNumber + INTERMEDIATE_BLOCK_INTERVAL * i));
            assembly {
                mstore(add(add(intermediateRoots, 0x20), mul(sub(i, 1), 0x20)), root)
            }
        }
        return intermediateRoots;
    }
}
