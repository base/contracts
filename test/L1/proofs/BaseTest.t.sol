// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { AnchorStateRegistry } from "src/L1/proofs/AnchorStateRegistry.sol";
import { DelayedWETH } from "src/L1/proofs/DelayedWETH.sol";
import { DisputeGameFactory } from "src/L1/proofs/DisputeGameFactory.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { GameType, GameTypes, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

import { Proxy } from "src/universal/Proxy.sol";
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";
import { ProtocolVersions } from "src/L1/ProtocolVersions.sol";
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

import { MockVerifier } from "test/mocks/MockVerifier.sol";

contract BaseTest is Test {
    uint256 internal constant L2_CHAIN_ID = 8453;
    uint256 internal constant L2_GENESIS_BLOCK_NUMBER = 0;
    uint64 internal constant L2_GENESIS_TIMESTAMP = 0;
    uint64 internal constant L2_BLOCK_TIME = 2;

    // AggregateVerifier expects evenly spaced intermediate roots.
    uint256 internal constant BLOCK_INTERVAL = 100;
    uint256 internal constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 private constant INTERMEDIATE_ROOTS_COUNT = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL;

    uint256 internal constant INIT_BOND = 1 ether;
    uint256 internal constant DELAYED_WETH_DELAY = 1 days;
    // Finality delay handled by the AggregateVerifier
    uint256 internal constant FINALITY_DELAY = 0 days;

    uint256 internal currentL2BlockNumber;

    address internal immutable TEE_PROVER = makeAddr("tee-prover");
    address internal immutable ZK_PROVER = makeAddr("zk-prover");

    bytes32 internal immutable TEE_IMAGE_HASH = keccak256("tee-image");
    bytes32 internal immutable ZK_RANGE_HASH = keccak256("zk-range");
    bytes32 internal immutable ZK_AGGREGATE_HASH = keccak256("zk-aggregate");
    bytes32 internal immutable CONFIG_HASH = keccak256("config");

    ProxyAdmin internal proxyAdmin;
    ISystemConfig internal systemConfig;

    DisputeGameFactory internal factory;
    AnchorStateRegistry internal anchorStateRegistry;
    DelayedWETH internal delayedWETH;

    MockVerifier internal teeVerifier;
    MockVerifier internal zkVerifier;
    ProtocolVersions internal protocolVersions;

    function setUp() public virtual {
        _deployContractsAndProxies();
        _initializeProxies();

        _deployAndSetAggregateVerifier();

        anchorStateRegistry.setRespectedGameType(GameTypes.AGGREGATE_VERIFIER);

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

        ProtocolVersions _protocolVersions = new ProtocolVersions();

        anchorStateRegistry = AnchorStateRegistry(_deployProxy(address(_anchorStateRegistry)));
        factory = DisputeGameFactory(_deployProxy(address(_factory)));
        delayedWETH = DelayedWETH(payable(_deployProxy(address(_delayedWETH))));
        protocolVersions = ProtocolVersions(_deployProxy(address(_protocolVersions)));

        teeVerifier = new MockVerifier(IAnchorStateRegistry(address(anchorStateRegistry)));
        zkVerifier = new MockVerifier(IAnchorStateRegistry(address(anchorStateRegistry)));
    }

    function _deployProxy(address implementation) private returns (address) {
        Proxy proxy = new Proxy(address(proxyAdmin));
        proxyAdmin.upgrade(payable(address(proxy)), implementation);
        return address(proxy);
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
        protocolVersions.initialize(address(0), new uint64[](0));
    }

    /// @dev Rebuilds the schedule registry around a preset schedule and rebinds the verifier to it.
    ///      These tests run on a compressed L2 timescale whose activations can never clear
    ///      MIN_NOTICE, so a schedule has to be imported at initialization rather than registered.
    ///      Must be called before any game is created, since games pin the registry they see.
    function _importProtocolVersionsSchedule(uint64[] memory schedule) internal {
        protocolVersions = ProtocolVersions(_deployProxy(address(new ProtocolVersions())));
        protocolVersions.initialize(address(0), schedule);
        _deployAndSetAggregateVerifier();
    }

    function _deployAndSetAggregateVerifier() internal {
        AggregateVerifier aggregateVerifierImpl = new AggregateVerifier(
            GameTypes.AGGREGATE_VERIFIER,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            AggregateVerifier.ZkHashes(ZK_RANGE_HASH, ZK_AGGREGATE_HASH),
            CONFIG_HASH,
            L2_CHAIN_ID,
            BLOCK_INTERVAL,
            INTERMEDIATE_BLOCK_INTERVAL,
            AggregateVerifier.ScheduleConfig({
                protocolVersions: IProtocolVersions(address(protocolVersions)),
                genesisBlockNumber: L2_GENESIS_BLOCK_NUMBER,
                genesisTimestamp: L2_GENESIS_TIMESTAMP,
                blockTime: L2_BLOCK_TIME
            })
        );

        factory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(aggregateVerifierImpl)));
        factory.setInitBond(GameTypes.AGGREGATE_VERIFIER, INIT_BOND);
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

        _warpToL2Timestamp(l2BlockNumber);

        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        return AggregateVerifier(
            address(
                factory.createWithInitData{ value: INIT_BOND }(
                    GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof
                )
            )
        );
    }

    /// @dev A game is only creatable once L1 has reached the claimed L2 block's deterministic
    ///      timestamp, which mirrors production: a block is proven well after it is produced.
    function _warpToL2Timestamp(uint256 l2BlockNumber) internal {
        uint256 claimTimestamp = L2_GENESIS_TIMESTAMP + (l2BlockNumber - L2_GENESIS_BLOCK_NUMBER) * L2_BLOCK_TIME;
        if (block.timestamp < claimTimestamp) vm.warp(claimTimestamp);
    }

    function _provideProof(AggregateVerifier game, address prover, bytes memory proofBytes) internal {
        vm.prank(prover);
        game.verifyProposalProof(proofBytes);
    }

    /// @dev Encodes proofType || l1OriginHash || l1OriginNumber || mock verifier payload.
    function _generateProof(
        bytes memory salt,
        AggregateVerifier.ProofType proofType
    )
        internal
        view
        returns (bytes memory)
    {
        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        return abi.encodePacked(uint8(proofType), l1OriginHash, l1OriginNumber, salt, bytes32(0), bytes32(0), uint8(27));
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
        return abi.encodePacked(l2BlockNumber, parentAddress, _generateIntermediateRoots(l2BlockNumber, rootClaim));
    }

    function _generateIntermediateRoots(uint256 l2BlockNumber, Claim rootClaim) internal pure returns (bytes memory) {
        bytes32[] memory intermediateRoots = new bytes32[](INTERMEDIATE_ROOTS_COUNT);
        uint256 startingL2BlockNumber = l2BlockNumber - BLOCK_INTERVAL;
        for (uint256 i = 1; i < INTERMEDIATE_ROOTS_COUNT; i++) {
            intermediateRoots[i - 1] = keccak256(abi.encode(startingL2BlockNumber + INTERMEDIATE_BLOCK_INTERVAL * i));
        }
        intermediateRoots[INTERMEDIATE_ROOTS_COUNT - 1] = rootClaim.raw();

        return abi.encodePacked(intermediateRoots);
    }
}
