// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Libraries
import {
    Hash,
    GameType
} from "src/libraries/bridge/LibUDT.sol";

/// @notice The current status of the dispute game.
enum GameStatus {
    // The game is currently in progress, and has not been resolved.
    IN_PROGRESS,
    // The game has concluded, and the `rootClaim` was challenged successfully.
    CHALLENGER_WINS,
    // The game has concluded, and the `rootClaim` could not be contested.
    DEFENDER_WINS
}

/// @notice Represents an L2 root and the L2 sequence number at which it was generated.
/// @custom:field root The output root.
/// @custom:field l2SequenceNumber The L2 Sequence Number ( e.g. block number / timestamp) at which the root was
/// generated.
struct Proposal {
    Hash root;
    uint256 l2SequenceNumber;
}

/// @title GameTypes
/// @notice A library that defines the IDs of games that can be played.
library GameTypes {
    /// @notice A dispute game type for aggregating results from TEE + ZK verifiers (multiproofs).
    GameType internal constant AGGREGATE_VERIFIER = GameType.wrap(621);
}
