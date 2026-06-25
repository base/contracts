// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/// @title MinimalProxyAdmin
/// @notice Minimal contract to satisfy DisputeGameFactory's proxy admin check.
/// @dev ProxyAdminOwnedBase.proxyAdminOwner() reads `owner()` off the proxy admin, so this
///      exposes an `owner` getter set to the expected initializer caller.
contract MinimalProxyAdmin {
    address public owner;

    constructor(address initialOwner) {
        owner = initialOwner;
    }
}
