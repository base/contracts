// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { GameStatus, GameTypes } from "src/libraries/bridge/Types.sol";
import { Claim, Timestamp } from "src/libraries/bridge/LibUDT.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { NoCreditToClaim } from "src/libraries/bridge/Errors.sol";

import { BaseTest } from "./BaseTest.t.sol";

/// @notice Tests for claimCredit's 14-day no-proof fallback path.
///
/// The AggregateVerifier has two distinct credit-claim paths:
///   (A) Normal path: expectedResolution != type(uint64).max  → requires resolvedAt != 0
///   (B) 14-day fallback: expectedResolution == type(uint64).max → requires 14 days since createdAt
///
/// Path (A) is exercised throughout AggregateVerifier.t.sol, Nullify.t.sol, and Challenge.t.sol.
/// Path (B) — entered when proofCount drops back to zero after all proofs are nullified — has
/// no coverage. This file closes that gap.
contract AggregateVerifierClaimCreditFallbackTest is BaseTest {
    uint256 private constant FOURTEEN_DAYS = 14 days;

    AggregateVerifier private game;
    uint256 private gameCreatedAt;

    function setUp() public override {
        super.setUp();

        // Create a game with one TEE proof (normal initialization).
        uint256 l2Block = SLOW_BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(l2Block)));
        uint256 count = SLOW_BLOCK_INTERVAL / SLOW_INTERMEDIATE_BLOCK_INTERVAL;
        bytes32[] memory roots = new bytes32[](count);
        for (uint256 i; i < count - 1; i++) roots[i] = keccak256(abi.encode(l2Block, i));
        roots[count - 1] = rootClaim.raw();

        bytes memory extraData =
            abi.encodePacked(l2Block, address(anchorStateRegistry), abi.encodePacked(roots));
        bytes memory proof = _generateProof(abi.encode(l2Block), AggregateVerifier.ProofType.TEE);

        vm.warp(L2_GENESIS_TIMESTAMP + l2Block * L2_BLOCK_TIME);
        gameCreatedAt = block.timestamp;

        vm.deal(TEE_PROVER, INIT_BOND);
        vm.prank(TEE_PROVER);
        game = AggregateVerifier(
            address(
                factory.createWithInitData{ value: INIT_BOND }(
                    GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof
                )
            )
        );

        assertEq(game.proofCount(), 1);

        // Nullify the TEE proof via a shared verifier nullified by another game,
        // which drives proofCount to 0 and expectedResolution back to type(uint64).max.
        //
        // Pattern taken from Nullify.t.sol:_assertResolveEarlyReturnWhenSharedVerifierNullifiedByAnotherGame:
        // create gameB on top of game, nullify gameB's TEE proof — that marks the shared teeVerifier
        // as nullified. Calling game.resolve() then detects the nullified verifier, calls
        // _proofRefutedUpdate internally, and resets expectedResolution to max.
        uint256 l2BlockB = SLOW_BLOCK_INTERVAL * 2;
        Claim rootClaimB = Claim.wrap(keccak256(abi.encode(l2BlockB)));
        bytes32[] memory rootsB = new bytes32[](count);
        for (uint256 i; i < count - 1; i++) rootsB[i] = keccak256(abi.encode(l2BlockB, i));
        rootsB[count - 1] = rootClaimB.raw();

        bytes memory extraDataB =
            abi.encodePacked(l2BlockB, address(game), abi.encodePacked(rootsB));
        bytes memory proofB = _generateProof(abi.encode(l2BlockB), AggregateVerifier.ProofType.TEE);

        vm.warp(L2_GENESIS_TIMESTAMP + l2BlockB * L2_BLOCK_TIME);
        vm.deal(TEE_PROVER, INIT_BOND);
        vm.prank(TEE_PROVER);
        AggregateVerifier gameB = AggregateVerifier(
            address(
                factory.createWithInitData{ value: INIT_BOND }(
                    GameTypes.AGGREGATE_VERIFIER, rootClaimB, extraDataB, proofB
                )
            )
        );

        // Nullify gameB with a different TEE claim to mark teeVerifier as nullified globally.
        uint256 lastIdx = count - 1;
        bytes32 counterRoot = keccak256("counter");
        bytes memory nullifyProof = _generateProof("nullify", AggregateVerifier.ProofType.TEE);
        vm.prank(TEE_PROVER);
        gameB.nullify(nullifyProof, lastIdx, counterRoot);

        // Now teeVerifier is globally nullified. Calling game.resolve() triggers the early-return
        // path that calls _proofRefutedUpdate, zeroing proofCount and resetting expectedResolution.
        vm.warp(block.timestamp + game.SLOW_FINALIZATION_DELAY());
        game.resolve(); // early return: IN_PROGRESS

        // Postconditions required by the tests below.
        assertEq(game.proofCount(), 0);
        assertEq(game.expectedResolution().raw(), type(uint64).max);
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));
    }

    /// @notice claimCredit reverts with GameNotOver before 14 days have elapsed when no valid
    ///         proof exists (expectedResolution == type(uint64).max).
    function test_claimCredit_noProof_beforeFourteenDays_reverts() public {
        vm.warp(gameCreatedAt + FOURTEEN_DAYS - 1);
        vm.expectRevert(AggregateVerifier.GameNotOver.selector);
        game.claimCredit();
    }

    /// @notice claimCredit succeeds after exactly 14 days when no valid proof exists,
    ///         returning the full bond to the original proposer via the two-step DelayedWETH flow.
    function test_claimCredit_noProof_afterFourteenDays_succeeds() public {
        vm.warp(gameCreatedAt + FOURTEEN_DAYS);

        address creator = game.gameCreator();
        uint256 balanceBefore = creator.balance;

        // Step 1: unlock
        game.claimCredit();
        // Step 2: withdraw after DelayedWETH delay
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();

        assertEq(creator.balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }

    /// @notice claimCredit reverts with NoCreditToClaim on a second full claim attempt.
    function test_claimCredit_noProof_doubleClaimReverts() public {
        vm.warp(gameCreatedAt + FOURTEEN_DAYS);
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();

        vm.expectRevert(NoCreditToClaim.selector);
        game.claimCredit();
    }
}
