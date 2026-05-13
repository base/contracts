// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType, Hash, Proposal } from "src/libraries/bridge/Types.sol";

/// @title MockAnchorStateRegistry
/// @notice Minimal mock for testing - stores anchor state and factory reference.
/// @dev We use a mock instead of the real AnchorStateRegistry because:
///      1. The real contract requires deploying the entire Optimism L1 stack
///         (SystemConfig, SuperchainConfig, ProxyAdmin, Guardian roles, etc.)
///      2. The real contract has "stack too deep" compilation issues that require
///         special compiler settings (via-ir) which significantly slow builds
///      3. For TEE prover testing, we only need getAnchorRoot() and setAnchorState()
contract MockAnchorStateRegistry {
    Hash public anchorRoot;
    uint256 public anchorL2BlockNumber;
    address public factory;
    GameType public respectedGameType;

    function initialize(
        address newFactory,
        Hash newAnchorRoot,
        uint256 newAnchorL2BlockNumber,
        GameType gameType
    )
        external
    {
        factory = newFactory;
        anchorRoot = newAnchorRoot;
        anchorL2BlockNumber = newAnchorL2BlockNumber;
        respectedGameType = gameType;
    }

    function getAnchorRoot() external view returns (Hash, uint256) {
        return (anchorRoot, anchorL2BlockNumber);
    }

    function getStartingAnchorRoot() external view returns (Proposal memory) {
        return Proposal({ root: anchorRoot, l2SequenceNumber: anchorL2BlockNumber });
    }

    function disputeGameFactory() external view returns (IDisputeGameFactory) {
        return IDisputeGameFactory(factory);
    }

    function setRespectedGameType(GameType gameType) external {
        respectedGameType = gameType;
    }

    function setAnchorState(Hash newAnchorRoot, uint256 newAnchorL2BlockNumber) external {
        anchorRoot = newAnchorRoot;
        anchorL2BlockNumber = newAnchorL2BlockNumber;
    }

    function isGameRegistered(IDisputeGame) external pure returns (bool) {
        return true;
    }

    function isGameBlacklisted(IDisputeGame) external pure returns (bool) {
        return false;
    }

    function isGameRetired(IDisputeGame) external pure returns (bool) {
        return false;
    }

    function isGameRespected(IDisputeGame) external pure returns (bool) {
        return true;
    }

    function isGameProper(IDisputeGame) external pure returns (bool) {
        return true;
    }

    function paused() external pure returns (bool) {
        return false;
    }

    /// @dev Mirrors the real AnchorStateRegistry: requires the game to be resolved.
    ///      Skips the airgap delay check since this is a dev mock without a finality delay.
    function isGameFinalized(IDisputeGame _game) external view returns (bool) {
        return _game.resolvedAt().raw() != 0;
    }

    function setAnchorState(IDisputeGame) external {
        // No-op in mock
    }
}
