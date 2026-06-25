// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { Test } from "lib/forge-std/src/Test.sol";

import { FeeDisburser } from "src/L2/FeeDisburser.sol";
import { IFeeVault, Types } from "interfaces/L2/IFeeVault.sol";
import { IStandardBridge } from "interfaces/universal/IStandardBridge.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";

/// @title FeeDisburserTest
/// @notice Comprehensive unit and fuzz tests for the FeeDisburser contract
contract FeeDisburserTest is Test {
    // Events (must match FeeDisburser.sol)
    event FeesDisbursed(uint256 disbursementTime, uint256 deprecated, uint256 totalFeesDisbursed);
    event FeesReceived(address indexed sender, uint256 amount);
    event NoFeesCollected();

    // Constants
    uint32 constant WITHDRAWAL_MIN_GAS = 35_000;
    uint256 constant DEFAULT_MIN_WITHDRAWAL = 10 ether;
    uint256 constant DEFAULT_DISBURSEMENT_INTERVAL = 24 hours;
    uint256 constant MAX_TEST_FEE = 10 ** 27;

    // Test addresses
    address payable constant L1_WALLET = payable(address(0x1001));
    address constant ALICE = address(0xA11CE);
    address constant BOB = address(0xB0B);

    // Contract instances
    FeeDisburser feeDisburser;

    function setUp() public virtual {
        // Deploy FeeDisburser
        feeDisburser = new FeeDisburser(L1_WALLET, DEFAULT_DISBURSEMENT_INTERVAL);

        // Warp time to allow first disbursement
        vm.warp(DEFAULT_DISBURSEMENT_INTERVAL + 1);

        // Setup default mocks for fee vaults (configured for L2 withdrawal to FeeDisburser)
        _mockFeeVault(
            Predeploys.SEQUENCER_FEE_WALLET, address(feeDisburser), DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L2
        );
        _mockFeeVault(
            Predeploys.BASE_FEE_VAULT, address(feeDisburser), DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L2
        );
        _mockFeeVault(
            Predeploys.L1_FEE_VAULT, address(feeDisburser), DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L2
        );
    }

    /// @notice Helper to mock a FeeVault's view functions
    function _mockFeeVault(
        address vault,
        address recipient,
        uint256 minWithdrawal,
        Types.WithdrawalNetwork network
    )
        internal
    {
        vm.mockCall(vault, abi.encodeCall(IFeeVault.WITHDRAWAL_NETWORK, ()), abi.encode(network));
        vm.mockCall(vault, abi.encodeCall(IFeeVault.RECIPIENT, ()), abi.encode(recipient));
        vm.mockCall(vault, abi.encodeCall(IFeeVault.MIN_WITHDRAWAL_AMOUNT, ()), abi.encode(minWithdrawal));
    }

    function _mockVaultWithdrawal(address vault, uint256 amount) internal {
        vm.deal(vault, amount);
        vm.mockCall(vault, abi.encodeCall(IFeeVault.withdraw, ()), abi.encode(amount));
        vm.deal(address(feeDisburser), address(feeDisburser).balance + amount);
    }

    function _expectBridgeETH(uint256 amount) internal {
        bytes memory bridgeCall =
            abi.encodeCall(IStandardBridge.bridgeETHTo, (L1_WALLET, WITHDRAWAL_MIN_GAS, bytes("")));

        vm.mockCall(Predeploys.L2_STANDARD_BRIDGE, bridgeCall, bytes(""));
        vm.expectCall(Predeploys.L2_STANDARD_BRIDGE, amount, bridgeCall);
    }

    function _resetAfterMockedBridge() internal {
        // vm.mockCall returns without moving ETH, so mirror the real bridge transfer between disbursements.
        vm.deal(address(feeDisburser), 0);
    }

    function _expectFeesDisbursed(uint256 totalFees) internal {
        vm.expectEmit(address(feeDisburser));
        emit FeesDisbursed(block.timestamp, 0, totalFees);
    }

    function _expectFeesReceived(address sender, uint256 amount) internal {
        vm.expectEmit(address(feeDisburser));
        emit FeesReceived(sender, amount);
    }

    function _expectNoFeesCollected() internal {
        vm.expectEmit(address(feeDisburser));
        emit NoFeesCollected();
    }

    // ============================================================
    //                    Constructor Tests
    // ============================================================

    function test_constructor_success() public {
        FeeDisburser newDisburser = new FeeDisburser(L1_WALLET, DEFAULT_DISBURSEMENT_INTERVAL);

        assertEq(newDisburser.L1_WALLET(), L1_WALLET);
        assertEq(newDisburser.FEE_DISBURSEMENT_INTERVAL(), DEFAULT_DISBURSEMENT_INTERVAL);
        assertEq(newDisburser.WITHDRAWAL_MIN_GAS(), WITHDRAWAL_MIN_GAS);
        assertEq(newDisburser.lastDisbursementTime(), 0);
    }

    function test_constructor_success_exactlyMinimumInterval() public {
        FeeDisburser newDisburser = new FeeDisburser(L1_WALLET, 24 hours);
        assertEq(newDisburser.FEE_DISBURSEMENT_INTERVAL(), 24 hours);
    }

    function test_constructor_success_largeInterval() public {
        uint256 largeInterval = 365 days;
        FeeDisburser newDisburser = new FeeDisburser(L1_WALLET, largeInterval);
        assertEq(newDisburser.FEE_DISBURSEMENT_INTERVAL(), largeInterval);
    }

    function test_constructor_revert_zeroAddress() public {
        vm.expectRevert(FeeDisburser.ZeroAddress.selector);
        new FeeDisburser(address(0), DEFAULT_DISBURSEMENT_INTERVAL);
    }

    function test_constructor_revert_intervalTooLow() public {
        vm.expectRevert(FeeDisburser.IntervalTooLow.selector);
        new FeeDisburser(L1_WALLET, 24 hours - 1);
    }

    function test_constructor_revert_intervalZero() public {
        vm.expectRevert(FeeDisburser.IntervalTooLow.selector);
        new FeeDisburser(L1_WALLET, 0);
    }

    // ============================================================
    //                    disburseFees Tests
    // ============================================================

    function test_disburseFees_success_noFees() public {
        // Don't setup any vault withdrawals - all vaults have 0 balance
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, 0);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectNoFeesCollected();

        feeDisburser.disburseFees();

        // lastDisbursementTime should NOT be updated when no fees collected
        assertEq(feeDisburser.lastDisbursementTime(), 0);
    }

    function test_disburseFees_success_sequencerFeesOnly() public {
        uint256 feeAmount = DEFAULT_MIN_WITHDRAWAL;

        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, feeAmount);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(feeAmount);

        _expectFeesDisbursed(feeAmount);

        feeDisburser.disburseFees();

        assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
    }

    function test_disburseFees_success_baseFeesOnly() public {
        uint256 feeAmount = DEFAULT_MIN_WITHDRAWAL;

        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, 0);
        _mockVaultWithdrawal(Predeploys.BASE_FEE_VAULT, feeAmount);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(feeAmount);

        _expectFeesDisbursed(feeAmount);

        feeDisburser.disburseFees();

        assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
    }

    function test_disburseFees_success_l1FeesOnly() public {
        uint256 feeAmount = DEFAULT_MIN_WITHDRAWAL;

        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, 0);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        _mockVaultWithdrawal(Predeploys.L1_FEE_VAULT, feeAmount);

        _expectBridgeETH(feeAmount);

        _expectFeesDisbursed(feeAmount);

        feeDisburser.disburseFees();

        assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
    }

    function test_disburseFees_success_allVaults() public {
        uint256 sequencerFees = DEFAULT_MIN_WITHDRAWAL;
        uint256 baseFees = DEFAULT_MIN_WITHDRAWAL * 2;
        uint256 l1Fees = DEFAULT_MIN_WITHDRAWAL * 3;
        uint256 totalFees = sequencerFees + baseFees + l1Fees;

        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, sequencerFees);
        _mockVaultWithdrawal(Predeploys.BASE_FEE_VAULT, baseFees);
        _mockVaultWithdrawal(Predeploys.L1_FEE_VAULT, l1Fees);

        _expectBridgeETH(totalFees);

        _expectFeesDisbursed(totalFees);

        feeDisburser.disburseFees();

        assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
    }

    function test_disburseFees_success_belowMinWithdrawal() public {
        // All vaults below minimum - no withdrawal should occur
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL - 1);
        vm.deal(Predeploys.BASE_FEE_VAULT, DEFAULT_MIN_WITHDRAWAL - 1);
        vm.deal(Predeploys.L1_FEE_VAULT, DEFAULT_MIN_WITHDRAWAL - 1);

        _expectNoFeesCollected();

        feeDisburser.disburseFees();

        assertEq(feeDisburser.lastDisbursementTime(), 0);
    }

    function test_disburseFees_success_partialWithdrawal() public {
        // Only sequencer vault has enough
        uint256 sequencerFees = DEFAULT_MIN_WITHDRAWAL;

        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, sequencerFees);
        vm.deal(Predeploys.BASE_FEE_VAULT, DEFAULT_MIN_WITHDRAWAL - 1);
        vm.deal(Predeploys.L1_FEE_VAULT, DEFAULT_MIN_WITHDRAWAL - 1);

        _expectBridgeETH(sequencerFees);

        _expectFeesDisbursed(sequencerFees);

        feeDisburser.disburseFees();
    }

    function test_disburseFees_revert_intervalNotReached() public {
        // First successful disbursal
        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);

        feeDisburser.disburseFees();

        // Second attempt should fail (no time has passed)
        vm.expectRevert(FeeDisburser.IntervalNotReached.selector);
        feeDisburser.disburseFees();
    }

    function test_disburseFees_revert_intervalNotReached_oneSecondBefore() public {
        // First disbursal
        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);

        feeDisburser.disburseFees();

        // Warp to one second before interval
        vm.warp(block.timestamp + DEFAULT_DISBURSEMENT_INTERVAL - 1);

        vm.expectRevert(FeeDisburser.IntervalNotReached.selector);
        feeDisburser.disburseFees();
    }

    function test_disburseFees_success_exactlyAtInterval() public {
        // First disbursal
        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);

        feeDisburser.disburseFees();
        uint256 firstDisbursementTime = block.timestamp;
        _resetAfterMockedBridge();

        // Warp exactly to the interval
        vm.warp(firstDisbursementTime + DEFAULT_DISBURSEMENT_INTERVAL);

        // Fund vaults again
        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);

        // Should succeed
        _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);
        feeDisburser.disburseFees();
        assertEq(feeDisburser.lastDisbursementTime(), firstDisbursementTime + DEFAULT_DISBURSEMENT_INTERVAL);
    }

    function test_disburseFees_success_multipleDisbursements() public {
        for (uint256 i = 0; i < 5; i++) {
            _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
            vm.deal(Predeploys.BASE_FEE_VAULT, 0);
            vm.deal(Predeploys.L1_FEE_VAULT, 0);

            _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);
            feeDisburser.disburseFees();
            assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
            _resetAfterMockedBridge();

            // Warp past interval for next iteration
            vm.warp(block.timestamp + DEFAULT_DISBURSEMENT_INTERVAL);
        }
    }

    function test_disburseFees_revert_feeVaultWithdrawalToL1() public {
        // Mock sequencer vault to withdraw to L1 instead of L2
        _mockFeeVault(
            Predeploys.SEQUENCER_FEE_WALLET, address(feeDisburser), DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L1
        );

        vm.expectRevert(FeeDisburser.FeeVaultMustWithdrawToL2.selector);
        feeDisburser.disburseFees();
    }

    function test_disburseFees_revert_feeVaultWrongRecipient() public {
        // Mock sequencer vault with wrong recipient (ALICE instead of feeDisburser)
        _mockFeeVault(Predeploys.SEQUENCER_FEE_WALLET, ALICE, DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L2);

        vm.expectRevert(FeeDisburser.FeeVaultMustWithdrawToFeeDisburser.selector);
        feeDisburser.disburseFees();
    }

    function test_disburseFees_revert_baseFeeVaultWrongNetwork() public {
        // Sequencer is fine
        _mockFeeVault(
            Predeploys.SEQUENCER_FEE_WALLET, address(feeDisburser), DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L2
        );
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, 0); // Below min so doesn't actually withdraw

        // Base vault configured to L1
        _mockFeeVault(
            Predeploys.BASE_FEE_VAULT, address(feeDisburser), DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L1
        );

        vm.expectRevert(FeeDisburser.FeeVaultMustWithdrawToL2.selector);
        feeDisburser.disburseFees();
    }

    function test_disburseFees_revert_l1FeeVaultWrongRecipient() public {
        // Sequencer and base are fine
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, 0);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);

        // L1 vault has wrong recipient
        _mockFeeVault(Predeploys.L1_FEE_VAULT, BOB, DEFAULT_MIN_WITHDRAWAL, Types.WithdrawalNetwork.L2);

        vm.expectRevert(FeeDisburser.FeeVaultMustWithdrawToFeeDisburser.selector);
        feeDisburser.disburseFees();
    }

    // ============================================================
    //                    receive() Tests
    // ============================================================

    function test_receive_success_anyAddress() public {
        uint256 amount = 1 ether;
        vm.deal(ALICE, amount);

        _expectFeesReceived(ALICE, amount);

        vm.prank(ALICE);
        (bool success,) = address(feeDisburser).call{ value: amount }("");
        assertTrue(success);

        assertEq(address(feeDisburser).balance, amount);
    }

    function test_receive_success_sequencerFeeVault() public {
        uint256 amount = 5 ether;
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, amount);

        _expectFeesReceived(Predeploys.SEQUENCER_FEE_WALLET, amount);

        vm.prank(Predeploys.SEQUENCER_FEE_WALLET);
        (bool success,) = address(feeDisburser).call{ value: amount }("");
        assertTrue(success);
    }

    function test_receive_success_baseFeeVault() public {
        uint256 amount = 5 ether;
        vm.deal(Predeploys.BASE_FEE_VAULT, amount);

        _expectFeesReceived(Predeploys.BASE_FEE_VAULT, amount);

        vm.prank(Predeploys.BASE_FEE_VAULT);
        (bool success,) = address(feeDisburser).call{ value: amount }("");
        assertTrue(success);
    }

    function test_receive_success_l1FeeVault() public {
        uint256 amount = 5 ether;
        vm.deal(Predeploys.L1_FEE_VAULT, amount);

        _expectFeesReceived(Predeploys.L1_FEE_VAULT, amount);

        vm.prank(Predeploys.L1_FEE_VAULT);
        (bool success,) = address(feeDisburser).call{ value: amount }("");
        assertTrue(success);
    }

    function test_receive_success_zeroValue() public {
        _expectFeesReceived(ALICE, 0);

        vm.prank(ALICE);
        (bool success,) = address(feeDisburser).call{ value: 0 }("");
        assertTrue(success);
    }

    function test_receive_success_multipleDeposits() public {
        uint256 totalExpected = 0;

        for (uint256 i = 1; i <= 5; i++) {
            uint256 amount = i * 1 ether;
            totalExpected += amount;
            vm.deal(ALICE, amount);

            vm.prank(ALICE);
            (bool success,) = address(feeDisburser).call{ value: amount }("");
            assertTrue(success);
        }

        assertEq(address(feeDisburser).balance, totalExpected);
    }

    // ============================================================
    //                    Fuzz Tests
    // ============================================================

    function testFuzz_constructor_validParams(address _l1Wallet, uint256 _interval) public {
        vm.assume(_l1Wallet != address(0));
        vm.assume(_interval >= 24 hours && _interval <= 365 days * 10); // Reasonable upper bound

        FeeDisburser newDisburser = new FeeDisburser(_l1Wallet, _interval);

        assertEq(newDisburser.L1_WALLET(), _l1Wallet);
        assertEq(newDisburser.FEE_DISBURSEMENT_INTERVAL(), _interval);
    }

    function testFuzz_constructor_revert_intervalTooLow(uint256 _interval) public {
        vm.assume(_interval < 24 hours);

        vm.expectRevert(FeeDisburser.IntervalTooLow.selector);
        new FeeDisburser(L1_WALLET, _interval);
    }

    function testFuzz_disburseFees_varyingAmounts(uint256 _sequencerFees, uint256 _baseFees, uint256 _l1Fees) public {
        // Bound fees to reasonable values to avoid overflow
        _sequencerFees = bound(_sequencerFees, 0, MAX_TEST_FEE);
        _baseFees = bound(_baseFees, 0, MAX_TEST_FEE);
        _l1Fees = bound(_l1Fees, 0, MAX_TEST_FEE);

        // Setup vault balances and mocks
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, _sequencerFees);
        vm.deal(Predeploys.BASE_FEE_VAULT, _baseFees);
        vm.deal(Predeploys.L1_FEE_VAULT, _l1Fees);

        // Calculate expected total (only vaults >= MIN_WITHDRAWAL_AMOUNT will be withdrawn)
        uint256 expectedTotal = 0;
        if (_sequencerFees >= DEFAULT_MIN_WITHDRAWAL) {
            _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, _sequencerFees);
            expectedTotal += _sequencerFees;
        }
        if (_baseFees >= DEFAULT_MIN_WITHDRAWAL) {
            _mockVaultWithdrawal(Predeploys.BASE_FEE_VAULT, _baseFees);
            expectedTotal += _baseFees;
        }
        if (_l1Fees >= DEFAULT_MIN_WITHDRAWAL) {
            _mockVaultWithdrawal(Predeploys.L1_FEE_VAULT, _l1Fees);
            expectedTotal += _l1Fees;
        }

        if (expectedTotal == 0) {
            _expectNoFeesCollected();
        } else {
            _expectBridgeETH(expectedTotal);
            _expectFeesDisbursed(expectedTotal);
        }

        feeDisburser.disburseFees();

        if (expectedTotal > 0) {
            assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
        } else {
            assertEq(feeDisburser.lastDisbursementTime(), 0);
        }
    }

    function testFuzz_disburseFees_multipleIntervals(uint8 _numDisbursements) public {
        uint256 numDisbursements = bound(_numDisbursements, 1, 10);

        for (uint256 i = 0; i < numDisbursements; i++) {
            _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
            vm.deal(Predeploys.BASE_FEE_VAULT, 0);
            vm.deal(Predeploys.L1_FEE_VAULT, 0);

            _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);
            feeDisburser.disburseFees();
            assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
            _resetAfterMockedBridge();

            // Warp past interval for next iteration
            vm.warp(block.timestamp + DEFAULT_DISBURSEMENT_INTERVAL);
        }
    }

    function testFuzz_receive_anyAmount(address _sender, uint256 _amount) public {
        vm.assume(_sender != address(0));
        _amount = bound(_amount, 0, MAX_TEST_FEE);
        vm.deal(_sender, _amount);

        _expectFeesReceived(_sender, _amount);

        vm.prank(_sender);
        (bool success,) = address(feeDisburser).call{ value: _amount }("");
        assertTrue(success);

        assertEq(address(feeDisburser).balance, _amount);
    }

    function testFuzz_intervalEnforcement(uint256 _timeWarp) public {
        // First disbursal
        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);

        feeDisburser.disburseFees();

        uint256 lastTime = feeDisburser.lastDisbursementTime();
        _resetAfterMockedBridge();

        // Bound time warp to avoid overflow
        _timeWarp = bound(_timeWarp, 0, 365 days * 100);
        vm.warp(lastTime + _timeWarp);

        // Fund again
        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);

        if (_timeWarp < DEFAULT_DISBURSEMENT_INTERVAL) {
            vm.expectRevert(FeeDisburser.IntervalNotReached.selector);
            feeDisburser.disburseFees();
        } else {
            // Should succeed
            _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);
            feeDisburser.disburseFees();
            assertEq(feeDisburser.lastDisbursementTime(), lastTime + _timeWarp);
        }
    }

    // ============================================================
    //                    Integration Tests
    // ============================================================

    function test_integration_fullDisbursementCycle() public {
        // Setup realistic fee amounts
        uint256 sequencerFees = 100 ether;
        uint256 baseFees = 50 ether;
        uint256 l1Fees = 200 ether;
        uint256 totalFees = sequencerFees + baseFees + l1Fees;

        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, sequencerFees);
        _mockVaultWithdrawal(Predeploys.BASE_FEE_VAULT, baseFees);
        _mockVaultWithdrawal(Predeploys.L1_FEE_VAULT, l1Fees);

        _expectBridgeETH(totalFees);

        // Verify initial state
        assertEq(feeDisburser.lastDisbursementTime(), 0);

        // Execute disbursement
        _expectFeesDisbursed(totalFees);

        feeDisburser.disburseFees();

        // Verify final state
        assertEq(feeDisburser.lastDisbursementTime(), block.timestamp);
    }

    function test_integration_noFeesDoesNotUpdateTimestamp() public {
        // All vaults empty
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, 0);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        // Verify timestamp doesn't change when no fees collected
        assertEq(feeDisburser.lastDisbursementTime(), 0);

        feeDisburser.disburseFees();

        // Timestamp should remain 0 since no fees were collected
        assertEq(feeDisburser.lastDisbursementTime(), 0);

        // Warp and try again - should succeed since lastDisbursementTime is still 0
        vm.warp(block.timestamp + 1 hours);

        feeDisburser.disburseFees();
        assertEq(feeDisburser.lastDisbursementTime(), 0);
    }

    // ============================================================
    //                    Edge Case Tests
    // ============================================================

    function test_edge_exactMinWithdrawalAmount() public {
        // Test with exactly the minimum withdrawal amount
        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);

        _expectFeesDisbursed(DEFAULT_MIN_WITHDRAWAL);

        feeDisburser.disburseFees();
    }

    function test_edge_oneWeiBelow() public {
        // Test with one wei below minimum - should not withdraw from any vault
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL - 1);
        vm.deal(Predeploys.BASE_FEE_VAULT, DEFAULT_MIN_WITHDRAWAL - 1);
        vm.deal(Predeploys.L1_FEE_VAULT, DEFAULT_MIN_WITHDRAWAL - 1);

        _expectNoFeesCollected();

        feeDisburser.disburseFees();
    }

    function test_edge_veryLargeFees() public {
        // Test with very large fee amounts
        uint256 largeFee = MAX_TEST_FEE; // 1 billion ETH

        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, largeFee);
        _mockVaultWithdrawal(Predeploys.BASE_FEE_VAULT, largeFee);
        _mockVaultWithdrawal(Predeploys.L1_FEE_VAULT, largeFee);

        uint256 totalFees = largeFee * 3;

        _expectBridgeETH(totalFees);

        _expectFeesDisbursed(totalFees);

        feeDisburser.disburseFees();
    }

    function test_edge_timestampAtMax() public {
        // Test at maximum timestamp value (avoiding overflow)
        uint256 maxSafeTimestamp = type(uint256).max - DEFAULT_DISBURSEMENT_INTERVAL;
        vm.warp(maxSafeTimestamp);

        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);

        feeDisburser.disburseFees();
        assertEq(feeDisburser.lastDisbursementTime(), maxSafeTimestamp);
    }

    // ============================================================
    //                    Invariant Tests
    // ============================================================

    function test_invariant_lastDisbursementTimeNeverDecreases() public {
        uint256 previousTime = 0;

        for (uint256 i = 0; i < 10; i++) {
            _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
            vm.deal(Predeploys.BASE_FEE_VAULT, 0);
            vm.deal(Predeploys.L1_FEE_VAULT, 0);

            _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);
            feeDisburser.disburseFees();

            uint256 currentTime = feeDisburser.lastDisbursementTime();
            assertTrue(currentTime >= previousTime, "lastDisbursementTime decreased!");
            previousTime = currentTime;
            _resetAfterMockedBridge();

            vm.warp(block.timestamp + DEFAULT_DISBURSEMENT_INTERVAL);
        }
    }

    function test_invariant_l1WalletNeverChanges() public {
        address initialL1Wallet = feeDisburser.L1_WALLET();

        // Perform multiple operations
        for (uint256 i = 0; i < 5; i++) {
            _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
            vm.deal(Predeploys.BASE_FEE_VAULT, 0);
            vm.deal(Predeploys.L1_FEE_VAULT, 0);

            _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);
            feeDisburser.disburseFees();
            _resetAfterMockedBridge();
            vm.warp(block.timestamp + DEFAULT_DISBURSEMENT_INTERVAL);
        }

        assertEq(feeDisburser.L1_WALLET(), initialL1Wallet, "L1_WALLET changed!");
    }

    function test_invariant_disbursementIntervalNeverChanges() public {
        uint256 initialInterval = feeDisburser.FEE_DISBURSEMENT_INTERVAL();

        // Perform multiple operations
        for (uint256 i = 0; i < 5; i++) {
            _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, DEFAULT_MIN_WITHDRAWAL);
            vm.deal(Predeploys.BASE_FEE_VAULT, 0);
            vm.deal(Predeploys.L1_FEE_VAULT, 0);

            _expectBridgeETH(DEFAULT_MIN_WITHDRAWAL);
            feeDisburser.disburseFees();
            _resetAfterMockedBridge();
            vm.warp(block.timestamp + DEFAULT_DISBURSEMENT_INTERVAL);
        }

        assertEq(feeDisburser.FEE_DISBURSEMENT_INTERVAL(), initialInterval, "FEE_DISBURSEMENT_INTERVAL changed!");
    }

    // ============================================================
    //                    Gas Usage Tests
    // ============================================================

    function test_gas_disburseFees_noFees() public {
        vm.deal(Predeploys.SEQUENCER_FEE_WALLET, 0);
        vm.deal(Predeploys.BASE_FEE_VAULT, 0);
        vm.deal(Predeploys.L1_FEE_VAULT, 0);

        uint256 gasBefore = gasleft();
        feeDisburser.disburseFees();
        uint256 gasUsed = gasBefore - gasleft();

        assertTrue(gasUsed < 100_000, "Gas usage too high for no-fee case");
    }

    function test_gas_disburseFees_allVaults() public {
        uint256 fee = DEFAULT_MIN_WITHDRAWAL;

        _mockVaultWithdrawal(Predeploys.SEQUENCER_FEE_WALLET, fee);
        _mockVaultWithdrawal(Predeploys.BASE_FEE_VAULT, fee);
        _mockVaultWithdrawal(Predeploys.L1_FEE_VAULT, fee);

        _expectBridgeETH(fee * 3);

        uint256 gasBefore = gasleft();
        feeDisburser.disburseFees();
        uint256 gasUsed = gasBefore - gasleft();

        assertTrue(gasUsed < 200_000, "Gas usage too high for all-vault case");
    }
}
