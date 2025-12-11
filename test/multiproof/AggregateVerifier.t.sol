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
        assertEq(game.bondRecipient(), ZK_PROVER);
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
        
        // Cannot claim bond before resolving
        vm.expectRevert(BondRecipientEmpty.selector);
        game.claimCredit();

        // Resolve after 7 days
        vm.warp(block.timestamp + 7 days);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        // Reclaim bond after resolving
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(address(game).balance, 0);

        // Update AnchorStateRegistry
        vm.warp(block.timestamp + 1);
        game.closeGame();
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), rootClaim.raw());
        assertEq(l2SequenceNumber, currentL2BlockNumber);
    }

    function testUpdatingAnchorStateRegistryWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "zk-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game, ZK_PROVER, false, proof);
        
        // Reclaim bond
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(address(game).balance, 0);

        // Resolve after 7 days
        vm.warp(block.timestamp + 7 days);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        // Update AnchorStateRegistry
        vm.warp(block.timestamp + 1);
        game.closeGame();
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), rootClaim.raw());
        assertEq(l2SequenceNumber, currentL2BlockNumber);
    }

    function testUpdatingAnchorStateRegistryWithBothProofs() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory teeProof = "tee-proof";
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game, TEE_PROVER, true, teeProof);
        _provideProof(game, ZK_PROVER, false, zkProof);
        
        // Reclaim bond
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(address(game).balance, 0);

        // Resolve after 1 day
        vm.warp(block.timestamp + 1 days);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        // Update AnchorStateRegistry
        vm.warp(block.timestamp + 1);
        game.closeGame();
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), rootClaim.raw());
        assertEq(l2SequenceNumber, currentL2BlockNumber);
    }

    function testProofCannotIncreaseExpectedResolution() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory teeProof = "tee-proof";
        bytes memory zkProof = "zk-proof";
        
        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        _provideProof(game, TEE_PROVER, true, teeProof);

        (, , , Timestamp originalExpectedResolution) = game.provingData();
        assertEq(originalExpectedResolution.raw(), block.timestamp + 7 days);

        vm.warp(block.timestamp + 7 days - 1);
        // Cannot resolve yet
        vm.expectRevert(GameNotOver.selector);
        game.resolve();

        // Provide ZK proof
        _provideProof(game, ZK_PROVER, false, zkProof);
        
        // Proof should not have increased expected resolution
        (, , , Timestamp expectedResolution) = game.provingData();
        assertEq(expectedResolution.raw(), originalExpectedResolution.raw());

        // Resolve after 1 second
        vm.warp(block.timestamp + 1);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));
    }

    function testParentGameMustHaveAProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = "tee-proof";
        
        AggregateVerifier parentGame = _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            type(uint32).max
        );

        uint256 parentGameIndex = factory.gameCount() - 1;
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaimChild = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        // Cannot create a child game without a proof for the parent
        vm.expectRevert(InvalidParentGame.selector);
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaimChild,
            currentL2BlockNumber,
            uint32(parentGameIndex)
        );

        // Provide proof for the parent game
        _provideProof(parentGame, TEE_PROVER, true, proof);
        
        // Create the child game
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaimChild,
            currentL2BlockNumber,
            uint32(parentGameIndex)
        );
    }
}
