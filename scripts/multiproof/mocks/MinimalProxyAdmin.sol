// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/// @title MinimalProxyAdmin
/// @notice Minimal contract to satisfy DisputeGameFactory's proxy admin check.
/// @dev DisputeGameFactory.initialize() requires msg.sender == proxyAdmin() or proxyAdminOwner().
///      We deploy this minimal contract and set it as the proxy admin via vm.store.
contract MinimalProxyAdmin {
    address public owner;

    constructor(address _owner) {
        owner = _owner;
    }
}
