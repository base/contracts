// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {ClaimAlreadyResolved} from "optimism/src/dispute/lib/Errors.sol";
import {Claim, GameStatus} from "optimism/src/dispute/lib/Types.sol";

import {AggregateVerifier} from "src/AggregateVerifier.sol";

import {BaseTest} from "test/BaseTest.t.sol";

contract NullifyTest is BaseTest {
    function testNullifyWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1, AggregateVerifier.ProofType.TEE);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = "tee-proof-2";

        _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, teeProof2, AggregateVerifier.ProofType.TEE);

        uint256 gameIndex = factory.gameCount() - 1;
        game1.nullify(gameIndex, AggregateVerifier.ProofType.TEE);

        assertEq(uint8(game1.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game1.bondRecipient(), TEE_PROVER);

        uint256 balanceBefore = game1.gameCreator().balance;
        game1.claimCredit();
        assertEq(game1.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(address(game1).balance, 0);
    }

    function testNullifyWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof1 = "zk-proof-1";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, zkProof1, AggregateVerifier.ProofType.ZK);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = "zk-proof-2";

        _createAggregateVerifierGame(ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, zkProof2, AggregateVerifier.ProofType.ZK);

        uint256 gameIndex = factory.gameCount() - 1;
        game1.nullify(gameIndex, AggregateVerifier.ProofType.ZK);

        assertEq(uint8(game1.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game1.bondRecipient(), ZK_PROVER);

        uint256 balanceBefore = game1.gameCreator().balance;
        game1.claimCredit();
        assertEq(game1.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(address(game1).balance, 0);
    }

    function testTEENullifyFailsIfNoTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof = "zk-proof";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, zkProof, AggregateVerifier.ProofType.ZK);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof = "tee-proof";

        _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, teeProof, AggregateVerifier.ProofType.TEE);

        uint256 gameIndex = factory.gameCount() - 1;
        vm.expectRevert(AggregateVerifier.MissingTEEProof.selector);
        game1.nullify(gameIndex, AggregateVerifier.ProofType.TEE);
    }

    function testZKNullifyFailsIfNoZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof = "tee-proof";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof, AggregateVerifier.ProofType.TEE);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory zkProof = "zk-proof";

        _createAggregateVerifierGame(ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, zkProof, AggregateVerifier.ProofType.ZK);

        uint256 gameIndex = factory.gameCount() - 1;
        vm.expectRevert(AggregateVerifier.MissingZKProof.selector);
        game1.nullify(gameIndex, AggregateVerifier.ProofType.ZK);
    }

    function testNullifyFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1, AggregateVerifier.ProofType.TEE);

        // Resolve game1
        vm.warp(block.timestamp + 7 days);
        game1.resolve();

        // Try to nullify game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory teeProof2 = "tee-proof-2";

            _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, teeProof2, AggregateVerifier.ProofType.TEE);

        uint256 challengeIndex = factory.gameCount() - 1;
        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.nullify(challengeIndex, AggregateVerifier.ProofType.TEE);
    }

    function testNullifyCanOverrideChallenge() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = "tee-proof-1";

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1, AggregateVerifier.ProofType.TEE);

        // Challenge game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = "zk-proof";

            _createAggregateVerifierGame(ZK_PROVER, rootClaim2, currentL2BlockNumber, type(uint32).max, zkProof, AggregateVerifier.ProofType.ZK);

        uint256 challengeIndex = factory.gameCount() - 1;
        game1.challenge(challengeIndex);
        assertEq(game1.bondRecipient(), ZK_PROVER);

        // Nullify can override challenge
        _provideProof(game1, ZK_PROVER, AggregateVerifier.ProofType.ZK, zkProof);
        game1.nullify(challengeIndex, AggregateVerifier.ProofType.ZK);

        assertEq(game1.bondRecipient(), TEE_PROVER);

        uint256 balanceBefore = game1.gameCreator().balance;
        game1.claimCredit();
        assertEq(game1.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(address(game1).balance, 0);
    }
}
