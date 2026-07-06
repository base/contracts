// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ISemver } from "interfaces/universal/ISemver.sol";

/// @title IProtocolVersions
/// @notice Interface for the ProtocolVersions upgrade schedule contract.
interface IProtocolVersions is ISemver {
    struct Upgrade {
        string name;
        uint64 timestamp;
        bytes32 scheduleId;
    }

    error ProtocolVersions_InvalidL2ChainId();
    error ProtocolVersions_UnknownUpgradeName(string upgradeId);
    error ProtocolVersions_UpgradeAlreadyRegistered(bytes32 key);
    error ProtocolVersions_InvalidUpgradeId();
    error ProtocolVersions_InvalidProtocolVersion();
    error ProtocolVersions_ActivationAlreadyPassed(bytes32 key, uint64 activationTimestamp);
    error ProtocolVersions_ActivationTimestampInPast(uint64 timestamp);
    error ProtocolVersions_NotChainTeam();
    error ProtocolVersions_NotScheduled(bytes32 key);
    error ProtocolVersions_DelayMustBeLater(uint64 currentTimestamp, uint64 newTimestamp);

    event UpgradeRegistered(bytes32 indexed key, uint256 indexed index, string upgradeId, uint256 protocolVersion);
    event TimestampSet(bytes32 indexed key, uint256 timestamp);

    event ChainTeamUpdated(address indexed previousChainTeam, address indexed newChainTeam);

    function MIN_NOTICE() external view returns (uint64);

    function chainTeam() external view returns (address);
    function scheduleId() external view returns (bytes32);
    function latestProtocolVersion() external view returns (uint256);

    function getTimestamp(string calldata upgradeId) external view returns (uint256);

    function getSchedule() external view returns (Upgrade[] memory);

    function registerUpgrade(string calldata upgradeId, uint256 protocolVersion) external;
    function setTimestamp(string calldata upgradeId, uint64 timestamp) external;
    function setChainTeam(address newChainTeam) external;
    function delayTimestamp(string calldata upgradeId, uint64 newTimestamp) external;
}
