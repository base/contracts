// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Libraries
import { Constants } from "src/libraries/Constants.sol";

// Interfaces
import { IBaseTime } from "interfaces/L2/IBaseTime.sol";

/// @custom:proxied true
/// @custom:predeploy 0x4200000000000000000000000000000000000030
/// @title BaseTime
/// @notice Exposes the millisecond component of the current L2 block timestamp.
contract BaseTime is IBaseTime {
    /// @notice Semantic version.
    /// @custom:semver 1.0.0
    string public constant version = "1.0.0";

    /// @inheritdoc IBaseTime
    uint16 public timestampMillisPart;

    /// @inheritdoc IBaseTime
    function timestampMs() external view returns (uint64 timestampMs_) {
        timestampMs_ = uint64(block.timestamp * 1000 + timestampMillisPart);
    }

    /// @inheritdoc IBaseTime
    function setTimestampMillisPart(uint16 _timestampMillisPart) external {
        if (msg.sender != Constants.DEPOSITOR_ACCOUNT) revert BaseTime_NotDepositor();
        if (_timestampMillisPart > 800 || _timestampMillisPart % 200 != 0) {
            revert BaseTime_InvalidTimestampMillisPart();
        }

        timestampMillisPart = _timestampMillisPart;
    }
}
