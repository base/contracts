// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ISemver } from "interfaces/universal/ISemver.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { IReinitializableBase } from "interfaces/universal/IReinitializableBase.sol";

/// @title IProtocolVersions
/// @notice Interface for the ProtocolVersions upgrade schedule contract.
interface IProtocolVersions is IProxyAdminOwnedBase, ISemver, IReinitializableBase {
    error ProtocolVersions_UnknownUpgrade(uint256 id);
    error ProtocolVersions_InvalidProtocolVersion();
    error ProtocolVersions_ActivationAlreadyPassed(uint256 id, uint64 activationTimestamp);
    error ProtocolVersions_NotIncidentResponder();
    error ProtocolVersions_NotScheduled(uint256 id);
    error ProtocolVersions_DelayMustBeLater(uint64 currentTimestamp, uint64 newTimestamp);
    error ProtocolVersions_NotInitialized();
    error ProtocolVersions_InsufficientNotice(uint64 timestamp);

    event UpgradeRegistered(uint256 indexed id);
    event MinimumProtocolVersionUpdated(uint256 indexed protocolVersion);
    event TimestampSet(uint256 indexed id, uint256 timestamp);
    event ScheduleIdUpdated(bytes32 indexed newScheduleId);
    event IncidentResponderUpdated(address indexed previousIncidentResponder, address indexed newIncidentResponder);
    event Initialized(uint8 version);

    function MIN_NOTICE() external view returns (uint64);

    function incidentResponder() external view returns (address);
    function scheduleId() external view returns (bytes32);
    function scheduleId(uint256 id) external view returns (bytes32);
    function minimumProtocolVersion() external view returns (uint256);

    function getSchedule() external view returns (uint64[] memory);

    function initialize(address _incidentResponder) external;
    function registerUpgrade(uint64 timestamp, uint256 minProtocolVersion) external returns (uint256);
    function setMinimumProtocolVersion(uint256 protocolVersion) external;
    function setTimestamp(uint256 id, uint64 timestamp) external;
    function setIncidentResponder(address newIncidentResponder) external;
    function delayTimestamp(uint256 id, uint64 newTimestamp) external;

    function __constructor__() external;
}
