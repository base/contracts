// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Interfaces
import { ISemver } from "interfaces/universal/ISemver.sol";

/// @title IBaseTime
/// @notice Interface for the BaseTime predeploy.
interface IBaseTime is ISemver {
    /// @notice Thrown when a caller other than the protocol depositor attempts to update BaseTime.
    error BaseTime_NotDepositor();

    /// @notice Thrown when the millisecond component is not aligned to a 200 millisecond interval.
    error BaseTime_InvalidTimestampMillisPart();

    /// @notice Returns the millisecond component of the current L2 block timestamp.
    function timestampMillisPart() external view returns (uint16);

    /// @notice Returns the current L2 block timestamp in milliseconds.
    function timestampMs() external view returns (uint64 timestampMs_);

    /// @notice Updates the millisecond component of the current L2 block timestamp.
    function setTimestampMillisPart(uint16 _timestampMillisPart) external;
}
