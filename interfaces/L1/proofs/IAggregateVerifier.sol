// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IDisputeGame } from "./IDisputeGame.sol";
import { IDisputeGameFactory } from "./IDisputeGameFactory.sol";
import { IDelayedWETH } from "./IDelayedWETH.sol";
import { IVerifier } from "./IVerifier.sol";
import { Proposal, Hash } from "src/libraries/bridge/Types.sol";
import { Timestamp } from "src/libraries/bridge/LibUDT.sol";

interface IAggregateVerifier is IDisputeGame {
    function SLOW_FINALIZATION_DELAY() external view returns (uint64);
    function FAST_FINALIZATION_DELAY() external view returns (uint64);
    function EIP2935_CONTRACT() external view returns (address);
    function BLOCKHASH_WINDOW() external view returns (uint256);
    function EIP2935_WINDOW() external view returns (uint256);
    function PROOF_THRESHOLD() external view returns (uint256);
    function DISPUTE_GAME_FACTORY() external view returns (IDisputeGameFactory);
    function DELAYED_WETH() external view returns (IDelayedWETH);
    function TEE_VERIFIER() external view returns (IVerifier);
    function TEE_IMAGE_HASH() external view returns (bytes32);
    function ZK_VERIFIER() external view returns (IVerifier);
    function ZK_RANGE_HASH() external view returns (bytes32);
    function ZK_AGGREGATE_HASH() external view returns (bytes32);
    function CONFIG_HASH() external view returns (bytes32);
    function L2_CHAIN_ID() external view returns (uint256);
    function BLOCK_INTERVAL() external view returns (uint256);
    function INTERMEDIATE_BLOCK_INTERVAL() external view returns (uint256);

    function startingOutputRoot() external view returns (Proposal memory);
    function bondRecipient() external view returns (address);
    function bondUnlocked() external view returns (bool);
    function bondClaimed() external view returns (bool);
    function bondAmount() external view returns (uint256);
    function counteredByIntermediateRootIndexPlusOne() external view returns (uint256);
    function expectedResolution() external view returns (Timestamp);
    function proofCount() external view returns (uint8);

    function initializeWithInitData(bytes calldata proof) external payable;
    function verifyProposalProof(bytes calldata proofBytes) external;
    function challenge(
        bytes calldata proofBytes,
        uint256 intermediateRootIndex,
        bytes32 intermediateRootToProve
    )
        external;
    function nullify(
        bytes calldata proofBytes,
        uint256 intermediateRootIndex,
        bytes32 intermediateRootToProve
    )
        external;
    function claimCredit() external;
    function closeGame() external;

    function startingBlockNumber() external view returns (uint256);
    function startingRootHash() external view returns (Hash);
    function teeProver() external view returns (address);
    function zkProver() external view returns (address);
    function gameOver() external view returns (bool);
    function intermediateOutputRootsCount() external view returns (uint256);
    function intermediateOutputRoots() external view returns (bytes memory);
    function intermediateOutputRoot(uint256 index) external view returns (bytes32);
    function parentAddress() external pure returns (address);
}
