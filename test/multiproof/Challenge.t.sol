// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {ClaimAlreadyResolved} from "optimism/src/dispute/lib/Errors.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {Claim, GameStatus, Hash} from "optimism/src/dispute/lib/Types.sol";

import {AggregateVerifier} from "src/AggregateVerifier.sol";

import {BaseTest} from "test/BaseTest.t.sol";

contract ChallengeTest is BaseTest {
    function testChallengeTEEProofWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Create first game with TEE proof
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof
        );

        // Create second game with different root claim and ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game2 = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, zkProof
        );

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
        assertEq(delayedWETH.balanceOf(address(game1)), INIT_BOND);
        game1.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game1.claimCredit();
        assertEq(ZK_PROVER.balance, INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game1)), 0);
    }

    function testChallengeFailsIfNoTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Create first game with ZK proof (no TEE proof)
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof1 = _generateProof("zk-proof-1", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, zkProof1
        );

        // Create second game with different root claim and ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);

        _createAggregateVerifierGame(
            ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, zkProof2
        );

        uint256 gameIndex = factory.gameCount() - 1;

        vm.expectRevert(AggregateVerifier.MissingTEEProof.selector);
        game1.challenge(gameIndex);
    }

    function testChallengeFailsIfDifferentParentIndex() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof
        );

        // Create game2 with game1 as parent
        uint256 game1Index = factory.gameCount() - 1;
        uint256 nextBlockNumber = currentL2BlockNumber + BLOCK_INTERVAL;
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(nextBlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        // forge-lint: disable-next-line(unsafe-typecast)
        _createAggregateVerifierGame(
            ZK_PROVER, rootClaim2, nextBlockNumber, uint32(game1Index), zkProof
        );

        uint256 gameIndex = factory.gameCount() - 1;

        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        game1.challenge(gameIndex);
    }

    function testChallengeFailsIfChallengingGameHasNoZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1
        );

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        _createAggregateVerifierGame(
            TEE_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, teeProof2
        );

        uint256 gameIndex = factory.gameCount() - 1;

        vm.expectRevert(AggregateVerifier.MissingZKProof.selector);
        game1.challenge(gameIndex);
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof
        );

        // Resolve game1
        vm.warp(block.timestamp + 7 days + 1);
        game1.resolve();

        // Try to challenge game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        _createAggregateVerifierGame(
            ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, zkProof
        );

        uint256 challengeIndex1 = factory.gameCount() - 1;
        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.challenge(challengeIndex1);
    }

    function testChallengeFailsIfParentGameStatusIsChallenged() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // create parent game
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory parentProof = _generateProof("parent-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier parentGame = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, parentProof
        );

        uint256 parentGameIndex = factory.gameCount() - 1;
        currentL2BlockNumber += BLOCK_INTERVAL;

        // create child game
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory childProof = _generateProof("child-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier childGame =
        // forge-lint: disable-next-line(unsafe-typecast)
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim2,
            currentL2BlockNumber,
            uint32(parentGameIndex),
            childProof
        );

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
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof
        );

        // blacklist game
        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(game)));

        // challenge game
        uint256 gameIndex = factory.gameCount() - 1;
        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        game.challenge(gameIndex);
    }
}
