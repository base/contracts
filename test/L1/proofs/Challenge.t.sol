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
        _assertStatus(game, GameStatus.IN_PROGRESS);
        // 2 proofs so that it can decrease to 1 if ZK is nullified and then the TEE proof can resolve
        assertEq(game.proofCount(), 2);

        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        _assertResolveStatus(game, GameStatus.CHALLENGER_WINS);
        assertEq(game.bondRecipient(), ZK_PROVER);

        _claimCreditAfterDelay(game, ZK_PROVER);
    }

    function testChallengeFailsIfNoTEEProof() public {
        AggregateVerifier game =
            _createGame(ZK_PROVER, "zk1", "zk-proof-1", AggregateVerifier.ProofType.ZK, address(anchorStateRegistry));

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        game.challenge(_proofOfType(AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, bytes32(0));
    }

    function testChallengeFailsIfNotZKProof() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        vm.expectRevert(AggregateVerifier.InvalidProofType.selector);
        game.challenge(_proofOfType(AggregateVerifier.ProofType.TEE), LAST_INTERMEDIATE_ROOT_INDEX, bytes32(0));
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        AggregateVerifier game =
            _createGame(TEE_PROVER, "tee", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY() + 1);
        game.resolve();

        vm.expectRevert(ClaimAlreadyResolved.selector);
        game.challenge(_proofOfType(AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, bytes32(0));
    }

    function testChallengeFailsIfParentGameStatusIsChallenged() public {
        AggregateVerifier parentGame = _createGame(
            TEE_PROVER, "tee", "parent-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        AggregateVerifier childGame =
            _createGame(TEE_PROVER, "tee2", "child-proof", AggregateVerifier.ProofType.TEE, address(parentGame));

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(parentGame)));

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        childGame.challenge(_proofOfType(AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, bytes32(0));
    }

    function testChallengeFailsIfGameItselfIsBlacklisted() public {
        AggregateVerifier game =
            _createGame(TEE_PROVER, "tee", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        anchorStateRegistry.blacklistDisputeGame(IDisputeGame(address(game)));

        vm.expectRevert(AggregateVerifier.InvalidGame.selector);
        game.challenge(_proofOfType(AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, bytes32(0));
    }

    function testChallengeFailsAfterTEENullification() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _nullify(game, AggregateVerifier.ProofType.TEE, "tee2");
        // TEE was nullified, so a ZK challenge cannot find the required TEE proof.
        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.TEE)
        );
        game.challenge(_proofOfType(AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, bytes32(0));
    }

    function testChallengeFailsAfterZKNullification() public {
        AggregateVerifier game =
            _createGame(ZK_PROVER, "tee", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        _provideProof(game, ZK_PROVER, _proofOfType(AggregateVerifier.ProofType.ZK));
        _nullify(game, AggregateVerifier.ProofType.ZK, "zk2");
        // ZK was nullified globally, so another ZK challenge proof is rejected by the verifier.
        vm.expectRevert(Verifier.Nullified.selector);
        game.challenge(_proofOfType(AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, _claim("zk3").raw());
    }

    function testChallengeRemovedWhenZkVerifierNullifiedByOtherGame() public {
        AggregateVerifier gameA = _createGame(
            TEE_PROVER, "tee-challenge", "tee-ch-a", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _challengeWithZk(gameA, "zk-challenge");
        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        AggregateVerifier gameB =
            _createGame(ZK_PROVER, "zk-only-b", "zk-init-b", AggregateVerifier.ProofType.ZK, address(gameA));
        _nullify(gameB, AggregateVerifier.ProofType.ZK, "zk-nullify-b");
        assertTrue(zkVerifier.nullified());

        _assertResolveStatus(gameA, GameStatus.IN_PROGRESS);
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.zkProver()), address(0));

        vm.warp(block.timestamp + gameA.SLOW_FINALIZATION_DELAY());
        _assertResolveStatus(gameA, GameStatus.DEFENDER_WINS);
        assertEq(gameA.bondRecipient(), TEE_PROVER);

        _claimCreditAfterDelay(gameA, TEE_PROVER);
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
        assertEq(gameA.proofCount(), 2);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);

        AggregateVerifier gameB =
            _createGame(TEE_PROVER, "tee-only-b", "tee-init-b", AggregateVerifier.ProofType.TEE, address(gameA));
        _nullify(gameB, AggregateVerifier.ProofType.TEE, "tee-nullify-b");
        assertTrue(teeVerifier.nullified());

        _assertResolveStatus(gameA, GameStatus.IN_PROGRESS);
        assertEq(gameA.proofCount(), 1);
        assertGt(gameA.counteredByIntermediateRootIndexPlusOne(), 0);
        assertEq(address(gameA.teeProver()), address(0));
        assertEq(gameA.zkProver(), ZK_PROVER);

        vm.warp(block.timestamp + gameA.SLOW_FINALIZATION_DELAY());
        _assertResolveStatus(gameA, GameStatus.CHALLENGER_WINS);
        assertEq(gameA.bondRecipient(), ZK_PROVER);

        _claimCreditAfterDelay(gameA, ZK_PROVER);
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
        Claim rootClaim = _advanceL2BlockAndClaim(claimSalt);
        bytes memory proof = _generateProof(proofSalt, proofType);
        return _createAggregateVerifierGame(prover, rootClaim, currentL2BlockNumber, parent, proof);
    }

    function _challengeWithZk(AggregateVerifier game, bytes memory claimSalt) private {
        vm.prank(ZK_PROVER);
        game.challenge(
            _proofOfType(AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, _claim(claimSalt).raw()
        );
    }

    function _nullify(AggregateVerifier game, AggregateVerifier.ProofType proofType, bytes memory claimSalt) private {
        game.nullify(_proofOfType(proofType), LAST_INTERMEDIATE_ROOT_INDEX, _claim(claimSalt).raw());
    }

    function _proofOfType(AggregateVerifier.ProofType proofType) private pure returns (bytes memory) {
        return abi.encodePacked(uint8(proofType), bytes1(0));
    }

    function _advanceL2BlockAndClaim(bytes memory salt) private returns (Claim) {
        currentL2BlockNumber += BLOCK_INTERVAL;
        return _claim(salt);
    }

    function _claim(bytes memory salt) private view returns (Claim) {
        return Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, salt)));
    }

    function _assertStatus(AggregateVerifier game, GameStatus expectedStatus) private view {
        assertEq(uint8(game.status()), uint8(expectedStatus));
    }

    function _assertResolveStatus(AggregateVerifier game, GameStatus expectedStatus) private {
        assertEq(uint8(game.resolve()), uint8(expectedStatus));
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
