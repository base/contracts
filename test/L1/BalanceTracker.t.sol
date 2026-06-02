// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Proxy } from "src/universal/Proxy.sol";

import { BalanceTracker } from "src/L1/BalanceTracker.sol";

import { Test } from "lib/forge-std/src/Test.sol";
import { ReenterProcessFees } from "test/mocks/ReenterProcessFees.sol";

contract BalanceTrackerTest is Test {
    event ProcessedFunds(
        address indexed systemAddress, bool indexed success, uint256 balanceNeeded, uint256 balanceSent
    );
    event SentProfit(address indexed profitWallet, bool indexed success, uint256 balanceSent);
    event ReceivedFunds(address indexed sender, uint256 amount);

    uint256 constant MAX_SYSTEM_ADDRESS_COUNT = 20;
    uint256 constant INITIAL_BALANCE_TRACKER_BALANCE = 2_000 ether;
    uint256 constant BATCH_SENDER_TARGET_BALANCE = 1_000 ether;
    uint256 constant L2_OUTPUT_PROPOSER_TARGET_BALANCE = 100 ether;

    address payable constant L1_STANDARD_BRIDGE = payable(address(1000));
    address payable constant PROFIT_WALLET = payable(address(1001));
    address payable constant BATCH_SENDER = payable(address(1002));
    address payable constant L2_OUTPUT_PROPOSER = payable(address(1003));
    address constant PROXY_ADMIN_OWNER = address(2048);

    BalanceTracker balanceTracker;

    address payable[] systemAddresses = [BATCH_SENDER, L2_OUTPUT_PROPOSER];
    uint256[] targetBalances = [BATCH_SENDER_TARGET_BALANCE, L2_OUTPUT_PROPOSER_TARGET_BALANCE];

    function setUp() public {
        BalanceTracker balanceTrackerImplementation = new BalanceTracker(PROFIT_WALLET);
        Proxy balanceTrackerProxy = new Proxy(PROXY_ADMIN_OWNER);
        vm.prank(PROXY_ADMIN_OWNER);
        balanceTrackerProxy.upgradeTo(address(balanceTrackerImplementation));
        balanceTracker = BalanceTracker(payable(address(balanceTrackerProxy)));
    }

    function test_constructor_fail_profitWallet_zeroAddress() external {
        vm.expectRevert("BalanceTracker: PROFIT_WALLET cannot be address(0)");
        new BalanceTracker(payable(address(0)));
    }

    function test_constructor_success() external {
        BalanceTracker tracker = new BalanceTracker(PROFIT_WALLET);

        assertEq(tracker.MAX_SYSTEM_ADDRESS_COUNT(), MAX_SYSTEM_ADDRESS_COUNT);
        assertEq(tracker.PROFIT_WALLET(), PROFIT_WALLET);
    }

    function test_initializer_fail_systemAddresses_zeroLength() external {
        delete systemAddresses;
        vm.expectRevert("BalanceTracker: systemAddresses cannot have a length of zero");
        balanceTracker.initialize(systemAddresses, targetBalances);
    }

    function test_initializer_fail_systemAddresses_greaterThanMaxLength() external {
        address payable[] memory oversizedSystemAddresses = new address payable[](MAX_SYSTEM_ADDRESS_COUNT + 1);
        uint256[] memory oversizedTargetBalances = new uint256[](MAX_SYSTEM_ADDRESS_COUNT + 1);

        vm.expectRevert("BalanceTracker: systemAddresses cannot have a length greater than 20");
        balanceTracker.initialize(oversizedSystemAddresses, oversizedTargetBalances);
    }

    function test_initializer_fail_systemAddresses_lengthNotEqualToTargetBalancesLength() external {
        systemAddresses.push(payable(address(0)));

        vm.expectRevert("BalanceTracker: systemAddresses and targetBalances length must be equal");
        balanceTracker.initialize(systemAddresses, targetBalances);
    }

    function test_initializer_fail_systemAddresses_containsZeroAddress() external {
        systemAddresses[1] = payable(address(0));

        vm.expectRevert("BalanceTracker: systemAddresses cannot contain address(0)");
        balanceTracker.initialize(systemAddresses, targetBalances);
    }

    function test_initializer_fail_targetBalances_containsZero() external {
        targetBalances[1] = 0;

        vm.expectRevert("BalanceTracker: targetBalances cannot contain 0 target");
        balanceTracker.initialize(systemAddresses, targetBalances);
    }

    function test_initializer_success() external {
        balanceTracker.initialize(systemAddresses, targetBalances);

        assertEq(balanceTracker.systemAddresses(0), systemAddresses[0]);
        assertEq(balanceTracker.systemAddresses(1), systemAddresses[1]);
        assertEq(balanceTracker.targetBalances(0), targetBalances[0]);
        assertEq(balanceTracker.targetBalances(1), targetBalances[1]);
    }

    function test_processFees_success_cannotBeReentered() external {
        vm.deal(address(balanceTracker), INITIAL_BALANCE_TRACKER_BALANCE);
        uint256 expectedProfitWalletBalance = INITIAL_BALANCE_TRACKER_BALANCE - L2_OUTPUT_PROPOSER_TARGET_BALANCE;
        address payable reentrancySystemAddress = payable(address(new ReenterProcessFees()));
        systemAddresses[0] = reentrancySystemAddress;
        balanceTracker.initialize(systemAddresses, targetBalances);

        _expectProcessedFunds(reentrancySystemAddress, false, BATCH_SENDER_TARGET_BALANCE, BATCH_SENDER_TARGET_BALANCE);
        _expectProcessedFunds(
            L2_OUTPUT_PROPOSER, true, L2_OUTPUT_PROPOSER_TARGET_BALANCE, L2_OUTPUT_PROPOSER_TARGET_BALANCE
        );
        _expectSentProfit(expectedProfitWalletBalance);

        balanceTracker.processFees();

        _assertBalances(0, expectedProfitWalletBalance, 0, L2_OUTPUT_PROPOSER_TARGET_BALANCE);
    }

    function test_processFees_fail_whenNotInitialized() external {
        vm.expectRevert("BalanceTracker: systemAddresses cannot have a length of zero");

        balanceTracker.processFees();
    }

    function test_processFees_success_continuesWhenSystemAddressReverts() external {
        vm.deal(address(balanceTracker), INITIAL_BALANCE_TRACKER_BALANCE);
        uint256 expectedProfitWalletBalance = INITIAL_BALANCE_TRACKER_BALANCE - L2_OUTPUT_PROPOSER_TARGET_BALANCE;
        balanceTracker.initialize(systemAddresses, targetBalances);
        vm.mockCallRevert(BATCH_SENDER, bytes(""), abi.encode("revert message"));
        _expectProcessedFunds(BATCH_SENDER, false, BATCH_SENDER_TARGET_BALANCE, BATCH_SENDER_TARGET_BALANCE);
        _expectProcessedFunds(
            L2_OUTPUT_PROPOSER, true, L2_OUTPUT_PROPOSER_TARGET_BALANCE, L2_OUTPUT_PROPOSER_TARGET_BALANCE
        );
        _expectSentProfit(expectedProfitWalletBalance);

        balanceTracker.processFees();

        _assertBalances(0, expectedProfitWalletBalance, 0, L2_OUTPUT_PROPOSER_TARGET_BALANCE);
    }

    function test_processFees_success_fundsSystemAddresses() external {
        vm.deal(address(balanceTracker), INITIAL_BALANCE_TRACKER_BALANCE);
        uint256 expectedProfitWalletBalance =
            INITIAL_BALANCE_TRACKER_BALANCE - BATCH_SENDER_TARGET_BALANCE - L2_OUTPUT_PROPOSER_TARGET_BALANCE;
        balanceTracker.initialize(systemAddresses, targetBalances);
        _expectProcessedFunds(BATCH_SENDER, true, BATCH_SENDER_TARGET_BALANCE, BATCH_SENDER_TARGET_BALANCE);
        _expectProcessedFunds(
            L2_OUTPUT_PROPOSER, true, L2_OUTPUT_PROPOSER_TARGET_BALANCE, L2_OUTPUT_PROPOSER_TARGET_BALANCE
        );
        _expectSentProfit(expectedProfitWalletBalance);

        balanceTracker.processFees();

        _assertBalances(0, expectedProfitWalletBalance, BATCH_SENDER_TARGET_BALANCE, L2_OUTPUT_PROPOSER_TARGET_BALANCE);
    }

    function test_processFees_success_noFunds() external {
        balanceTracker.initialize(systemAddresses, targetBalances);
        _expectProcessedFunds(BATCH_SENDER, true, BATCH_SENDER_TARGET_BALANCE, 0);
        _expectProcessedFunds(L2_OUTPUT_PROPOSER, true, L2_OUTPUT_PROPOSER_TARGET_BALANCE, 0);
        _expectSentProfit(0);

        balanceTracker.processFees();

        _assertBalances(0, 0, 0, 0);
    }

    function test_processFees_success_partialFunds() external {
        uint256 partialBalanceTrackerBalance = INITIAL_BALANCE_TRACKER_BALANCE / 3;
        vm.deal(address(balanceTracker), partialBalanceTrackerBalance);
        balanceTracker.initialize(systemAddresses, targetBalances);
        _expectProcessedFunds(BATCH_SENDER, true, BATCH_SENDER_TARGET_BALANCE, partialBalanceTrackerBalance);
        _expectProcessedFunds(L2_OUTPUT_PROPOSER, true, L2_OUTPUT_PROPOSER_TARGET_BALANCE, 0);
        _expectSentProfit(0);

        balanceTracker.processFees();

        _assertBalances(0, 0, partialBalanceTrackerBalance, 0);
    }

    function test_processFees_success_skipsAddressesAtTargetBalance() external {
        vm.deal(address(balanceTracker), INITIAL_BALANCE_TRACKER_BALANCE);
        vm.deal(BATCH_SENDER, BATCH_SENDER_TARGET_BALANCE);
        vm.deal(L2_OUTPUT_PROPOSER, L2_OUTPUT_PROPOSER_TARGET_BALANCE);
        balanceTracker.initialize(systemAddresses, targetBalances);
        _expectProcessedFunds(BATCH_SENDER, false, 0, 0);
        _expectProcessedFunds(L2_OUTPUT_PROPOSER, false, 0, 0);
        _expectSentProfit(INITIAL_BALANCE_TRACKER_BALANCE);

        balanceTracker.processFees();

        _assertBalances(
            0, INITIAL_BALANCE_TRACKER_BALANCE, BATCH_SENDER_TARGET_BALANCE, L2_OUTPUT_PROPOSER_TARGET_BALANCE
        );
    }

    function test_processFees_success_maximumSystemAddresses() external {
        vm.deal(address(balanceTracker), INITIAL_BALANCE_TRACKER_BALANCE);
        address payable[] memory maxSystemAddresses = new address payable[](MAX_SYSTEM_ADDRESS_COUNT);
        uint256[] memory maxTargetBalances = new uint256[](MAX_SYSTEM_ADDRESS_COUNT);
        for (uint256 i = 0; i < MAX_SYSTEM_ADDRESS_COUNT; i++) {
            maxSystemAddresses[i] = payable(vm.addr(i + 100));
            maxTargetBalances[i] = L2_OUTPUT_PROPOSER_TARGET_BALANCE;
        }
        balanceTracker.initialize(maxSystemAddresses, maxTargetBalances);

        balanceTracker.processFees();

        assertEq(address(balanceTracker).balance, 0);
        for (uint256 i = 0; i < MAX_SYSTEM_ADDRESS_COUNT; i++) {
            assertEq(maxSystemAddresses[i].balance, L2_OUTPUT_PROPOSER_TARGET_BALANCE);
        }
        assertEq(PROFIT_WALLET.balance, 0);
    }

    function test_receive_success() external {
        uint256 value = 100;
        vm.deal(L1_STANDARD_BRIDGE, value);

        vm.prank(L1_STANDARD_BRIDGE);
        vm.expectEmit(address(balanceTracker));
        emit ReceivedFunds(L1_STANDARD_BRIDGE, value);

        (bool success,) = payable(address(balanceTracker)).call{ value: value }("");
        assertTrue(success);

        assertEq(address(balanceTracker).balance, value);
    }

    function _expectProcessedFunds(
        address systemAddress,
        bool success,
        uint256 balanceNeeded,
        uint256 balanceSent
    )
        internal
    {
        vm.expectEmit(address(balanceTracker));
        emit ProcessedFunds(systemAddress, success, balanceNeeded, balanceSent);
    }

    function _expectSentProfit(uint256 balanceSent) internal {
        vm.expectEmit(address(balanceTracker));
        emit SentProfit(PROFIT_WALLET, true, balanceSent);
    }

    function _assertBalances(
        uint256 trackerBalance,
        uint256 profitWalletBalance,
        uint256 batchSenderBalance,
        uint256 l2OutputProposerBalance
    )
        internal
        view
    {
        assertEq(address(balanceTracker).balance, trackerBalance);
        assertEq(PROFIT_WALLET.balance, profitWalletBalance);
        assertEq(BATCH_SENDER.balance, batchSenderBalance);
        assertEq(L2_OUTPUT_PROPOSER.balance, l2OutputProposerBalance);
    }
}
