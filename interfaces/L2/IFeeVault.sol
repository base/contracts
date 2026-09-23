// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { Types } from "src/libraries/Types.sol";

/// @title IFeeVault
/// @notice Interface for the FeeVault contract that manages fee withdrawals
///         and recipient configurations.
/// @dev Security Note: The `initialize` function MUST be protected with OpenZeppelin's
///      `initializer` modifier to prevent reinitialization attacks. The implementation
///      contract MUST be initialized in the constructor using `_disableInitializers()`
///      to prevent front-running attacks on the uninitialized implementation.
///
/// @custom:risk Uninitialized Implementation Front-Running
/// If the implementation contract is not initialized during deployment, an attacker
/// could call `initialize` directly on the implementation, setting their own recipient
/// and withdrawal network. Always call `_disableInitializers()` in the constructor.
interface IFeeVault is IProxyAdminOwnedBase {
    error InvalidInitialization();
    error NotInitializing();

    event Initialized(uint64 version);

    event Withdrawal(uint256 value, address to, address from);
    event Withdrawal(uint256 value, address to, address from, Types.WithdrawalNetwork withdrawalNetwork);
    event MinWithdrawalAmountUpdated(uint256 oldWithdrawalAmount, uint256 newWithdrawalAmount);
    event RecipientUpdated(address oldRecipient, address newRecipient);
    event WithdrawalNetworkUpdated(
        Types.WithdrawalNetwork oldWithdrawalNetwork, Types.WithdrawalNetwork newWithdrawalNetwork
    );

    receive() external payable;

    /// @notice Initializes the FeeVault with recipient, min withdrawal amount, and network.
    /// @dev    IMPORTANT: This function MUST be protected with the `initializer` modifier.
    ///         Only the first call should succeed; subsequent calls must revert.
    /// @param _recipient The address that will receive withdrawn fees.
    /// @param _minWithdrawalAmount Minimum amount required to trigger a withdrawal.
    /// @param _withdrawalNetwork The target network for withdrawals (L1 or L2).
    /// @custom:risk Multiple calls to initialize can hijack the contract's configuration.
    function initialize(
        address _recipient,
        uint256 _minWithdrawalAmount,
        Types.WithdrawalNetwork _withdrawalNetwork
    )
        external;
    function MIN_WITHDRAWAL_AMOUNT() external view returns (uint256);
    function RECIPIENT() external view returns (address);
    function WITHDRAWAL_NETWORK() external view returns (Types.WithdrawalNetwork);
    function minWithdrawalAmount() external view returns (uint256);
    function recipient() external view returns (address);
    function totalProcessed() external view returns (uint256);
    function withdraw() external returns (uint256 value_);
    function withdrawalNetwork() external view returns (Types.WithdrawalNetwork);
    function setMinWithdrawalAmount(uint256 _newMinWithdrawalAmount) external;
    function setRecipient(address _newRecipient) external;
    function setWithdrawalNetwork(Types.WithdrawalNetwork _newWithdrawalNetwork) external;

    /// @notice One-time initialization function for proxy setup.
    /// @dev    IMPORTANT: This function MUST be protected so it can only be called once.
    ///         Implementers should use OpenZeppelin's Initializable pattern.
    /// @custom:risk Reinitialization can lead to configuration hijacking.
    function __constructor__() external;
}
