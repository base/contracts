// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/libraries/bridge/Errors.sol";
import { GameStatus } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract NullifyTest is BaseTest {
    uint256 private constant LAST_INTERMEDIATE_ROOT_INDEX = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1;
    uint256 private constant NO_PROOF_RESOLUTION_DELAY = 14 days;

    function testNullifyWithTEEProof() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _nullify(game, "tee-proof-2", AggregateVerifier.ProofType.TEE, "tee2");
        _assertNullifiedToNoProofs(game, TEE_PROVER);

        vm.warp(block.timestamp + NO_PROOF_RESOLUTION_DELAY);
        _claimCreditAfterDelay(game);
    }

    function testNullifyWithZKProof() public {
        AggregateVerifier game =
            _createGame(ZK_PROVER, "zk1", "zk-proof-1", AggregateVerifier.ProofType.ZK, address(anchorStateRegistry));

        _nullify(game, "zk-proof-2", AggregateVerifier.ProofType.ZK, "zk2");
        _assertNullifiedToNoProofs(game, ZK_PROVER);

        vm.warp(block.timestamp + NO_PROOF_RESOLUTION_DELAY);
        _claimCreditAfterDelay(game);
    }

    function testNullifyWithTEEProofWhenTEEAndZKProofsAreProvided() public {
        AggregateVerifier game = _createGame(
            TEE_PROVER, "tee1", "tee-proof-1", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _provideProof(game, ZK_PROVER, _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK));

        assertEq(game.expectedResolution().raw(), block.timestamp + game.FAST_FINALIZATION_DELAY());

        _nullify(game, "tee-proof-2", AggregateVerifier.ProofType.TEE, "tee2");

        _assertStatus(game, GameStatus.IN_PROGRESS);
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
        _claimCreditAfterDelay(game);
    }

    /// @notice `resolve` runs `_updateProofCount`; when the shared TEE verifier was nullified by another game,
    ///         refutation persists and `resolve` returns early `IN_PROGRESS` (no `Resolved` event) instead of
    /// reverting. @dev All clones share the same `MockVerifier` TEE instance; `Verifier.nullify` requires a proper
    /// factory game.
    function testResolveEarlyReturnWhenSharedTeeVerifierNullifiedByAnotherGame() public {
        AggregateVerifier gameA = _createGame(
            TEE_PROVER, "game-a", "tee-proof-a", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        AggregateVerifier gameB =
            _createGame(TEE_PROVER, "game-b", "tee-proof-b", AggregateVerifier.ProofType.TEE, address(gameA));

        vm.warp(block.timestamp + gameA.SLOW_FINALIZATION_DELAY());
        assertTrue(gameA.gameOver());
        assertEq(gameA.proofCount(), 1);

        _nullify(gameB, "tee-nullify-b", AggregateVerifier.ProofType.TEE, "nullify-b");

        assertTrue(teeVerifier.nullified());
        assertEq(gameA.proofCount(), 1);

        _resolveAndAssertStatus(gameA, GameStatus.IN_PROGRESS);
        assertEq(gameA.proofCount(), 0);
        assertEq(gameA.expectedResolution().raw(), type(uint64).max);

        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        gameA.resolve();
    }

    function testResolveEarlyReturnWhenSharedZkVerifierNullifiedByAnotherGame() public {
        AggregateVerifier gameA = _createGame(
            ZK_PROVER, "zk-game-a", "zk-proof-a", AggregateVerifier.ProofType.ZK, address(anchorStateRegistry)
        );

        AggregateVerifier gameB =
            _createGame(ZK_PROVER, "zk-game-b", "zk-proof-b", AggregateVerifier.ProofType.ZK, address(gameA));

        vm.warp(block.timestamp + gameA.SLOW_FINALIZATION_DELAY());
        assertTrue(gameA.gameOver());
        assertEq(gameA.proofCount(), 1);

        _nullify(gameB, "zk-nullify-b", AggregateVerifier.ProofType.ZK, "zk-nullify-b");

        assertTrue(zkVerifier.nullified());
        assertEq(gameA.proofCount(), 1);

        _resolveAndAssertStatus(gameA, GameStatus.IN_PROGRESS);
        assertEq(gameA.proofCount(), 0);
        assertEq(gameA.expectedResolution().raw(), type(uint64).max);

        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        gameA.resolve();
    }

    /// @notice With TEE + ZK, the fast window is 1 day. Another game nullifies the shared ZK verifier; the first
    ///         `resolve` persists the ZK refutation and returns `IN_PROGRESS`. After `SLOW_FINALIZATION_DELAY`
    ///         (7 days) from that moment, a second `resolve` finalizes with only the TEE proof.
    function testTwoProofsResolveDelayedAfterExternalVerifierNullify() public {
        AggregateVerifier gameA = _createGame(
            TEE_PROVER, "dual-a", "tee-dual-a", AggregateVerifier.ProofType.TEE, address(anchorStateRegistry)
        );

        _provideProof(gameA, ZK_PROVER, _generateProof("zk-dual-a", AggregateVerifier.ProofType.ZK));

        assertEq(gameA.proofCount(), 2);
        assertEq(gameA.expectedResolution().raw(), block.timestamp + gameA.FAST_FINALIZATION_DELAY());

        vm.warp(block.timestamp + gameA.FAST_FINALIZATION_DELAY());
        assertTrue(gameA.gameOver());

        AggregateVerifier gameB =
            _createGame(ZK_PROVER, "dual-b", "zk-dual-b", AggregateVerifier.ProofType.ZK, address(gameA));

        _nullify(gameB, "zk-nullify-dual", AggregateVerifier.ProofType.ZK, "dual-nullify-b");
        assertTrue(zkVerifier.nullified());

        _resolveAndAssertStatus(gameA, GameStatus.IN_PROGRESS);
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.expectedResolution().raw(), block.timestamp + gameA.SLOW_FINALIZATION_DELAY());

        vm.warp(block.timestamp + gameA.SLOW_FINALIZATION_DELAY());
        _resolveAndAssertStatus(gameA, GameStatus.DEFENDER_WINS);
        _assertStatus(gameA, GameStatus.DEFENDER_WINS);
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
        return _createAggregateVerifierGame(
            prover, _claim(claimSalt), currentL2BlockNumber, parent, _generateProof(proofSalt, proofType)
        );
    }

    function _claim(bytes memory salt) private view returns (Claim) {
        return Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, salt)));
    }

    function _nullify(
        AggregateVerifier game,
        bytes memory proofSalt,
        AggregateVerifier.ProofType proofType,
        bytes memory claimSalt
    )
        private
    {
        game.nullify(_generateProof(proofSalt, proofType), LAST_INTERMEDIATE_ROOT_INDEX, _claim(claimSalt).raw());
    }

    function _assertNullifiedToNoProofs(AggregateVerifier game, address expectedBondRecipient) private view {
        _assertStatus(game, GameStatus.IN_PROGRESS);
        assertEq(game.bondRecipient(), expectedBondRecipient);
        assertEq(game.proofCount(), 0);
        assertEq(game.expectedResolution().raw(), type(uint64).max);
    }

    function _assertStatus(AggregateVerifier game, GameStatus expectedStatus) private view {
        assertEq(uint8(game.status()), uint8(expectedStatus));
    }

    function _resolveAndAssertStatus(AggregateVerifier game, GameStatus expectedStatus) private {
        assertEq(uint8(game.resolve()), uint8(expectedStatus));
    }

    function _claimCreditAfterDelay(AggregateVerifier game) private {
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }
}
