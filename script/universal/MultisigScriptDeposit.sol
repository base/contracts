// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {CBMulticall} from "src/utils/CBMulticall.sol";

import {MultisigScript} from "./MultisigScript.sol";
import {Enum} from "./IGnosisSafe.sol";

/// @notice Interface for OptimismPortal2's depositTransaction function
interface IOptimismPortal2 {
    /// @notice Creates a deposit transaction on L2
    /// @param _to Target address on L2
    /// @param _value ETH value to send with the transaction
    /// @param _gasLimit Minimum gas limit for L2 execution
    /// @param _isCreation Whether the transaction creates a contract
    /// @param _data Calldata for the L2 transaction
    function depositTransaction(address _to, uint256 _value, uint64 _gasLimit, bool _isCreation, bytes memory _data)
        external
        payable;
}

/// @title MultisigScriptDeposit
/// @notice Extension of MultisigScript for L1 → L2 deposit transactions.
///
/// @dev This contract simplifies the creation of L1 multisig transactions that trigger actions on L2
///      via the OptimismPortal's depositTransaction mechanism. Task writers only need to define the
///      L2 calls they want to execute; this contract handles wrapping them in the appropriate
///      depositTransaction call automatically.
///
///      Example usage:
///      ```solidity
///      contract MyL2Task is MultisigScriptDeposit {
///          function _ownerSafe() internal view override returns (address) {
///              return vm.envAddress("OWNER_SAFE");
///          }
///
///          function _buildL2Calls() internal view override returns (CBMulticall.Call3Value[] memory) {
///              CBMulticall.Call3Value[] memory calls = new CBMulticall.Call3Value[](1);
///              calls[0] = CBMulticall.Call3Value({
///                  target: L2_CONTRACT,
///                  allowFailure: false,
///                  callData: abi.encodeCall(IL2Contract.someFunction, (arg1, arg2)),
///                  value: 0
///              });
///              return calls;
///          }
///      }
///      ```
///
///      The example above uses the default implementation for `_optimismPortal()` (chain-based).
///      Task writers must set the `L2_GAS_LIMIT` environment variable or override `_l2GasLimit()`.
///
/// @dev Future Enhancements:
///      1. L2 Post-Check Hook: Currently, `_postCheck` runs on L1 and cannot verify L2 state changes.
///         A future enhancement could add an `_postCheckL2` hook that forks L2 state and simulates
///         the deposit transaction's effect. This is non-trivial because deposit transactions are
///         not immediately reflected on L2.
abstract contract MultisigScriptDeposit is MultisigScript {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Constants                                    ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice OptimismPortalProxy address on L1 Mainnet (for Base Mainnet)
    address internal constant OPTIMISM_PORTAL_MAINNET = 0x49048044D57e1C92A77f79988d21Fa8fAF74E97e;

    /// @notice OptimismPortalProxy address on L1 Sepolia (for Base Sepolia)
    address internal constant OPTIMISM_PORTAL_SEPOLIA = 0x49f53e41452C74589E85cA1677426Ba426459e85;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Virtual Functions                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Returns the OptimismPortal address on L1
    /// @dev Default implementation returns the correct address based on chain ID.
    ///      Supports L1 Mainnet (chain 1) and L1 Sepolia (chain 11155111).
    ///      Override this function for other chains or custom portal addresses.
    function _optimismPortal() internal view virtual returns (address) {
        if (block.chainid == 1) {
            return OPTIMISM_PORTAL_MAINNET;
        } else if (block.chainid == 11155111) {
            return OPTIMISM_PORTAL_SEPOLIA;
        }
        revert("MultisigScriptDeposit: unsupported chain, override _optimismPortal()");
    }

    /// @notice Returns the minimum gas limit for L2 execution
    /// @dev Default implementation reads from the `L2_GAS_LIMIT` environment variable.
    ///      All signers must use the same gas limit to produce matching signatures.
    ///
    ///      To specify a fixed gas limit, override this function in your task contract:
    ///      ```solidity
    ///      function _l2GasLimit() internal pure override returns (uint64) {
    ///          return 200_000; // Your estimated gas limit
    ///      }
    ///      ```
    ///
    ///      Common gas limit starting points:
    ///      - Single simple call: 100,000 - 200,000
    ///      - Multiple calls or complex operations: 500,000+
    ///
    ///      If the gas limit is too low, the L2 transaction will fail but the deposit
    ///      will still be recorded (ETH may be stuck until manually recovered).
    function _l2GasLimit() internal view virtual returns (uint64) {
        return uint64(vm.envUint("L2_GAS_LIMIT"));
    }

    /// @notice Build the calls that will be executed on L2
    /// @dev Task writers implement this to define what actions should occur on L2.
    ///      These calls will be batched into a single CBMulticall.aggregate3Value call
    ///      and wrapped in a depositTransaction to the OptimismPortal.
    ///
    ///      The `value` field in each Call3Value struct specifies ETH to send with that
    ///      specific L2 call. The total ETH will be bridged via the deposit transaction.
    /// @return calls Array of calls to execute on L2 via CBMulticall
    function _buildL2Calls() internal view virtual returns (CBMulticall.Call3Value[] memory);

    //////////////////////////////////////////////////////////////////////////////////////
    ///                              Overridden Functions                              ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Wraps L2 calls in a depositTransaction to the OptimismPortal
    /// @dev Task writers should NOT override this function. Instead, implement `_buildL2Calls`
    ///      to define the L2 operations. This function handles the L1 deposit wrapping automatically.
    ///
    ///      The L2 calls are encoded as a CBMulticall.aggregate3Value call, which is then
    ///      passed as the data payload to OptimismPortal.depositTransaction. This allows
    ///      multiple L2 operations to be batched into a single deposit transaction.
    ///
    ///      ETH bridging: If any L2 calls include a non-zero `value`, the total ETH is
    ///      summed and sent with the deposit transaction. The CBMulticall.aggregate3Value
    ///      function on L2 automatically distributes the ETH to each call according to its
    ///      specified `value` field - no additional developer action is required.
    function _buildCalls() internal view virtual override returns (Call[] memory) {
        CBMulticall.Call3Value[] memory l2Calls = _buildL2Calls();
        uint256 totalValue = _sumL2CallValues(l2Calls);

        // Encode L2 calls as a multicall
        // Note: We use aggregate3Value to support per-call ETH distribution on L2
        bytes memory l2Data = abi.encodeCall(CBMulticall.aggregate3Value, (l2Calls));

        // Wrap in depositTransaction call to OptimismPortal
        Call[] memory l1Calls = new Call[](1);
        l1Calls[0] = Call({
            operation: Enum.Operation.Call,
            target: _optimismPortal(),
            data: abi.encodeCall(
                IOptimismPortal2.depositTransaction,
                (
                    CB_MULTICALL, // L2 target: CBMulticall at same address on L2
                    totalValue, // ETH to bridge
                    _l2GasLimit(), // Gas limit for L2 execution
                    false, // Not a contract creation
                    l2Data // Encoded multicall
                )
            ),
            value: totalValue
        });

        return l1Calls;
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                              Internal Functions                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Sums the ETH values from an array of L2 calls
    /// @param l2Calls The array of L2 calls to sum values from
    /// @return total The total ETH value across all calls
    function _sumL2CallValues(CBMulticall.Call3Value[] memory l2Calls) internal pure returns (uint256 total) {
        for (uint256 i; i < l2Calls.length; i++) {
            total += l2Calls[i].value;
        }
        return total;
    }
}
