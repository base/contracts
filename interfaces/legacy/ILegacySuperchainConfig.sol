// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ILegacySuperchainConfig {
    function pausable(address _identifier) external view returns (bool);
}

interface ILegacySystemConfig {
    function superchainConfig() external view returns (ILegacySuperchainConfig);
}
