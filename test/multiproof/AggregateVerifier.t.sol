// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

import {BadExtraData} from "optimism/src/dispute/lib/Errors.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {IDelayedWETH} from "optimism/interfaces/dispute/IDelayedWETH.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "optimism/interfaces/dispute/IDisputeGameFactory.sol";
import {Claim, GameStatus, Hash, Timestamp} from "optimism/src/dispute/lib/Types.sol";

import {AggregateVerifier} from "src/AggregateVerifier.sol";
import {IVerifier} from "src/interfaces/IVerifier.sol";

import {BaseTest} from "test/BaseTest.t.sol";

contract AggregateVerifierTest is BaseTest {
    function testInitializeWithTEEProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof);

        assertEq(game.wasRespectedGameTypeWhenCreated(), true);
        assertEq(address(game.teeProver()), TEE_PROVER);
        assertEq(address(game.zkProver()), address(0));
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.l2SequenceNumber(), currentL2BlockNumber);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.parentIndex(), type(uint32).max);
        assertEq(game.gameType().raw(), AGGREGATE_VERIFIER_GAME_TYPE.raw());
        assertEq(game.gameCreator(), TEE_PROVER);
        assertEq(
            game.extraData(), abi.encodePacked(currentL2BlockNumber, type(uint32).max, game.intermediateOutputRoots())
        );
        assertEq(game.bondRecipient(), address(0));
        assertEq(anchorStateRegistry.isGameProper(IDisputeGame(address(game))), true);
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
    }

    function testInitializeWithZKProof() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory proof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof);

        assertEq(game.wasRespectedGameTypeWhenCreated(), true);
        assertEq(address(game.teeProver()), address(0));
        assertEq(address(game.zkProver()), ZK_PROVER);
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
        assertEq(game.l2SequenceNumber(), currentL2BlockNumber);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.parentIndex(), type(uint32).max);
        assertEq(game.gameType().raw(), AGGREGATE_VERIFIER_GAME_TYPE.raw());
        assertEq(game.gameCreator(), ZK_PROVER);
        assertEq(
            game.extraData(), abi.encodePacked(currentL2BlockNumber, type(uint32).max, game.intermediateOutputRoots())
        );
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
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof);

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
        bytes memory proof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game =
            _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proof);

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
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, teeProof);

        _provideProof(game, ZK_PROVER, zkProof);

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
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, teeProof);

        Timestamp originalExpectedResolution = game.expectedResolution();
        assertEq(originalExpectedResolution.raw(), block.timestamp + 7 days);

        vm.warp(block.timestamp + 7 days - 1);
        // Cannot resolve yet
        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        game.resolve();

        // Provide ZK proof
        _provideProof(game, ZK_PROVER, zkProof);

        // Proof should not have increased expected resolution
        Timestamp expectedResolution = game.expectedResolution();
        assertEq(expectedResolution.raw(), originalExpectedResolution.raw());

        // Resolve after 1 second
        vm.warp(block.timestamp + 1);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));
    }

    function testCannotCreateSameProposal() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game =
            _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, teeProof);

        Hash uuid = factory.getGameUUID(
            AGGREGATE_VERIFIER_GAME_TYPE,
            rootClaim,
            abi.encodePacked(currentL2BlockNumber, type(uint32).max, game.intermediateOutputRoots())
        );
        vm.expectRevert(abi.encodeWithSelector(IDisputeGameFactory.GameAlreadyExists.selector, uuid));
        _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, zkProof);
    }

    function testVerifyFailsWithL1OriginInFuture() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        // Use a future block number
        uint256 l1OriginNumber = block.number + 1;
        bytes32 l1OriginHash = bytes32(uint256(1)); // Fake hash
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        bytes memory proofBytes = abi.encodePacked(uint8(AggregateVerifier.ProofType.TEE), l1OriginHash, l1OriginNumber, rootClaim.raw());

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.L1OriginInFuture.selector, l1OriginNumber, block.number)
        );
        _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proofBytes);
    }

    function testVerifyFailsWithL1OriginTooOld() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Roll forward many blocks to make old blocks unavailable
        vm.roll(block.number + 300);

        // Use a block number that's too old (outside both blockhash window and EIP-2935 window)
        uint256 l1OriginNumber = 1;
        bytes32 l1OriginHash = bytes32(uint256(1)); // Fake hash
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        bytes memory proofBytes = abi.encodePacked(uint8(AggregateVerifier.ProofType.TEE), l1OriginHash, l1OriginNumber, rootClaim.raw());

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.L1OriginTooOld.selector, l1OriginNumber, block.number));
        _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proofBytes);
    }

    function testVerifyFailsWithL1OriginHashMismatch() public {
        currentL2BlockNumber += BLOCK_INTERVAL;
        uint256 l1OriginNumber = block.number - 1;
        bytes32 wrongHash = bytes32(uint256(0xdeadbeef)); // Wrong hash
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        bytes memory proofBytes = abi.encodePacked(uint8(AggregateVerifier.ProofType.TEE), wrongHash, l1OriginNumber, rootClaim.raw());

        bytes32 actualHash = blockhash(l1OriginNumber);
        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.L1OriginHashMismatch.selector, wrongHash, actualHash));
        _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proofBytes);
    }

    function testVerifyWithBlockhashWindow() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Test verification within the 256 block window
        vm.roll(block.number + 100);

        // Use a block that's within the 256 block window
        uint256 l1OriginNumber = block.number - 50;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        bytes memory proofBytes = abi.encodePacked(uint8(AggregateVerifier.ProofType.TEE), l1OriginHash, l1OriginNumber, rootClaim.raw());

        _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proofBytes);
    }

    function testVerifyWithEIP2935Window() public {
        currentL2BlockNumber += BLOCK_INTERVAL;

        // Roll forward past the 256 blockhash window
        vm.roll(block.number + 300);

        // Use a block that's outside blockhash window but within EIP-2935 window
        uint256 l1OriginNumber = block.number - 260; // 260 > 256, so blockhash() returns 0
        bytes32 expectedHash = keccak256(abi.encodePacked("mock-blockhash", l1OriginNumber));
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));

        // Mock the EIP-2935 contract response
        vm.mockCall(
            0x0000F90827F1C53a10cb7A02335B175320002935, // EIP-2935 contract address
            abi.encode(l1OriginNumber), // raw 32-byte calldata
            abi.encode(expectedHash) // returns the blockhash
        );

        bytes memory proofBytes = abi.encodePacked(uint8(AggregateVerifier.ProofType.TEE), expectedHash, l1OriginNumber, rootClaim.raw());

        _createAggregateVerifierGame(TEE_PROVER, rootClaim, currentL2BlockNumber, type(uint32).max, proofBytes);
    }

    function testDeployWithInvalidBlockIntervals() public {
        // Case 1: BLOCK_INTERVAL is 0
        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.InvalidBlockInterval.selector, 0, INTERMEDIATE_BLOCK_INTERVAL)
        );
        new AggregateVerifier(
            AGGREGATE_VERIFIER_GAME_TYPE,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            ZK_IMAGE_HASH,
            CONFIG_HASH,
            L2_CHAIN_ID,
            0,
            INTERMEDIATE_BLOCK_INTERVAL
        );

        // Case 2: INTERMEDIATE_BLOCK_INTERVAL is 0
        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.InvalidBlockInterval.selector, BLOCK_INTERVAL, 0));
        new AggregateVerifier(
            AGGREGATE_VERIFIER_GAME_TYPE,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            ZK_IMAGE_HASH,
            CONFIG_HASH,
            L2_CHAIN_ID,
            BLOCK_INTERVAL,
            0
        );

        // Case 3: BLOCK_INTERVAL is not divisible by INTERMEDIATE_BLOCK_INTERVAL
        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.InvalidBlockInterval.selector, 3, 2));
        new AggregateVerifier(
            AGGREGATE_VERIFIER_GAME_TYPE,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_IMAGE_HASH,
            ZK_IMAGE_HASH,
            CONFIG_HASH,
            L2_CHAIN_ID,
            3,
            2
        );
    }
}
