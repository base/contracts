// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Libraries
import { GameType, Hash, Claim } from "src/libraries/bridge/LibUDT.sol";

////////////////////////////////////////////////////////////////
//                `DisputeGameFactory` Errors                 //
////////////////////////////////////////////////////////////////

/// @notice Thrown when a dispute game is attempted to be created with an unsupported game type.
/// @param gameType The unsupported game type.
error NoImplementation(GameType gameType);

/// @notice Thrown when a dispute game that already exists is attempted to be created.
/// @param uuid The UUID of the dispute game that already exists.
error GameAlreadyExists(Hash uuid);

/// @notice Thrown when a supplied bond is not equal to the required bond amount to cover the cost of the interaction.
error IncorrectBondAmount();

////////////////////////////////////////////////////////////////
//                `OptimismPortal2` Errors.                   //
////////////////////////////////////////////////////////////////

/// @notice Thrown when an output root proof is invalid.
error InvalidOutputRootProof();

////////////////////////////////////////////////////////////////
//                `AggregateVerifier` Errors                  //
////////////////////////////////////////////////////////////////

/// @notice Thrown when a dispute game has already been initialized.
error AlreadyInitialized();

/// @notice Thrown when a credit claim is attempted for a value of 0.
error NoCreditToClaim();

/// @notice Thrown when the transfer of credit to a recipient account reverts.
error BondTransferFailed();

/// @notice Thrown when the `extraData` passed to the CWIA proxy is of improper length, or contains invalid information.
error BadExtraData();

/// @notice Thrown when an action that requires the game to be `IN_PROGRESS` is invoked when
///         the game is not in progress.
error GameNotInProgress();

/// @notice Thrown when resolving a claim that has already been resolved.
error ClaimAlreadyResolved();

/// @notice Thrown when the game is not yet finalized.
error GameNotFinalized();

/// @notice Thrown when the game is not yet resolved.
error GameNotResolved();

/// @notice Thrown when trying to close a game while the system is paused.
error GamePaused();

/// @notice Thrown when the parent game is invalid.
error InvalidParentGame();

/// @notice Thrown when the parent game is not resolved.
error ParentGameNotResolved();

/// @notice Thrown when the game is over.
error GameOver();

/// @notice Thrown when the game is not over.
error GameNotOver();

////////////////////////////////////////////////////////////////
//              `LibGameArgs` Errors                          //
////////////////////////////////////////////////////////////////

/// @notice Thrown when the length of the game arguments is invalid.
error InvalidGameArgsLength();
