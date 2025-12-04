// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import "test/SetupTest.t.sol";
import {GameStatus, Timestamp, AggregateVerifier} from "src/AggregateVerifier.sol";
import {
    MissingTEEProof,
    MissingZKProof,
    IncorrectRootClaim,
    IncorrectBlockNumber,
    IncorrectParentIndex,
    NotAuthorized
} from "src/Errors.sol";

// Optimism
import { ClaimAlreadyResolved, GameAlreadyExists } from "optimism/src/dispute/lib/Errors.sol";

contract AggregateVerifierTest is SetupTest {
    function setUp() public override {
        super.setUp();
        anchorStateRegistry.setRespectedGameType(AGGREGATE_VERIFIER_GAME_TYPE);
    }

    function testInitializeWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "tee-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game, TEE_PROVER, true, proof);
        
        assertEq(game.wasRespectedGameTypeWhenCreated(), true);
        assertEq(address(game.teeProver()), TEE_PROVER);
        assertEq(address(game.zkProver()), address(0));
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.l2SequenceNumber(), currentL2BlockNumber);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.parentIndex(), type(uint32).max);
        assertEq(game.gameType().raw(), AGGREGATE_VERIFIER_GAME_TYPE.raw());
        assertEq(game.gameCreator(), TEE_PROVER);
        assertEq(game.extraData(), abi.encodePacked(currentL2BlockNumber, type(uint32).max));
        assertEq(game.bondRecipient(), address(0));
        assertEq(anchorStateRegistry.isGameProper(IDisputeGame(address(game))), true);
        assertEq(address(game).balance, INIT_BOND);
    }

    function testInitializeWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "zk-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max);
        _provideProof(game, ZK_PROVER, false, proof);
        
        assertEq(game.wasRespectedGameTypeWhenCreated(), true);
        assertEq(address(game.teeProver()), address(0));
        assertEq(address(game.zkProver()), ZK_PROVER);
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.l2SequenceNumber(), currentL2BlockNumber);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.parentIndex(), type(uint32).max);
        assertEq(game.gameType().raw(), AGGREGATE_VERIFIER_GAME_TYPE.raw());
        assertEq(game.gameCreator(), ZK_PROVER);
        assertEq(game.extraData(), abi.encodePacked(currentL2BlockNumber, type(uint32).max));
        assertEq(game.bondRecipient(), address(0));
        assertEq(anchorStateRegistry.isGameProper(IDisputeGame(address(game))), true);
        assertEq(address(game).balance, INIT_BOND);
    }

    function testInitializeFailsIfNotTEEProposer() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "tee-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        vm.expectRevert(NotAuthorized.selector);
        _provideProof(game, ZK_PROVER, true, proof);
    }

    function testChallengeTEEProofWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        // Create first game with TEE proof
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = "tee-proof";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game1, TEE_PROVER, true, teeProof);
        
        // Create second game with different root claim and ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game2, ZK_PROVER, false, zkProof);

        // Get game index from factory
        uint256 gameIndex = factory.gameCount() - 1;
        
        // Challenge game1 with game2
        game1.challenge(gameIndex);

        assertEq(uint8(game1.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game1.bondRecipient(), ZK_PROVER);
        (address counteredBy,,,) = game1.provingData();
        assertEq(counteredBy, address(game2));

        // Retrieve bond after challenge
        vm.warp(block.timestamp + 7 days + 1);
        game2.resolve();
        assertEq(uint8(game2.status()), uint8(GameStatus.DEFENDER_WINS));
        assertEq(ZK_PROVER.balance, 0);
        assertEq(address(game1).balance, INIT_BOND);
        vm.prank(ZK_PROVER);
        game1.claimCredit();
        assertEq(ZK_PROVER.balance, INIT_BOND);
        assertEq(address(game1).balance, 0);
    }

    function testChallengeFailsIfNoTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        // Create first game with ZK proof (no TEE proof)
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof1 = "zk-proof-1";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game1, ZK_PROVER, false, zkProof1);
        
        // Create second game with different root claim and ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = "zk-proof-2";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game2, ZK_PROVER, false, zkProof2);
        
        uint256 gameIndex = factory.gameCount() - 1;
        
        vm.expectRevert(MissingTEEProof.selector);
        game1.challenge(gameIndex);
    }

    function testCannotCreateSameProposal() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max);
        
        Hash uuid = factory.getGameUUID(AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, abi.encodePacked(currentL2BlockNumber, type(uint32).max));
        vm.expectRevert(abi.encodeWithSelector(GameAlreadyExists.selector, uuid));
        _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );
    }

    function testChallengeFailsIfDifferentParentIndex() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = "tee-proof";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );
        
        _provideProof(game1, TEE_PROVER, true, teeProof);

        // Create game2 with game1 as parent
        uint256 game1Index = factory.gameCount() - 1;
        uint256 nextBlockNumber = currentL2BlockNumber + BLOCK_INTERVAL;
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(nextBlockNumber, "zk")));
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim2,
            nextBlockNumber,
            uint32(game1Index)
        );

        _provideProof(game2, ZK_PROVER, false, zkProof);
        uint256 gameIndex = factory.gameCount() - 1;
        
        vm.expectRevert(IncorrectParentIndex.selector);
        game1.challenge(gameIndex);
    }

    function testChallengeFailsIfChallengingGameHasNoZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game1, TEE_PROVER, true, teeProof1);
        
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = "tee-proof-2";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game2, TEE_PROVER, true, teeProof2);
        
        uint256 gameIndex = factory.gameCount() - 1;
        
        vm.expectRevert(MissingZKProof.selector);
        game1.challenge(gameIndex);
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = "tee-proof";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game1, TEE_PROVER, true, teeProof);

        // Resolve game1
        vm.warp(block.timestamp + 7 days + 1);
        game1.resolve();
        
        // Try to challenge game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game2, ZK_PROVER, false, zkProof);
        
        uint256 challengeIndex1 = factory.gameCount() - 1;
        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.challenge(challengeIndex1);
    }

    function testNullifyWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game1, TEE_PROVER, true, teeProof1);
        
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = "tee-proof-2";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game2, TEE_PROVER, true, teeProof2);
        
        uint256 gameIndex = factory.gameCount() - 1;
        game1.nullify(gameIndex, AggregateVerifier.ProofType.TEE);
    
        assertEq(uint8(game1.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game1.bondRecipient(), TEE_PROVER);
    }

    function testNullifyWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof1 = "zk-proof-1";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game1, ZK_PROVER, false, zkProof1);
        
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = "zk-proof-2";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game2, ZK_PROVER, false, zkProof2);
        
        uint256 gameIndex = factory.gameCount() - 1;
        game1.nullify(gameIndex, AggregateVerifier.ProofType.ZK);

        assertEq(uint8(game1.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game1.bondRecipient(), ZK_PROVER);
    }

    function testNullifyFailsIfNoTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game1, ZK_PROVER, false, zkProof);
        
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof = "tee-proof";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game2, TEE_PROVER, true, teeProof);
        
        uint256 gameIndex = factory.gameCount() - 1;
        vm.expectRevert(MissingTEEProof.selector);
        game1.nullify(gameIndex, AggregateVerifier.ProofType.TEE);
    }

    function testNullifyFailsIfNoZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof = "tee-proof";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game1, TEE_PROVER, true, teeProof);
        
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game2, ZK_PROVER, false, zkProof);
        
        uint256 gameIndex = factory.gameCount() - 1;
        vm.expectRevert(MissingZKProof.selector);
        game1.nullify(gameIndex, AggregateVerifier.ProofType.ZK);
    }

    function testNullifyFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max);
        _provideProof(game1, TEE_PROVER, true, teeProof1);

        // Resolve game1
        vm.warp(block.timestamp + 7 days + 1);
        game1.resolve();
        
        // Try to nullify game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory teeProof2 = "tee-proof-2";
        AggregateVerifier game2 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game2, TEE_PROVER, true, teeProof2);
        
        uint256 challengeIndex = factory.gameCount() - 1;
        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.nullify(challengeIndex, AggregateVerifier.ProofType.TEE);
    }

    function testNullifyCanOverrideChallenge() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";
        
        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim1,
            currentL2BlockNumber,
            type(uint32).max
        );  
        _provideProof(game1, TEE_PROVER, true, teeProof1);
        
        // Challenge game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            type(uint32).max
        );
        _provideProof(game2, ZK_PROVER, false, zkProof);
        
        uint256 challengeIndex = factory.gameCount() - 1;
        game1.challenge(challengeIndex);
        assertEq(game1.bondRecipient(), ZK_PROVER);
        
        // Nullify can override challenge
        _provideProof(game1, ZK_PROVER, false, zkProof);
        game1.nullify(challengeIndex, AggregateVerifier.ProofType.ZK);
        
        assertEq(game1.bondRecipient(), TEE_PROVER);
    }

    // Helper function to create a game via factory
    function _createAggregateVerifierGame(
        address creator,
        Claim rootClaim,
        uint256 l2BlockNumber,
        uint32 parentIndex
    ) internal returns (AggregateVerifier game) {
        bytes memory extraData = abi.encodePacked(
            uint256(l2BlockNumber),
            uint32(parentIndex)
        );
        
        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        return AggregateVerifier(address(factory.create{value: INIT_BOND}(
            AGGREGATE_VERIFIER_GAME_TYPE,
            rootClaim,
            extraData
        )));
    }

    function _provideProof(AggregateVerifier game, address prover, bool isTeeProof, bytes memory proof) internal {
        vm.prank(prover);
        if (isTeeProof) {
            game.verifyTeeProof(proof);
        } else {
            game.verifyZkProof(proof);
        }
    }
}
