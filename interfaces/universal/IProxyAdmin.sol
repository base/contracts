// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";

/// @title IProxyAdmin
/// @notice Interface for the ProxyAdmin contract that manages proxy upgrades
///         and administrative functions across ERC1967, Chugsplash, and Resolved proxies.
/// @dev Security Note: All implementers of this interface MUST ensure that
///      the `__constructor__` function can only be called ONCE during initialization.
///      Use OpenZeppelin's Initializable pattern with `_disableInitializers()` in
///      the constructor to prevent reinitialization attacks.
///
/// @custom:risk Reinitialization Attack
/// If `__constructor__` is not protected against multiple calls, an attacker
/// could reinitialize the contract after deployment, potentially hijacking ownership.
interface IProxyAdmin {
    enum ProxyType {
        ERC1967,
        CHUGSPLASH,
        RESOLVED
    }

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    function addressManager() external view returns (IAddressManager);
    function changeProxyAdmin(address payable _proxy, address _newAdmin) external;
    function getProxyAdmin(address payable _proxy) external view returns (address);
    function getProxyImplementation(address _proxy) external view returns (address);
    function implementationName(address) external view returns (string memory);
    function isUpgrading() external view returns (bool);
    function owner() external view returns (address);
    function proxyType(address) external view returns (ProxyType);
    function renounceOwnership() external;
    function setAddress(string memory _name, address _address) external;
    function setAddressManager(IAddressManager _address) external;
    function setImplementationName(address _address, string memory _name) external;
    function setProxyType(address _address, ProxyType _type) external;
    function setUpgrading(bool _upgrading) external;
    function transferOwnership(address newOwner) external; // nosemgrep
    function upgrade(address payable _proxy, address _implementation) external;
    function upgradeAndCall(address payable _proxy, address _implementation, bytes memory _data) external payable;

    /// @notice One-time initialization function.
    /// @dev    IMPORTANT: This function MUST be protected so it can only be called once.
    ///         Implementers should use OpenZeppelin's Initializable pattern.
    /// @param _owner The initial owner address for the ProxyAdmin contract.
    /// @custom:risk Calling this function more than once can lead to ownership hijacking.
    function __constructor__(address _owner) external;
}
