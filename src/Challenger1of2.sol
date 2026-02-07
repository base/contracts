// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {Address} from "@openzeppelin/contracts/utils/Address.sol";

// solhint-disable max-line-length

/// @title Challenger1of2
///
/// @notice This contract serves the role of the Challenger, defined in L2OutputOracle.sol:
///         https://github.com/ethereum-optimism/optimism/blob/3580bf1b41d80fcb2b895d5610836bfad27fc989/packages/contracts-bedrock/contracts/L1/L2OutputOracle.sol
///         It enforces a simple 1 of 2 design, where neither party can remove the other's permissions to execute a
///         Challenger call.
/// @custom:deprecated This contract was the original challenger before fault proofs were introduced. It no longer serves a purpose.
contract Challenger1of2 {
    using Address for address;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Constants                                    ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @dev The address of Optimism's signer (likely a multisig)
    address public immutable OP_SIGNER;

    /// @dev The address of counterparty's signer (likely a multisig)
    address public immutable OTHER_SIGNER;

    /// @dev The address of the L2OutputOracleProxy contract.
    address public immutable L2_OUTPUT_ORACLE_PROXY;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                     Events                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @dev Emitted when a Challenger call is made by a signer.
    ///
    /// @param caller The signer making the call.
    /// @param data   The data of the call being made.
    /// @param result The result of the call being made.
    event ChallengerCallExecuted(address indexed caller, bytes data, bytes result);

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                  Constructor                                   ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @dev Constructor to set the values of the constants.
    ///
    /// @param opSigner            Address of Optimism signer.
    /// @param otherSigner         Address of counterparty signer.
    /// @param l2OutputOracleProxy Address of the L2OutputOracleProxy contract.
    constructor(address opSigner, address otherSigner, address l2OutputOracleProxy) {
        require(opSigner != address(0), "Challenger1of2: opSigner cannot be zero address");
        require(otherSigner != address(0), "Challenger1of2: otherSigner cannot be zero address");
        require(l2OutputOracleProxy.isContract(), "Challenger1of2: l2OutputOracleProxy must be a contract");

        OP_SIGNER = opSigner;
        OTHER_SIGNER = otherSigner;
        L2_OUTPUT_ORACLE_PROXY = l2OutputOracleProxy;
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               External Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Executes a call as the Challenger (must be called by Optimism or counterparty signer).
    ///
    /// @param data Data for function call.
    function execute(bytes memory data) external {
        require(
            msg.sender == OTHER_SIGNER || msg.sender == OP_SIGNER,
            "Challenger1of2: must be an approved signer to execute"
        );

        bytes memory result = Address.functionCall({
            target: L2_OUTPUT_ORACLE_PROXY, data: data, errorMessage: "Challenger1of2: failed to execute"
        });

        emit ChallengerCallExecuted({caller: msg.sender, data: data, result: result});
    }
}
