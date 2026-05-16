// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/libraries/bridge/Errors.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { GameStatus } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { Verifier } from "src/L1/proofs/Verifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract ChallengeTest is BaseTest {
    uint256 private constant LAST_INTERMEDIATE_ROOT_INDEX = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1;

    function testChallengeTEEProofWithZKProof() public {
        Claim rootClaim1 = _advanceL2BlockAndClaim("tee");
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        Claim rootClaim2 = _claim("zk");
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.prank(ZK_PROVER);
        game.challenge(zkProof, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim2.raw());

        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        // 2 proofs so that it can decrease to 1 if ZK is nullified and then the TEE proof can resolve
        assertEq(game.proofCount(), 2);

        vm.warp(block.timestamp + 7 days);
        game.resolve();

        assertEq(uint8(game.status()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(game.bondRecipient(), ZK_PROVER);

        _claimCreditAfterDelay(game, ZK_PROVER);
    }

    function testChallengeFailsIfNoTEEProof() public {
        Claim rootClaim1 = _advanceL2BlockAndClaim("zk1");
        bytes memory zkProof1 = _generateProof("zk-proof-1", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), zkProof1
        );

        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        game1.challenge(zkProof2, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim1.raw());
    }

    function testChallengeFailsIfNotZKProof() public {
        Claim rootClaim1 = _advanceL2BlockAndClaim("tee1");
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        Claim rootClaim2 = _claim("tee2");
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(AggregateVerifier.InvalidProofType.selector);
        game1.challenge(teeProof2, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim2.raw());
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        Claim rootClaim1 = _advanceL2BlockAndClaim("tee");
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        vm.warp(block.timestamp + 7 days + 1);
        game1.resolve();

        Claim rootClaim2 = _claim("zk1");
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.challenge(zkProof, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim2.raw());
    }

    function testChallengeFailsIfParentGameStatusIsChallenged() public {
        Claim rootClaim1 = _advanceL2BlockAndClaim("tee");
        bytes memory parentProof = _generateProof("parent-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier parentGame = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), parentProof
        );

        Claim rootClaim2 = _advanceL2BlockAndClaim("tee2");
        bytes memory childProof = _generateProof("child-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier childGame =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, address(parentGame), childProof);

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(parentGame)));

        Claim rootClaim3 = _claim("zk");
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        childGame.challenge(zkProof, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim3.raw());
    }

    function testChallengeFailsIfGameItselfIsBlacklisted() public {
        Claim rootClaim = _advanceL2BlockAndClaim("tee");
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(game)));

        Claim rootClaim2 = _claim("zk");
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        game.challenge(zkProof, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim2.raw());
    }

    function testChallengeFailsAfterTEENullification() public {
        Claim rootClaim1 = _advanceL2BlockAndClaim("tee1");
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        Claim rootClaim2 = _claim("tee2");
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        game.nullify(teeProof2, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim2.raw());

        // TEE was nullified, so a ZK challenge cannot find the required TEE proof.
        Claim rootClaim3 = _claim("zk");
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        game.challenge(zkProof, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim3.raw());
    }

    function testChallengeFailsAfterZKNullification() public {
        Claim rootClaim1 = _advanceL2BlockAndClaim("tee");
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof1 = _generateProof("zk-proof-1", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );
        game.verifyProposalProof(zkProof1);

        Claim rootClaim2 = _claim("zk2");
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);
        game.nullify(zkProof2, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim2.raw());

        // ZK was nullified globally, so another ZK challenge proof is rejected by the verifier.
        Claim rootClaim3 = _claim("zk3");
        bytes memory zkProof3 = _generateProof("zk-proof-3", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(Verifier.Nullified.selector);
        game.challenge(zkProof3, LAST_INTERMEDIATE_ROOT_INDEX, rootClaim3.raw());
    }

    /// @notice A TEE+ZK challenge on game A is cleared when another game nullifies the shared ZK verifier; A then
    ///         resolves as defender after `SLOW_FINALIZATION_DELAY`.
    function testChallengeRemovedWhenZkVerifierNullifiedByOtherGame() public {
        Claim rootClaimA = _advanceL2BlockAndClaim("tee-challenge");
        bytes memory teeProofA = _generateProof("tee-ch-a", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameA = _createAggregateVerifierGame(
            TEE_PROVER, rootClaimA, currentL2BlockNumber, address(anchorStateRegistry), teeProofA
        );

        Claim rootChallenge = _claim("zk-challenge");
        bytes memory zkChallenge = _generateProof("zk-challenge", AggregateVerifier.ProofType.ZK);
        vm.prank(ZK_PROVER);
        gameA.challenge(zkChallenge, LAST_INTERMEDIATE_ROOT_INDEX, rootChallenge.raw());

        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        Claim rootClaimB = _advanceL2BlockAndClaim("zk-only-b");
        bytes memory zkProofB = _generateProof("zk-init-b", AggregateVerifier.ProofType.ZK);
        AggregateVerifier gameB =
            _createAggregateVerifierGame(ZK_PROVER, rootClaimB, currentL2BlockNumber, address(gameA), zkProofB);

        Claim rootNullifyB = _claim("zk-nullify-b");
        bytes memory zkNullifyB = _generateProof("zk-nullify-b", AggregateVerifier.ProofType.ZK);
        gameB.nullify(zkNullifyB, LAST_INTERMEDIATE_ROOT_INDEX, rootNullifyB.raw());
        assertTrue(zkVerifier.nullified());

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.zkProver()), address(0));

        vm.warp(block.timestamp + 7 days);
        assertEq(uint8(gameA.resolve()), uint8(GameStatus.DEFENDER_WINS));
        assertEq(gameA.bondRecipient(), TEE_PROVER);

        _claimCreditAfterDelay(gameA, TEE_PROVER);
    }

    /// @notice Game A is created with TEE and challenged with ZK. Another game nullifies the shared TEE verifier.
    ///         The first `resolve` persists the TEE refutation; after `SLOW_FINALIZATION_DELAY`, A finalizes as
    ///         challenger wins and the bond goes to the ZK challenger.
    function testChallengeWinsWhenSharedTeeVerifierNullifiedByOtherGame() public {
        Claim rootClaimA = _advanceL2BlockAndClaim("tee-challenge-tee-null");
        bytes memory teeProofA = _generateProof("tee-proof-a", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameA = _createAggregateVerifierGame(
            TEE_PROVER, rootClaimA, currentL2BlockNumber, address(anchorStateRegistry), teeProofA
        );

        Claim rootChallenge = _claim("zk-challenge");
        bytes memory zkChallenge = _generateProof("zk-challenge", AggregateVerifier.ProofType.ZK);
        vm.prank(ZK_PROVER);
        gameA.challenge(zkChallenge, LAST_INTERMEDIATE_ROOT_INDEX, rootChallenge.raw());

        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        Claim rootClaimB = _advanceL2BlockAndClaim("tee-only-b");
        bytes memory teeProofB = _generateProof("tee-init-b", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameB =
            _createAggregateVerifierGame(TEE_PROVER, rootClaimB, currentL2BlockNumber, address(gameA), teeProofB);

        Claim rootNullifyB = _claim("tee-nullify-b");
        bytes memory teeNullifyB = _generateProof("tee-nullify-b", AggregateVerifier.ProofType.TEE);
        gameB.nullify(teeNullifyB, LAST_INTERMEDIATE_ROOT_INDEX, rootNullifyB.raw());
        assertTrue(teeVerifier.nullified());

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.teeProver()), address(0));
        assertEq(gameA.zkProver(), ZK_PROVER);

        vm.warp(block.timestamp + 7 days);
        assertEq(uint8(gameA.resolve()), uint8(GameStatus.CHALLENGER_WINS));
        assertEq(gameA.bondRecipient(), ZK_PROVER);

        _claimCreditAfterDelay(gameA, ZK_PROVER);
    }

    function _advanceL2BlockAndClaim(bytes memory salt) private returns (Claim) {
        currentL2BlockNumber += BLOCK_INTERVAL;
        return _claim(salt);
    }

    function _claim(bytes memory salt) private view returns (Claim) {
        return Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, salt)));
    }

    function _claimCreditAfterDelay(AggregateVerifier game, address recipient) private {
        uint256 balanceBefore = recipient.balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(recipient.balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }
}
