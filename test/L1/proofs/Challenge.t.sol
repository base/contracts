// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/libraries/bridge/Errors.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { GameStatus, Hash } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { Verifier } from "src/L1/proofs/Verifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract ChallengeTest is BaseTest {
    function testChallengeTEEProofWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Create game with TEE proof
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        // Challenge game with ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.prank(ZK_PROVER);
        game.challenge(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        // 2 proofs so that it can decrease to 1 if ZK is nullified and then the TEE proof can resolve
        assertEq(game.proofCount(), 2);

        // Resolve after SLOW_FINALIZATION_DELAY
        vm.warp(block.timestamp + 7 days);
        game.resolve();

        assertEq(uint8(game.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game.bondRecipient(), ZK_PROVER);

        uint256 balanceBefore = ZK_PROVER.balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(ZK_PROVER.balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }

    function testChallengeFailsIfNoTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Create first game with ZK proof (no TEE proof)
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof1 = _generateProof("zk-proof-1", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), zkProof1
        );

        // Challenge game with ZK proof
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        game1.challenge(zkProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim1.raw());
    }

    function testChallengeFailsIfNotZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(AggregateVerifier.InvalidProofType.selector);
        game1.challenge(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        // Resolve game1
        vm.warp(block.timestamp + 7 days + 1);
        game1.resolve();

        // Try to challenge game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk1")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.challenge(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testChallengeFailsIfParentGameStatusIsChallenged() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // create parent game
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory parentProof = _generateProof("parent-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier parentGame = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), parentProof
        );

        currentL2BlockNumber += BLOCK_INTERVAL;

        // create child game
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory childProof = _generateProof("child-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier childGame =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, address(parentGame), childProof);

        // blacklist parent game
        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(parentGame)));

        // challenge child game with ZK proof
        Claim rootClaim3 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        childGame.challenge(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim3.raw());
    }

    function testChallengeFailsIfGameItselfIsBlacklisted() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        // blacklist game
        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(game)));

        // challenge game
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        game.challenge(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testChallengeFailsAfterTEENullification() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        game.nullify(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        // challenge game — TEE proof was nullified, so MissingProof(TEE) is expected
        Claim rootClaim3 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        game.challenge(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim3.raw());
    }

    function testChallengeFailsAfterZKNullification() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof1 = _generateProof("zk-proof-1", AggregateVerifier.ProofType.ZK);

        // create game with both proofs
        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );
        game.verifyProposalProof(zkProof1);

        // nullify ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);
        game.nullify(zkProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        // challenge game — ZK is nullified so Nullified() is expected
        Claim rootClaim3 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk3")));
        bytes memory zkProof3 = _generateProof("zk-proof-3", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(Verifier.Nullified.selector);
        game.challenge(zkProof3, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim3.raw());
    }

    /// @notice A TEE+ZK challenge on game A is cleared when another game nullifies the shared ZK verifier; A then
    ///         resolves as defender after `SLOW_FINALIZATION_DELAY`.
    function testChallengeRemovedWhenZkVerifierNullifiedByOtherGame() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaimA = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee-challenge")));
        bytes memory teeProofA = _generateProof("tee-ch-a", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameA = _createAggregateVerifierGame(
            TEE_PROVER, rootClaimA, currentL2BlockNumber, address(anchorStateRegistry), teeProofA
        );

        Claim rootChallenge = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk-challenge")));
        bytes memory zkChallenge = _generateProof("zk-challenge", AggregateVerifier.ProofType.ZK);
        vm.prank(ZK_PROVER);
        gameA.challenge(zkChallenge, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootChallenge.raw());

        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaimB = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk-only-b")));
        bytes memory zkProofB = _generateProof("zk-init-b", AggregateVerifier.ProofType.ZK);
        AggregateVerifier gameB =
            _createAggregateVerifierGame(ZK_PROVER, rootClaimB, currentL2BlockNumber, address(gameA), zkProofB);

        Claim rootNullifyB = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk-nullify-b")));
        bytes memory zkNullifyB = _generateProof("zk-nullify-b", AggregateVerifier.ProofType.ZK);
        uint256 lastIdx = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1;
        gameB.nullify(zkNullifyB, lastIdx, rootNullifyB.raw());
        assertTrue(zkVerifier.nullified());

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.zkProver()), address(0));

        vm.warp(block.timestamp + 7 days);
        assertEq(uint8(gameA.resolve()), uint8(GameStatus.DEFENDER_WINS));
        assertEq(gameA.bondRecipient(), TEE_PROVER);

        uint256 balanceBefore = TEE_PROVER.balance;
        gameA.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        gameA.claimCredit();
        assertEq(TEE_PROVER.balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(gameA)), 0);
    }

    /// @notice Game A is created with TEE and challenged with ZK. Another game nullifies the shared TEE verifier.
    ///         The first `resolve` persists the TEE refutation; after `SLOW_FINALIZATION_DELAY`, A finalizes as
    ///         challenger wins and the bond goes to the ZK challenger.
    function testChallengeWinsWhenSharedTeeVerifierNullifiedByOtherGame() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaimA = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee-challenge-tee-null")));
        bytes memory teeProofA = _generateProof("tee-proof-a", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameA = _createAggregateVerifierGame(
            TEE_PROVER, rootClaimA, currentL2BlockNumber, address(anchorStateRegistry), teeProofA
        );

        Claim rootChallenge = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk-challenge")));
        bytes memory zkChallenge = _generateProof("zk-challenge", AggregateVerifier.ProofType.ZK);
        vm.prank(ZK_PROVER);
        gameA.challenge(zkChallenge, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootChallenge.raw());

        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaimB = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee-only-b")));
        bytes memory teeProofB = _generateProof("tee-init-b", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameB =
            _createAggregateVerifierGame(TEE_PROVER, rootClaimB, currentL2BlockNumber, address(gameA), teeProofB);

        Claim rootNullifyB = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee-nullify-b")));
        bytes memory teeNullifyB = _generateProof("tee-nullify-b", AggregateVerifier.ProofType.TEE);
        uint256 lastIdx = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1;
        gameB.nullify(teeNullifyB, lastIdx, rootNullifyB.raw());
        assertTrue(teeVerifier.nullified());

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.teeProver()), address(0));
        assertEq(gameA.zkProver(), ZK_PROVER);

        vm.warp(block.timestamp + 7 days);
        assertEq(uint8(gameA.resolve()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(gameA.bondRecipient(), ZK_PROVER);

        uint256 balanceBefore = ZK_PROVER.balance;
        gameA.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        gameA.claimCredit();
        assertEq(ZK_PROVER.balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(gameA)), 0);
    }
}
