// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";

/// @title Recovery
///
/// @notice A utility contract for recovering funds from addresses controlled by signers owned by Coinbase.
///
/// @dev This contract provides a flexible mechanism to recover ETH (and potentially other assets in future
///      upgrades) that have been sent to addresses where Coinbase has control over the deployment nonce.
///      The contract is designed as UUPS upgradeable to allow adaptation to various recovery scenarios,
///      such as recovering different asset types or implementing new recovery patterns as needs evolve.
contract Recovery is UUPSUpgradeable {
    /// @dev The address of the owner.
    address public immutable OWNER;

    /// @dev Error thrown when the caller is not the owner.
    error Unauthorized();

    /// @dev Modifier to check if the caller is the owner.
    modifier onlyOwner() {
        if (msg.sender != OWNER) revert Unauthorized();
        _;
    }

    /// @notice Constructor.
    ///
    /// @param owner The address of the owner.
    constructor(address owner) {
        OWNER = owner;
    }

    /// @inheritdoc UUPSUpgradeable
    function _authorizeUpgrade(address) internal view override onlyOwner { }

    /// @notice Withdraw ETH from the contract.
    ///
    /// @dev This function is only callable by the owner.
    ///
    /// @param targets The addresses to send the ETH to.
    /// @param amounts The amounts of ETH to send to each address.
    function withdrawETH(address[] calldata targets, uint256[] calldata amounts) public onlyOwner {
        for (uint256 i = 0; i < targets.length; i++) {
            (bool success,) = targets[i].call{ value: amounts[i] }("");
            if (!success) continue;
        }
    }
}
