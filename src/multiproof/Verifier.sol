// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";
import { IDisputeGame } from "interfaces/dispute/IDisputeGame.sol";
import { IVerifier } from "interfaces/multiproof/IVerifier.sol";

abstract contract Verifier is IVerifier {
    /// @notice The anchor state registry.
    IAnchorStateRegistry public immutable ANCHOR_STATE_REGISTRY;

    /// @notice Whether this verifier has been nullified.
    /// @dev This is used to prevent further proof verification after a soundness issue is found.
    bool public nullified;

    /// @notice Emitted when the verifier is nullified.
    /// @param game The game that nullified this verifier.
    event VerifierNullified(IDisputeGame game);

    /// @notice Thrown when the verifier has been nullified.
    error Nullified();

    /// @notice Thrown when the caller trying to nullify is not a proper dispute game.
    error NotProperGame();

    /// @notice Modifier to prevent execution if the verifier has been nullified.
    modifier notNullified() {
        if (nullified) revert Nullified();
        _;
    }

    constructor(IAnchorStateRegistry anchorStateRegistry) {
        ANCHOR_STATE_REGISTRY = anchorStateRegistry;
    }

    /// @notice Nullifies the verifier to prevent further proof verification.
    /// @dev Should only occur if a soundness issue is found.
    /// @dev Should only be callable by a proper dispute game.
    function nullify() external override {
        if (
            !ANCHOR_STATE_REGISTRY.isGameProper(IDisputeGame(msg.sender))
                || !ANCHOR_STATE_REGISTRY.isGameRespected(IDisputeGame(msg.sender))
        ) revert NotProperGame();
        nullified = true;

        emit VerifierNullified(IDisputeGame(msg.sender));
    }
}
