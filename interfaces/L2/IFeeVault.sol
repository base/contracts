// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { Types } from "src/libraries/Types.sol";

/// @title IFeeVault
/// @notice Interface for fee vault contracts using the upgradeable proxy pattern.
/// @dev Implementing contracts MUST include initialization guards to prevent
///      Uninitialized Proxy/Implementation Front-running attacks. Specifically:
///      - The implementation contract's constructor MUST call _disableInitializers()
///        (or equivalent) to prevent unauthorized initialization.
///      - The initialize() function MUST use an initializer modifier to ensure
///        it can only be called once.
///      - Access control (e.g., only ProxyAdmin or ProxyAdmin owner) SHOULD be
///        enforced on the initialize() function.
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

    function __constructor__() external;
}
