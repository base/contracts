// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { BadExtraData, GameNotResolved } from "src/libraries/bridge/Errors.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameStatus, Hash } from "src/libraries/bridge/Types.sol";
import { Claim, Timestamp } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract AggregateVerifierTest is BaseTest {
    AggregateVerifier private aggregateVerifierImpl;

    function setUp() public override {
        super.setUp();
        aggregateVerifierImpl = AggregateVerifier(address(factory.gameImpls(AGGREGATE_VERIFIER_GAME_TYPE)));
    }

    function testInitializeWithTEEProof() public {
        _createAndAssertInitializedGame(
            "tee-proof", AggregateVerifier.ProofType.TEE, TEE_PROVER, TEE_PROVER, address(0)
        );
    }

    function testInitializeWithZKProof() public {
        _createAndAssertInitializedGame("zk-proof", AggregateVerifier.ProofType.ZK, ZK_PROVER, address(0), ZK_PROVER);
    }

    function testInitializeFailsIfInvalidCallDataSize() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.deal(TEE_PROVER, INIT_BOND);
        bytes memory extraData = "";
        bytes memory initData = "";

        vm.prank(TEE_PROVER);
        vm.expectRevert(BadExtraData.selector);
        factory.createWithInitData{ value: INIT_BOND }(AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, extraData, initData);
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

        Hash gameId = factory.getGameUUID(AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, game.extraData());
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
        assertEq(game.gameType().raw(), AGGREGATE_VERIFIER_GAME_TYPE.raw());
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

    function _deployAggregateVerifierWithIntervals(
        uint256 blockInterval,
        uint256 intermediateBlockInterval
    )
        private
        returns (AggregateVerifier)
    {
        return new AggregateVerifier(
            AGGREGATE_VERIFIER_GAME_TYPE,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            AggregateVerifier.ZkHashes(ZK_RANGE_HASH, ZK_AGGREGATE_HASH),
            CONFIG_HASH,
            L2_CHAIN_ID,
            blockInterval,
            intermediateBlockInterval
        );
    }

}
