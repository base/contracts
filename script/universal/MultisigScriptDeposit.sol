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
///      The example above uses default implementations for `_optimismPortal()` (chain-based) and
///      `_l2GasLimit()` (automatic estimation via L2 fork). Task writers can override these if needed.
///
/// @dev Future Enhancements:
///      1. L2 Post-Check Hook: Currently, `_postCheck` runs on L1 and cannot verify L2 state changes.
///         A future enhancement could add an `_postCheckL2` hook that forks L2 state and simulates
///         the deposit transaction's effect. This is non-trivial because deposit transactions are
///         not immediately reflected on L2.
///
///      2. Per-Call Gas Hints: With automatic gas estimation via L2 fork, per-call gas hints are
///         less critical. However, they could still be useful when L2 RPC is unavailable or when
///         task writers want manual control without overriding `_l2GasLimit()` entirely.
///
/// @dev Testing Note:
///      Unit tests override `_l2GasLimit()` directly to avoid external dependencies. The L2 gas
///      estimation mechanism is tested via an integration test that requires `L2_RPC_URL`:
///      `L2_RPC_URL=<rpc> forge test --match-test test_integration_gasEstimation -vvv`
///      This integration test is automatically skipped in CI when `L2_RPC_URL` is not set.
abstract contract MultisigScriptDeposit is MultisigScript {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Constants                                    ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice OptimismPortalProxy address on L1 Mainnet (for Base Mainnet)
    address internal constant OPTIMISM_PORTAL_MAINNET = 0x49048044D57e1C92A77f79988d21Fa8fAF74E97e;

    /// @notice OptimismPortalProxy address on L1 Sepolia (for Base Sepolia)
    address internal constant OPTIMISM_PORTAL_SEPOLIA = 0x49f53e41452C74589E85cA1677426Ba426459e85;

    /// @notice Gas estimation safety buffer (50% overhead)
    /// @dev Applied to the estimated gas to account for variations in execution
    uint256 internal constant GAS_ESTIMATION_BUFFER_PERCENT = 150;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                 State Variables                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Cached L2 gas limit from estimation
    uint64 private _cachedL2GasLimit;

    /// @notice Whether the L2 gas limit has been cached
    bool private _l2GasLimitCached;

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
    /// @dev Default implementation estimates gas by forking L2 and simulating the call.
    ///      Requires the `L2_RPC_URL` environment variable to be set.
    ///
    ///      To manually specify a gas limit instead of using automatic estimation,
    ///      override this function in your task contract:
    ///      ```solidity
    ///      function _l2GasLimit() internal view override returns (uint64) {
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
        require(_l2GasLimitCached, "MultisigScriptDeposit: L2 gas limit not estimated, ensure L2_RPC_URL is set");
        return _cachedL2GasLimit;
    }

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
    ///                             Overridden Entry Points                            ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Override sign to ensure L2 gas is estimated before building calls
    function sign(address[] memory safes) public virtual override {
        _ensureL2GasLimitCached();
        super.sign(safes);
    }

    /// @notice Override approve to ensure L2 gas is estimated before building calls
    function approve(address[] memory safes, bytes memory signatures) public virtual override {
        _ensureL2GasLimitCached();
        super.approve(safes, signatures);
    }

    /// @notice Override simulate to ensure L2 gas is estimated before building calls
    function simulate(bytes memory signatures) public virtual override {
        _ensureL2GasLimitCached();
        super.simulate(signatures);
    }

    /// @notice Override run to ensure L2 gas is estimated before building calls
    function run(bytes memory signatures) public virtual override {
        _ensureL2GasLimitCached();
        super.run(signatures);
    }

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
        uint256 totalValue = _sumL2CallValues(l2Calls);

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

    //////////////////////////////////////////////////////////////////////////////////////
    ///                              Internal Functions                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Ensures the L2 gas limit is cached before building calls
    /// @dev Called by overridden entry points (sign, run, etc.) to trigger estimation.
    ///      If you override `_l2GasLimit()` to return a fixed value, you should also
    ///      override this function to be a no-op to skip the L2_RPC_URL requirement.
    function _ensureL2GasLimitCached() internal virtual {
        if (_l2GasLimitCached) return;

        // Get L2 RPC URL for forking
        string memory l2RpcUrl;
        try vm.envString("L2_RPC_URL") returns (string memory url) {
            l2RpcUrl = url;
        } catch {
            revert(
                "MultisigScriptDeposit: L2_RPC_URL env var required for gas estimation. "
                "Alternatively, override _l2GasLimit() to specify a manual gas limit."
            );
        }

        // Estimate gas via L2 fork
        _cachedL2GasLimit = _estimateL2GasViaFork(l2RpcUrl);
        _l2GasLimitCached = true;
    }

    /// @notice Estimates L2 gas by forking the L2 chain and simulating the multicall
    /// @param l2RpcUrl The RPC URL of the L2 chain to fork
    /// @return estimatedGas The estimated gas limit with safety buffer applied
    function _estimateL2GasViaFork(string memory l2RpcUrl) internal returns (uint64) {
        // Build L2 call data
        IMulticall3.Call3Value[] memory l2Calls = _buildL2Calls();
        bytes memory l2Data = abi.encodeCall(IMulticall3.aggregate3Value, (l2Calls));
        uint256 totalValue = _sumL2CallValues(l2Calls);

        // Store current fork (if any) to restore later
        uint256 originalFork;
        bool hadActiveFork;
        try vm.activeFork() returns (uint256 forkId) {
            originalFork = forkId;
            hadActiveFork = true;
        } catch {
            hadActiveFork = false;
        }

        // Create and select L2 fork
        uint256 l2Fork = vm.createFork(l2RpcUrl);
        vm.selectFork(l2Fork);

        // Fund the CBMulticall address if we need ETH for the simulation
        if (totalValue > 0) {
            vm.deal(CB_MULTICALL, totalValue);
        }

        // Measure gas for the L2 call
        uint256 gasBefore = gasleft();
        (bool success,) = CB_MULTICALL.call{value: totalValue}(l2Data);
        uint256 gasUsed = gasBefore - gasleft();

        // Restore original fork if there was one
        if (hadActiveFork) {
            vm.selectFork(originalFork);
        }

        require(success, "MultisigScriptDeposit: L2 gas estimation failed, call reverted");

        // Apply safety buffer and return
        uint256 estimatedGas = (gasUsed * GAS_ESTIMATION_BUFFER_PERCENT) / 100;

        // Ensure we don't overflow uint64
        require(estimatedGas <= type(uint64).max, "MultisigScriptDeposit: estimated gas exceeds uint64");

        return uint64(estimatedGas);
    }

    /// @notice Sums the ETH values from an array of L2 calls
    /// @param l2Calls The array of L2 calls to sum values from
    /// @return total The total ETH value across all calls
    function _sumL2CallValues(IMulticall3.Call3Value[] memory l2Calls) internal pure returns (uint256 total) {
        for (uint256 i; i < l2Calls.length; i++) {
            total += l2Calls[i].value;
        }
    }
}
