// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { BadExtraData, GameNotResolved } from "src/libraries/bridge/Errors.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameStatus, GameTypes } from "src/libraries/bridge/Types.sol";
import { Claim, Hash } from "src/libraries/bridge/LibUDT.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";

import { BaseTest } from "./BaseTest.t.sol";

contract AggregateVerifierTest is BaseTest {
    function testInitializeWithTEEProof() public {
        AggregateVerifier game =
            _createGame(TEE_PROVER, _advanceL2BlockAndClaim(), "tee-proof", AggregateVerifier.ProofType.TEE);

        assertEq(game.teeProver(), TEE_PROVER);
        assertEq(game.zkProver(), address(0));
        assertEq(game.bondRecipient(), TEE_PROVER);
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
        assertEq(game.proofCount(), 1);
    }

    function testInitializeWithZKProof() public {
        AggregateVerifier game =
            _createGame(ZK_PROVER, _advanceL2BlockAndClaim(), "zk-proof", AggregateVerifier.ProofType.ZK);

        assertEq(game.teeProver(), address(0));
        assertEq(game.zkProver(), ZK_PROVER);
        assertEq(game.bondRecipient(), ZK_PROVER);
        assertEq(delayedWETH.balanceOf(address(game)), INIT_BOND);
        assertEq(game.proofCount(), 1);
    }

    function testInitializeFailsIfInvalidCallDataSize() public {
        hoax(TEE_PROVER, INIT_BOND);
        vm.expectRevert(BadExtraData.selector);
        factory.createWithInitData{ value: INIT_BOND }(
            GameTypes.AGGREGATE_VERIFIER, _advanceL2BlockAndClaim(), hex"", hex""
        );
    }

    function testUpdatingAnchorStateRegistryWithTEEProof() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        AggregateVerifier game = _createGame(TEE_PROVER, rootClaim, "tee-proof", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(GameNotResolved.selector);
        game.claimCredit();

        _resolveSlowAndClose(game, rootClaim);
    }

    function testUpdatingAnchorStateRegistryWithZKProof() public {
        Claim rootClaim = _advanceL2BlockAndClaim();
        AggregateVerifier game = _createGame(ZK_PROVER, rootClaim, "zk-proof", AggregateVerifier.ProofType.ZK);

        _resolveSlowAndClose(game, rootClaim);
    }

    function testUpdatingAnchorStateRegistryWithBothProofs() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        AggregateVerifier game = _createGame(TEE_PROVER, rootClaim, "tee-proof", AggregateVerifier.ProofType.TEE);

        _provideProof(game, ZK_PROVER, _generateProof("zk-proof", AggregateVerifier.ProofType.ZK));
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
        AggregateVerifier game =
            _createGame(TEE_PROVER, _advanceL2BlockAndClaim(), "tee-proof", AggregateVerifier.ProofType.TEE);
        uint256 slowDelay = game.SLOW_FINALIZATION_DELAY();

        uint256 originalExpectedResolution = game.expectedResolution().raw();
        assertEq(originalExpectedResolution, block.timestamp + slowDelay);

        vm.warp(block.timestamp + slowDelay - 1);
        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        game.resolve();

        _provideProof(game, ZK_PROVER, _generateProof("zk-proof", AggregateVerifier.ProofType.ZK));
        assertEq(game.expectedResolution().raw(), originalExpectedResolution);

        vm.warp(block.timestamp + 1);
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));
    }

    function testCannotCreateSameProposal() public {
        Claim rootClaim = _advanceL2BlockAndClaim();

        AggregateVerifier game = _createGame(TEE_PROVER, rootClaim, "tee-proof", AggregateVerifier.ProofType.TEE);

        vm.expectRevert(
            abi.encodeWithSelector(
                IDisputeGameFactory.GameAlreadyExists.selector,
                factory.getGameUUID(GameTypes.AGGREGATE_VERIFIER, rootClaim, game.extraData())
            )
        );
        _createGame(ZK_PROVER, rootClaim, "zk-proof", AggregateVerifier.ProofType.ZK);
    }

    function testInitializeFailsIfParentGameNotFactoryRegistered() public {
        Claim parentRootClaim = _advanceL2BlockAndClaim();
        AggregateVerifier unregisteredParent =
            _createGame(TEE_PROVER, parentRootClaim, "parent-tee", AggregateVerifier.ProofType.TEE);

        vm.mockCall(
            address(factory),
            abi.encodeCall(
                IDisputeGameFactory.games,
                (GameTypes.AGGREGATE_VERIFIER, parentRootClaim, unregisteredParent.extraData())
            ),
            abi.encode(address(0), uint64(0))
        );

        currentL2BlockNumber += BLOCK_INTERVAL;

        vm.expectRevert(AggregateVerifier.InvalidParentGame.selector);
        _createAggregateVerifierGame(
            TEE_PROVER,
            Claim.wrap(keccak256(abi.encode(currentL2BlockNumber, "child"))),
            currentL2BlockNumber,
            address(unregisteredParent),
            _generateProof("child-tee", AggregateVerifier.ProofType.TEE)
        );
    }

    function testVerifyFailsWithL1OriginInFuture() public {
        uint256 l1OriginNumber = block.number + 1;

        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.L1OriginInFuture.selector, l1OriginNumber, block.number)
        );
        _createTEEGameWithOrigin(_advanceL2BlockAndClaim(), bytes32(uint256(1)), l1OriginNumber);
    }

    function testVerifyFailsWithL1OriginTooOld() public {
        vm.roll(block.number + 300);

        uint256 l1OriginNumber = 1;

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.L1OriginTooOld.selector, l1OriginNumber, block.number));
        _createTEEGameWithOrigin(_advanceL2BlockAndClaim(), bytes32(uint256(1)), l1OriginNumber);
    }

    function testVerifyFailsWithL1OriginHashMismatch() public {
        uint256 l1OriginNumber = block.number - 1;
        bytes32 wrongHash = bytes32(uint256(0xdeadbeef));

        vm.expectRevert(
            abi.encodeWithSelector(
                AggregateVerifier.L1OriginHashMismatch.selector, wrongHash, blockhash(l1OriginNumber)
            )
        );
        _createTEEGameWithOrigin(_advanceL2BlockAndClaim(), wrongHash, l1OriginNumber);
    }

    function testVerifyWithBlockhashWindow() public {
        vm.roll(block.number + 100);

        uint256 l1OriginNumber = block.number - 50;

        _createTEEGameWithOrigin(_advanceL2BlockAndClaim(), blockhash(l1OriginNumber), l1OriginNumber);
    }

    function testVerifyWithEIP2935Window() public {
        vm.roll(block.number + 300);

        uint256 l1OriginNumber = block.number - 260;
        bytes32 expectedHash = keccak256(abi.encodePacked("mock-blockhash", l1OriginNumber));

        vm.mockCall(
            AggregateVerifier(address(factory.gameImpls(GameTypes.AGGREGATE_VERIFIER))).EIP2935_CONTRACT(),
            abi.encode(l1OriginNumber),
            abi.encode(expectedHash)
        );

        _createTEEGameWithOrigin(_advanceL2BlockAndClaim(), expectedHash, l1OriginNumber);
    }

    function testDeployWithInvalidBlockIntervals() public {
        vm.expectRevert(
            abi.encodeWithSelector(AggregateVerifier.InvalidBlockInterval.selector, 0, INTERMEDIATE_BLOCK_INTERVAL)
        );
        _newAggregateVerifier(0, INTERMEDIATE_BLOCK_INTERVAL);

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.InvalidBlockInterval.selector, BLOCK_INTERVAL, 0));
        _newAggregateVerifier(BLOCK_INTERVAL, 0);

        vm.expectRevert(abi.encodeWithSelector(AggregateVerifier.InvalidBlockInterval.selector, 3, 2));
        _newAggregateVerifier(3, 2);
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

    function _resolveSlowAndClose(AggregateVerifier game, Claim rootClaim) private {
        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        game.resolve();
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));

        _claimCreditAfterDelay(game, game.gameCreator());

        vm.warp(block.timestamp + 1);
        game.closeGame();
        _assertAnchorRoot(rootClaim);
    }

    function _createGame(
        address creator,
        Claim rootClaim,
        bytes memory proofSalt,
        AggregateVerifier.ProofType proofType
    )
        private
        returns (AggregateVerifier)
    {
        return _createAggregateVerifierGame(
            creator, rootClaim, currentL2BlockNumber, address(anchorStateRegistry), _generateProof(proofSalt, proofType)
        );
    }

    function _createTEEGameWithOrigin(Claim rootClaim, bytes32 l1OriginHash, uint256 l1OriginNumber) private {
        _createAggregateVerifierGame(
            TEE_PROVER,
            rootClaim,
            currentL2BlockNumber,
            address(anchorStateRegistry),
            abi.encodePacked(
                uint8(AggregateVerifier.ProofType.TEE),
                l1OriginHash,
                l1OriginNumber,
                _generateProofBody("tee-proof", AggregateVerifier.ProofType.TEE)
            )
        );
    }
}
