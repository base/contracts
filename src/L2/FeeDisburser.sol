// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { IL2StandardBridge } from "interfaces/L2/IL2StandardBridge.sol";
import { IFeeVault, Types } from "interfaces/L2/IFeeVault.sol";
import { ISemver } from "interfaces/universal/ISemver.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { SafeCall } from "src/libraries/SafeCall.sol";
import { Initializable } from "src/vendor/Initializable.sol";
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";

/// @custom:proxied true
/// @title FeeDisburser
/// @notice Withdraws funds from system FeeVault contracts and bridges to L1.
contract FeeDisburser is Initializable, ProxyAdminOwnedBase, ISemver {
    ////////////////////////////////////////////////////////////////
    ///                     Constants
    ////////////////////////////////////////////////////////////////

    /// @notice The minimum gas limit for the FeeDisburser withdrawal transaction to L1.
    uint32 public constant WITHDRAWAL_MIN_GAS = 35_000;

    /// @notice The maximum number of system addresses that can be funded.
    uint256 public constant MAX_SYSTEM_ADDRESS_COUNT = 20;

    ////////////////////////////////////////////////////////////////
    ///                     Immutables
    ////////////////////////////////////////////////////////////////

    /// @notice The address of the L1 wallet that will receive the OP chain runner's share of fees.
    address public immutable L1_WALLET;

    /// @notice The minimum amount of time in seconds that must pass between fee disbursals.
    uint256 public immutable FEE_DISBURSEMENT_INTERVAL;

    ////////////////////////////////////////////////////////////////
    ///                     Storage
    ////////////////////////////////////////////////////////////////

    /// @notice The timestamp of the last disbursal.
    uint256 public lastDisbursementTime;

    /// @custom:legacy
    /// @notice Previously tracked the aggregate net fee revenue (sum of sequencer and base fees).
    ///         This variable is deprecated and its value should not be relied upon.
    uint256 public netFeeRevenue;

    /// @notice Reentrancy guard status for disburseFees.
    uint256 private _disburseFeesEntered;

    /// @notice The L2 system addresses being funded.
    address payable[] public systemAddresses;

    /// @notice The target balances for L2 system addresses.
    uint256[] public targetBalances;

    ////////////////////////////////////////////////////////////////
    ///                       Events
    ////////////////////////////////////////////////////////////////

    /// @notice Emitted when fees are disbursed.
    ///
    /// @param disbursementTime The time of the disbursement.
    /// @param deprecated This parameter is deprecated and will always be 0.
    /// @param totalFeesDisbursed The total amount of fees disbursed.
    event FeesDisbursed(uint256 disbursementTime, uint256 deprecated, uint256 totalFeesDisbursed);

    /// @notice Emitted when fees are received from FeeVaults.
    ///
    /// @param sender The FeeVault that sent the fees.
    /// @param amount The amount of fees received.
    event FeesReceived(address indexed sender, uint256 amount);

    /// @notice Emitted when no fees are collected from FeeVaults at time of disbursement.
    event NoFeesCollected();

    /// @notice Emitted when the FeeDisburser sends funds to a system address.
    ///
    /// @param systemAddress The system address being funded.
    /// @param success       A boolean denoting whether a fund send occurred and its success or failure.
    /// @param balanceNeeded The amount of funds the given system address needs to reach its target balance.
    /// @param balanceSent   The amount of funds attempted to be sent. When success is false, the
    ///                      recipient rejected the transfer and no funds were actually received.
    event ProcessedFunds(
        address indexed systemAddress, bool indexed success, uint256 balanceNeeded, uint256 balanceSent
    );

    /// @notice Emitted when the L2 system address refund configuration is updated.
    ///
    /// @param systemAddressCount The number of configured system addresses.
    event SystemAddressesUpdated(uint256 systemAddressCount);

    ////////////////////////////////////////////////////////////////
    ///                        Errors
    ////////////////////////////////////////////////////////////////

    /// @notice Thrown when the L1 wallet address is the zero address.
    error ZeroAddress();

    /// @notice Thrown when the fee disbursement interval is less than 24 hours.
    error IntervalTooLow();

    /// @notice Thrown when disburseFees is called before the disbursement interval has passed.
    error IntervalNotReached();

    /// @notice Thrown when a FeeVault's withdrawal network is not set to L2.
    error FeeVaultMustWithdrawToL2();

    /// @notice Thrown when a FeeVault's recipient is not set to the FeeDisburser contract.
    error FeeVaultMustWithdrawToFeeDisburser();

    /// @notice Thrown when system address and target balance array lengths do not match.
    error ArrayLengthMismatch();

    /// @notice Thrown when system address configuration exceeds the maximum length.
    error TooManySystemAddresses();

    /// @notice Thrown when a system address target balance is zero.
    error ZeroTargetBalance();

    /// @notice Thrown when disburseFees is reentered.
    error ReentrantCall();

    ////////////////////////////////////////////////////////////////
    ///                       Modifiers
    ////////////////////////////////////////////////////////////////

    /// @notice Prevents reentrancy into disburseFees while preserving the existing storage layout.
    ///         Uses 1/2 sentinel values so the slot stays nonzero after first use, keeping
    ///         subsequent SSTOREs warm. Uninitialized (0) is treated as not-entered.
    modifier nonReentrantDisbursement() {
        if (_disburseFeesEntered == 2) revert ReentrantCall();
        _disburseFeesEntered = 2;
        _;
        _disburseFeesEntered = 1;
    }

    ////////////////////////////////////////////////////////////////
    ///                     Constructor
    ////////////////////////////////////////////////////////////////

    /// @notice Constructor for the FeeDisburser contract which validates and sets immutable variables.
    ///
    /// @param l1Wallet                The L1 address which receives the remainder of the revenue.
    /// @param feeDisbursementInterval The minimum amount of time in seconds that must pass between fee disbursals.
    constructor(address l1Wallet, uint256 feeDisbursementInterval) {
        if (l1Wallet == address(0)) revert ZeroAddress();
        if (feeDisbursementInterval < 24 hours) revert IntervalTooLow();

        L1_WALLET = l1Wallet;
        FEE_DISBURSEMENT_INTERVAL = feeDisbursementInterval;
    }

    ////////////////////////////////////////////////////////////////
    ///                     External Functions
    ////////////////////////////////////////////////////////////////

    /// @notice Withdraws funds from FeeVaults and bridges to L1.
    function disburseFees() external virtual nonReentrantDisbursement {
        if (block.timestamp < lastDisbursementTime + FEE_DISBURSEMENT_INTERVAL) revert IntervalNotReached();

        // Sequencer, base, and L1 FeeVaults will withdraw fees to the FeeDisburser contract.
        _feeVaultWithdrawal(payable(Predeploys.SEQUENCER_FEE_WALLET));
        _feeVaultWithdrawal(payable(Predeploys.BASE_FEE_VAULT));
        _feeVaultWithdrawal(payable(Predeploys.L1_FEE_VAULT));
        // Note: OPERATOR_FEE_VAULT is intentionally omitted because Base does not currently use it.

        // Gross revenue is the sum of all fees
        uint256 feeBalance = address(this).balance;

        // Stop execution if no fees were collected
        if (feeBalance == 0) {
            emit NoFeesCollected();
            return;
        }

        uint256 disbursementTime = block.timestamp;
        lastDisbursementTime = disbursementTime;

        uint256 systemAddressesLength = systemAddresses.length;
        for (uint256 i; i < systemAddressesLength;) {
            _refillBalanceIfNeeded({ systemAddress: systemAddresses[i], targetBalance: targetBalances[i] });
            unchecked {
                i++;
            }
        }

        uint256 bridgeBalance = address(this).balance;
        if (bridgeBalance == 0) {
            emit FeesDisbursed(disbursementTime, 0, 0);
            return;
        }

        // Send remaining funds to L1 wallet on L1
        IL2StandardBridge(payable(Predeploys.L2_STANDARD_BRIDGE)).bridgeETHTo{ value: bridgeBalance }(
            L1_WALLET, WITHDRAWAL_MIN_GAS, bytes("")
        );

        emit FeesDisbursed(disbursementTime, 0, bridgeBalance);
    }

    /// @notice Configures the L2 system addresses to refund and their target balances.
    ///         Called via upgradeAndCall when upgrading to this version.
    ///
    /// @param systemAddresses_ The system addresses being funded.
    /// @param targetBalances_  The target balances for system addresses.
    function initialize(
        address payable[] memory systemAddresses_,
        uint256[] memory targetBalances_
    )
        external
        reinitializer(2)
    {
        uint256 systemAddressesLength = systemAddresses_.length;
        if (systemAddressesLength > MAX_SYSTEM_ADDRESS_COUNT) revert TooManySystemAddresses();
        if (systemAddressesLength != targetBalances_.length) revert ArrayLengthMismatch();

        for (uint256 i; i < systemAddressesLength;) {
            if (systemAddresses_[i] == address(0)) revert ZeroAddress();
            if (targetBalances_[i] == 0) revert ZeroTargetBalance();
            unchecked {
                i++;
            }
        }

        systemAddresses = systemAddresses_;
        targetBalances = targetBalances_;

        emit SystemAddressesUpdated(systemAddressesLength);
    }

    /// @notice Receives ETH fees withdrawn from L2 FeeVaults.
    receive() external payable virtual {
        emit FeesReceived(msg.sender, msg.value);
    }

    /// @custom:semver 1.1.0
    function version() external pure virtual returns (string memory) {
        return "1.1.0";
    }

    ////////////////////////////////////////////////////////////////
    ///                     Internal Functions
    ////////////////////////////////////////////////////////////////

    /// @notice Checks the balance of the target address and refills it back up to the target balance if needed.
    ///
    /// @param systemAddress The system address being funded.
    /// @param targetBalance The target balance for the system address being funded.
    function _refillBalanceIfNeeded(address systemAddress, uint256 targetBalance) internal {
        uint256 systemAddressBalance = systemAddress.balance;
        if (systemAddressBalance >= targetBalance) {
            emit ProcessedFunds({ systemAddress: systemAddress, success: false, balanceNeeded: 0, balanceSent: 0 });
            return;
        }

        uint256 valueNeeded = targetBalance - systemAddressBalance;
        uint256 feeDisburserBalance = address(this).balance;
        uint256 valueToSend = valueNeeded > feeDisburserBalance ? feeDisburserBalance : valueNeeded;

        bool success = SafeCall.send({ _target: systemAddress, _gas: gasleft(), _value: valueToSend });
        emit ProcessedFunds({
            systemAddress: systemAddress, success: success, balanceNeeded: valueNeeded, balanceSent: valueToSend
        });
    }

    ////////////////////////////////////////////////////////////////
    ///                     Private Functions
    ////////////////////////////////////////////////////////////////

    /// @notice Withdraws fees from a FeeVault.
    ///
    /// @dev Withdrawal will only occur if the given FeeVault's balance is greater than or equal to the minimum
    ///      withdrawal amount.
    ///
    /// @param feeVault The address of the FeeVault to withdraw from.
    function _feeVaultWithdrawal(address payable feeVault) private {
        if (IFeeVault(feeVault).WITHDRAWAL_NETWORK() != Types.WithdrawalNetwork.L2) revert FeeVaultMustWithdrawToL2();
        if (IFeeVault(feeVault).RECIPIENT() != address(this)) revert FeeVaultMustWithdrawToFeeDisburser();

        if (feeVault.balance >= IFeeVault(feeVault).MIN_WITHDRAWAL_AMOUNT()) {
            IFeeVault(feeVault).withdraw();
        }
    }
}
