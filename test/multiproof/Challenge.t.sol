// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {ClaimAlreadyResolved} from "optimism/src/dispute/lib/Errors.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "optimism/interfaces/dispute/IDisputeGameFactory.sol";
import {Claim, GameStatus, Hash} from "optimism/src/dispute/lib/Types.sol";

import {AggregateVerifier} from "src/AggregateVerifier.sol";

import {BaseTest} from "test/BaseTest.t.sol";

contract ChallengeTest is BaseTest {
    function testChallengeTEEProofWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Create first game with TEE proof
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = "tee-proof";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max);

        _provideProof(game1, TEE_PROVER, true, teeProof);

        // Create second game with different root claim and ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = "zk-proof";

        AggregateVerifier game2 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max);

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
        vm.warp(block.timestamp + 7 days);
        game2.resolve();
        assertEq(uint8(game2.status()), uint8(GameStatus.DEFENDER_WINS));
        assertEq(ZK_PROVER.balance, 0);
        assertEq(address(game1).balance, INIT_BOND);
        game1.claimCredit();
        assertEq(ZK_PROVER.balance, INIT_BOND);
        assertEq(address(game1).balance, 0);
    }

    function testChallengeFailsIfNoTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Create first game with ZK proof (no TEE proof)
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof1 = "zk-proof-1";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max);

        _provideProof(game1, ZK_PROVER, false, zkProof1);

        // Create second game with different root claim and ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = "zk-proof-2";

        AggregateVerifier game2 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max);

        _provideProof(game2, ZK_PROVER, false, zkProof2);

        uint256 gameIndex = factory.gameCount() - 1;

        vm.expectRevert(AggregateVerifier.MissingTEEProof.selector);
        game1.challenge(gameIndex);
    }

    function testCannotCreateSameProposal() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max);

        Hash uuid = factory.getGameUUID(
            AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, abi.encodePacked(currentL2BlockNumber, type(uint32).max)
        );
        vm.expectRevert(abi.encodeWithSelector(IDisputeGameFactory.GameAlreadyExists.selector, uuid));
        _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max);
    }

    function testChallengeFailsIfDifferentParentIndex() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = "tee-proof";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max);

        _provideProof(game1, TEE_PROVER, true, teeProof);

        // Create game2 with game1 as parent
        uint256 game1Index = factory.gameCount() - 1;
        uint256 nextBlockNumber = currentL2BlockNumber + BLOCK_INTERVAL;
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(nextBlockNumber, "zk")));
        bytes memory zkProof = "zk-proof";

        AggregateVerifier game2 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim2, nextBlockNumber, uint32(game1Index));

        _provideProof(game2, ZK_PROVER, false, zkProof);
        uint256 gameIndex = factory.gameCount() - 1;

        vm.expectRevert(AggregateVerifier.IncorrectParentIndex.selector);
        game1.challenge(gameIndex);
    }

    function testChallengeFailsIfChallengingGameHasNoZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max);

        _provideProof(game1, TEE_PROVER, true, teeProof1);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = "tee-proof-2";

        AggregateVerifier game2 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max);

        _provideProof(game2, TEE_PROVER, true, teeProof2);

        uint256 gameIndex = factory.gameCount() - 1;

        vm.expectRevert(AggregateVerifier.MissingZKProof.selector);
        game1.challenge(gameIndex);
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = "tee-proof";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max);

        _provideProof(game1, TEE_PROVER, true, teeProof);

        // Resolve game1
        vm.warp(block.timestamp + 7 days + 1);
        game1.resolve();

        // Try to challenge game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof = "zk-proof";

        AggregateVerifier game2 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max);

        _provideProof(game2, ZK_PROVER, false, zkProof);

        uint256 challengeIndex1 = factory.gameCount() - 1;
        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.challenge(challengeIndex1);
    }

    function testChallengeFailsIfParentGameStatusIsChallenged() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // create parent game
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory parentProof = "parent-proof";

        AggregateVerifier parentGame =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max);

        _provideProof(parentGame, TEE_PROVER, true, parentProof);

        uint256 parentGameIndex = factory.gameCount() - 1;
        currentL2BlockNumber += BLOCK_INTERVAL;

        // create child game
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory childProof = "child-proof";

        AggregateVerifier childGame =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, uint32(parentGameIndex));

        _provideProof(childGame, TEE_PROVER, true, childProof);

        // blacklist parent game
        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(parentGame)));

        // challenge child game
        uint256 childGameIndex = factory.gameCount() - 1;
        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        childGame.challenge(childGameIndex);
    }

    function testChallengeFailsIfGameItselfIsBlacklisted() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max);

        // blacklist game
        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(game)));

        // challenge game
        uint256 gameIndex = factory.gameCount() - 1;
        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        game.challenge(gameIndex);
    }
}
