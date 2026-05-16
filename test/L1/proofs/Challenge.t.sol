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
        AggregateVerifier game =
            _createGame(TEE_PROVER, "tee", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        _challengeWithZk(game, "zk");
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.proofCount(), 2);

        _resolveAfterSlowDelayAndClaim(game, GameStatus.CHALLENGER_WINS, ZK_PROVER);
    }

    function testChallengeFailsIfNoTEEProof() public {
        AggregateVerifier game =
            _createGame(ZK_PROVER, "zk1", "zk-proof-1", AggregateVerifier.ProofType.ZK, address(anchorStateRegistry));

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        _challenge(game, AggregateVerifier.ProofType.ZK, bytes32(0));
    }

    function testChallengeFailsIfNotZKProof() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        vm.expectRevert(AggregateVerifier.InvalidProofType.selector);
        _challenge(game, AggregateVerifier.ProofType.TEE, bytes32(0));
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        AggregateVerifier game =
            _createGame(TEE_PROVER, "tee", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY() + 1);
        game.resolve();

        vm.expectRevert(ClaimAlreadyResolved.selector);
        _challenge(game, AggregateVerifier.ProofType.ZK, bytes32(0));
    }

    function testChallengeFailsIfParentGameStatusIsChallenged() public {
        AggregateVerifier parentGame = _createGame(
            TEE_PROVER, "tee", "parent-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        AggregateVerifier childGame =
            _createGame(TEE_PROVER, "tee2", "child-proof", AggregateVerifier.ProofType.TEE, address(parentGame));

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(parentGame)));

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        _challenge(childGame, AggregateVerifier.ProofType.ZK, bytes32(0));
    }

    function testChallengeFailsIfGameItselfIsBlacklisted() public {
        AggregateVerifier game =
            _createGame(TEE_PROVER, "tee", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(game)));

        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        _challenge(game, AggregateVerifier.ProofType.ZK, bytes32(0));
    }

    function testChallengeFailsAfterTEENullification() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _nullify(game, AggregateVerifier.ProofType.TEE, "tee2");
        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        _challenge(game, AggregateVerifier.ProofType.ZK, bytes32(0));
    }

    function testChallengeFailsAfterZKNullification() public {
        AggregateVerifier game =
            _createGame(ZK_PROVER, "tee", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        _provideProof(game, ZK_PROVER, _proofOfType(AggregateVerifier.ProofType.ZK));
        _nullify(game, AggregateVerifier.ProofType.ZK, "zk2");
        vm.expectRevert(Verifier.Nullified.selector);
        _challenge(game, AggregateVerifier.ProofType.ZK, _claim("zk3").raw());
    }

    function testChallengeRemovedWhenZkVerifierNullifiedByOtherGame() public {
        AggregateVerifier gameA = _createGame(
            TEE_PROVER, "tee-challenge", "tee-ch-a", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _challengeWithZk(gameA, "zk-challenge");
        _assertChallengeRecorded(gameA);

        AggregateVerifier gameB =
            _createGame(ZK_PROVER, "zk-only-b", "zk-init-b", AggregateVerifier.ProofType.ZK, address(gameA));
        _nullify(gameB, AggregateVerifier.ProofType.ZK, "zk-nullify-b");
        assertTrue(zkVerifier.nullified());

        _resolveAndAssertStatus(gameA, GameStatus.IN_PROGRESS);
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.zkProver()), address(0));

        _resolveAfterSlowDelayAndClaim(gameA, GameStatus.DEFENDER_WINS, TEE_PROVER);
    }

    function testChallengeWinsWhenSharedTeeVerifierNullifiedByOtherGame() public {
        AggregateVerifier gameA = _createGame(
            TEE_PROVER,
            "tee-challenge-tee-null",
            "tee-proof-a",
            AggregateVerifier.ProofType.TEE,
            address(anchorStateRegistry)
        );

        _challengeWithZk(gameA, "zk-challenge");
        _assertChallengeRecorded(gameA);

        AggregateVerifier gameB =
            _createGame(TEE_PROVER, "tee-only-b", "tee-init-b", AggregateVerifier.ProofType.TEE, address(gameA));
        _nullify(gameB, AggregateVerifier.ProofType.TEE, "tee-nullify-b");
        assertTrue(teeVerifier.nullified());

        _resolveAndAssertStatus(gameA, GameStatus.IN_PROGRESS);
        assertEq(gameA.proofCount(), 1);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.teeProver()), address(0));
        assertEq(gameA.zkProver(), ZK_PROVER);

        _resolveAfterSlowDelayAndClaim(gameA, GameStatus.CHALLENGER_WINS, ZK_PROVER);
    }

    function _createGame(
        address prover,
        bytes memory claimSalt,
        bytes memory proofSalt,
        AggregateVerifier.ProofType proofType,
        address parent
    )
        private
        returns (AggregateVerifier)
    {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = _claim(claimSalt);
        bytes memory proof = _generateProof(proofSalt, proofType);
        return _createAggregateVerifierGame(prover, rootClaim, currentL2BlockNumber, parent, proof);
    }

    function _challenge(AggregateVerifier game, AggregateVerifier.ProofType proofType, bytes32 claimRoot) private {
        game.challenge(_proofOfType(proofType), LAST_INTERMEDIATE_ROOT_INDEX, claimRoot);
    }

    function _challengeWithZk(AggregateVerifier game, bytes memory claimSalt) private {
        vm.prank(ZK_PROVER);
        _challenge(game, AggregateVerifier.ProofType.ZK, _claim(claimSalt).raw());
    }

    function _nullify(AggregateVerifier game, AggregateVerifier.ProofType proofType, bytes memory claimSalt) private {
        game.nullify(_proofOfType(proofType), LAST_INTERMEDIATE_ROOT_INDEX, _claim(claimSalt).raw());
    }

    function _proofOfType(AggregateVerifier.ProofType proofType) private pure returns (bytes memory) {
        return abi.encodePacked(uint8(proofType), bytes1(0));
    }

    function _claim(bytes memory salt) private view returns (Claim) {
        return Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, salt)));
    }

    function _assertChallengeRecorded(AggregateVerifier game) private view {
        assertEq(game.proofCount(), 2);
        assertGt(game.counteredByIntermediateRootIndexPlusOne(), 0);
    }

    function _resolveAndAssertStatus(AggregateVerifier game, GameStatus expectedStatus) private {
        assertEq(uint8(game.resolve()), uint8(expectedStatus));
    }

    function _resolveAfterSlowDelayAndClaim(
        AggregateVerifier game,
        GameStatus expectedStatus,
        address recipient
    )
        private
    {
        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        _resolveAndAssertStatus(game, expectedStatus);
        assertEq(game.bondRecipient(), recipient);
        _claimCreditAfterDelay(game, recipient);
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
