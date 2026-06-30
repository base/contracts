// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/libraries/bridge/Errors.sol";
import { GameStatus } from "src/libraries/bridge/Types.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract NullifyTest is BaseTest {
    function testNullifyWithTEEProof() public {
        _assertNullifyWithProof(TEE_PROVER, AggregateVerifier.ProofType.TEE);
    }

    function testNullifyWithZKProof() public {
        _assertNullifyWithProof(ZK_PROVER, AggregateVerifier.ProofType.ZK);
    }

    function testNullifyWithTEEProofWhenTEEAndZKProofsAreProvided() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _provideProof(game, ZK_PROVER, _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK));

        assertEq(game.expectedResolution().raw(), block.timestamp + game.FAST_FINALIZATION_DELAY());

        _nullify(game, "tee-proof-2", AggregateVerifier.ProofType.TEE, "tee2");

        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.bondRecipient(), TEE_PROVER);
        assertEq(game.proofCount(), 1);
        assertEq(game.expectedResolution().raw(), block.timestamp + game.SLOW_FINALIZATION_DELAY());
    }

    function testZKNullifyFailsIfNoZKProof() public {
        AggregateVerifier game =
            _createGame(TEE_PROVER, "tee1", "tee-proof", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry));

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.ZK));
        _nullify(game, "zk-proof", AggregateVerifier.ProofType.ZK, "tee2");
    }

    function testNullifyFailsIfGameAlreadyResolved() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        game.resolve();

        vm.expectRevert(ClaimAlreadyResolved.selector);
        _nullify(game, "tee-proof-2", AggregateVerifier.ProofType.TEE, "zk");
    }

    function testNullifyCanOverrideChallenge() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        vm.prank(ZK_PROVER);
        game.challenge(
            _generateProof("zk-proof", AggregateVerifier.ProofType.ZK), LAST_INTERMEDIATE_ROOT_INDEX, _claim("zk").raw()
        );

        _nullify(game, "zk-proof-2", AggregateVerifier.ProofType.ZK, "tee1");

        assertEq(game.bondRecipient(), TEE_PROVER);
        assertEq(game.expectedResolution().raw(), block.timestamp + game.SLOW_FINALIZATION_DELAY());

        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        game.resolve();
        _claimCreditAfterDelay(game, game.gameCreator());
    }

    function testResolveEarlyReturnWhenSharedTeeVerifierNullifiedByAnotherGame() public {
        _assertResolveEarlyReturnWhenSharedVerifierNullifiedByAnotherGame(AggregateVerifier.ProofType.TEE);
    }

    function testResolveEarlyReturnWhenSharedZkVerifierNullifiedByAnotherGame() public {
        _assertResolveEarlyReturnWhenSharedVerifierNullifiedByAnotherGame(AggregateVerifier.ProofType.ZK);
    }

    function testTwoProofsResolveDelayedAfterExternalVerifierNullify() public {
        AggregateVerifier gameA = _createGame(
            TEE_PROVER, "dual-a", "tee-dual-a", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _provideProof(gameA, ZK_PROVER, _generateProof("zk-dual-a", AggregateVerifier.ProofType.ZK));

        vm.warp(block.timestamp + gameA.FAST_FINALIZATION_DELAY());

        AggregateVerifier gameB =
            _createGame(ZK_PROVER, "dual-b", "zk-dual-b", AggregateVerifier.ProofType.ZK, address(gameA));

        _nullify(gameB, "zk-nullify-dual", AggregateVerifier.ProofType.ZK, "dual-nullify-b");

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.expectedResolution().raw(), block.timestamp + gameA.SLOW_FINALIZATION_DELAY());

        vm.warp(block.timestamp + gameA.SLOW_FINALIZATION_DELAY());
        assertEq(uint8(gameA.resolve()), uint8(GameStatus.DEFENDER_WINS));
    }

    function _assertNullifyWithProof(address prover, AggregateVerifier.ProofType proofType) private {
        AggregateVerifier game = _createGame(prover, "claim", "proof-1", proofType, address(anchorStateRegistry));

        _nullify(game, "proof-2", proofType, "nullify-claim");
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.bondRecipient(), prover);
        assertEq(game.proofCount(), 0);
        assertEq(game.expectedResolution().raw(), type(uint64).max);

        vm.warp(block.timestamp + 14 days);
        _claimCreditAfterDelay(game, game.gameCreator());
    }

    function _nullify(
        AggregateVerifier game,
        bytes memory proofSalt,
        AggregateVerifier.ProofType proofType,
        bytes memory claimSalt
    )
        private
    {
        game.nullify(
            _generateProposalProof(proofSalt, proofType), LAST_INTERMEDIATE_ROOT_INDEX, _claim(claimSalt).raw()
        );
    }

    function _assertResolveEarlyReturnWhenSharedVerifierNullifiedByAnotherGame(AggregateVerifier.ProofType proofType)
        private
    {
        address prover = proofType == AggregateVerifier.ProofType.TEE ? TEE_PROVER : ZK_PROVER;
        AggregateVerifier gameA = _createGame(prover, "game-a", "proof-a", proofType, address(anchorStateRegistry));
        AggregateVerifier gameB = _createGame(prover, "game-b", "proof-b", proofType, address(gameA));

        vm.warp(block.timestamp + gameA.SLOW_FINALIZATION_DELAY());

        _nullify(gameB, "nullify-proof", proofType, "nullify-claim");

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 0);
        assertEq(gameA.expectedResolution().raw(), type(uint64).max);

        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        gameA.resolve();
    }
}
