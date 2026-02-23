// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";

interface ISuperchainConfig is IProxyAdminOwnedBase {
    event Paused(address identifier);
    event Unpaused(address identifier);
    event PauseExtended(address identifier);

    error SuperchainConfig_OnlyGuardian();
    error SuperchainConfig_OnlyGuardianOrIncidentResponder();
    error SuperchainConfig_AlreadyPaused(address identifier);
    error SuperchainConfig_NotAlreadyPaused(address identifier);

    function guardian() external view returns (address);
    function incidentResponder() external view returns (address);
    function pause(address _identifier) external;
    function unpause(address _identifier) external;
    function pausable(address _identifier) external view returns (bool);
    function paused() external view returns (bool);
    function paused(address _identifier) external view returns (bool);
    function expiration(address _identifier) external view returns (uint256);
    function extend(address _identifier) external;
    function version() external view returns (string memory);
    function pauseTimestamps(address) external view returns (uint256);
    function pauseExpiry() external view returns (uint256);

    function __constructor__(address _guardian, address _incidentResponder) external;
}
