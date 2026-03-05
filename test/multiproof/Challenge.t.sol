// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import { ClaimAlreadyResolved } from "src/dispute/lib/Errors.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";
import { IDisputeGame } from "interfaces/dispute/IDisputeGame.sol";
import { Claim, GameStatus, Hash } from "src/dispute/lib/Types.sol";

import { AggregateVerifier } from "src/multiproof/AggregateVerifier.sol";
import { Verifier } from "src/multiproof/Verifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract ChallengeTest is BaseTest {
    function testChallengeTEEProofWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Create game with TEE proof
        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof);

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

        AggregateVerifier game1 =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, zkProof1);

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

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1);

        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory teeProof2 = _generateProof("tee-proof-2", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(AggregateVerifier.InvalidProofType.selector);
        game1.challenge(teeProof2, BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL - 1, rootClaim2.raw());
    }

    function testChallengeFailsIfGameAlreadyResolved() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        Claim rootClaim1 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee")));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game1 =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof);

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

        AggregateVerifier parentGame =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, parentProof);

        uint256 parentGameIndex = factory.gameCount() - 1;
        currentL2BlockNumber += BLOCK_INTERVAL;

        // create child game
        Claim rootClaim2 = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "tee2")));
        bytes memory childProof = _generateProof("child-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier childGame =
        // forge-lint: disable-next-line(unsafe-typecast)
        _createAggregateVerifierGame(TEE_PROVER, rootClaim2, currentL2BlockNumber, uint32(parentGameIndex), childProof);

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

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof);

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

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof1);

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
        AggregateVerifier game =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim1, currentL2BlockNumber, type(uint32).max, teeProof);
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
}
