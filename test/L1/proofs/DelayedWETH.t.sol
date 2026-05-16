// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";

// Libraries
import { ForgeArtifacts, StorageSlot } from "scripts/libraries/ForgeArtifacts.sol";
import { Burn } from "src/libraries/Burn.sol";

// Interfaces
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";

/// @title DelayedWETH_FallbackGasUser_Harness
/// @notice Contract that burns gas in the fallback function.
contract DelayedWETH_FallbackGasUser_Harness {
    /// @notice Amount of gas to use in the fallback function.
    uint256 public gas;

    /// @param _gas Amount of gas to use in the fallback function.
    constructor(uint256 _gas) {
        gas = _gas;
    }

    /// @notice Burn gas on fallback.
    fallback() external payable {
        Burn.gas(gas);
    }

    /// @notice Burn gas on receive.
    receive() external payable {
        Burn.gas(gas);
    }
}

/// @title DelayedWETH_FallbackReverter_Harness
/// @notice Contract that reverts in the fallback function.
contract DelayedWETH_FallbackReverter_Harness {
    /// @notice Revert on fallback.
    fallback() external payable {
        revert("FallbackReverter: revert");
    }

    /// @notice Revert on receive.
    receive() external payable {
        revert("FallbackReverter: revert");
    }
}

/// @title DelayedWETH_TestBase
/// @notice Reusable test utilities for `DelayedWETH` tests.
abstract contract DelayedWETH_TestBase is CommonTest {
    address internal constant DUMMY_SYSTEM_CONFIG = address(1234);
    uint256 internal constant DEFAULT_AMOUNT = 1 ether;

    event Approval(address indexed src, address indexed guy, uint256 wad);
    event Withdrawal(address indexed src, uint256 wad);
}

/// @title DelayedWETH_Initialize_Test
/// @notice Tests the `initialize` function of the `DelayedWETH` contract.
contract DelayedWETH_Initialize_Test is DelayedWETH_TestBase {
    bytes32 internal initializedSlot;

    function setUp() public override {
        super.setUp();

        StorageSlot memory slot = ForgeArtifacts.getSlot("DelayedWETH", "_initialized");
        initializedSlot = bytes32(slot.slot);
    }

    /// @notice Tests that initialization is successful.
    function test_initialize_succeeds() public view {
        assertEq(delayedWeth.proxyAdminOwner(), proxyAdminOwner);
        assertEq(address(delayedWeth.systemConfig()), address(systemConfig));
        assertEq(address(delayedWeth.config()), address(systemConfig.superchainConfig()));
    }

    /// @notice Tests that the initializer value is correct. Trivial test for normal initialization
    ///         but confirms that the initValue is not incremented incorrectly if an upgrade
    ///         function is not present.
    function test_initialize_correctInitializerValue_succeeds() public view {
        bytes32 slotVal = vm.load(address(delayedWeth), initializedSlot);
        uint8 val = uint8(uint256(slotVal) & 0xFF);

        assertEq(val, delayedWeth.initVersion());
    }

    /// @notice Tests that initialization reverts if called by a non-proxy admin or proxy admin
    ///         owner.
    /// @param _sender The address of the sender to test.
    function testFuzz_initialize_notProxyAdminOrProxyAdminOwner_reverts(address _sender) public {
        vm.assume(_sender != address(delayedWeth.proxyAdmin()) && _sender != delayedWeth.proxyAdminOwner());

        vm.store(address(delayedWeth), initializedSlot, bytes32(0));

        vm.expectRevert(IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner.selector);
        vm.prank(_sender);
        delayedWeth.initialize(ISystemConfig(DUMMY_SYSTEM_CONFIG));
    }
}

/// @title DelayedWETH_Unlock_Test
/// @notice Tests the `unlock` function of the `DelayedWETH` contract.
contract DelayedWETH_Unlock_Test is DelayedWETH_TestBase {
    /// @notice Tests that unlocking once is successful.
    function test_unlock_once_succeeds() public {
        delayedWeth.unlock(alice, DEFAULT_AMOUNT);
        (uint256 amount, uint256 timestamp) = delayedWeth.withdrawals(address(this), alice);
        assertEq(amount, DEFAULT_AMOUNT);
        assertEq(timestamp, block.timestamp);
    }

    /// @notice Tests that unlocking twice is successful and timestamp/amount is updated.
    function test_unlock_twice_succeeds() public {
        // Unlock once.
        uint256 ts = block.timestamp;
        delayedWeth.unlock(alice, DEFAULT_AMOUNT);
        (uint256 amount1, uint256 timestamp1) = delayedWeth.withdrawals(address(this), alice);
        assertEq(amount1, DEFAULT_AMOUNT);
        assertEq(timestamp1, ts);

        // Go forward in time.
        vm.warp(ts + 1);

        // Unlock again works.
        delayedWeth.unlock(alice, DEFAULT_AMOUNT);
        (uint256 amount2, uint256 timestamp2) = delayedWeth.withdrawals(address(this), alice);
        assertEq(amount2, 2 * DEFAULT_AMOUNT);
        assertEq(timestamp2, ts + 1);
    }
}

/// @title DelayedWETH_Withdraw_Test
/// @notice Tests the `withdraw` function of the `DelayedWETH` contract.
contract DelayedWETH_Withdraw_Test is DelayedWETH_TestBase {
    function _depositAlice(uint256 _amount) internal returns (uint256 balanceAfterDeposit_) {
        vm.prank(alice);
        delayedWeth.deposit{ value: _amount }();
        balanceAfterDeposit_ = address(alice).balance;
    }

    function _unlockAlice(uint256 _amount) internal {
        vm.prank(alice);
        delayedWeth.unlock(alice, _amount);
    }

    function _warpPastDelay() internal {
        vm.warp(block.timestamp + delayedWeth.delay() + 1);
    }

    function _warpBeforeDelay() internal {
        vm.warp(block.timestamp + delayedWeth.delay() - 1);
    }

    function _pauseSuperchain() internal {
        vm.prank(optimismPortal2.guardian());
        superchainConfig.pause(address(0));
    }

    function _prepareUnlockedWithdrawal() internal returns (uint256 balanceAfterDeposit_) {
        balanceAfterDeposit_ = _depositAlice(DEFAULT_AMOUNT);
        _unlockAlice(DEFAULT_AMOUNT);
        _warpPastDelay();
    }

    function _preparePausedWithdrawal() internal {
        _depositAlice(DEFAULT_AMOUNT);
        _unlockAlice(DEFAULT_AMOUNT);
        _warpPastDelay();
        _pauseSuperchain();
    }

    /// @notice Tests that withdrawing while unlocked and delay has passed is successful.
    function test_withdraw_whileUnlocked_succeeds() public {
        uint256 balance = _prepareUnlockedWithdrawal();

        vm.expectEmit(true, true, false, false);
        emit Withdrawal(address(alice), DEFAULT_AMOUNT);
        vm.prank(alice);
        delayedWeth.withdraw(DEFAULT_AMOUNT);
        assertEq(address(alice).balance, balance + DEFAULT_AMOUNT);
    }

    /// @notice Tests that withdrawing when unlock was not called fails.
    function test_withdraw_whileLocked_fails() public {
        uint256 balance = _depositAlice(DEFAULT_AMOUNT);

        vm.expectRevert("DelayedWETH: withdrawal not unlocked");
        vm.prank(alice);
        delayedWeth.withdraw(0 ether);
        assertEq(address(alice).balance, balance);
    }

    /// @notice Tests that withdrawing while locked and delay has not passed fails.
    function test_withdraw_whileLockedNotLongEnough_fails() public {
        uint256 balance = _depositAlice(DEFAULT_AMOUNT);
        _unlockAlice(DEFAULT_AMOUNT);
        _warpBeforeDelay();

        vm.expectRevert("DelayedWETH: withdrawal delay not met");
        vm.prank(alice);
        delayedWeth.withdraw(DEFAULT_AMOUNT);
        assertEq(address(alice).balance, balance);
    }

    /// @notice Tests that withdrawing more than unlocked amount fails.
    function test_withdraw_tooMuch_fails() public {
        uint256 balance = _prepareUnlockedWithdrawal();

        vm.expectRevert("DelayedWETH: insufficient unlocked withdrawal");
        vm.prank(alice);
        delayedWeth.withdraw(2 * DEFAULT_AMOUNT);
        assertEq(address(alice).balance, balance);
    }

    /// @notice Tests that withdrawing while paused fails.
    function test_withdraw_whenPaused_fails() public {
        _preparePausedWithdrawal();

        vm.expectRevert("DelayedWETH: contract is paused");
        vm.prank(alice);
        delayedWeth.withdraw(DEFAULT_AMOUNT);
    }

    /// @notice Tests that withdrawing while unlocked and delay has passed is successful.
    function test_withdraw_withdrawFromWhileUnlocked_succeeds() public {
        uint256 balance = _prepareUnlockedWithdrawal();

        vm.expectEmit(true, true, false, false);
        emit Withdrawal(address(alice), DEFAULT_AMOUNT);
        vm.prank(alice);
        delayedWeth.withdraw(alice, DEFAULT_AMOUNT);
        assertEq(address(alice).balance, balance + DEFAULT_AMOUNT);
    }

    /// @notice Tests that withdrawing when unlock was not called fails.
    function test_withdraw_withdrawFromWhileLocked_fails() public {
        uint256 balance = _depositAlice(DEFAULT_AMOUNT);

        vm.expectRevert("DelayedWETH: withdrawal not unlocked");
        vm.prank(alice);
        delayedWeth.withdraw(alice, 0 ether);
        assertEq(address(alice).balance, balance);
    }

    /// @notice Tests that withdrawing while locked and delay has not passed fails.
    function test_withdraw_withdrawFromWhileLockedNotLongEnough_fails() public {
        uint256 balance = _depositAlice(DEFAULT_AMOUNT);
        _unlockAlice(DEFAULT_AMOUNT);
        _warpBeforeDelay();

        vm.expectRevert("DelayedWETH: withdrawal delay not met");
        vm.prank(alice);
        delayedWeth.withdraw(alice, DEFAULT_AMOUNT);
        assertEq(address(alice).balance, balance);
    }

    /// @notice Tests that withdrawing more than unlocked amount fails.
    function test_withdraw_withdrawFromTooMuch_fails() public {
        uint256 balance = _prepareUnlockedWithdrawal();

        vm.expectRevert("DelayedWETH: insufficient unlocked withdrawal");
        vm.prank(alice);
        delayedWeth.withdraw(alice, 2 * DEFAULT_AMOUNT);
        assertEq(address(alice).balance, balance);
    }

    /// @notice Tests that withdrawing while paused fails.
    function test_withdraw_withdrawFromWhenPaused_fails() public {
        _preparePausedWithdrawal();

        vm.expectRevert("DelayedWETH: contract is paused");
        vm.prank(alice);
        delayedWeth.withdraw(alice, DEFAULT_AMOUNT);
    }
}

/// @title DelayedWETH_Recover_Test
/// @notice Tests the `recover` function of the `DelayedWETH` contract.
contract DelayedWETH_Recover_Test is DelayedWETH_TestBase {
    uint256 internal constant MAX_FALLBACK_GAS_USAGE = 20_000_000;

    function _mockProxyAdminOwner(address _owner) internal {
        vm.mockCall(address(proxyAdmin), abi.encodeCall(IProxyAdmin.owner, ()), abi.encode(_owner));
    }

    /// @notice Tests that recovering WETH succeeds. Makes sure that doing so succeeds with any
    ///         amount of ETH in the contract and any amount of gas used in the fallback function
    ///         up to a maximum of 20,000,000 gas. Owner contract should never be using that much
    ///         gas but we might as well set a very large upper bound for ourselves.
    /// @param _amount Amount of WETH to recover.
    /// @param _fallbackGasUsage Amount of gas to use in the fallback function.
    function testFuzz_recover_succeeds(uint256 _amount, uint256 _fallbackGasUsage) public {
        _fallbackGasUsage = bound(_fallbackGasUsage, 0, MAX_FALLBACK_GAS_USAGE);

        DelayedWETH_FallbackGasUser_Harness gasUser = new DelayedWETH_FallbackGasUser_Harness(_fallbackGasUsage);

        _mockProxyAdminOwner(address(gasUser));

        vm.deal(address(delayedWeth), _amount);

        uint256 initialBalance = address(gasUser).balance;

        vm.prank(address(gasUser));
        delayedWeth.recover(_amount);

        assertEq(address(delayedWeth).balance, 0);
        assertEq(address(gasUser).balance, initialBalance + _amount);
    }

    /// @notice Tests that recovering WETH by non-owner fails.
    function test_recover_byNonOwner_fails() public {
        vm.prank(alice);

        vm.expectRevert("DelayedWETH: not owner");
        delayedWeth.recover(DEFAULT_AMOUNT);
    }

    /// @notice Tests that recovering more than the balance recovers what it can.
    function test_recover_moreThanBalance_succeeds() public {
        _mockProxyAdminOwner(alice);

        vm.deal(address(delayedWeth), 0.5 ether);

        uint256 initialBalance = address(alice).balance;

        vm.prank(alice);
        delayedWeth.recover(DEFAULT_AMOUNT);

        assertEq(address(delayedWeth).balance, 0);
        assertEq(address(alice).balance, initialBalance + 0.5 ether);
    }

    /// @notice Tests that recover reverts when recipient reverts.
    function test_recover_whenRecipientReverts_fails() public {
        DelayedWETH_FallbackReverter_Harness reverter = new DelayedWETH_FallbackReverter_Harness();

        _mockProxyAdminOwner(address(reverter));

        vm.deal(address(delayedWeth), DEFAULT_AMOUNT);

        vm.expectRevert("DelayedWETH: recover failed");
        vm.prank(address(reverter));
        delayedWeth.recover(DEFAULT_AMOUNT);
    }
}

/// @title DelayedWETH_Hold_Test
/// @notice Tests the `hold` function of the `DelayedWETH` contract.
contract DelayedWETH_Hold_Test is DelayedWETH_TestBase {
    function _depositAliceAndExpectHoldApproval(uint256 _amount) internal returns (uint256 initialBalance_) {
        vm.prank(alice);
        delayedWeth.deposit{ value: _amount }();

        initialBalance_ = delayedWeth.balanceOf(address(proxyAdminOwner));

        vm.expectEmit(true, true, true, false);
        emit Approval(alice, address(proxyAdminOwner), _amount);
    }

    /// @notice Tests that holding WETH succeeds.
    function test_hold_byOwner_succeeds() public {
        uint256 initialBalance = _depositAliceAndExpectHoldApproval(DEFAULT_AMOUNT);

        vm.prank(proxyAdminOwner);
        delayedWeth.hold(alice, DEFAULT_AMOUNT);

        uint256 finalBalance = delayedWeth.balanceOf(address(proxyAdminOwner));

        assertEq(finalBalance, initialBalance + DEFAULT_AMOUNT);
    }

    /// @notice Tests that holding all WETH succeeds when the amount is omitted.
    function test_hold_withoutAmount_succeeds() public {
        uint256 initialBalance = _depositAliceAndExpectHoldApproval(DEFAULT_AMOUNT);

        vm.prank(proxyAdminOwner);
        delayedWeth.hold(alice);

        uint256 finalBalance = delayedWeth.balanceOf(address(proxyAdminOwner));

        assertEq(finalBalance, initialBalance + DEFAULT_AMOUNT);
    }

    /// @notice Tests that holding WETH by non-owner fails.
    function test_hold_byNonOwner_fails() public {
        vm.prank(alice);

        vm.expectRevert("DelayedWETH: not owner");
        delayedWeth.hold(bob, DEFAULT_AMOUNT);
    }
}
