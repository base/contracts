// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {GameType, Hash} from "optimism/src/dispute/lib/Types.sol";


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

    /// @notice Initializes the mock registry.
    /// @param _factory The dispute game factory address.
    /// @param _anchorRoot The initial anchor root.
    /// @param _anchorL2BlockNumber The initial anchor L2 block number.
    /// @param _gameType The respected game type.
    function initialize(address _factory, Hash _anchorRoot, uint256 _anchorL2BlockNumber, GameType _gameType) external {
        factory = _factory;
        anchorRoot = _anchorRoot;
        anchorL2BlockNumber = _anchorL2BlockNumber;
        respectedGameType = _gameType;
    }

    /// @notice Returns the anchor root and block number.
    /// @return The anchor root hash and L2 block number.
    function getAnchorRoot() external view returns (Hash, uint256) {
        return (anchorRoot, anchorL2BlockNumber);
    }

    /// @notice Returns the dispute game factory address.
    /// @return The factory address.
    function disputeGameFactory() external view returns (address) {
        return factory;
    }

    /// @notice Sets the respected game type.
    /// @param _gameType The new game type.
    function setRespectedGameType(GameType _gameType) external {
        respectedGameType = _gameType;
    }

    /// @notice Updates the anchor state (for testing purposes).
    /// @param _anchorRoot The new anchor root.
    /// @param _anchorL2BlockNumber The new anchor L2 block number.
    function setAnchorState(Hash _anchorRoot, uint256 _anchorL2BlockNumber) external {
        anchorRoot = _anchorRoot;
        anchorL2BlockNumber = _anchorL2BlockNumber;
    }

    /// @notice Checks if a game is registered.
    /// @return Always returns true for testing.
    function isGameRegistered(IDisputeGame) external pure returns (bool) {
        return true;
    }

    /// @notice Checks if a game is blacklisted.
    /// @return Always returns false for testing.
    function isGameBlacklisted(IDisputeGame) external pure returns (bool) {
        return false;
    }

    /// @notice Checks if a game is retired.
    /// @return Always returns false for testing.
    function isGameRetired(IDisputeGame) external pure returns (bool) {
        return false;
    }

    /// @notice Checks if a game is respected.
    /// @return Always returns true for testing.
    function isGameRespected(IDisputeGame) external pure returns (bool) {
        return true;
    }
}
