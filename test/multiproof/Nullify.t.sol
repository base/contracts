// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {GameNotInProgress} from "optimism/src/dispute/lib/Errors.sol";
import {Claim, GameStatus} from "optimism/src/dispute/lib/Types.sol";

import {AggregateVerifier} from "src/AggregateVerifier.sol";

import {BaseTest} from "test/BaseTest.t.sol";

contract NullifyTest is BaseTest {
    function testNullifyWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        game.nullify(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        assertEq(uint8(game.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game.bondRecipient(), TEE_PROVER);

        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }

    function testNullifyWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof1 = _generateProof("zk-proof-1", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game1 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, zkProof1);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);

        game1.nullify(zkProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        assertEq(uint8(game1.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game1.bondRecipient(), ZK_PROVER);

        uint256 balanceBefore = game1.gameCreator().balance;
        game1.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game1.claimCredit();
        assertEq(game1.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game1)), 0);
    }

    function testTEENullifyFailsIfNoTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game1 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, zkProof);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(AggregateVerifier.MissingTEEProof.selector);
        game1.nullify(teeProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testZKNullifyFailsIfNoZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(AggregateVerifier.MissingZKProof.selector);
        game1.nullify(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testNullifyFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1);

        // Resolve game1
        vm.warp(block.timestamp + 7 days);
        game1.resolve();

        // Try to nullify game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(GameNotInProgress.selector);
        game1.nullify(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testNullifyCanOverrideChallenge() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1);

        // Challenge game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game2 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, zkProof);

        uint256 challengeIndex = factory.gameCount() - 1;
        game1.challenge(challengeIndex);
        assertEq(game1.bondRecipient(), ZK_PROVER);

        // Nullify can override challenge
        game2.nullify(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim1.raw());

        uint256 balanceBefore = game1.gameCreator().balance;
        game1.claimCredit();
        assertEq(game1.bondRecipient(), TEE_PROVER);
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game1.claimCredit();
        assertEq(game1.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game1)), 0);
    }
}
