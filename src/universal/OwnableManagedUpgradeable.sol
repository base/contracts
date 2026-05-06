// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import { Initializable } from "lib/openzeppelin-contracts-upgradeable/contracts/proxy/utils/Initializable.sol";
import { ContextUpgradeable } from "lib/openzeppelin-contracts-upgradeable/contracts/utils/ContextUpgradeable.sol";

/**
 * @dev Extension of OpenZeppelin's OwnableUpgradeable contract that adds an additional manager role.
 */
abstract contract OwnableManagedUpgradeable is Initializable, ContextUpgradeable {
    address private _owner;
    address private _manager;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event ManagementTransferred(address indexed previousManager, address indexed newManager);

    /**
     * @dev Initializes the contract setting the deployer as the initial owner + manager.
     */
    function __OwnableManaged_init() internal onlyInitializing {
        __OwnableManaged_init_unchained();
    }

    function __OwnableManaged_init_unchained() internal onlyInitializing {
        _transferOwnership(_msgSender());
        _transferManagement(_msgSender());
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    /**
     * @dev Throws if called by any account other than the manager.
     */
    modifier onlyManager() {
        _checkManager();
        _;
    }

    /**
     * @dev Throws if called by any account other than the owner or manager.
     */
    modifier onlyOwnerOrManager() {
        _checkOwnerOrManager();
        _;
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view virtual returns (address) {
        return _owner;
    }

    /**
     * @dev Returns the address of the current manager.
     */
    function manager() public view virtual returns (address) {
        return _manager;
    }

    /**
     * @dev Throws if the sender is not the owner.
     */
    function _checkOwner() internal view virtual {
        require(owner() == _msgSender(), "OwnableManaged: caller is not the owner");
    }

    /**
     * @dev Throws if the sender is not the manager.
     */
    function _checkManager() internal view virtual {
        require(manager() == _msgSender(), "OwnableManaged: caller is not the manager");
    }

    /**
     * @dev Throws if the sender is not the owner or the manager.
     */
    function _checkOwnerOrManager() internal view virtual {
        require(
            owner() == _msgSender() || manager() == _msgSender(),
            "OwnableManaged: caller is not the owner or the manager"
        );
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    function renounceOwnership() public virtual onlyOwner {
        _transferOwnership(address(0));
    }

    /**
     * @dev Leaves the contract without manager. It will not be possible to call
     * `onlyManager` functions anymore. Can only be called by the current owner or manager.
     *
     * NOTE: Renouncing management will leave the contract without an manager,
     * thereby removing any functionality that is only available to the manager.
     */
    function renounceManagement() public virtual onlyOwnerOrManager {
        _transferManagement(address(0));
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        require(newOwner != address(0), "OwnableManaged: new owner is the zero address");
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers management of the contract to a new account (`newManager`).
     * Can only be called by the current owner or manager.
     */
    function transferManagement(address newManager) public virtual onlyOwnerOrManager {
        require(newManager != address(0), "OwnableManaged: new manager is the zero address");
        _transferManagement(newManager);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }

    /**
     * @dev Transfers management of the contract to a new account (`newManager`).
     * Internal function without access restriction.
     */
    function _transferManagement(address newManager) internal virtual {
        address oldManager = _manager;
        _manager = newManager;
        emit ManagementTransferred(oldManager, newManager);
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[48] private __gap;
}
