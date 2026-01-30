// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Script} from "lib/forge-std/src/Script.sol";
import {stdJson} from "lib/forge-std/src/StdJson.sol";
import {Vm} from "lib/forge-std/src/Vm.sol";

import {ICBMulticall, Call3, Call3Value} from "../../src/utils/ICBMulticall.sol";

import {IGnosisSafe, Enum} from "./IGnosisSafe.sol";
import {Signatures} from "./Signatures.sol";
import {StateDiff} from "./StateDiff.sol";
import {Simulation} from "./Simulation.sol";

/// @title MultisigScript
/// @notice Script builder for Forge scripts that require signatures from Safes. Supports both non-nested
///         Safes, as well as nested Safes of arbitrary depth (Safes where the signers are other Safes).
///
/// 1. Non-nested example:
///
/// Setup:
/// ┌───────┐┌───────┐
/// │Signer1││Signer2│
/// └┬──────┘└┬──────┘
/// ┌▽────────▽┐
/// │Multisig  │
/// └┬─────────┘
/// ┌▽─────────┐
/// │ProxyAdmin│
/// └──────────┘
///
/// Sequence:
/// ┌───────┐┌───────┐┌───────────┐┌──────────────┐
/// │Signer1││Signer2││Facilitator││MultisigScript│
/// └───┬───┘└───┬───┘└─────┬─────┘└───────┬──────┘
///     │        │     sign()              │
///     │─────────────────────────────────>│
///     │      <sig1>       │              │
///     │──────────────────>│              │
///     │        │         sign()          │
///     │        │────────────────────────>│
///     │        │  <sig2>  │              │
///     │        │─────────>│              │
///     │        │          │run(sig1,sig2)│
///     │        │          │─────────────>│
///
///
/// 2. Single-layer nested example:
///
/// Setup:
/// ┌───────┐┌───────┐┌───────┐┌───────┐
/// │Signer1││Signer2││Signer3││Signer4│
/// └┬──────┘└┬──────┘└┬──────┘└┬──────┘
/// ┌▽────────▽┐┌──────▽────────▽┐
/// │Safe1     ││Safe2           │
/// └┬─────────┘└┬───────────────┘
/// ┌▽───────────▽┐
/// │Safe3        │
/// └┬────────────┘
/// ┌▽─────────┐
/// │ProxyAdmin│
/// └──────────┘
///
/// Sequence:
/// ┌───────┐┌───────┐┌───────┐┌───────┐┌───────────┐          ┌──────────────┐
/// │Signer1││Signer2││Signer3││Signer4││Facilitator│          │MultisigScript│
/// └───┬───┘└───┬───┘└───┬───┘└───┬───┘└─────┬─────┘          └───────┬──────┘
///     │        │        │       sign(Safe1) │                        │
///     │─────────────────────────────────────────────────────────────>│
///     │        │      <sig1>     │          │                        │
///     │────────────────────────────────────>│                        │
///     │        │        │        │   sign(Safe1)                     │
///     │        │────────────────────────────────────────────────────>│
///     │        │        │  <sig2>│          │                        │
///     │        │───────────────────────────>│                        │
///     │        │        │        │          │approve(Safe1,sig1|sig2)│
///     │        │        │        │          │───────────────────────>│
///     │        │        │        │       sign(Safe2)                 │
///     │        │        │───────────────────────────────────────────>│
///     │        │        │      <sig3>       │                        │
///     │        │        │──────────────────>│                        │
///     │        │        │        │          │ sign(Safe2)            │
///     │        │        │        │──────────────────────────────────>│
///     │        │        │        │  <sig4>  │                        │
///     │        │        │        │─────────>│                        │
///     │        │        │        │          │approve(Safe2,sig3|sig4)│
///     │        │        │        │          │───────────────────────>│
///     │        │        │        │          │         run()          │
///     │        │        │        │          │───────────────────────>│
///
///
/// 3. Multi-layer nested example:
///
/// Setup:
/// ┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐
/// │Signer1││Signer2││Signer3││Signer4││Signer5││Signer6│
/// └┬──────┘└┬──────┘└┬──────┘└┬──────┘└┬──────┘└┬──────┘
/// ┌▽────────▽┐┌──────▽────────▽┐┌──────▽────────▽┐
/// │Safe1     ││Safe2           ││Safe3           │
/// └┬─────────┘└┬───────────────┘└┬───────────────┘
/// ┌▽───────────▽┐                │
/// │Safe4        │                │
/// └┬────────────┘                │
/// ┌▽─────────────────────────────▽┐
/// │Safe5                          │
/// └┬──────────────────────────────┘
/// ┌▽─────────┐
/// │ProxyAdmin│
/// └──────────┘
///
/// Sequence:
/// ┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐┌───────────┐                ┌──────────────┐
/// │Signer1││Signer2││Signer3││Signer4││Signer5││Signer6││Facilitator│                │MultisigScript│
/// └───┬───┘└───┬───┘└───┬───┘└───┬───┘└───┬───┘└───┬───┘└─────┬─────┘                └───────┬──────┘
///     │        │        │        │       sign(Safe1,Safe4)    │                              │
///     │─────────────────────────────────────────────────────────────────────────────────────>│
///     │        │        │      <sig1>     │        │          │                              │
///     │──────────────────────────────────────────────────────>│                              │
///     │        │        │        │        │   sign(Safe1,Safe4)                              │
///     │        │────────────────────────────────────────────────────────────────────────────>│
///     │        │        │        │  <sig2>│        │          │                              │
///     │        │─────────────────────────────────────────────>│                              │
///     │        │        │        │        │        │          │approve(Safe1,Safe4,sig1|sig2)│
///     │        │        │        │        │        │          │─────────────────────────────>│
///     │        │        │        │        │       sign(Safe2,Safe4)                          │
///     │        │        │───────────────────────────────────────────────────────────────────>│
///     │        │        │        │      <sig3>     │          │                              │
///     │        │        │────────────────────────────────────>│                              │
///     │        │        │        │        │        │   sign(Safe2,Safe4)                     │
///     │        │        │        │──────────────────────────────────────────────────────────>│
///     │        │        │        │        │  <sig4>│          │                              │
///     │        │        │        │───────────────────────────>│                              │
///     │        │        │        │        │        │          │approve(Safe2,Safe4,sig3|sig4)│
///     │        │        │        │        │        │          │─────────────────────────────>│
///     │        │        │        │        │        │          │        approve(Safe4)        │
///     │        │        │        │        │        │          │─────────────────────────────>│
///     │        │        │        │        │        │          sign(Safe3)                    │
///     │        │        │        │        │─────────────────────────────────────────────────>│
///     │        │        │        │        │      <sig5>       │                              │
///     │        │        │        │        │──────────────────>│                              │
///     │        │        │        │        │        │          │    sign(Safe3)               │
///     │        │        │        │        │        │────────────────────────────────────────>│
///     │        │        │        │        │        │  <sig6>  │                              │
///     │        │        │        │        │        │─────────>│                              │
///     │        │        │        │        │        │          │   approve(Safe3,sig5|sig6)   │
///     │        │        │        │        │        │          │─────────────────────────────>│
///     │        │        │        │        │        │          │            run()             │
///     │        │        │        │        │        │          │─────────────────────────────>│
abstract contract MultisigScript is Script {
    address internal constant CB_MULTICALL = 0xA8B8CA1d6F0F5Ce63dCEA9121A01b302c5801303;

    /// @notice A struct representing a call to a contract.
    ///
    /// @param operation The operation to perform on the contract.
    /// @param target The address of the contract to call.
    /// @param data The data to call the contract with.
    /// @param value The value to send with the call.
    struct Call {
        Enum.Operation operation;
        address target;
        bytes data;
        uint256 value;
    }

    /// @notice The available types of for call3 calls.
    enum Call3Type {
        DELEGATE_CALL,
        CALL,
        CALL_VALUE
    }

    /// @dev Event emitted from a `sign()` call containing the data to sign. Used in testing.
    event DataToSign(bytes data);

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Virtual Functions                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Returns the safe address to execute the final transaction from
    function _ownerSafe() internal view virtual returns (address);

    /// @notice Creates the calldata for signatures (`sign`), approvals (`approve`), and execution (`run`)
    function _buildCalls() internal view virtual returns (Call[] memory);

    /// @notice Follow up assertions to ensure that the script ran to completion.
    /// @dev Called after `sign` and `run`, but not `approve`.
    function _postCheck(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual;

    /// @notice Follow up assertions on state and simulation after a `sign` call.
    function _postSign(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual {}

    /// @notice Follow up assertions on state and simulation after a `approve` call.
    function _postApprove(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual {}

    /// @notice Follow up assertions on state and simulation after a `run` call.
    function _postRun(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual {}

    // Tenderly simulations can accept generic state overrides. This hook enables this functionality.
    // By default, an empty (no-op) override is returned.
    function _simulationOverrides() internal view virtual returns (Simulation.StateOverride[] memory overrides_) {}

    /// @notice Controls whether the safe tx is printed as hashes or structured EIP-712 data.
    ///
    /// @dev Override and return `true` to print hashed data (domain + message hash) instead of
    ///      the typed EIP-712 JSON structure. By default, returns `false` to use EIP-712 JSON.
    function _printDataHashes() internal view virtual returns (bool) {
        return true;
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                Public Functions                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// Step 1
    /// ======
    /// Generate a transaction approval data to sign. This method should be called by a threshold of
    /// multisig owners.
    ///
    /// For non-nested multisigs, the signatures can then be used to execute the transaction (see step 3).
    ///
    /// For nested multisigs, the signatures can be used to execute an approval transaction for each
    /// multisig (see step 2).
    ///
    /// @param safes A list of nested safes (excluding the executing safe returned by `_ownerSafe`).
    function sign(address[] memory safes) public {
        safes = _appendOwnerSafe({safes: safes});

        // Snapshot and restore Safe nonce after simulation, otherwise the data logged to sign
        // would not match the actual data we need to sign, because the simulation
        // would increment the nonce.
        uint256[] memory originalNonces = new uint256[](safes.length);
        for (uint256 i; i < safes.length; i++) {
            originalNonces[i] = _getNonce({safe: safes[i]});
        }

        Call[] memory callsChain = _buildCallsChain({safes: safes});

        vm.startMappingRecording();
        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _simulateForSigner({safes: safes, callsChain: callsChain});
        (StateDiff.MappingParent[] memory parents, string memory json) =
            StateDiff.collectStateDiff(StateDiff.CollectStateDiffOpts({accesses: accesses, simPayload: simPayload}));
        vm.stopMappingRecording();

        _postSign({accesses: accesses, simPayload: simPayload});
        _postCheck({accesses: accesses, simPayload: simPayload});

        // Restore the original nonce.
        for (uint256 i; i < safes.length; i++) {
            vm.store({target: safes[i], slot: Simulation.SAFE_NONCE_SLOT, value: bytes32(originalNonces[i])});
        }

        bytes memory txData = _encodeTransactionData({safe: safes[0], call: callsChain[0]});
        StateDiff.recordStateDiff({json: json, parents: parents, txData: txData, targetSafe: _ownerSafe()});

        _printDataToSign({safe: safes[0], call: callsChain[0]});
    }

    /// Step 1.1 (optional)
    /// ======
    /// Verify the signatures generated from step 1 are valid.
    /// This allows transactions to be pre-signed and stored safely before execution.
    ///
    /// @param safes      A list of nested safes (excluding the executing safe returned by `_ownerSafe`).
    /// @param signatures The signatures to verify (concatenated, 65-bytes per sig).
    function verify(address[] memory safes, bytes memory signatures) public view {
        safes = _appendOwnerSafe({safes: safes});

        Call[] memory callsChain = _buildCallsChain({safes: safes});
        _checkSignatures({safe: safes[0], call: callsChain[0], signatures: signatures});
    }

    /// Step 2 (optional for non-nested setups)
    /// ======
    /// Execute an approval transaction. This method should be called by a facilitator
    /// (non-signer), once for each of the multisigs involved in the nested multisig,
    /// after collecting a threshold of signatures for each multisig (see step 1).
    ///
    /// For multiple layers of nesting, this should be called for each layer of nesting (once
    /// the inner multisigs have registered their approval). The array of safes passed to
    /// `safes` should get smaller by one for each layer of nesting.
    ///
    /// @param safes      A list of nested safes (excluding the executing safe returned by `_ownerSafe`).
    /// @param signatures The signatures from step 1 (concatenated, 65-bytes per sig)
    function approve(address[] memory safes, bytes memory signatures) public {
        safes = _appendOwnerSafe({safes: safes});

        Call[] memory callsChain = _buildCallsChain({safes: safes});
        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _executeTransaction({safe: safes[0], call: callsChain[0], signatures: signatures, broadcast: true});

        _postApprove({accesses: accesses, simPayload: simPayload});
    }

    /// Step 2.1 (optional)
    /// ======
    /// Simulate the transaction. This method should be called by a facilitator (non-signer), after all of the
    /// signatures have been collected (non-nested case, see step 1), or the approval transactions have been
    /// submitted onchain (nested case, see step 2, in which case `signatures` can be empty).
    ///
    /// Differs from `run` in that you can override the safe nonce for simulation purposes.
    ///
    /// @param signatures The signatures from step 1 (concatenated, 65-bytes per sig)
    function simulate(bytes memory signatures) public {
        address ownerSafe = _ownerSafe();
        Call[] memory callsChain = _buildCallsChain({safes: _toArray(ownerSafe)});

        vm.store({target: ownerSafe, slot: Simulation.SAFE_NONCE_SLOT, value: bytes32(_getNonce({safe: ownerSafe}))});

        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _executeTransaction({safe: ownerSafe, call: callsChain[0], signatures: signatures, broadcast: false});

        _postRun({accesses: accesses, simPayload: simPayload});
        _postCheck({accesses: accesses, simPayload: simPayload});
    }

    /// Step 3
    /// ======
    /// Execute the transaction. This method should be called by a facilitator (non-signer), after all of the
    /// signatures have been collected (non-nested case, see step 1), or the approval transactions have been
    /// submitted onchain (nested case, see step 2, in which case `signatures` can be empty).
    ///
    /// @param signatures The signatures from step 1 (concatenated, 65-bytes per sig)
    function run(bytes memory signatures) public {
        address ownerSafe = _ownerSafe();
        Call[] memory callsChain = _buildCallsChain({safes: _toArray(ownerSafe)});

        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _executeTransaction({safe: ownerSafe, call: callsChain[0], signatures: signatures, broadcast: true});

        _postRun({accesses: accesses, simPayload: simPayload});
        _postCheck({accesses: accesses, simPayload: simPayload});
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Internal Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Appends the owner safe to the list of safes.
    ///
    /// @param safes The list of safes to append the owner safe to.
    ///
    /// @return The list of safes with the owner safe appended.
    function _appendOwnerSafe(address[] memory safes) internal view returns (address[] memory) {
        address[] memory extendedSafes = new address[](safes.length + 1);
        for (uint256 i; i < safes.length; i++) {
            extendedSafes[i] = safes[i];
        }
        extendedSafes[extendedSafes.length - 1] = _ownerSafe();
        return extendedSafes;
    }

    /// @notice Wrapper around `_buildCalls` that checks that the script calls are valid.
    ///
    /// @return The list of calls.
    function _buildCallsChecked() internal view returns (Call[] memory) {
        Call[] memory scriptCalls = _buildCalls();
        for (uint256 i; i < scriptCalls.length; i++) {
            Call memory call = scriptCalls[i];

            require(
                call.operation == Enum.Operation.Call || call.value == 0,
                "MultisigScript::_buildCallsChecked: Value must be 0 for delegate calls"
            );
        }

        return scriptCalls;
    }

    /// @notice Build the list of safe-to-safe approval calls followed by the final script call.
    ///
    /// @param safes The list of safes to build the calls chain for.
    ///
    /// @return callsChain The calls chain for the given safes.
    function _buildCallsChain(address[] memory safes) internal view returns (Call[] memory callsChain) {
        // Build the script calls.
        Call[] memory scriptCalls = _buildCallsChecked();

        // Build the final script call.
        Call memory aggregatedScriptCall = _buildAggregatedScriptCall({scriptCalls: scriptCalls});

        // The very last call is the actual call to execute
        callsChain = new Call[](safes.length);
        callsChain[callsChain.length - 1] = aggregatedScriptCall;

        // The first n-1 calls are the nested approval calls. We build the approvals backwards, starting from the last safe.
        for (uint256 i = safes.length - 1; i > 0; i--) {
            address safe = safes[i];
            Call memory callToApprove = callsChain[i];

            callsChain[i - 1] = _buildApproveCall({safe: safe, call: callToApprove});
        }
    }

    /// @notice Builds the aggregated script call.
    ///
    /// @param scriptCalls The list of script calls to aggregate.
    ///
    /// @return The aggregated script call.
    function _buildAggregatedScriptCall(Call[] memory scriptCalls) internal pure returns (Call memory) {
        // When there is only one call, we return it directly as there is no need to aggregate it into a Multicall call.
        if (scriptCalls.length == 1) {
            return scriptCalls[0];
        }

        Call3[] memory rootCalls = new Call3[](scriptCalls.length);
        uint256 rootCallsIndex;

        Call[] memory currentGroup = new Call[](scriptCalls.length);
        currentGroup[0] = scriptCalls[0];
        uint256 currentGroupIndex;

        for (uint256 i; i < scriptCalls.length; i++) {
            Call memory currentCall = scriptCalls[i];
            Call3Type currentType = _getCall3Type(currentCall);
            Call3Type groupType = _getCall3Type(currentGroup[0]);

            // If the current call has the same type as the current group, add it to the current group and continue.
            if (groupType == currentType) {
                currentGroup[currentGroupIndex] = currentCall;
                currentGroupIndex++;
                continue;
            }

            // Consume the current group and append the calls to the root calls.
            rootCallsIndex += _aggregateCalls({
                groupType: groupType,
                rootCalls: rootCalls,
                rootCallsIndex: rootCallsIndex,
                currentGroup: currentGroup,
                currentGroupIndex: currentGroupIndex
            });

            // Reset the current group (for the next group)
            currentGroup[0] = currentCall;
            currentGroupIndex = 1;
        }

        // Process the final group left in the current group.
        rootCallsIndex += _aggregateCalls({
            groupType: _getCall3Type(currentGroup[0]),
            rootCalls: rootCalls,
            rootCallsIndex: rootCallsIndex,
            currentGroup: currentGroup,
            currentGroupIndex: currentGroupIndex
        });

        // NOTE: When aggregating via a Multicall call, the root call is always a delegatecall to `aggregateDelegateCalls`
        //       as it offers the most flexibility and allows perofming any other type of call.
        return Call({
            operation: Enum.Operation.DelegateCall,
            target: CB_MULTICALL,
            data: abi.encodeCall(ICBMulticall.aggregateDelegateCalls, (rootCalls)),
            value: 0
        });
    }

    /// @notice Aggregates the current group of calls into a single Multicall call.
    ///
    /// @param groupType The type of the current group.
    /// @param rootCalls The root calls to append the calls to.
    /// @param rootCallsIndex The index of the root calls to append the calls to.
    /// @param currentGroup The current group of calls to consume.
    /// @param currentGroupIndex The index of the current group.
    ///
    /// @return rootCallsCount The number of root calls appended.
    function _aggregateCalls(
        Call3Type groupType,
        Call3[] memory rootCalls,
        uint256 rootCallsIndex,
        Call[] memory currentGroup,
        uint256 currentGroupIndex
    ) internal pure returns (uint256 rootCallsCount) {
        uint256 rootCallsIndexSaved = rootCallsIndex;

        // Append the call3 delegate calls directly to the root calls.
        if (groupType == Call3Type.DELEGATE_CALL) {
            for (uint256 j; j < currentGroupIndex; j++) {
                rootCalls[rootCallsIndex] = _toDelegateCall3(currentGroup[j]);
                rootCallsIndex++;
            }
        }
        // Otherwise aggregate the calls into a single Multicall call.
        else {
            Call3 memory rootCall;

            if (groupType == Call3Type.CALL) {
                Call3[] memory call3s = new Call3[](currentGroupIndex);
                for (uint256 j; j < currentGroupIndex; j++) {
                    call3s[j] = _toCall3(currentGroup[j]);
                }

                rootCall = Call3({
                    target: CB_MULTICALL,
                    allowFailure: false,
                    callData: abi.encodeCall(ICBMulticall.aggregate3, (call3s))
                });
            } else {
                Call3Value[] memory call3Values = new Call3Value[](currentGroupIndex);
                for (uint256 j; j < currentGroupIndex; j++) {
                    call3Values[j] = _toCall3Value(currentGroup[j]);
                }

                rootCall = Call3({
                    target: CB_MULTICALL,
                    allowFailure: false,
                    callData: abi.encodeCall(ICBMulticall.aggregate3Value, (call3Values))
                });
            }

            rootCalls[rootCallsIndex] = rootCall;
            rootCallsIndex++;
        }

        // Return the number of root calls appended.
        return rootCallsIndex - rootCallsIndexSaved;
    }

    /// @notice Builds the approve call (`approveHash`) for the given safe and call.
    ///
    /// @param safe The address of the safe to approve.
    /// @param call The call to approve.
    ///
    /// @return The approve call.
    function _buildApproveCall(address safe, Call memory call) internal view returns (Call memory) {
        bytes32 hash = _getTransactionHash({safe: safe, call: call});

        console.log("---\nNested hash for safe %s:", safe);
        console.logBytes32(hash);

        return Call({
            operation: Enum.Operation.Call,
            target: safe,
            data: abi.encodeCall(IGnosisSafe(safe).approveHash, (hash)),
            value: 0
        });
    }

    /// @notice Prints the data to sign for the given safe and call.
    ///
    /// @dev Uses `_printDataHashes()` to determine the output format:
    ///      - `true`: prints raw transaction data (hashes)
    ///      - `false` (default): prints EIP-712 JSON structure for hardware wallets
    ///
    /// @param safe The address of the safe to print the data to sign for.
    /// @param call The call to print the data to sign for.
    function _printDataToSign(address safe, Call memory call) internal {
        bytes memory txData = _printDataHashes()
            ? _encodeTransactionData({safe: safe, call: call})
            : _encodeEip712Json({safe: safe, call: call});

        emit DataToSign({data: txData});

        console.log("---\nIf submitting onchain, call Safe.approveHash on %s with the following hash:", safe);
        bytes32 hash = _getTransactionHash({safe: safe, call: call});
        console.logBytes32(hash);

        console.log("---\nData to sign:");
        console.log("vvvvvvvv");
        console.logBytes(txData);
        console.log("^^^^^^^^\n");

        console.log("########## IMPORTANT ##########");
        console.log(
            // solhint-disable-next-line max-line-length
            "Please make sure that the 'Data to sign' displayed above matches what you see in the simulation and on your hardware wallet."
        );
        console.log("This is a critical step that must not be skipped.");
        console.log("###############################");
    }

    /// @notice Executes the given transaction.
    ///
    /// @param safe The address of the safe to execute the transaction from.
    /// @param call The call to execute.
    /// @param signatures The signatures to use for the transaction.
    /// @param broadcast Whether to broadcast the transaction.
    ///
    /// @return The account accesses and simulation payload.
    function _executeTransaction(address safe, Call memory call, bytes memory signatures, bool broadcast)
        internal
        returns (Vm.AccountAccess[] memory, Simulation.Payload memory)
    {
        bytes32 hash = _getTransactionHash({safe: safe, call: call});
        signatures = Signatures.prepareSignatures({safe: safe, hash: hash, signatures: signatures});

        Call memory simCall = _buildExecTransactionCall({safe: safe, call: call, signatures: signatures});
        Simulation.logSimulationLink({to: safe, data: simCall.data, from: msg.sender});

        vm.startStateDiffRecording();
        bool success = _execTransaction({safe: safe, call: call, signatures: signatures, broadcast: broadcast});
        Vm.AccountAccess[] memory accesses = vm.stopAndReturnStateDiff();
        require(success, "MultisigScript::_executeTransaction: Transaction failed");
        require(accesses.length > 0, "MultisigScript::_executeTransaction: No state changes");

        // This can be used to e.g. call out to the Tenderly API and get additional
        // data about the state diff before broadcasting the transaction.
        Simulation.Payload memory simPayload = Simulation.Payload({
            from: msg.sender, to: safe, data: simCall.data, stateOverrides: new Simulation.StateOverride[](0)
        });
        return (accesses, simPayload);
    }

    /// @notice Simulates the given `callsChain` associated to the given `safes` as if initiated by `msg.sender`.
    ///
    /// @param safes The list of safes to simulate the transaction for.
    /// @param callsChain The list of calls to simulate the transaction for.
    ///
    /// @return The account accesses and simulation payload.
    function _simulateForSigner(address[] memory safes, Call[] memory callsChain)
        internal
        returns (Vm.AccountAccess[] memory, Simulation.Payload memory)
    {
        // Define the state overrides for the simulation.
        bytes32 firstCallDataHash = _getTransactionHash({safe: safes[0], call: callsChain[0]});
        Simulation.StateOverride[] memory overrides = _overrides({safes: safes, firstCallDataHash: firstCallDataHash});

        // Build the `execTransaction` calls chain for all the safe-to-safe approvals followed by the final script call.
        Call[] memory execTransactionCalls = _buildExecTransactionCalls({safes: safes, callsChain: callsChain});
        bytes memory txData = abi.encodeCall(ICBMulticall.aggregate3, (_toCall3s(execTransactionCalls)));
        console.logBytes(txData);

        console.log("---\nSimulation link:");
        Simulation.logSimulationLink({to: CB_MULTICALL, data: txData, from: msg.sender, overrides: overrides});

        // Forge simulation of the data logged in the link. If the simulation fails we revert to make it explicit that the simulation failed.
        Simulation.Payload memory simPayload =
            Simulation.Payload({to: CB_MULTICALL, data: txData, from: msg.sender, stateOverrides: overrides});
        Vm.AccountAccess[] memory accesses = Simulation.simulateFromSimPayload({simPayload: simPayload});
        return (accesses, simPayload);
    }

    /// @notice Wraps each of the given calls in a `execTransaction` call.
    ///
    /// @param safes The list of safes to execute the calls from.
    /// @param callsChain The list of calls to wrap in a `execTransaction` call.
    ///
    /// @return execTransactionCalls The list of `execTransaction` calls.
    function _buildExecTransactionCalls(address[] memory safes, Call[] memory callsChain)
        internal
        view
        returns (Call[] memory execTransactionCalls)
    {
        require(
            safes.length == callsChain.length,
            "MultisigScript::_buildExecTransactionCalls: Safes and callsChain must have the same length"
        );

        execTransactionCalls = new Call[](safes.length);
        for (uint256 i; i < safes.length; i++) {
            address signer = i == 0 ? msg.sender : safes[i - 1];

            execTransactionCalls[i] = _buildExecTransactionCall({
                safe: safes[i], call: callsChain[i], signatures: Signatures.genPrevalidatedSignature(signer)
            });
        }
    }

    // The state change simulation can set the threshold, owner address and/or nonce.
    // This allows simulation of the final transaction by overriding the threshold to 1.
    // State changes reflected in the simulation as a result of these overrides will
    // not be reflected in the prod execution.
    function _overrides(address[] memory safes, bytes32 firstCallDataHash)
        internal
        view
        returns (Simulation.StateOverride[] memory)
    {
        Simulation.StateOverride[] memory simOverrides = _simulationOverrides();
        Simulation.StateOverride[] memory overrides = new Simulation.StateOverride[](safes.length + simOverrides.length);

        uint256 nonce = _getNonce({safe: safes[0]});
        overrides[0] = Simulation.overrideSafeThresholdApprovalAndNonce({
            safe: safes[0], nonce: nonce, owner: msg.sender, dataHash: firstCallDataHash
        });

        for (uint256 i = 1; i < safes.length; i++) {
            overrides[i] =
                Simulation.overrideSafeThresholdAndNonce({safe: safes[i], nonce: _getNonce({safe: safes[i]})});
        }

        for (uint256 i; i < simOverrides.length; i++) {
            overrides[i + safes.length] = simOverrides[i];
        }

        return overrides;
    }

    // Get the nonce to use for the given safe, for signing and simulations.
    //
    // If you override it, ensure that the behavior is correct for all contexts.
    // As an example, if you are pre-signing a message that needs safe.nonce+1 (before
    // safe.nonce is executed), you should explicitly set the nonce value with an env var.
    // Overriding this method with safe.nonce+1 will cause issues upon execution because
    // the transaction hash will differ from the one signed.
    //
    // The process for determining a nonce override is as follows:
    //   1. We look for an env var of the name SAFE_NONCE_{UPPERCASE_SAFE_ADDRESS}. For example,
    //      SAFE_NONCE_0X6DF4742A3C28790E63FE933F7D108FE9FCE51EA4.
    //   2. If it exists, we use it as the nonce override for the safe.
    //   3. If it does not exist, we do the same for the SAFE_NONCE env var.
    //   4. Otherwise we fallback to the safe's current nonce (no override).
    function _getNonce(address safe) internal view virtual returns (uint256 nonce) {
        uint256 safeNonce = IGnosisSafe(safe).nonce();
        nonce = safeNonce;

        // first try SAFE_NONCE
        try vm.envUint({name: "SAFE_NONCE"}) {
            nonce = vm.envUint({name: "SAFE_NONCE"});
        } catch {}

        // then try SAFE_NONCE_{UPPERCASE_SAFE_ADDRESS}
        string memory envVarName = string.concat("SAFE_NONCE_", vm.toUppercase({input: vm.toString({value: safe})}));
        try vm.envUint({name: envVarName}) {
            nonce = vm.envUint({name: envVarName});
        } catch {}

        // print if any override
        if (nonce != safeNonce) {
            console.log("Overriding nonce for safe %s: %d -> %d", safe, safeNonce, nonce);
        }
    }

    /// @notice Returns the result of `encodeTransactionData` function from the given safe for the given call.
    ///
    /// @param safe The address of the safe that will execute the transaction.
    /// @param call The call to get the encoded transaction data for.
    ///
    /// @return The result of `encodeTransactionData` function from the given safe for the given call.
    function _encodeTransactionData(address safe, Call memory call) internal view returns (bytes memory) {
        return IGnosisSafe(safe)
            .encodeTransactionData({
                to: call.target,
                value: call.value,
                data: call.data,
                operation: call.operation,
                safeTxGas: 0,
                baseGas: 0,
                gasPrice: 0,
                gasToken: address(0),
                refundReceiver: address(0),
                _nonce: _getNonce(safe)
            });
    }

    /// @notice Encodes the transaction as EIP-712 structured JSON for hardware wallet signing.
    ///
    /// @param safe The address of the safe that will execute the transaction.
    /// @param call The call to encode.
    ///
    /// @return The EIP-712 JSON structure as bytes.
    function _encodeEip712Json(address safe, Call memory call) internal returns (bytes memory) {
        // EIP-712 type definitions for Safe transaction
        string memory types = '{"EIP712Domain":[' '{"name":"chainId","type":"uint256"},'
            '{"name":"verifyingContract","type":"address"}],' '"SafeTx":[' '{"name":"to","type":"address"},'
            '{"name":"value","type":"uint256"},' '{"name":"data","type":"bytes"},'
            '{"name":"operation","type":"uint8"},' '{"name":"safeTxGas","type":"uint256"},'
            '{"name":"baseGas","type":"uint256"},' '{"name":"gasPrice","type":"uint256"},'
            '{"name":"gasToken","type":"address"},' '{"name":"refundReceiver","type":"address"},'
            '{"name":"nonce","type":"uint256"}]}';

        // Build domain object
        string memory domain = stdJson.serialize("domain", "chainId", uint256(block.chainid));
        domain = stdJson.serialize("domain", "verifyingContract", safe);

        // Build message object with transaction details
        string memory message = stdJson.serialize("message", "to", call.target);
        message = stdJson.serialize("message", "value", call.value);
        message = stdJson.serialize("message", "data", call.data);
        message = stdJson.serialize("message", "operation", uint256(call.operation));
        message = stdJson.serialize("message", "safeTxGas", uint256(0));
        message = stdJson.serialize("message", "baseGas", uint256(0));
        message = stdJson.serialize("message", "gasPrice", uint256(0));
        message = stdJson.serialize("message", "gasToken", address(0));
        message = stdJson.serialize("message", "refundReceiver", address(0));
        message = stdJson.serialize("message", "nonce", _getNonce(safe));

        // Combine into final JSON structure
        string memory json = stdJson.serialize("", "primaryType", string("SafeTx"));
        json = stdJson.serialize("", "types", types);
        json = stdJson.serialize("", "domain", domain);
        json = stdJson.serialize("", "message", message);

        return abi.encodePacked(json);
    }

    /// @notice Checks the signatures for the given safe and call.
    ///
    /// @param safe The address of the safe to check the signatures for.
    /// @param call The call to check the signatures for.
    /// @param signatures The signatures to check.
    function _checkSignatures(address safe, Call memory call, bytes memory signatures) internal view {
        bytes32 hash = _getTransactionHash({safe: safe, call: call});
        signatures = Signatures.prepareSignatures({safe: safe, hash: hash, signatures: signatures});

        IGnosisSafe(safe)
            .checkSignatures({
                dataHash: hash,
                data: _encodeTransactionData({safe: safe, call: call}), // NOTE: This field is the data preimage but not strictly required as `checkSignatures` ignores it.
                signatures: signatures
            });
    }

    /// @notice Gets the transaction hash for the given safe and call.
    ///
    /// @param safe The address of the safe that will execute the transaction.
    /// @param call The call to get the transaction hash for.
    ///
    /// @return The transaction hash for the given safe and call.
    function _getTransactionHash(address safe, Call memory call) internal view returns (bytes32) {
        return keccak256(_encodeTransactionData({safe: safe, call: call}));
    }

    /// @notice Wrapps the given `call` in a `execTransaction` call.
    ///
    /// @param safe The address of the safe to execute the transaction from.
    /// @param call The call to execute.
    /// @param signatures The signatures to use for the transaction.
    ///
    /// @return The execTransaction call.
    function _buildExecTransactionCall(address safe, Call memory call, bytes memory signatures)
        internal
        pure
        returns (Call memory)
    {
        return Call({
            operation: Enum.Operation.Call,
            target: safe,
            data: abi.encodeCall(
                IGnosisSafe(safe).execTransaction,
                (
                    call.target, // to
                    call.value, // value
                    call.data, // data
                    call.operation, // operation
                    0, // safeTxGas
                    0, // baseGas
                    0, // gasPrice
                    address(0), // gasToken
                    payable(address(0)), // refundReceiver
                    signatures // signatures
                )
            ),
            value: 0
        });
    }

    /// @notice Executes the given call from the given safe.
    ///
    /// @param safe The address of the safe to execute the call from.
    /// @param call The call to execute.
    /// @param signatures The signatures to use for the transaction.
    /// @param broadcast Whether to broadcast the transaction.
    ///
    /// @return The result of the transaction.
    function _execTransaction(address safe, Call memory call, bytes memory signatures, bool broadcast)
        internal
        returns (bool)
    {
        if (broadcast) {
            vm.broadcast();
        }

        return IGnosisSafe(safe)
            .execTransaction({
                to: call.target,
                value: call.value,
                data: call.data,
                operation: call.operation,
                safeTxGas: 0,
                baseGas: 0,
                gasPrice: 0,
                gasToken: address(0),
                refundReceiver: payable(address(0)),
                signatures: signatures
            });
    }

    /// @notice Gets the type for the given call.
    ///
    /// @param call The call to get the type for.
    ///
    /// @return The type for the given call.
    function _getCall3Type(Call memory call) internal pure returns (Call3Type) {
        if (call.operation == Enum.Operation.DelegateCall) {
            return Call3Type.DELEGATE_CALL;
        }

        if (call.value == 0) {
            return Call3Type.CALL;
        }

        return Call3Type.CALL_VALUE;
    }

    /// @notice Converts the given call to the format expected by the `ICBMulticall.aggregate3` function.
    ///
    /// @param call The call to convert to the format expected by the `ICBMulticall.aggregate3` function.
    ///
    /// @return The call in the format expected by the `ICBMulticall.aggregate3` function.
    function _toCall3(Call memory call) internal pure returns (Call3 memory) {
        require(call.operation == Enum.Operation.Call, "MultisigScript::_toCall3: Operation must be Call");
        require(call.value == 0, "MultisigScript::_toCall3: Value must be 0");

        return Call3({target: call.target, allowFailure: false, callData: call.data});
    }

    /// @notice Converts the given call to the format expected by the `ICBMulticall.aggregate3Value` function.
    ///
    /// @param call The call to convert to the format expected by the `ICBMulticall.aggregate3Value` function.
    ///
    /// @return The call in the format expected by the `ICBMulticall.aggregate3Value` function.
    function _toCall3Value(Call memory call) internal pure returns (Call3Value memory) {
        require(call.operation == Enum.Operation.Call, "MultisigScript::_toCall3Value: Operation must be Call");
        require(call.value > 0, "MultisigScript::_toCall3Value: Value must be greater than 0");

        return Call3Value({target: call.target, allowFailure: false, value: call.value, callData: call.data});
    }

    /// @notice Converts the given call to the format expected by the `ICBMulticall.aggregateDelegateCalls` function.
    ///
    /// @param call The call to convert to the format expected by the `ICBMulticall.aggregateDelegateCalls` function.
    ///
    /// @return The call in the format expected by the `ICBMulticall.aggregateDelegateCalls` function.
    function _toDelegateCall3(Call memory call) internal pure returns (Call3 memory) {
        require(
            call.operation == Enum.Operation.DelegateCall,
            "MultisigScript::_toDelegateCall3: Operation must be DelegateCall"
        );
        require(call.value == 0, "MultisigScript::_toDelegateCall3: Value must be 0");

        return Call3({target: call.target, allowFailure: false, callData: call.data});
    }

    /// @notice Converts the given calls to the format expected by the `aggregate3` function.
    ///
    /// @param calls The calls to get the call3 values for.
    ///
    /// @return The calls in the format expected by the `aggregate3` function.
    function _toCall3s(Call[] memory calls) internal pure returns (Call3[] memory) {
        Call3[] memory call3s = new Call3[](calls.length);
        for (uint256 i; i < calls.length; i++) {
            call3s[i] = _toCall3(calls[i]);
        }

        return call3s;
    }

    /// @notice Converts the given calls to the format expected by the `aggregate3Value` function.
    ///
    /// @param calls The calls to get the call3 values for.
    ///
    /// @return The calls in the format expected by the `aggregate3` function.
    function _toCall3Values(Call[] memory calls) internal pure returns (Call3Value[] memory) {
        Call3Value[] memory call3Values = new Call3Value[](calls.length);
        for (uint256 i; i < calls.length; i++) {
            call3Values[i] = _toCall3Value(calls[i]);
        }

        return call3Values;
    }

    /// @notice Converts the given calls to the format expected by the `aggregateDelegateCalls` function.
    ///
    /// @param calls The calls to get the call3 values for.
    ///
    /// @return The calls in the format expected by the `aggregateDelegateCalls` function.
    function _toDelegateCall3s(Call[] memory calls) internal pure returns (Call3[] memory) {
        Call3[] memory delegateCall3s = new Call3[](calls.length);
        for (uint256 i; i < calls.length; i++) {
            delegateCall3s[i] = _toDelegateCall3(calls[i]);
        }

        return delegateCall3s;
    }

    /// @notice Wraps the given address in an array of one address.
    ///
    /// @param addr The address to wrap.
    ///
    /// @return The address wrapped in an array of one address.
    function _toArray(address addr) internal pure returns (address[] memory) {
        address[] memory array = new address[](1);
        array[0] = addr;
        return array;
    }
}
