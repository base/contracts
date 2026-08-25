// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Interfaces
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

/// @title ProtocolVersionsConfig
/// @notice Shared configuration and validation for ProtocolVersions initial schedules.
library ProtocolVersionsConfig {
    /// @notice Number of contract-backed upgrades in the node's positional schedule, from Regolith through Denim.
    /// @dev This must match `BaseUpgrade::CONTRACT_VARIANTS` in base/base and be updated only by appending upgrades.
    uint256 internal constant INITIAL_UPGRADE_COUNT = 14;

    /// @notice Rejects non-empty initial schedules that do not cover the complete known upgrade ladder.
    function assertValidInitialScheduleLength(uint256 _length) internal pure {
        if (_length != 0 && _length != INITIAL_UPGRADE_COUNT) {
            revert IProtocolVersions.ProtocolVersions_InvalidInitialScheduleLength(_length, INITIAL_UPGRADE_COUNT);
        }
    }
}
