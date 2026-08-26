// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

library ProtocolVersionsDeployUtils {
    /// @dev Must match ProtocolVersions.MIN_NOTICE.
    uint64 internal constant MIN_NOTICE = 1 hours;
    /// @dev Reserves a full notice window for sequential deployment transactions to be mined before initialization.
    uint64 internal constant DEPLOYMENT_NOTICE_BUFFER = MIN_NOTICE;

    /// @notice Validates imported ProtocolVersions state before a deployment script broadcasts any transactions.
    function assertValidInitialState(uint64[] memory _schedule, uint256 _minimumProtocolVersion) internal view {
        if (_minimumProtocolVersion > type(uint128).max) {
            revert IProtocolVersions.ProtocolVersions_InvalidProtocolVersion();
        }

        uint64 currentTimestamp = uint64(block.timestamp);
        uint64 minimumFutureTimestamp = currentTimestamp + MIN_NOTICE + DEPLOYMENT_NOTICE_BUFFER;
        uint256 previousId;
        uint64 previousTimestamp;
        for (uint256 id = 0; id < _schedule.length; id++) {
            uint64 timestamp = _schedule[id];
            if (timestamp != 0 && _minimumProtocolVersion == 0) {
                revert IProtocolVersions.ProtocolVersions_InvalidProtocolVersion();
            }
            if (timestamp > currentTimestamp && timestamp < minimumFutureTimestamp) {
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
