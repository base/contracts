// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ISemver } from "interfaces/universal/ISemver.sol";

interface IProtocolVersions is ISemver {
    function l2ChainId() external view returns (uint256);
    function chainTeam() external view returns (address);
    function scheduleId() external view returns (bytes32);
    function lastUpdatedAtBlock() external view returns (uint256);
    function upgradeCount() external view returns (uint256);
    function upgradeIdAt(uint256 index) external view returns (bytes32);
    function upgradeIds() external view returns (bytes32[] memory);
    function getTimestamp(string calldata upgradeId) external view returns (uint256);
    function getProtocolVersion(string calldata upgradeId) external view returns (uint256);

    function registerUpgrade(string calldata upgradeId, uint256 protocolVersion) external;
    function setTimestamp(string calldata upgradeId, uint64 timestamp) external;
    function setChainTeam(address newChainTeam) external;
    function delayTimestamp(string calldata upgradeId, uint64 newTimestamp) external;
}
