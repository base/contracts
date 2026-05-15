// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/libraries/bridge/Errors.sol";
import { GameStatus } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract NullifyTest is BaseTest {
    function testNullifyWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        game.nullify(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.bondRecipient(), TEE_PROVER);
        assertEq(game.proofCount(), 0);
        assertEq(game.expectedResolution().raw(), type(uint64).max);

        // expectedResolution is uint64.max (no proofs left), so must wait 14 days from creation
        vm.warp(block.timestamp + 14 days);

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

        AggregateVerifier game1 = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), zkProof1
        );

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);

        game1.nullify(zkProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        assertEq(uint8(game1.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game1.bondRecipient(), ZK_PROVER);
        assertEq(game1.proofCount(), 0);
        assertEq(game1.expectedResolution().raw(), type(uint64).max);

        // expectedResolution is uint64.max (no proofs left), so must wait 14 days from creation
        vm.warp(block.timestamp + 14 days);

        uint256 balanceBefore = game1.gameCreator().balance;
        game1.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game1.claimCredit();
        assertEq(game1.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game1)), 0);
    }

    function testNullifyWithTEEProofWhenTEEAndZKProofsAreProvided() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        bytes memory zkProof = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);
        game.verifyProposalProof(zkProof);

        assertEq(game.expectedResolution().raw(), block.timestamp + 1 days);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);
        game.nullify(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.bondRecipient(), TEE_PROVER);
        assertEq(game.proofCount(), 1);
        assertEq(game.expectedResolution().raw(), block.timestamp + 7 days);
    }

    function testZKNullifyFailsIfNoZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.ZK));
        game1.nullify(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testNullifyFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        // Resolve game1
        vm.warp(block.timestamp + 7 days);
        game1.resolve();

        // Try to nullify game1
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.nullify(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testNullifyCanOverrideChallenge() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim1, currentL2BlockNumber, address(anchorStateRegistry), teeProof1
        );

        // Challenge game1 with ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.prank(ZK_PROVER);
        game1.challenge(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        // Nullify can override challenge
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);
        game1.nullify(zkProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim1.raw());

        assertEq(game1.bondRecipient(), TEE_PROVER);

        // After nullify, only TEE proof remains; expectedResolution = now + 7 days
        vm.warp(block.timestamp + 7 days);
        game1.resolve();

        uint256 balanceBefore = game1.gameCreator().balance;
        game1.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game1.claimCredit();
        assertEq(game1.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game1)), 0);
    }

    /// @notice `resolve` runs `_updateProofCount`; when the shared TEE verifier was nullified by another game,
    ///         refutation persists and `resolve` returns early `IN_PROGRESS` (no `Resolved` event) instead of
    /// reverting. @dev All clones share the same `MockVerifier` TEE instance; `Verifier.nullify` requires a proper
    /// factory game.
    function testResolveEarlyReturnWhenSharedTeeVerifierNullifiedByAnotherGame() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaimA = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "game-a")));
        bytes memory teeProofA = _generateProof("tee-proof-a", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameA = _createAggregateVerifierGame(
            TEE_PROVER, rootClaimA, currentL2BlockNumber, address(anchorStateRegistry), teeProofA
        );

        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaimB = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "game-b")));
        bytes memory teeProofB = _generateProof("tee-proof-b", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameB =
            _createAggregateVerifierGame(TEE_PROVER, rootClaimB, currentL2BlockNumber, address(gameA), teeProofB);

        vm.warp(block.timestamp + 7 days);
        assertTrue(gameA.gameOver());
        assertEq(gameA.proofCount(), 1);

        Claim rootClaimNullify = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "nullify-b")));
        bytes memory teeProofNullify = _generateProof("tee-nullify-b", AggregateVerifier.ProofType.TEE);
        uint256 lastIntermediateIdx = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1;
        gameB.nullify(teeProofNullify, lastIntermediateIdx, rootClaimNullify.raw());

        assertTrue(teeVerifier.nullified());
        assertEq(gameA.proofCount(), 1);

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 0);
        assertEq(gameA.expectedResolution().raw(), type(uint64).max);

        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        gameA.resolve();
    }

    /// @notice Same as `testResolveEarlyReturnWhenSharedTeeVerifierNullifiedByAnotherGame` but for the shared ZK
    /// verifier.
    function testResolveEarlyReturnWhenSharedZkVerifierNullifiedByAnotherGame() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaimA = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk-game-a")));
        bytes memory zkProofA = _generateProof("zk-proof-a", AggregateVerifier.ProofType.ZK);
        AggregateVerifier gameA = _createAggregateVerifierGame(
            ZK_PROVER, rootClaimA, currentL2BlockNumber, address(anchorStateRegistry), zkProofA
        );

        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaimB = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk-game-b")));
        bytes memory zkProofB = _generateProof("zk-proof-b", AggregateVerifier.ProofType.ZK);
        AggregateVerifier gameB =
            _createAggregateVerifierGame(ZK_PROVER, rootClaimB, currentL2BlockNumber, address(gameA), zkProofB);

        vm.warp(block.timestamp + 7 days);
        assertTrue(gameA.gameOver());
        assertEq(gameA.proofCount(), 1);

        Claim rootClaimNullify = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk-nullify-b")));
        bytes memory zkProofNullify = _generateProof("zk-nullify-b", AggregateVerifier.ProofType.ZK);
        uint256 lastIntermediateIdx = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1;
        gameB.nullify(zkProofNullify, lastIntermediateIdx, rootClaimNullify.raw());

        assertTrue(zkVerifier.nullified());
        assertEq(gameA.proofCount(), 1);

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 0);
        assertEq(gameA.expectedResolution().raw(), type(uint64).max);

        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        gameA.resolve();
    }

    /// @notice With TEE + ZK, the fast window is 1 day. Another game nullifies the shared ZK verifier; the first
    ///         `resolve` persists the ZK refutation and returns `IN_PROGRESS`. After `SLOW_FINALIZATION_DELAY`
    ///         (7 days) from that moment, a second `resolve` finalizes with only the TEE proof.
    function testTwoProofsResolveDelayedAfterExternalVerifierNullify() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaimA = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "dual-a")));
        bytes memory teeProofA = _generateProof("tee-dual-a", AggregateVerifier.ProofType.TEE);
        AggregateVerifier gameA = _createAggregateVerifierGame(
            TEE_PROVER, rootClaimA, currentL2BlockNumber, address(anchorStateRegistry), teeProofA
        );

        bytes memory zkProofA = _generateProof("zk-dual-a", AggregateVerifier.ProofType.ZK);
        vm.prank(ZK_PROVER);
        gameA.verifyProposalProof(zkProofA);

        assertEq(gameA.proofCount(), 2);
        assertEq(gameA.expectedResolution().raw(), block.timestamp + 1 days);

        vm.warp(block.timestamp + 1 days);
        assertTrue(gameA.gameOver());

        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaimB = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "dual-b")));
        bytes memory zkProofB = _generateProof("zk-dual-b", AggregateVerifier.ProofType.ZK);
        AggregateVerifier gameB =
            _createAggregateVerifierGame(ZK_PROVER, rootClaimB, currentL2BlockNumber, address(gameA), zkProofB);

        Claim rootClaimNullify = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "dual-nullify-b")));
        bytes memory zkProofNullify = _generateProof("zk-nullify-dual", AggregateVerifier.ProofType.ZK);
        uint256 lastIntermediateIdx = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1;
        gameB.nullify(zkProofNullify, lastIntermediateIdx, rootClaimNullify.raw());
        assertTrue(zkVerifier.nullified());

        assertEq(uint8(gameA.resolve()), uint8(GameStatus.IN_PROGRESS));
        assertEq(gameA.proofCount(), 1);
        assertEq(gameA.expectedResolution().raw(), block.timestamp + 7 days);

        vm.warp(block.timestamp + 7 days);
        assertEq(uint8(gameA.resolve()), uint8(GameStatus.DEFENDER_WINS));
        assertEq(uint8(gameA.status()), uint8(GameStatus.DEFENDER_WINS));
    }
}
