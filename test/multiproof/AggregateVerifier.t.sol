// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import "test/BaseTest.t.sol";

contract AggregateVerifierTest is BaseTest {

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

    function testUpdatingAnchorStateRegistryWithTEEProof() public {
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
        
        // Resolve after 7 days
        vm.warp(block.timestamp + 7 days + 1);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        // Update AnchorStateRegistry
        vm.warp(block.timestamp + 1);
        game.closeGame();
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), rootClaim.raw());
        assertEq(l2SequenceNumber, currentL2BlockNumber);
    }
}
