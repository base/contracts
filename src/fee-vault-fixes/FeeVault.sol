// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// solhint-disable max-line-length

/// @title FeeVault
/// @notice The FeeVault contract is intended to:
///         1. Be upgraded to by the Base FeeVault contracts
///         2. Set `totalProcessed` to the correct value
///         3. Be upgraded from to back to Optimism's FeeVault
///
/// This contract was only used as an intermediate step in a script sequence to fix our existing FeeVault contract back
/// in August 2023. We are leaving this contract here for historical purposes.
///
/// See the upgrade script in which it was used here:
///
/// https://github.com/base/contract-deployments/tree/e37887061eb02f9f0d93000c91d4a765bafcaf2b/mainnet/2023-08-22-fee-vault-fix
contract FeeVault {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                                    Storage                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Total amount of wei processed by the contract.
    uint256 public totalProcessed;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               External Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Allow the contract to receive ETH.
    receive() external payable {}

    /// @notice Sets total processed to its correct value.
    ///
    /// @param correctTotalProcessed The correct total processed value.
    function setTotalProcessed(uint256 correctTotalProcessed) external {
        totalProcessed = correctTotalProcessed;
    }
}
