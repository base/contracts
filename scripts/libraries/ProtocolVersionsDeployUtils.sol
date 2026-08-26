// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

library ProtocolVersionsDeployUtils {
    /// @dev Must match ProtocolVersions.MIN_NOTICE.
    uint64 internal constant MIN_NOTICE = 1 hours;

    /// @notice Validates imported ProtocolVersions state before a deployment script broadcasts any transactions.
    function assertValidInitialState(uint64[] memory _schedule, uint256 _minimumProtocolVersion) internal view {
        if (_minimumProtocolVersion > type(uint128).max) {
            revert IProtocolVersions.ProtocolVersions_InvalidProtocolVersion();
        }

        uint256 previousId;
        uint64 previousTimestamp;
        for (uint256 id = 0; id < _schedule.length; id++) {
            uint64 timestamp = _schedule[id];
            if (timestamp != 0 && _minimumProtocolVersion == 0) {
                revert IProtocolVersions.ProtocolVersions_InvalidProtocolVersion();
            }
            if (timestamp > uint64(block.timestamp) && timestamp < uint64(block.timestamp) + MIN_NOTICE) {
                revert IProtocolVersions.ProtocolVersions_InsufficientNotice(timestamp);
            }
            if (timestamp == 0) continue;
            if (previousTimestamp != 0 && timestamp < previousTimestamp) {
                revert IProtocolVersions.ProtocolVersions_TimestampNotAfterPrevious(
                    id, previousId, previousTimestamp, timestamp
                );
            }
            previousId = id;
            previousTimestamp = timestamp;
        }
    }
}
