// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";
import { Reverter } from "test/mocks/Callers.sol";

// Interfaces
import { IFeeVault } from "interfaces/L2/IFeeVault.sol";
import { IL2ToL1MessagePasser } from "interfaces/L2/IL2ToL1MessagePasser.sol";

// Libraries
import { Hashing } from "src/libraries/Hashing.sol";
import { Types } from "src/libraries/Types.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { Features } from "src/libraries/Features.sol";

/// @title FeeVault_Uncategorized_Test
/// @notice Abstract test contract for fee vault testing.
///         Subclasses can override the feeVault-specific variables.
abstract contract FeeVault_Uncategorized_Test is CommonTest {
    uint32 internal constant WITHDRAWAL_MIN_GAS = 400_000;

    // Variables that can be overridden by concrete test contracts
    address recipient;
    IFeeVault feeVault;
    string feeVaultName;
    uint256 minWithdrawalAmount;
    Types.WithdrawalNetwork withdrawalNetwork;

    /// @notice Helper function to set up L2 withdrawal configuration.
    function _setupL2Withdrawal() internal {
        vm.prank(proxyAdminOwner);
        feeVault.setWithdrawalNetwork(Types.WithdrawalNetwork.L2);
    }

    function _expectWithdrawalEvents(uint256 _amount, address _recipient, Types.WithdrawalNetwork _network) internal {
        vm.expectEmit(address(feeVault));
        emit Withdrawal(_amount, _recipient, address(this));
        vm.expectEmit(address(feeVault));
        emit Withdrawal(_amount, _recipient, address(this), _network);
    }

    /// @notice Tests that the initialize function succeeds.
    function test_initialize_succeeds() external view {
        assertEq(feeVault.recipient(), recipient);
        assertEq(feeVault.minWithdrawalAmount(), minWithdrawalAmount);
        assertEq(uint8(feeVault.withdrawalNetwork()), uint8(withdrawalNetwork));
    }

    /// @notice Tests that the initialize function reverts if the contract is already initialized.
    function test_initialize_reinitialization_reverts() external {
        _setupL2Withdrawal();

        vm.expectRevert(IFeeVault.InvalidInitialization.selector);
        feeVault.initialize(recipient, minWithdrawalAmount, Types.WithdrawalNetwork.L1);
    }

    /// @notice Tests that the immutable values match the storage getters.
    function test_immutableMatchesStorageVariables_succeeds() external view {
        assertEq(feeVault.RECIPIENT(), feeVault.recipient());
        assertEq(feeVault.MIN_WITHDRAWAL_AMOUNT(), feeVault.minWithdrawalAmount());
        assertEq(uint8(feeVault.WITHDRAWAL_NETWORK()), uint8(feeVault.withdrawalNetwork()));
    }

    /// @notice Tests that the fee feeVault is able to receive ETH.
    function test_receive_succeeds() external {
        uint256 balance = address(feeVault).balance;

        vm.prank(alice);
        (bool success,) = address(feeVault).call{ value: 100 }(hex"");

        assertTrue(success);
        assertEq(address(feeVault).balance, balance + 100);
    }

    /// @notice Tests that `withdraw` reverts if the balance is less than the minimum withdrawal
    ///         amount.
    function testFuzz_withdraw_notEnough_reverts(uint256 _minWithdrawalAmount) external {
        _minWithdrawalAmount = bound(_minWithdrawalAmount, 1, type(uint256).max);
        vm.prank(proxyAdminOwner);
        feeVault.setMinWithdrawalAmount(_minWithdrawalAmount);

        vm.deal(address(feeVault), _minWithdrawalAmount - 1);

        vm.expectRevert("FeeVault: withdrawal amount must be greater than minimum withdrawal amount");
        feeVault.withdraw();
    }

    /// @notice Tests that `withdraw` successfully initiates a withdrawal to L1.
    function test_withdraw_toL1_succeeds() external {
        skipIfSysFeatureEnabled(Features.CUSTOM_GAS_TOKEN);

        vm.prank(proxyAdminOwner);
        feeVault.setWithdrawalNetwork(Types.WithdrawalNetwork.L1);

        uint256 amount = minWithdrawalAmount + 1;
        vm.deal(address(feeVault), amount);

        assertEq(feeVault.totalProcessed(), 0);
        _expectWithdrawalEvents(amount, recipient, Types.WithdrawalNetwork.L1);

        vm.expectCall(
            Predeploys.L2_TO_L1_MESSAGE_PASSER,
            amount,
            abi.encodeCall(IL2ToL1MessagePasser.initiateWithdrawal, (recipient, WITHDRAWAL_MIN_GAS, hex""))
        );

        uint256 nonce = l2ToL1MessagePasser.messageNonce();
        bytes32 withdrawalHash = Hashing.hashWithdrawal(
            Types.WithdrawalTransaction({
                nonce: nonce,
                sender: address(feeVault),
                target: recipient,
                value: amount,
                gasLimit: WITHDRAWAL_MIN_GAS,
                data: hex""
            })
        );

        vm.expectEmit(Predeploys.L2_TO_L1_MESSAGE_PASSER);
        emit MessagePassed(nonce, address(feeVault), recipient, amount, WITHDRAWAL_MIN_GAS, hex"", withdrawalHash);

        feeVault.withdraw();

        assertEq(feeVault.totalProcessed(), amount);
        assertEq(address(feeVault).balance, 0);
        assertEq(Predeploys.L2_TO_L1_MESSAGE_PASSER.balance, amount);
    }

    /// @notice Tests that `withdraw` successfully initiates a withdrawal to L2.
    function test_withdraw_toL2_succeeds() public {
        _setupL2Withdrawal();

        uint256 amount = minWithdrawalAmount + 1;
        vm.deal(address(feeVault), amount);

        assertEq(feeVault.totalProcessed(), 0);
        _expectWithdrawalEvents(amount, recipient, Types.WithdrawalNetwork.L2);

        vm.expectCall(recipient, amount, bytes(""));

        uint256 withdrawnAmount = feeVault.withdraw();

        assertEq(withdrawnAmount, amount);
        assertEq(feeVault.totalProcessed(), amount);
        assertEq(address(feeVault).balance, 0);
        assertEq(recipient.balance, amount);
    }

    /// @notice Tests that `withdraw` fails if the Recipient reverts. This also serves to simulate
    ///         a situation where insufficient gas is provided to the RECIPIENT.
    function test_withdraw_toL2recipientReverts_fails() external {
        _setupL2Withdrawal();

        uint256 amount = minWithdrawalAmount;

        vm.deal(address(feeVault), amount);
        assertEq(feeVault.totalProcessed(), 0);

        // Ensure the RECIPIENT reverts
        vm.etch(recipient, type(Reverter).runtimeCode);

        vm.expectCall(recipient, amount, bytes(""));
        vm.expectRevert("FeeVault: failed to send ETH to L2 fee recipient");
        feeVault.withdraw();
        assertEq(feeVault.totalProcessed(), 0);
    }

    /// @notice Tests that the owner can successfully set minimum withdrawal amount with fuzz testing.
    function testFuzz_setMinWithdrawalAmount_succeeds(uint256 _newMinWithdrawalAmount) external {
        vm.prank(proxyAdminOwner);
        feeVault.setMinWithdrawalAmount(_newMinWithdrawalAmount);

        assertEq(feeVault.minWithdrawalAmount(), _newMinWithdrawalAmount);
    }

    /// @notice Tests that non-owner cannot set minimum withdrawal amount with fuzz testing.
    function testFuzz_setMinWithdrawalAmount_onlyOwner_reverts(address _caller, uint256 _newAmount) external {
        vm.assume(_caller != proxyAdminOwner);

        uint256 initialAmount = feeVault.minWithdrawalAmount();

        vm.prank(_caller);
        vm.expectRevert(IFeeVault.FeeVault_OnlyProxyAdminOwner.selector);
        feeVault.setMinWithdrawalAmount(_newAmount);

        assertEq(feeVault.minWithdrawalAmount(), initialAmount);
    }

    /// @notice Tests that the owner can successfully set recipient with fuzz testing.
    function testFuzz_setRecipient_succeeds(address _newRecipient) external {
        vm.prank(proxyAdminOwner);
        feeVault.setRecipient(_newRecipient);

        assertEq(feeVault.recipient(), _newRecipient);
    }

    /// @notice Tests that non-owner cannot set recipient with fuzz testing.
    function testFuzz_setRecipient_onlyOwner_reverts(address _caller, address _newRecipient) external {
        vm.assume(_caller != proxyAdminOwner);

        address initialRecipient = feeVault.recipient();

        vm.prank(_caller);
        vm.expectRevert(IFeeVault.FeeVault_OnlyProxyAdminOwner.selector);
        feeVault.setRecipient(_newRecipient);

        assertEq(feeVault.recipient(), initialRecipient);
    }

    /// @notice Tests that the owner can successfully set withdrawal network with fuzz testing.
    function testFuzz_setWithdrawalNetwork_succeeds(uint8 _networkValue) external {
        _networkValue = uint8(bound(_networkValue, 0, 1));
        Types.WithdrawalNetwork newNetwork = Types.WithdrawalNetwork(_networkValue);

        vm.prank(proxyAdminOwner);
        feeVault.setWithdrawalNetwork(newNetwork);

        assertEq(uint8(feeVault.withdrawalNetwork()), uint8(newNetwork));
    }

    /// @notice Tests that non-owner cannot set withdrawal network with fuzz testing.
    function testFuzz_setWithdrawalNetwork_onlyOwner_reverts(address _caller, uint8 _networkValue) external {
        vm.assume(_caller != proxyAdminOwner);

        _networkValue = uint8(bound(_networkValue, 0, 1));
        Types.WithdrawalNetwork newNetwork = Types.WithdrawalNetwork(_networkValue);

        Types.WithdrawalNetwork initialNetwork = feeVault.withdrawalNetwork();

        vm.prank(_caller);
        vm.expectRevert(IFeeVault.FeeVault_OnlyProxyAdminOwner.selector);
        feeVault.setWithdrawalNetwork(newNetwork);

        assertEq(uint8(feeVault.withdrawalNetwork()), uint8(initialNetwork));
    }
}
