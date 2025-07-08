// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import {SafeCall} from "@eth-optimism-bedrock/src/libraries/SafeCall.sol";

/// @title BalanceTracker
///
/// @notice Funds system addresses and sends the remaining profits to the profit wallet.
contract BalanceTracker is ReentrancyGuardUpgradeable {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Constants                                    ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice The maximum number of system addresses that can be funded.
    uint256 public constant MAX_SYSTEM_ADDRESS_COUNT = 20;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Immutables                                   ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice The address of the wallet receiving profits.
    address payable public immutable PROFIT_WALLET;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                    Storage                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice The system addresses being funded.
    address payable[] public systemAddresses;

    /// @notice The target balances for system addresses.
    uint256[] public targetBalances;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                     Events                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Emitted when the BalanceTracker sends funds to a system address.
    ///
    /// @param systemAddress The system address being funded.
    /// @param success       A boolean denoting whether a fund send occurred and its success or failure.
    /// @param balanceNeeded The amount of funds the given system address needs to reach its target balance.
    /// @param balanceSent   The amount of funds sent to the system address.
    event ProcessedFunds(
        address indexed systemAddress, bool indexed success, uint256 balanceNeeded, uint256 balanceSent
    );

    /// @notice Emitted when the BalanceTracker attempts to send funds to the profit wallet.
    ///
    /// @param profitWallet The address of the profit wallet.
    /// @param success      A boolean denoting the success or failure of fund send.
    /// @param balanceSent  The amount of funds sent to the profit wallet.
    event SentProfit(address indexed profitWallet, bool indexed success, uint256 balanceSent);

    /// @notice Emitted when funds are received.
    ///
    /// @param sender The address sending funds.
    /// @param amount The amount of funds received from the sender.
    event ReceivedFunds(address indexed sender, uint256 amount);

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                  Constructor                                   ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @dev Constructor for the BalanceTracker contract that sets an immutable variable.
    ///
    /// @param profitWallet The address to send remaining ETH profits to.
    constructor(address payable profitWallet) {
        require(profitWallet != address(0), "BalanceTracker: PROFIT_WALLET cannot be address(0)");

        PROFIT_WALLET = profitWallet;

        _disableInitializers();
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               External Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Fallback function to receive funds from L2 fee withdrawals and additional top up funds if L2 fees are
    ///         insufficient to fund L1 system addresses.
    receive() external payable {
        emit ReceivedFunds({sender: msg.sender, amount: msg.value});
    }

    /// @notice Initializes the BalanceTracker contract.
    ///
    /// @param systemAddresses_ The system addresses being funded.
    /// @param targetBalances_  The target balances for system addresses.
    function initialize(address payable[] memory systemAddresses_, uint256[] memory targetBalances_)
        external
        reinitializer(2)
    {
        uint256 systemAddressesLength = systemAddresses_.length;
        require(systemAddressesLength > 0, "BalanceTracker: systemAddresses cannot have a length of zero");
        require(
            systemAddressesLength <= MAX_SYSTEM_ADDRESS_COUNT,
            "BalanceTracker: systemAddresses cannot have a length greater than 20"
        );
        require(
            systemAddressesLength == targetBalances_.length,
            "BalanceTracker: systemAddresses and targetBalances length must be equal"
        );
        for (uint256 i; i < systemAddressesLength;) {
            require(systemAddresses_[i] != address(0), "BalanceTracker: systemAddresses cannot contain address(0)");
            require(targetBalances_[i] > 0, "BalanceTracker: targetBalances cannot contain 0 target");
            unchecked {
                i++;
            }
        }

        systemAddresses = systemAddresses_;
        targetBalances = targetBalances_;

        __ReentrancyGuard_init();
    }

    /// @notice Funds system addresses and sends remaining profits to the profit wallet.
    function processFees() external nonReentrant {
        uint256 systemAddressesLength = systemAddresses.length;
        require(systemAddressesLength > 0, "BalanceTracker: systemAddresses cannot have a length of zero");
        // Refills balances of systems addresses up to their target balances
        for (uint256 i; i < systemAddressesLength;) {
            _refillBalanceIfNeeded({systemAddress: systemAddresses[i], targetBalance: targetBalances[i]});
            unchecked {
                i++;
            }
        }

        // Send remaining profits to profit wallet
        uint256 valueToSend = address(this).balance;
        bool success = SafeCall.send({_target: PROFIT_WALLET, _gas: gasleft(), _value: valueToSend});
        emit SentProfit({profitWallet: PROFIT_WALLET, success: success, balanceSent: valueToSend});
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Internal Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Checks the balance of the target address and refills it back up to the target balance if needed.
    ///
    /// @param systemAddress The system address being funded.
    /// @param targetBalance The target balance for the system address being funded.
    function _refillBalanceIfNeeded(address systemAddress, uint256 targetBalance) internal {
        uint256 systemAddressBalance = systemAddress.balance;
        if (systemAddressBalance >= targetBalance) {
            emit ProcessedFunds({systemAddress: systemAddress, success: false, balanceNeeded: 0, balanceSent: 0});
            return;
        }

        uint256 valueNeeded = targetBalance - systemAddressBalance;
        uint256 balanceTrackerBalance = address(this).balance;
        uint256 valueToSend = valueNeeded > balanceTrackerBalance ? balanceTrackerBalance : valueNeeded;

        bool success = SafeCall.send({_target: systemAddress, _gas: gasleft(), _value: valueToSend});
        emit ProcessedFunds({
            systemAddress: systemAddress,
            success: success,
            balanceNeeded: valueNeeded,
            balanceSent: valueToSend
        });
    }
}
