// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import "test/SetupTest.t.sol";
import {GameStatus, Timestamp, AggregateVerifier} from "src/AggregateVerifier.sol";
import {
    MissingTEEProof,
    MissingZKProof,
    IncorrectRootClaim,
    IncorrectBlockNumber,
    IncorrectParentIndex,
    NotAuthorized
} from "src/Errors.sol";

// Optimism
import { ClaimAlreadyResolved, GameAlreadyExists } from "optimism/src/dispute/lib/Errors.sol";

contract AggregateVerifierTest is SetupTest {
    function setUp() public override {
        super.setUp();
        anchorStateRegistry.setRespectedGameType(AGGREGATE_VERIFIER_GAME_TYPE);
    }

    function testInitializeWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "tee-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game, TEE_PROVER, true, proof);
        
        assertEq(game.wasRespectedGameTypeWhenCreated(), true);
        assertEq(address(game.teeProver()), TEE_PROVER);
        assertEq(address(game.zkProver()), address(0));
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.l2SequenceNumber(), currentL2BlockNumber);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.parentIndex(), type(uint32).max);
        assertEq(game.gameType().raw(), AGGREGATE_VERIFIER_GAME_TYPE.raw());
        assertEq(game.gameCreator(), TEE_PROVER);
        assertEq(game.extraData(), abi.encodePacked(currentL2BlockNumber, type(uint32).max));
        assertEq(game.bondRecipient(), address(0));
        assertEq(anchorStateRegistry.isGameProper(IDisputeGame(address(game))), true);
        assertEq(address(game).balance, INIT_BOND);
    }

    function testInitializeWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "zk-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max);
        _provideProof(game, ZK_PROVER, false, proof);
        
        assertEq(game.wasRespectedGameTypeWhenCreated(), true);
        assertEq(address(game.teeProver()), address(0));
        assertEq(address(game.zkProver()), ZK_PROVER);
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.l2SequenceNumber(), currentL2BlockNumber);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.parentIndex(), type(uint32).max);
        assertEq(game.gameType().raw(), AGGREGATE_VERIFIER_GAME_TYPE.raw());
        assertEq(game.gameCreator(), ZK_PROVER);
        assertEq(game.extraData(), abi.encodePacked(currentL2BlockNumber, type(uint32).max));
        assertEq(game.bondRecipient(), address(0));
        assertEq(anchorStateRegistry.isGameProper(IDisputeGame(address(game))), true);
        assertEq(address(game).balance, INIT_BOND);
    }

    function testInitializeFailsIfNotTEEProposer() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "tee-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        vm.expectRevert(NotAuthorized.selector);
        _provideProof(game, ZK_PROVER, true, proof);
    }

    // Helper function to create a game via factory
    function _createAggregateVerifierGame(
        address creator,
        Claim rootClaim,
        uint256 l2BlockNumber,
        uint32 parentIndex
    ) internal returns (AggregateVerifier game) {
        bytes memory extraData = abi.encodePacked(
            uint256(l2BlockNumber),
            uint32(parentIndex)
        );
        
        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        return AggregateVerifier(address(factory.create{value: INIT_BOND}(
            AGGREGATE_VERIFIER_GAME_TYPE,
            rootClaim,
            extraData
        )));
    }

    function _provideProof(AggregateVerifier game, address prover, bool isTeeProof, bytes memory proof) internal {
        vm.prank(prover);
        if (isTeeProof) {
            game.verifyProof(proof, AggregateVerifier.ProofType.TEE);
        } else {
            game.verifyProof(proof, AggregateVerifier.ProofType.ZK);
        }
    }
}
