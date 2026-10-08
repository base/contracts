// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Interfaces
import { ISemver } from "interfaces/universal/ISemver.sol";

/// @title IBaseTime
/// @notice Interface for the BaseTime predeploy.
interface IBaseTime is ISemver {
    /// @notice Thrown when a caller other than the protocol depositor attempts to update BaseTime.
    error BaseTime_NotDepositor();

    /// @notice Thrown when the millisecond component is above 800 or not a multiple of 200.
    error BaseTime_InvalidTimestampMillisPart();

    /// @notice Returns the millisecond component of the current L2 block timestamp.
    ///
    /// @return The millisecond component, one of 0, 200, 400, 600, or 800.
    function timestampMillisPart() external view returns (uint16);

    /// @notice Returns the current L2 block timestamp in milliseconds.
    ///
    /// @dev Until the block's `tx[1]` BaseTime update deposit executes (e.g. during the L1-info deposit at `tx[0]`),
    ///      combines the current block's seconds with the previous block's millisecond component.
    ///
    /// @return The current L2 block timestamp in milliseconds.
    function timestampMs() external view returns (uint64);

    /// @notice Updates the millisecond component of the current L2 block timestamp.
    ///
    /// @dev Reverts with `BaseTime_NotDepositor` when the caller is not the depositor account.
    /// @dev Reverts with `BaseTime_InvalidTimestampMillisPart` when `_timestampMillisPart` is above 800 or not a
    ///      multiple of 200.
    ///
    /// @param _timestampMillisPart The millisecond component of the current L2 block timestamp.
    function setTimestampMillisPart(uint16 _timestampMillisPart) external;
}
