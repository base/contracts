// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Types } from "src/libraries/Types.sol";

/// @title Events
/// @dev Contains various events that are tested against. This contract needs to
///      exist until we either modularize the implementations or use a newer version of
///      solc that allows for referencing events from other contracts.
abstract contract Events {
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event TransactionDeposited(address indexed from, address indexed to, uint256 indexed version, bytes opaqueData);

    event WithdrawalFinalized(bytes32 indexed withdrawalHash, bool success);
    event WithdrawalProven(bytes32 indexed withdrawalHash, address indexed from, address indexed to);
    event WithdrawalProvenExtension1(bytes32 indexed withdrawalHash, address indexed proofSubmitter);

    event SentMessage(address indexed target, address sender, bytes message, uint256 messageNonce, uint256 gasLimit);
    event SentMessageExtension1(address indexed sender, uint256 value);
    event MessagePassed(
        uint256 indexed nonce,
        address indexed sender,
        address indexed target,
        uint256 value,
        uint256 gasLimit,
        bytes data,
        bytes32 withdrawalHash
    );
    event WithdrawerBalanceBurnt(uint256 indexed amount);
    event RelayedMessage(bytes32 indexed msgHash);
    event FailedRelayedMessage(bytes32 indexed msgHash);

    event Withdrawal(uint256 value, address to, address from);
    event Withdrawal(uint256 value, address to, address from, Types.WithdrawalNetwork withdrawalNetwork);

    event ETHDepositInitiated(address indexed from, address indexed to, uint256 amount, bytes data);

    event ETHWithdrawalFinalized(address indexed from, address indexed to, uint256 amount, bytes data);

    event ERC20DepositInitiated(
        address indexed l1Token, address indexed l2Token, address indexed from, address to, uint256 amount, bytes data
    );

    event ERC20WithdrawalFinalized(
        address indexed l1Token, address indexed l2Token, address indexed from, address to, uint256 amount, bytes data
    );

    event WithdrawalInitiated(
        address indexed l1Token, address indexed l2Token, address indexed from, address to, uint256 amount, bytes data
    );

    event DepositFinalized(
        address indexed l1Token, address indexed l2Token, address indexed from, address to, uint256 amount, bytes data
    );

    event ETHBridgeInitiated(address indexed from, address indexed to, uint256 amount, bytes data);

    event ETHBridgeFinalized(address indexed from, address indexed to, uint256 amount, bytes data);

    event ERC20BridgeInitiated(
        address indexed localToken,
        address indexed remoteToken,
        address indexed from,
        address to,
        uint256 amount,
        bytes data
    );

    event ERC20BridgeFinalized(
        address indexed localToken,
        address indexed remoteToken,
        address indexed from,
        address to,
        uint256 amount,
        bytes data
    );

    event Paused(address identifier);

    event Unpaused(address identifier);

    event PauseExtended(address identifier);
}
