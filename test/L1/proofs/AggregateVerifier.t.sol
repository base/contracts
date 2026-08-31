// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { BadExtraData, GameNotResolved } from "src/libraries/bridge/Errors.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameStatus, GameType, GameTypes, Hash } from "src/libraries/bridge/Types.sol";
import { Claim, Timestamp } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

import { LibClone } from "lib/solady/src/utils/LibClone.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract AggregateVerifierTest is BaseTest {
    using LibClone for address;

    uint256 private constant DENIM_UPGRADE_INDEX = 13;
    uint256 private constant DENIM_BLOCKS_PER_SECOND = 5;
    uint256 private constant DENIM_BLOCK_INTERVAL = 6000;
    uint256 private constant DENIM_INTERMEDIATE_BLOCK_INTERVAL = 300;
    GameType private constant DENIM_GAME_TYPE = GameType.wrap(622);

    AggregateVerifier private aggregateVerifierImpl;

    function setUp() public override {
        super.setUp();
        aggregateVerifierImpl = AggregateVerifier(address(factory.gameImpls(GameTypes.AGGREGATE_VERIFIER)));
    }

    function testInitializeWithTEEProof() public {
        _createAndAssertInitializedGame(
            "tee-proof", AggregateVerifier.ProofType.TEE, TEE_PROVER, TEE_PROVER, address(0)
        );
    }

    function testInitializeWithZKProof() public {
        _createAndAssertInitializedGame("zk-proof", AggregateVerifier.ProofType.ZK, ZK_PROVER, address(0), ZK_PROVER);
    }

    /// @notice Initialization pins the upgrades active at the claimed L2 block, independently of
    ///         the L1 game-creation timestamp, and later schedule changes cannot alter the pin.
    function test_initialize_pinsScheduleId_succeeds() public {
        uint64 firstGameTimestamp = _l2Timestamp(BLOCK_INTERVAL);
        uint64 secondActivationTimestamp = _l2Timestamp(BLOCK_INTERVAL + BLOCK_INTERVAL / 2);

        // The first upgrade is active at the first game's L2 timestamp; the second is not.
        uint64[] memory schedule = new uint64[](2);
        schedule[0] = firstGameTimestamp;
        schedule[1] = secondActivationTimestamp;
        _importProtocolVersionsSchedule(schedule);

        bytes32 pinned = protocolVersions.activatedScheduleId(firstGameTimestamp);
        assertTrue(pinned != bytes32(0));
        assertNotEq(protocolVersions.scheduleId(), pinned);

        // Even though the L1 clock has passed the second activation, the game's schedule is
        // selected by its claimed L2 block timestamp.
        vm.warp(secondActivationTimestamp);

        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        assertEq(game.scheduleId(), pinned);

        // Move the live schedule after creation; the game's snapshot remains pinned. A fresh
        // registration now has to clear MIN_NOTICE, which is what keeps it clear of the L2
        // timestamps the sequencer has already reached.
        protocolVersions.registerUpgrade(uint64(block.timestamp) + protocolVersions.MIN_NOTICE(), 3);
        assertNotEq(protocolVersions.scheduleId(), pinned);
        assertEq(game.scheduleId(), pinned);
    }

    /// @notice Consecutive games on opposite sides of an L2 activation boundary pin different
    ///         schedule commitments.
    function test_initialize_scheduleChangesAtL2ActivationBoundary_succeeds() public {
        uint64 activationTimestamp = _l2Timestamp(BLOCK_INTERVAL + BLOCK_INTERVAL / 2);
        uint64[] memory schedule = new uint64[](1);
        schedule[0] = activationTimestamp;
        _importProtocolVersionsSchedule(schedule);

        Claim firstClaim = _advanceL2BlockAndClaim();
        AggregateVerifier firstGame = _createAggregateVerifierGame(
            TEE_PROVER,
            firstClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _generateProof("first", AggregateVerifier.ProofType.TEE)
        );
        assertEq(firstGame.scheduleId(), bytes32(0));

        Claim secondClaim = _advanceL2BlockAndClaim();
        AggregateVerifier secondGame = _createAggregateVerifierGame(
            TEE_PROVER,
            secondClaim,
            currentL2BlockNumber,
            address(firstGame),
            _generateProof("second", AggregateVerifier.ProofType.TEE)
        );
        assertEq(secondGame.scheduleId(), protocolVersions.activatedScheduleId(_l2Timestamp(currentL2BlockNumber)));
    }

    function test_initialize_denimBlockTimestamps_succeeds() public {
        uint64 denimActivationTimestamp = L2_GENESIS_TIMESTAMP + 1;
        uint64 denimBlockTimestamp = L2_GENESIS_TIMESTAMP + L2_BLOCK_TIME;
        uint64[] memory schedule = new uint64[](DENIM_UPGRADE_INDEX + 2);
        for (uint256 i; i < DENIM_UPGRADE_INDEX; i++) {
            schedule[i] = L2_GENESIS_TIMESTAMP;
        }
        schedule[DENIM_UPGRADE_INDEX] = denimActivationTimestamp;
        schedule[DENIM_UPGRADE_INDEX + 1] = denimBlockTimestamp + 1;
        _importProtocolVersionsSchedule(schedule);

        _setSingleBlockAggregateVerifier(L2_GENESIS_TIMESTAMP);

        bytes32 denimScheduleId = protocolVersions.scheduleId(DENIM_UPGRADE_INDEX);
        bytes32 postDenimScheduleId = protocolVersions.scheduleId(DENIM_UPGRADE_INDEX + 1);
        address parent = address(anchorStateRegistry);

        for (uint256 l2BlockNumber = 1; l2BlockNumber <= DENIM_BLOCKS_PER_SECOND + 1; l2BlockNumber++) {
            vm.warp(denimBlockTimestamp + (l2BlockNumber - 1) / DENIM_BLOCKS_PER_SECOND);
            AggregateVerifier game = _createSingleBlockGame(l2BlockNumber, parent);
            assertEq(
                game.scheduleId(), l2BlockNumber <= DENIM_BLOCKS_PER_SECOND ? denimScheduleId : postDenimScheduleId
            );
            parent = address(game);
        }
    }

    function test_initialize_denimCrossingSixThousandBlockRange_succeeds() public {
        uint64 denimActivationTimestamp = L2_GENESIS_TIMESTAMP + L2_BLOCK_TIME;
        uint64[] memory schedule = new uint64[](DENIM_UPGRADE_INDEX + 1);
        for (uint256 i; i < DENIM_UPGRADE_INDEX; i++) {
            schedule[i] = L2_GENESIS_TIMESTAMP;
        }
        schedule[DENIM_UPGRADE_INDEX] = denimActivationTimestamp;
        _importProtocolVersionsSchedule(schedule);

        AggregateVerifier implementation = _deployAggregateVerifier(
            DENIM_BLOCK_INTERVAL,
            DENIM_INTERMEDIATE_BLOCK_INTERVAL,
            AggregateVerifier.ScheduleConfig({
                protocolVersions: IProtocolVersions(address(protocolVersions)),
                genesisBlockNumber: L2_GENESIS_BLOCK_NUMBER,
                genesisTimestamp: L2_GENESIS_TIMESTAMP,
                blockTime: L2_BLOCK_TIME
            })
        );
        factory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(implementation)));

        uint256 claimBlock = L2_GENESIS_BLOCK_NUMBER + DENIM_BLOCK_INTERVAL;
        uint64 claimTimestamp = denimActivationTimestamp + uint64((DENIM_BLOCK_INTERVAL - 1) / DENIM_BLOCKS_PER_SECOND);
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(claimBlock)));
        bytes memory extraData = _aggregateVerifierExtraDataForIntervals(
            rootClaim, claimBlock, address(anchorStateRegistry), DENIM_BLOCK_INTERVAL, DENIM_INTERMEDIATE_BLOCK_INTERVAL
        );
        bytes memory proof = _generateProof("denim-crossing", AggregateVerifier.ProofType.TEE);

        vm.warp(claimTimestamp - 1);
        vm.deal(TEE_PROVER, INIT_BOND);
        vm.prank(TEE_PROVER);
        vm.expectRevert(
            abi.encodeWithSelector(
                AggregateVerifier.L2TimestampInFuture.selector, claimTimestamp, uint256(claimTimestamp - 1)
            )
        );
        factory.createWithInitData{ value: INIT_BOND }(GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof);

        vm.warp(claimTimestamp);
        vm.prank(TEE_PROVER);
        AggregateVerifier game = AggregateVerifier(
            address(
                factory.createWithInitData{ value: INIT_BOND }(
                    GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof
                )
            )
        );

        (, uint256 startingBlock) = game.startingOutputRoot();
        assertEq(game.scheduleId(), protocolVersions.scheduleId(DENIM_UPGRADE_INDEX));
        assertEq(startingBlock, L2_GENESIS_BLOCK_NUMBER);
        assertEq(game.BLOCK_INTERVAL(), DENIM_BLOCK_INTERVAL);
        assertEq(game.INTERMEDIATE_BLOCK_INTERVAL(), DENIM_INTERMEDIATE_BLOCK_INTERVAL);
        assertEq(game.intermediateOutputRootsCount(), 20);
    }

    function test_initialize_newGameTypeCanParentPreviouslyRespectedGame_succeeds() public {
        Claim oldClaim = _advanceL2BlockAndClaim();
        AggregateVerifier oldGame = _createAggregateVerifierGame(
            TEE_PROVER,
            oldClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _generateProof("old-game", AggregateVerifier.ProofType.TEE)
        );
        assertTrue(oldGame.wasRespectedGameTypeWhenCreated());

        AggregateVerifier implementation = _deployAggregateVerifier(
            DENIM_GAME_TYPE,
            DENIM_BLOCK_INTERVAL,
            DENIM_INTERMEDIATE_BLOCK_INTERVAL,
            AggregateVerifier.ScheduleConfig({
                protocolVersions: IProtocolVersions(address(protocolVersions)),
                genesisBlockNumber: L2_GENESIS_BLOCK_NUMBER,
                genesisTimestamp: L2_GENESIS_TIMESTAMP,
                blockTime: L2_BLOCK_TIME
            })
        );
        factory.setImplementation(DENIM_GAME_TYPE, IDisputeGame(address(implementation)));
        factory.setInitBond(DENIM_GAME_TYPE, INIT_BOND);
        anchorStateRegistry.setRespectedGameType(DENIM_GAME_TYPE);

        uint256 claimBlock = oldGame.l2SequenceNumber() + DENIM_BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(claimBlock)));
        bytes memory extraData = _aggregateVerifierExtraDataForIntervals(
            rootClaim, claimBlock, address(oldGame), DENIM_BLOCK_INTERVAL, DENIM_INTERMEDIATE_BLOCK_INTERVAL
        );
        bytes memory proof = _generateProof("new-game", AggregateVerifier.ProofType.TEE);

        _warpToL2Timestamp(claimBlock);
        vm.deal(TEE_PROVER, INIT_BOND);
        vm.prank(TEE_PROVER);
        AggregateVerifier game = AggregateVerifier(
            address(factory.createWithInitData{ value: INIT_BOND }(DENIM_GAME_TYPE, rootClaim, extraData, proof))
        );

        (Hash startingRoot, uint256 startingBlock) = game.startingOutputRoot();
        assertEq(game.parentAddress(), address(oldGame));
        assertEq(startingRoot.raw(), oldClaim.raw());
        assertEq(startingBlock, oldGame.l2SequenceNumber());
        assertTrue(game.wasRespectedGameTypeWhenCreated());
    }

    function test_initialize_unscheduledDenimUsesLegacyTimestamp_succeeds() public {
        uint64[] memory schedule = new uint64[](DENIM_UPGRADE_INDEX + 1);
        for (uint256 i; i < DENIM_UPGRADE_INDEX; i++) {
            schedule[i] = L2_GENESIS_TIMESTAMP;
        }
        _importProtocolVersionsSchedule(schedule);
        _setSingleBlockAggregateVerifier(L2_GENESIS_TIMESTAMP);

        uint64 legacyBlockTimestamp = L2_GENESIS_TIMESTAMP + L2_BLOCK_TIME;
        vm.warp(legacyBlockTimestamp - 1);
        vm.expectRevert(
            abi.encodeWithSelector(
                AggregateVerifier.L2TimestampInFuture.selector, legacyBlockTimestamp, legacyBlockTimestamp - 1
            )
        );
        _createSingleBlockGame(1, address(anchorStateRegistry));

        vm.warp(legacyBlockTimestamp);
        AggregateVerifier game = _createSingleBlockGame(1, address(anchorStateRegistry));
        assertEq(game.scheduleId(), protocolVersions.scheduleId(DENIM_UPGRADE_INDEX - 1));
    }

    /// @notice A claim whose L2 timestamp L1 has not yet reached cannot open a game, so a game can
    ///         never pin an activation that the owner is still able to clear or delay.
    function test_initialize_l2TimestampInFuture_reverts() public {
        // Scheduling the upgrade at the first game's deterministic L2 timestamp and leaving the L1
        // clock short of it is the window the finding exploits.
        uint64 activationTimestamp = _l2Timestamp(BLOCK_INTERVAL);
        uint64[] memory schedule = new uint64[](1);
        schedule[0] = activationTimestamp;
        _importProtocolVersionsSchedule(schedule);
        assertLt(block.timestamp, activationTimestamp);

        // Created straight through the factory, since the shared helper advances L1 to the claim.
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory proof = _generateProof("future-l2", AggregateVerifier.ProofType.TEE);
        bytes memory extraData =
            _aggregateVerifierExtraData(rootClaim, currentL2BlockNumber, address(anchorStateRegistry));

        vm.deal(TEE_PROVER, INIT_BOND);
        vm.prank(TEE_PROVER);
        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.L2TimestampInFuture.selector, activationTimestamp, block.timestamp)
        );
        factory.createWithInitData{ value: INIT_BOND }(GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof);

        // Once L1 reaches the activation, the schedule is frozen and the game becomes creatable.
        vm.warp(activationTimestamp);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, 0, activationTimestamp
            )
        );
        protocolVersions.setTimestamp(0, 0);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );
        assertEq(game.scheduleId(), protocolVersions.activatedScheduleId(activationTimestamp));
    }

    function test_constructor_zeroL2BlockTime_reverts() public {
        AggregateVerifier.ScheduleConfig memory scheduleConfig = AggregateVerifier.ScheduleConfig({
            protocolVersions: IProtocolVersions(address(protocolVersions)),
            genesisBlockNumber: 0,
            genesisTimestamp: 0,
            blockTime: 0
        });

        vm.expectRevert(AggregateVerifier.InvalidL2BlockTime.selector);
        _deployAggregateVerifierWithScheduleConfig(scheduleConfig);
    }

    function test_initialize_l2BlockBeforeGenesis_reverts() public {
        AggregateVerifier.ScheduleConfig memory scheduleConfig = AggregateVerifier.ScheduleConfig({
            protocolVersions: IProtocolVersions(address(protocolVersions)),
            genesisBlockNumber: BLOCK_INTERVAL + 1,
            genesisTimestamp: 0,
            blockTime: L2_BLOCK_TIME
        });
        AggregateVerifier implementation = _deployAggregateVerifierWithScheduleConfig(scheduleConfig);
        factory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(implementation)));

        Claim rootClaim = _advanceL2BlockAndClaim();
        vm.expectRevert(
            abi.encodeWithSelector(
                AggregateVerifier.L2BlockBeforeGenesis.selector, currentL2BlockNumber, BLOCK_INTERVAL + 1
            )
        );
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _generateProof("before-genesis", AggregateVerifier.ProofType.TEE)
        );
    }

    function test_initialize_l2TimestampOverflow_reverts() public {
        AggregateVerifier.ScheduleConfig memory scheduleConfig = AggregateVerifier.ScheduleConfig({
            protocolVersions: IProtocolVersions(address(protocolVersions)),
            genesisBlockNumber: 0,
            genesisTimestamp: type(uint64).max,
            blockTime: L2_BLOCK_TIME
        });
        AggregateVerifier implementation = _deployAggregateVerifierWithScheduleConfig(scheduleConfig);
        factory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(implementation)));

        Claim rootClaim = _advanceL2BlockAndClaim();
        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.L2TimestampOverflow.selector, currentL2BlockNumber));
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _generateProof("timestamp-overflow", AggregateVerifier.ProofType.TEE)
        );
    }

    function test_initialize_denimTimestampOverflow_reverts() public {
        uint64 genesisTimestamp = type(uint64).max - L2_BLOCK_TIME;
        uint64[] memory schedule = new uint64[](DENIM_UPGRADE_INDEX + 1);
        for (uint256 i; i < DENIM_UPGRADE_INDEX; i++) {
            schedule[i] = genesisTimestamp;
        }
        schedule[DENIM_UPGRADE_INDEX] = genesisTimestamp + 1;
        _importProtocolVersionsSchedule(schedule);
        _setSingleBlockAggregateVerifier(genesisTimestamp);

        vm.warp(uint256(type(uint64).max) + 3);
        address parent = address(anchorStateRegistry);
        for (uint256 l2BlockNumber = 1; l2BlockNumber <= DENIM_BLOCKS_PER_SECOND; l2BlockNumber++) {
            parent = address(_createSingleBlockGame(l2BlockNumber, parent));
        }

        uint256 overflowingBlock = DENIM_BLOCKS_PER_SECOND + 1;
        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.L2TimestampOverflow.selector, overflowingBlock));
        _createSingleBlockGame(overflowingBlock, parent);
    }

    function testInitializeFailsIfInvalidCallDataSize() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.deal(TEE_PROVER, INIT_BOND);
        bytes memory extraData = "";
        bytes memory initData = "";

        vm.prank(TEE_PROVER);
        vm.expectRevert(BadExtraData.selector);
        factory.createWithInitData{ value: INIT_BOND }(GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, initData);
    }

    function testUpdatingAnchorStateRegistryWithTEEProof() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        vm.expectRevert(GameNotResolved.selector);
        game.claimCredit();

        vm.warp(block.timestamp + aggregateVerifierImpl.SLOW_FINALIZATION_DELAY());
        game.resolve();
        _assertStatus(game, GameStatus.DEFENDER_WINS);

        _claimCreditAfterDelay(game);

        vm.warp(block.timestamp + 1);
        game.closeGame();
        _assertAnchorRoot(rootClaim);
    }

    function testUpdatingAnchorStateRegistryWithZKProof() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory proof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        vm.warp(block.timestamp + aggregateVerifierImpl.SLOW_FINALIZATION_DELAY());
        game.resolve();
        _assertStatus(game, GameStatus.DEFENDER_WINS);

        _claimCreditAfterDelay(game);

        vm.warp(block.timestamp + 1);
        game.closeGame();
        _assertAnchorRoot(rootClaim);
    }

    function testUpdatingAnchorStateRegistryWithBothProofs() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        _provideProof(game, ZK_PROVER, zkProof);
        assertEq(game.proofCount(), 2);

        vm.warp(block.timestamp + aggregateVerifierImpl.FAST_FINALIZATION_DELAY());
        game.resolve();
        _assertStatus(game, GameStatus.DEFENDER_WINS);

        vm.warp(block.timestamp + 1);
        game.closeGame();
        _assertAnchorRoot(rootClaim);

        _claimCreditAfterDelay(game);
    }

    function testProofCannotIncreaseExpectedResolution() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);
        uint256 slowDelay = aggregateVerifierImpl.SLOW_FINALIZATION_DELAY();

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        Timestamp originalExpectedResolution = game.expectedResolution();
        assertEq(originalExpectedResolution.raw(), block.timestamp + slowDelay);

        vm.warp(block.timestamp + slowDelay - 1);
        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        game.resolve();

        _provideProof(game, ZK_PROVER, zkProof);
        assertEq(game.expectedResolution().raw(), originalExpectedResolution.raw());

        vm.warp(block.timestamp + 1);
        game.resolve();
        _assertStatus(game, GameStatus.DEFENDER_WINS);
    }

    function testCannotCreateSameProposal() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        Hash gameId = factory.getGameUUID(GameTypes.AGGREGATE_VERIFIER, rootClaim, game.extraData());
        vm.expectRevert(abi.encodeWithSelector(IDisputeGameFactory.GameAlreadyExists.selector, gameId));
        _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), zkProof);
    }

    /// @notice Reverts when the parent is not factory-registered: `_isValidGame` requires
    ///         `AnchorStateRegistry.isGameRegistered`, which checks `DisputeGameFactory.games(...) == parent`.
    /// @dev Parent is a real `AggregateVerifier` clone initialized like a factory game, but deployed without
    ///      `_finalizeGameCreation`, so the factory UUID mapping has no entry.
    function testInitializeFailsIfParentGameNotFactoryRegistered() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim parentRootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "parent")));
        AggregateVerifier unregisteredParent = _deployAggregateVerifierCloneWithoutFactoryRegistration(
            TEE_PROVER,
            parentRootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _generateProof("parent-tee", AggregateVerifier.ProofType.TEE)
        );

        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim childRootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "child")));

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        _createAggregateVerifierGame(
            TEE_PROVER,
            childRootClaim,
            currentL2BlockNumber,
            address(unregisteredParent),
            _generateProof("child-tee", AggregateVerifier.ProofType.TEE)
        );
    }

    function testVerifyFailsWithL1OriginInFuture() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        uint256 l1OriginNumber = block.number + 1;
        bytes32 l1OriginHash = bytes32(uint256(1));

        bytes memory proofBytes = _teeProof(l1OriginHash, l1OriginNumber, rootClaim);

        _expectCreateGameRevertsForTeeProof(
            rootClaim,
            proofBytes,
            abi.encodeWithSelector(AggregateVerifier.L1OriginInFuture.selector, l1OriginNumber, block.number)
        );
    }

    function testVerifyFailsWithL1OriginTooOld() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.roll(block.number + 300);

        uint256 l1OriginNumber = 1;
        bytes32 l1OriginHash = bytes32(uint256(1));

        bytes memory proofBytes = _teeProof(l1OriginHash, l1OriginNumber, rootClaim);

        _expectCreateGameRevertsForTeeProof(
            rootClaim,
            proofBytes,
            abi.encodeWithSelector(AggregateVerifier.L1OriginTooOld.selector, l1OriginNumber, block.number)
        );
    }

    function testVerifyFailsWithL1OriginHashMismatch() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        uint256 l1OriginNumber = block.number - 1;
        bytes32 wrongHash = bytes32(uint256(0xdeadbeef));

        bytes memory proofBytes = _teeProof(wrongHash, l1OriginNumber, rootClaim);

        bytes32 actualHash = blockhash(l1OriginNumber);
        _expectCreateGameRevertsForTeeProof(
            rootClaim,
            proofBytes,
            abi.encodeWithSelector(AggregateVerifier.L1OriginHashMismatch.selector, wrongHash, actualHash)
        );
    }

    function testVerifyWithBlockhashWindow() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.roll(block.number + 100);

        uint256 l1OriginNumber = block.number - 50;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        bytes memory proofBytes = _teeProof(l1OriginHash, l1OriginNumber, rootClaim);

        _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proofBytes
        );
    }

    function testVerifyWithEIP2935Window() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.roll(block.number + 300);

        uint256 l1OriginNumber = block.number - 260;
        bytes32 expectedHash = keccak256(abi.encodePacked("mock-blockhash", l1OriginNumber));

        vm.mockCall(aggregateVerifierImpl.EIP2935_CONTRACT(), abi.encode(l1OriginNumber), abi.encode(expectedHash));

        bytes memory proofBytes = _teeProof(expectedHash, l1OriginNumber, rootClaim);

        _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proofBytes
        );
    }

    function testDeployWithInvalidBlockIntervals() public {
        _expectDeployWithInvalidBlockIntervalsReverts(0, INTERMEDIATE_BLOCK_INTERVAL);
        _expectDeployWithInvalidBlockIntervalsReverts(BLOCK_INTERVAL, 0);
        _expectDeployWithInvalidBlockIntervalsReverts(3, 2);
    }

    function _advanceL2BlockAndClaim() private returns (Claim rootClaim) {
        currentL2BlockNumber += BLOCK_INTERVAL;
        return Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
    }

    function _createSingleBlockGame(uint256 l2BlockNumber, address parent) private returns (AggregateVerifier) {
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(l2BlockNumber)));
        bytes memory extraData = abi.encodePacked(l2BlockNumber, parent, rootClaim.raw());
        bytes memory proof = _generateProof(abi.encode(l2BlockNumber), AggregateVerifier.ProofType.TEE);

        vm.deal(TEE_PROVER, INIT_BOND);
        vm.prank(TEE_PROVER);
        return AggregateVerifier(
            address(
                factory.createWithInitData{ value: INIT_BOND }(
                    GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof
                )
            )
        );
    }

    function _setSingleBlockAggregateVerifier(uint64 genesisTimestamp) private {
        AggregateVerifier implementation = _deployAggregateVerifier(
            1,
            1,
            AggregateVerifier.ScheduleConfig({
                protocolVersions: IProtocolVersions(address(protocolVersions)),
                genesisBlockNumber: L2_GENESIS_BLOCK_NUMBER,
                genesisTimestamp: genesisTimestamp,
                blockTime: L2_BLOCK_TIME
            })
        );
        factory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(implementation)));
    }

    function _createAndAssertInitializedGame(
        bytes memory proofSalt,
        AggregateVerifier.ProofType proofType,
        address prover,
        address expectedTeeProver,
        address expectedZkProver
    )
        private
    {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory proof = _generateProof(proofSalt, proofType);

        AggregateVerifier game = _createAggregateVerifierGame(
            prover, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        _assertInitializedGame(game, rootClaim, prover, expectedTeeProver, expectedZkProver);
    }

    function _assertInitializedGame(
        AggregateVerifier game,
        Claim rootClaim,
        address expectedCreator,
        address expectedTeeProver,
        address expectedZkProver
    )
        private
        view
    {
        assertTrue(game.wasRespectedGameTypeWhenCreated());
        assertEq(game.teeProver(), expectedTeeProver);
        assertEq(game.zkProver(), expectedZkProver);
        _assertStatus(game, GameStatus.IN_PROGRESS);
        assertEq(game.l2SequenceNumber(), currentL2BlockNumber);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.parentAddress(), address(anchorStateRegistry));
        assertEq(game.gameType().raw(), GameTypes.AGGREGATE_VERIFIER.raw());
        assertEq(game.gameCreator(), expectedCreator);
        bytes memory intermediateOutputRoots = game.intermediateOutputRoots();
        assertEq(
            game.extraData(),
            abi.encodePacked(currentL2BlockNumber, address(anchorStateRegistry), intermediateOutputRoots)
        );
        assertEq(game.bondRecipient(), expectedCreator);
        assertTrue(anchorStateRegistry.isGameProper(IDisputeGame(address(game))));
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
        assertEq(game.proofCount(), 1);
    }

    function _assertStatus(AggregateVerifier game, GameStatus expectedStatus) private view {
        assertEq(uint8(game.status()), uint8(expectedStatus));
    }

    function _assertAnchorRoot(Claim rootClaim) private view {
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), rootClaim.raw());
        assertEq(l2SequenceNumber, currentL2BlockNumber);
    }

    function _claimCreditAfterDelay(AggregateVerifier game) private {
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }

    function _expectCreateGameRevertsForTeeProof(
        Claim rootClaim,
        bytes memory proofBytes,
        bytes memory revertData
    )
        private
    {
        vm.expectRevert(revertData);
        _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proofBytes
        );
    }

    function _teeProof(
        bytes32 l1OriginHash,
        uint256 l1OriginNumber,
        Claim rootClaim
    )
        private
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(uint8(AggregateVerifier.ProofType.TEE), l1OriginHash, l1OriginNumber, rootClaim.raw());
    }

    function _expectDeployWithInvalidBlockIntervalsReverts(
        uint256 blockInterval,
        uint256 intermediateBlockInterval
    )
        private
    {
        vm.expectRevert(
            abi.encodeWithSelector(
                AggregateVerifier.InvalidBlockInterval.selector, blockInterval, intermediateBlockInterval
            )
        );
        _deployAggregateVerifierWithIntervals(blockInterval, intermediateBlockInterval);
    }

    /// @notice Clones the implementation like the factory, but skips `_finalizeGameCreation`.
    function _deployAggregateVerifierCloneWithoutFactoryRegistration(
        address creator,
        Claim rootClaim,
        uint256 l2BlockNumber,
        address parentAddress,
        bytes memory proof
    )
        private
        returns (AggregateVerifier)
    {
        IDisputeGame impl = factory.gameImpls(GameTypes.AGGREGATE_VERIFIER);
        bytes memory extraData = _aggregateVerifierExtraData(rootClaim, l2BlockNumber, parentAddress);
        bytes32 l1Head = blockhash(block.number - 1);
        address clone = address(impl).clone(abi.encodePacked(creator, rootClaim, l1Head, extraData));

        _warpToL2Timestamp(l2BlockNumber);

        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        AggregateVerifier(payable(clone)).initializeWithInitData{ value: INIT_BOND }(proof);

        return AggregateVerifier(payable(clone));
    }

    function _deployAggregateVerifierWithIntervals(
        uint256 blockInterval,
        uint256 intermediateBlockInterval
    )
        private
        returns (AggregateVerifier)
    {
        return _deployAggregateVerifier(
            blockInterval,
            intermediateBlockInterval,
            AggregateVerifier.ScheduleConfig({
                protocolVersions: IProtocolVersions(address(protocolVersions)),
                genesisBlockNumber: L2_GENESIS_BLOCK_NUMBER,
                genesisTimestamp: L2_GENESIS_TIMESTAMP,
                blockTime: L2_BLOCK_TIME
            })
        );
    }

    function _deployAggregateVerifierWithScheduleConfig(AggregateVerifier.ScheduleConfig memory scheduleConfig)
        private
        returns (AggregateVerifier)
    {
        return _deployAggregateVerifier(BLOCK_INTERVAL, INTERMEDIATE_BLOCK_INTERVAL, scheduleConfig);
    }

    function _deployAggregateVerifier(
        uint256 blockInterval,
        uint256 intermediateBlockInterval,
        AggregateVerifier.ScheduleConfig memory scheduleConfig
    )
        private
        returns (AggregateVerifier)
    {
        return _deployAggregateVerifier(
            GameTypes.AGGREGATE_VERIFIER, blockInterval, intermediateBlockInterval, scheduleConfig
        );
    }

    function _deployAggregateVerifier(
        GameType gameType,
        uint256 blockInterval,
        uint256 intermediateBlockInterval,
        AggregateVerifier.ScheduleConfig memory scheduleConfig
    )
        private
        returns (AggregateVerifier)
    {
        return new AggregateVerifier(
            gameType,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            AggregateVerifier.ZkHashes(ZK_RANGE_HASH, ZK_AGGREGATE_HASH),
            CONFIG_HASH,
            L2_CHAIN_ID,
            blockInterval,
            intermediateBlockInterval,
            scheduleConfig
        );
    }

    function _aggregateVerifierExtraDataForIntervals(
        Claim rootClaim,
        uint256 l2BlockNumber,
        address parentAddress,
        uint256 blockInterval,
        uint256 intermediateBlockInterval
    )
        private
        pure
        returns (bytes memory)
    {
        uint256 rootsCount = blockInterval / intermediateBlockInterval;
        bytes32[] memory intermediateRoots = new bytes32[](rootsCount);
        uint256 startingL2BlockNumber = l2BlockNumber - blockInterval;
        for (uint256 i = 1; i < rootsCount; i++) {
            intermediateRoots[i - 1] = keccak256(abi.encode(startingL2BlockNumber + intermediateBlockInterval * i));
        }
        intermediateRoots[rootsCount - 1] = rootClaim.raw();
        return abi.encodePacked(l2BlockNumber, parentAddress, intermediateRoots);
    }
}
