// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/libraries/bridge/Errors.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { GameStatus } from "src/libraries/bridge/Types.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { Verifier } from "src/L1/proofs/Verifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract ChallengeTest is BaseTest {
    function testChallengeTEEProofWithZKProof() public {
        AggregateVerifier game = _rootTee(TEE_PROVER, "tee", "tee-proof");

        _challengeWithZk(game, "zk");
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.proofCount(), 2);

        _resolveSlow(game, GameStatus.CHALLENGER_WINS, ZK_PROVER);
    }

    function testChallengeFailsIfNoTEEProof() public {
        AggregateVerifier game =
            _createGame(ZK_PROVER, "zk1", "zk-proof-1", AggregateVerifier.ProofType.ZK, address(anchorStateRegistry));

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        _challengeWithEmptyZk(game);
    }

    function testChallengeFailsIfNotZKProof() public {
        AggregateVerifier game = _rootTee(TEE_PROVER, "tee1", "tee-proof-1");

        vm.expectRevert(AggregateVerifier.InvalidProofType.selector);
        _challenge(game, AggregateVerifier.ProofType.TEE, bytes32(0));
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        AggregateVerifier game = _rootTee(TEE_PROVER, "tee", "tee-proof");

        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY() + 1);
        game.resolve();

        vm.expectRevert(ClaimAlreadyResolved.selector);
        _challengeWithEmptyZk(game);
    }

    function testChallengeFailsIfParentGameStatusIsChallenged() public {
        AggregateVerifier parentGame = _rootTee(TEE_PROVER, "tee", "parent-proof");

        AggregateVerifier childGame =
            _createGame(TEE_PROVER, "tee2", "child-proof", AggregateVerifier.ProofType.TEE, address(parentGame));

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(parentGame)));

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        _challengeWithEmptyZk(childGame);
    }

    function testChallengeFailsIfGameItselfIsBlacklisted() public {
        AggregateVerifier game = _rootTee(TEE_PROVER, "tee", "tee-proof");

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(game)));

        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        _challengeWithEmptyZk(game);
    }

    function testChallengeFailsAfterTEENullification() public {
        AggregateVerifier game = _rootTee(TEE_PROVER, "tee1", "tee-proof-1");

        _nullify(game, AggregateVerifier.ProofType.TEE, "tee2");

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        _challengeWithEmptyZk(game);
    }

    function testChallengeFailsAfterZKNullification() public {
        AggregateVerifier game = _rootTee(ZK_PROVER, "tee", "tee-proof");

        _provideProof(game, ZK_PROVER, abi.encodePacked(uint8(AggregateVerifier.ProofType.ZK), bytes1(0)));
        _nullify(game, AggregateVerifier.ProofType.ZK, "zk2");
        vm.expectRevert(Verifier.Nullified.selector);
        _challenge(game, AggregateVerifier.ProofType.ZK, _claim("zk3").raw());
    }

    function testChallengeRemovedWhenZkVerifierNullifiedByOtherGame() public {
        AggregateVerifier gameA = _rootTee(TEE_PROVER, "tee-challenge", "tee-ch-a");

        _challengeWithZk(gameA, "zk-challenge");
        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        AggregateVerifier gameB =
            _createGame(ZK_PROVER, "zk-only-b", "zk-init-b", AggregateVerifier.ProofType.ZK, address(gameA));
        _nullify(gameB, AggregateVerifier.ProofType.ZK, "zk-nullify-b");
        assertTrue(zkVerifier.nullified());

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(gameA.zkProver(), address(0));

        _resolveSlow(gameA, GameStatus.DEFENDER_WINS, TEE_PROVER);
    }

    function testChallengeWinsWhenSharedTeeVerifierNullifiedByOtherGame() public {
        AggregateVerifier gameA = _rootTee(TEE_PROVER, "tee-challenge-tee-null", "tee-proof-a");

        _challengeWithZk(gameA, "zk-challenge");
        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        AggregateVerifier gameB =
            _createGame(TEE_PROVER, "tee-nullify-b", "tee-proof-b", AggregateVerifier.ProofType.TEE, address(gameA));

        _nullify(gameB, AggregateVerifier.ProofType.TEE, "tee-nullify-b-root");
        assertTrue(teeVerifier.nullified());

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(gameA.teeProver(), address(0));
        assertEq(gameA.zkProver(), ZK_PROVER);

        _resolveSlow(gameA, GameStatus.CHALLENGER_WINS, ZK_PROVER);
    }

    function _rootTee(
        address prover,
        bytes memory claimSalt,
        bytes memory proofSalt
    )
        private
        returns (AggregateVerifier)
    {
        return _createGame(prover, claimSalt, proofSalt, AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));
    }

    function _challenge(AggregateVerifier game, AggregateVerifier.ProofType proofType, bytes32 claimRoot) private {
        game.challenge(abi.encodePacked(uint8(proofType), bytes1(0)), LAST_INTERMEDIATE_ROOT_INDEX, claimRoot);
    }

    function _challengeWithEmptyZk(AggregateVerifier game) private {
        _challenge(game, AggregateVerifier.ProofType.ZK, bytes32(0));
    }

    function _challengeWithZk(AggregateVerifier game, bytes memory claimSalt) private {
        vm.prank(ZK_PROVER);
        _challenge(game, AggregateVerifier.ProofType.ZK, _claim(claimSalt).raw());
    }

    function _nullify(AggregateVerifier game, AggregateVerifier.ProofType proofType, bytes memory claimSalt) private {
        game.nullify(
            _generateProposalProof(claimSalt, proofType), LAST_INTERMEDIATE_ROOT_INDEX, _claim(claimSalt).raw()
        );
    }

    function _resolveSlow(AggregateVerifier game, GameStatus expectedStatus, address expectedBondRecipient) private {
        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        assertEq(uint8(game.resolve()), uint8(expectedStatus));
        assertEq(game.bondRecipient(), expectedBondRecipient);
        _claimCreditAfterDelay(game, expectedBondRecipient);
    }
}
