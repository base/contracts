// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { BadExtraData, GameNotResolved } from "src/libraries/bridge/Errors.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameStatus, GameTypes, Hash } from "src/libraries/bridge/Types.sol";
import { Claim, Timestamp } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract AggregateVerifierTest is BaseTest {
    function testInitializeWithTEEProof() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        assertEq(game.teeProver(), TEE_PROVER);
        assertEq(game.zkProver(), address(0));
        assertEq(game.bondRecipient(), TEE_PROVER);
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
        assertEq(game.proofCount(), 1);
    }

    function testInitializeWithZKProof() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory proof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            ZK_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), proof
        );

        assertEq(game.teeProver(), address(0));
        assertEq(game.zkProver(), ZK_PROVER);
        assertEq(game.bondRecipient(), ZK_PROVER);
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
        assertEq(game.proofCount(), 1);
    }

    function testInitializeFailsIfInvalidCallDataSize() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.deal(TEE_PROVER, INIT_BOND);

        vm.prank(TEE_PROVER);
        vm.expectRevert(BadExtraData.selector);
        factory.createWithInitData{ value: INIT_BOND }(GameTypes.AGGREGATE_VERIFIER, rootClaim, hex"", hex"");
    }

    function testUpdatingAnchorStateRegistryWithTEEProof() public {
        (AggregateVerifier game, Claim rootClaim) =
            _createGameForAnchorUpdate(TEE_PROVER, "tee-proof", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(GameNotResolved.selector);
        game.claimCredit();

        _resolveSlowAndClose(game, rootClaim);
    }

    function testUpdatingAnchorStateRegistryWithZKProof() public {
        (AggregateVerifier game, Claim rootClaim) =
            _createGameForAnchorUpdate(ZK_PROVER, "zk-proof", AggregateVerifier.ProofType.ZK);

        _resolveSlowAndClose(game, rootClaim);
    }

    function testUpdatingAnchorStateRegistryWithBothProofs() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        _provideProof(game, ZK_PROVER, zkProof);
        assertEq(game.proofCount(), 2);

        vm.warp(block.timestamp + game.FAST_FINALIZATION_DELAY());
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        vm.warp(block.timestamp + 1);
        game.closeGame();
        _assertAnchorRoot(rootClaim);

        _claimCreditAfterDelay(game, game.gameCreator());
    }

    function testProofCannotIncreaseExpectedResolution() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );
        uint256 slowDelay = game.SLOW_FINALIZATION_DELAY();

        Timestamp originalExpectedResolution = game.expectedResolution();
        assertEq(originalExpectedResolution.raw(), block.timestamp + slowDelay);

        vm.warp(block.timestamp + slowDelay - 1);
        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        game.resolve();

        _provideProof(game, ZK_PROVER, zkProof);
        assertEq(game.expectedResolution().raw(), originalExpectedResolution.raw());

        vm.warp(block.timestamp + 1);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));
    }

    function testCannotCreateSameProposal() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        bytes memory teeProof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        bytes memory zkProof = _generateProof("zk-proof", AggregateVerifier.ProofType.ZK);

        AggregateVerifier game = _createAggregateVerifierGame(
            TEE_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), teeProof
        );

        Hash gameId = factory.getGameUUID(GameTypes.AGGREGATE_VERIFIER, rootClaim, game.extraData());
        vm.expectRevert(abi.encodeWithSelector(IDisputeGameFactory.GameAlreadyExists.selector, gameId));
        _createAggregateVerifierGame(ZK_PROVER, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), zkProof);
    }

    /// @notice Reverts when the parent is not factory-registered: `_isValidGame` requires
    ///         `AnchorStateRegistry.isGameRegistered`, which checks `DisputeGameFactory.games(...) == parent`.
    /// @dev Parent is a real factory game, then the factory lookup is mocked to emulate a missing UUID entry.
    function testInitializeFailsIfParentGameNotFactoryRegistered() public {
        Claim parentRootClaim = _advanceL2BlockAndClaim();
        AggregateVerifier unregisteredParent = _createAggregateVerifierGame(
            TEE_PROVER,
            parentRootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _generateProof("parent-tee", AggregateVerifier.ProofType.TEE)
        );

        vm.mockCall(
            address(factory),
            abi.encodeCall(
                IDisputeGameFactory.games,
                (GameTypes.AGGREGATE_VERIFIER, parentRootClaim, unregisteredParent.extraData())
            ),
            abi.encode(IDisputeGame(address(0)), Timestamp.wrap(0))
        );

        currentL2BlockNumber += BLOCK_INTERVAL;
        Claim childRootClaim = Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "child")));

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        _createAggregateVerifierGame(
            TEE_PROVER,
            childRootClaim,
            currentL2BlockNumber,
            address(unregisteredParent),
            _generateProof("child-tee", AggregateVerifier.ProofType.TEE)
        );
    }

    function testVerifyFailsWithL1OriginInFuture() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        uint256 l1OriginNumber = block.number + 1;
        bytes32 l1OriginHash = bytes32(uint256(1));

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.L1OriginInFuture.selector, l1OriginNumber, block.number)
        );
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _teeProof(l1OriginHash, l1OriginNumber)
        );
    }

    function testVerifyFailsWithL1OriginTooOld() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.roll(block.number + 300);

        uint256 l1OriginNumber = 1;
        bytes32 l1OriginHash = bytes32(uint256(1));

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.L1OriginTooOld.selector, l1OriginNumber, block.number));
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _teeProof(l1OriginHash, l1OriginNumber)
        );
    }

    function testVerifyFailsWithL1OriginHashMismatch() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        uint256 l1OriginNumber = block.number - 1;
        bytes32 wrongHash = bytes32(uint256(0xdeadbeef));

        bytes32 actualHash = blockhash(l1OriginNumber);
        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.L1OriginHashMismatch.selector, wrongHash, actualHash));
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _teeProof(wrongHash, l1OriginNumber)
        );
    }

    function testVerifyWithBlockhashWindow() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.roll(block.number + 100);

        uint256 l1OriginNumber = block.number - 50;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _teeProof(l1OriginHash, l1OriginNumber)
        );
    }

    function testVerifyWithEIP2935Window() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        vm.roll(block.number + 300);

        uint256 l1OriginNumber = block.number - 260;
        bytes32 expectedHash = keccak256(abi.encodePacked("mock-blockhash", l1OriginNumber));
        address eip2935 = AggregateVerifier(address(factory.gameImpls(GameTypes.AGGREGATE_VERIFIER))).EIP2935_CONTRACT();

        vm.mockCall(eip2935, abi.encode(l1OriginNumber), abi.encode(expectedHash));

        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            _teeProof(expectedHash, l1OriginNumber)
        );
    }

    function testDeployWithInvalidBlockIntervals() public {
        _expectDeployWithInvalidBlockIntervalsReverts(0, INTERMEDIATE_BLOCK_INTERVAL);
        _expectDeployWithInvalidBlockIntervalsReverts(BLOCK_INTERVAL, 0);
        _expectDeployWithInvalidBlockIntervalsReverts(3, 2);
    }

    function _advanceL2BlockAndClaim() private returns (Claim rootClaim) {
        currentL2BlockNumber += BLOCK_INTERVAL;
        return Claim.wrap(keccak256(abi.encode(currentL2BlockNumber)));
    }

    function _assertAnchorRoot(Claim rootClaim) private view {
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), rootClaim.raw());
        assertEq(l2SequenceNumber, currentL2BlockNumber);
    }

    function _createGameForAnchorUpdate(
        address prover,
        bytes memory proofSalt,
        AggregateVerifier.ProofType proofType
    )
        private
        returns (AggregateVerifier game, Claim rootClaim)
    {
        rootClaim = _advanceL2BlockAndClaim();
        game = _createAggregateVerifierGame(
            prover, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), _generateProof(proofSalt, proofType)
        );
    }

    function _resolveSlowAndClose(AggregateVerifier game, Claim rootClaim) private {
        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        _claimCreditAfterDelay(game, game.gameCreator());

        vm.warp(block.timestamp + 1);
        game.closeGame();
        _assertAnchorRoot(rootClaim);
    }

    function _teeProof(bytes32 l1OriginHash, uint256 l1OriginNumber) private pure returns (bytes memory) {
        return abi.encodePacked(
            uint8(AggregateVerifier.ProofType.TEE),
            l1OriginHash,
            l1OriginNumber,
            _generateProofBody("tee-proof", AggregateVerifier.ProofType.TEE)
        );
    }

    function _expectDeployWithInvalidBlockIntervalsReverts(
        uint256 blockInterval,
        uint256 intermediateBlockInterval
    )
        private
    {
        vm.expectRevert(
            abi.encodeWithSelector(
                AggregateVerifier.InvalidBlockInterval.selector, blockInterval, intermediateBlockInterval
            )
        );
        new AggregateVerifier(
            GameTypes.AGGREGATE_VERIFIER,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            TEE_NITRO_IMAGE_HASH,
            TEE_TDX_IMAGE_HASH,
            AggregateVerifier.ZkHashes(ZK_RANGE_HASH, ZK_AGGREGATE_HASH),
            CONFIG_HASH,
            L2_CHAIN_ID,
            blockInterval,
            intermediateBlockInterval
        );
    }
}
