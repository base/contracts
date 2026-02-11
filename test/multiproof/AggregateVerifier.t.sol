// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {BadExtraData} from "optimism/src/dispute/lib/Errors.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "optimism/interfaces/dispute/IDisputeGameFactory.sol";
import {Claim, GameStatus, Hash, Timestamp} from "optimism/src/dispute/lib/Types.sol";

import {AggregateVerifier} from "src/AggregateVerifier.sol";

import {BaseTest} from "test/BaseTest.t.sol";

contract AggregateVerifierTest is BaseTest {
    function testInitializeWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = _generateProof("tee-proof");

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof, AggregateVerifier.ProofType.TEE
        );

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
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
    }

    function testInitializeWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = _generateProof("zk-proof");

        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof, AggregateVerifier.ProofType.ZK
        );

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
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
    }

    function testInitializeFailsIfInvalidCallDataSize() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        vm.deal(TEE_PROVER, INIT_BOND);
        bytes memory extraData = "";
        bytes memory initData = "";

        vm.prank(TEE_PROVER);
        vm.expectRevert(BadExtraData.selector);
        factory.create{value: INIT_BOND}(AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, extraData, initData);
    }

    function testUpdatingAnchorStateRegistryWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = _generateProof("tee-proof");

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof, AggregateVerifier.ProofType.TEE
        );

        // Cannot claim bond before resolving
        vm.expectRevert(AggregateVerifier.BondRecipientEmpty.selector);
        game.claimCredit();

        // Resolve after 7 days
        vm.warp(block.timestamp + 7 days);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        // Unlock and reclaim bond after resolving
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);

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
        bytes memory proof = _generateProof("zk-proof");

        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof, AggregateVerifier.ProofType.ZK
        );

        // Unlock and reclaim bond after delay
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);

        // Resolve after another 7 days
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
        bytes memory teeProof = _generateProof("tee-proof");
        bytes memory zkProof = _generateProof("zk-proof");

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, teeProof, AggregateVerifier.ProofType.TEE
        );

        _provideProof(game, ZK_PROVER, AggregateVerifier.ProofType.ZK, zkProof);

        // Unlock bond
        uint256 balanceBefore = game.gameCreator().balance;
        game.claimCredit();

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

        // Unlock and reclaim bond after delay
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(game.gameCreator().balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }

    function testProofCannotIncreaseExpectedResolution() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory teeProof = _generateProof("tee-proof");
        bytes memory zkProof = _generateProof("zk-proof");

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, teeProof, AggregateVerifier.ProofType.TEE
        );

        (,,, Timestamp originalExpectedResolution) = game.provingData();
        assertEq(originalExpectedResolution.raw(), block.timestamp + 7 days);

        vm.warp(block.timestamp + 7 days - 1);
        // Cannot resolve yet
        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        game.resolve();

        // Provide ZK proof
        _provideProof(game, ZK_PROVER, AggregateVerifier.ProofType.ZK, zkProof);

        // Proof should not have increased expected resolution
        (,,, Timestamp expectedResolution) = game.provingData();
        assertEq(expectedResolution.raw(), originalExpectedResolution.raw());

        // Resolve after 1 second
        vm.warp(block.timestamp + 1);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));
    }

    function testCannotCreateSameProposal() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory teeProof = _generateProof("tee-proof");
        bytes memory zkProof = _generateProof("zk-proof");

        _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, teeProof, AggregateVerifier.ProofType.TEE
        );

        Hash uuid = factory.getGameUUID(
            AGGREGATE_VERIFIER_GAME_TYPE, rootClaim, abi.encodePacked(currentL2BlockNumber, type(uint32).max)
        );
        vm.expectRevert(abi.encodeWithSelector(IDisputeGameFactory.GameAlreadyExists.selector, uuid));
        _createAggregateVerifierGame(
            ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, zkProof, AggregateVerifier.ProofType.ZK
        );
    }
}
