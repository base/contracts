// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {IMulticall3} from "lib/forge-std/src/interfaces/IMulticall3.sol";

import {MultisigScript} from "./MultisigScript.sol";

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
///          function _optimismPortal() internal view override returns (address) {
///              return vm.envAddress("L1_PORTAL");
///          }
///
///          function _l2GasLimit() internal view override returns (uint64) {
///              return 200_000; // Estimated gas for L2 execution
///          }
///
///          function _ownerSafe() internal view override returns (address) {
///              return vm.envAddress("OWNER_SAFE");
///          }
///
///          function _buildL2Calls() internal view override returns (IMulticall3.Call3Value[] memory) {
///              IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
///              calls[0] = IMulticall3.Call3Value({
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
/// @dev Future Enhancements:
///      1. L2 Post-Check Hook: Currently, `_postCheck` runs on L1 and cannot verify L2 state changes.
///         A future enhancement could add an `_postCheckL2` hook that forks L2 state and simulates
///         the deposit transaction's effect. This is non-trivial because deposit transactions are
///         not immediately reflected on L2.
///
///      2. Per-Call Gas Estimation: Currently, `_l2GasLimit` is a single value covering the entire
///         L2 multicall. A future enhancement could allow each L2 call to specify its own gas
///         requirement, with the total being calculated automatically.
abstract contract MultisigScriptDeposit is MultisigScript {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Virtual Functions                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Returns the OptimismPortal address on L1
    /// @dev This is the portal contract that will be called to initiate the L2 deposit
    function _optimismPortal() internal view virtual returns (address);

    /// @notice Returns the minimum gas limit for L2 execution
    /// @dev Task writers must estimate the gas required for their L2 calls to execute.
    ///      This value should account for:
    ///      - Gas for the CBMulticall.aggregate3Value call overhead
    ///      - Gas for each individual L2 call
    ///      - A safety margin for any unexpected gas consumption
    ///
    ///      If the gas limit is too low, the L2 transaction will fail but the deposit
    ///      will still be recorded (ETH will be stuck until manually recovered).
    ///
    ///      Common starting points:
    ///      - Single simple call: 100,000 - 200,000
    ///      - Multiple calls or complex operations: 500,000+
    ///
    ///      Future enhancement: This could be expanded to calculate gas automatically
    ///      from per-call gas estimates provided in an extended Call3Value struct.
    function _l2GasLimit() internal view virtual returns (uint64);

    /// @notice Build the calls that will be executed on L2
    /// @dev Task writers implement this to define what actions should occur on L2.
    ///      These calls will be batched into a single CBMulticall.aggregate3Value call
    ///      and wrapped in a depositTransaction to the OptimismPortal.
    ///
    ///      The `value` field in each Call3Value struct specifies ETH to send with that
    ///      specific L2 call. The total ETH will be bridged via the deposit transaction.
    /// @return calls Array of calls to execute on L2 via CBMulticall
    function _buildL2Calls() internal view virtual returns (IMulticall3.Call3Value[] memory);

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
    function _buildCalls() internal view virtual override returns (IMulticall3.Call3Value[] memory) {
        IMulticall3.Call3Value[] memory l2Calls = _buildL2Calls();

        // Sum ETH values from L2 calls for bridging
        uint256 totalValue = 0;
        for (uint256 i; i < l2Calls.length; i++) {
            totalValue += l2Calls[i].value;
        }

        // Encode L2 calls as a multicall
        // Note: We use aggregate3Value to support per-call ETH distribution on L2
        bytes memory l2Data = abi.encodeCall(IMulticall3.aggregate3Value, (l2Calls));

        // Wrap in depositTransaction call to OptimismPortal
        IMulticall3.Call3Value[] memory l1Calls = new IMulticall3.Call3Value[](1);
        l1Calls[0] = IMulticall3.Call3Value({
            target: _optimismPortal(),
            allowFailure: false,
            callData: abi.encodeCall(
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
}

