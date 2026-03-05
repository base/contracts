// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/dispute/lib/Errors.sol";
import { Claim, GameStatus } from "src/dispute/lib/Types.sol";

import { AggregateVerifier } from "src/multiproof/AggregateVerifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

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

        AggregateVerifier game1 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, zkProof1);

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

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1);

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

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.MissingProof.selector, AggregateVerifier.ProofType.ZK));
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

        vm.expectRevert(ClaimAlreadyResolved.selector);
        game1.nullify(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testNullifyCanOverrideChallenge() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee1")));
        bytes memory teeProof1 = _generateProof("tee-proof-1", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1);

        // Challenge game1 with ZK proof
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk")));
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        vm.prank(ZK_PROVER);
        game1.challenge(zkProof, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());

        // Nullify can override challenge
        Claim rootClaim3 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "zk2")));
        bytes memory zkProof2 = _generateProof("zk-proof-2", AggregateVerifier.ProofType.ZK);
        game1.nullify(zkProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim3.raw());

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
}
