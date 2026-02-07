// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {L2StandardBridge} from "lib/optimism/packages/contracts-bedrock/src/L2/L2StandardBridge.sol";
import {Predeploys} from "lib/optimism/packages/contracts-bedrock/src/libraries/Predeploys.sol";
import {SafeCall} from "lib/optimism/packages/contracts-bedrock/src/libraries/SafeCall.sol";
import {FeeVault} from "lib/optimism/packages/contracts-bedrock/src/universal/FeeVault.sol";

/// @title FeeDisburser
///
/// @notice Withdraws funds from system FeeVault contracts, shares revenue with Optimism, and bridges the rest of funds
///         to L1.
contract FeeDisburser {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Constants                                    ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice The basis point scale which revenue share splits are denominated in.
    uint32 public constant BASIS_POINT_SCALE = 10_000;

    /// @notice The minimum gas limit for the FeeDisburser withdrawal transaction to L1.
    uint32 public constant WITHDRAWAL_MIN_GAS = 35_000;

    /// @notice The net revenue percentage denominated in basis points that is used in Optimism revenue share
    ///         calculation.
    uint256 public constant OPTIMISM_NET_REVENUE_SHARE_BASIS_POINTS = 1_500;

    /// @notice The gross revenue percentage denominated in basis points that is used in Optimism revenue share
    ///         calculation.
    uint256 public constant OPTIMISM_GROSS_REVENUE_SHARE_BASIS_POINTS = 250;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Immutables                                   ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice The address of the Optimism wallet that will receive Optimism's revenue share.
    address payable public immutable OPTIMISM_WALLET;

    /// @notice The address of the L1 wallet that will receive the OP chain runner's share of fees.
    address public immutable L1_WALLET;

    /// @notice The minimum amount of time in seconds that must pass between fee disbursals.
    uint256 public immutable FEE_DISBURSEMENT_INTERVAL;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                    Storage                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice The timestamp of the last disbursal.
    uint256 public lastDisbursementTime;

    /// @notice Tracks aggregate net fee revenue which is the sum of sequencer and base fees.
    ///
    /// @dev Explicitly tracking Net Revenue is required to separate L1FeeVault initiated withdrawals from Net Revenue
    ///      calculations.
    uint256 public netFeeRevenue;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                     Events                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Emitted when fees are disbursed.
    ///
    /// @param disbursementTime   The time of the disbursement.
    /// @param paidToOptimism     The amount of fees disbursed to Optimism.
    /// @param totalFeesDisbursed The total amount of fees disbursed.
    event FeesDisbursed(uint256 disbursementTime, uint256 paidToOptimism, uint256 totalFeesDisbursed);

    /// @notice Emitted when fees are received from FeeVaults.
    ///
    /// @param sender The FeeVault that sent the fees.
    /// @param amount The amount of fees received.
    event FeesReceived(address indexed sender, uint256 amount);

    /// @notice Emitted when no fees are collected from FeeVaults at time of disbursement.
    event NoFeesCollected();

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                  Constructor                                   ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Constructor for the FeeDisburser contract which validates and sets immutable variables.
    ///
    /// @param optimismWallet          The address which receives Optimism's revenue share.
    /// @param l1Wallet                The L1 address which receives the remainder of the revenue.
    /// @param feeDisbursementInterval The minimum amount of time in seconds that must pass between fee disbursals.
    constructor(address payable optimismWallet, address l1Wallet, uint256 feeDisbursementInterval) {
        require(optimismWallet != address(0), "FeeDisburser: OptimismWallet cannot be address(0)");
        require(l1Wallet != address(0), "FeeDisburser: L1Wallet cannot be address(0)");
        require(
            feeDisbursementInterval >= 24 hours, "FeeDisburser: FeeDisbursementInterval cannot be less than 24 hours"
        );

        OPTIMISM_WALLET = optimismWallet;
        L1_WALLET = l1Wallet;
        FEE_DISBURSEMENT_INTERVAL = feeDisbursementInterval;
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               External Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @dev Receives ETH fees withdrawn from L2 FeeVaults.
    /// @dev Will revert if ETH is not sent from L2 FeeVaults.
    receive() external payable virtual {
        if (msg.sender == Predeploys.SEQUENCER_FEE_WALLET || msg.sender == Predeploys.BASE_FEE_VAULT) {
            // Adds value received to net fee revenue if the sender is the sequencer or base FeeVault
            netFeeRevenue += msg.value;
        } else if (msg.sender != Predeploys.L1_FEE_VAULT) {
            revert("FeeDisburser: Only FeeVaults can send ETH to FeeDisburser");
        }
        emit FeesReceived({sender: msg.sender, amount: msg.value});
    }

    /// @notice Withdraws funds from FeeVaults, sends Optimism their revenue share, and withdraws remaining funds to L1.
    ///
    /// @dev Implements revenue share business logic as follows:
    ///          Net Revenue             = sequencer FeeVault fee revenue + base FeeVault fee revenue
    ///          Gross Revenue           = Net Revenue + l1 FeeVault fee revenue
    ///          Optimism Revenue Share  = Maximum of 15% of Net Revenue and 2.5% of Gross Revenue
    ///          L1 Wallet Revenue Share = Gross Revenue - Optimism Revenue Share
    function disburseFees() external virtual {
        require(
            block.timestamp >= lastDisbursementTime + FEE_DISBURSEMENT_INTERVAL,
            "FeeDisburser: Disbursement interval not reached"
        );

        // Sequencer and base FeeVaults will withdraw fees to the FeeDisburser contract mutating netFeeRevenue
        _feeVaultWithdrawal({feeVault: payable(Predeploys.SEQUENCER_FEE_WALLET)});
        _feeVaultWithdrawal({feeVault: payable(Predeploys.BASE_FEE_VAULT)});

        _feeVaultWithdrawal({feeVault: payable(Predeploys.L1_FEE_VAULT)});

        // Gross revenue is the sum of all fees
        uint256 feeBalance = address(this).balance;

        // Stop execution if no fees were collected
        if (feeBalance == 0) {
            emit NoFeesCollected();
            return;
        }

        lastDisbursementTime = block.timestamp;

        // Net revenue is the sum of sequencer fees and base fees
        uint256 optimismNetRevenueShare = netFeeRevenue * OPTIMISM_NET_REVENUE_SHARE_BASIS_POINTS / BASIS_POINT_SCALE;
        netFeeRevenue = 0;

        uint256 optimismGrossRevenueShare = feeBalance * OPTIMISM_GROSS_REVENUE_SHARE_BASIS_POINTS / BASIS_POINT_SCALE;

        // Optimism's revenue share is the maximum of net and gross revenue
        uint256 optimismRevenueShare = Math.max({a: optimismNetRevenueShare, b: optimismGrossRevenueShare});

        // Send Optimism their revenue share on L2
        require(
            SafeCall.send({_target: OPTIMISM_WALLET, _gas: gasleft(), _value: optimismRevenueShare}),
            "FeeDisburser: Failed to send funds to Optimism"
        );

        // Send remaining funds to L1 wallet on L1
        L2StandardBridge(payable(Predeploys.L2_STANDARD_BRIDGE)).bridgeETHTo{value: address(this).balance}({
            _to: L1_WALLET, _minGasLimit: WITHDRAWAL_MIN_GAS, _extraData: bytes("")
        });
        emit FeesDisbursed({
            disbursementTime: lastDisbursementTime, paidToOptimism: optimismRevenueShare, totalFeesDisbursed: feeBalance
        });
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Internal Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Withdraws fees from a FeeVault.
    ///
    /// @dev Withdrawal will only occur if the given FeeVault's balance is greater than or equal to the minimum
    ///      withdrawal amount.
    ///
    /// @param feeVault The address of the FeeVault to withdraw from.
    function _feeVaultWithdrawal(address payable feeVault) internal {
        require(
            FeeVault(feeVault).WITHDRAWAL_NETWORK() == FeeVault.WithdrawalNetwork.L2,
            "FeeDisburser: FeeVault must withdraw to L2"
        );
        require(
            FeeVault(feeVault).RECIPIENT() == address(this),
            "FeeDisburser: FeeVault must withdraw to FeeDisburser contract"
        );
        if (feeVault.balance >= FeeVault(feeVault).MIN_WITHDRAWAL_AMOUNT()) {
            FeeVault(feeVault).withdraw();
        }
    }
}
