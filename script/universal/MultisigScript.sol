// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// solhint-disable no-console
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";
import {Vm} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";

import {IGnosisSafe, Enum} from "./IGnosisSafe.sol";
import {Signatures} from "./Signatures.sol";
import {Simulation} from "./Simulation.sol";

/**
 * @title MultisigScript
 * @notice Script builder for Forge scripts that require signatures from Safes. Supports both non-nested
 *         Safes, as well as nested Safes of arbitrary depth (Safes where the signers are other Safes).
 *
 * 1. Non-nested example:
 *
 * Setup:
 * ┌───────┐┌───────┐
 * │Signer1││Signer2│
 * └┬──────┘└┬──────┘
 * ┌▽────────▽┐
 * │Multisig  │
 * └┬─────────┘
 * ┌▽─────────┐
 * │ProxyAdmin│
 * └──────────┘
 *
 * Sequence:
 * ┌───────┐┌───────┐┌───────────┐┌──────────────┐
 * │Signer1││Signer2││Facilitator││MultisigScript│
 * └───┬───┘└───┬───┘└─────┬─────┘└───────┬──────┘
 *     │        │     sign()              │
 *     │─────────────────────────────────>│
 *     │      <sig1>       │              │
 *     │──────────────────>│              │
 *     │        │         sign()          │
 *     │        │────────────────────────>│
 *     │        │  <sig2>  │              │
 *     │        │─────────>│              │
 *     │        │          │run(sig1,sig2)│
 *     │        │          │─────────────>│
 *
 *
 * 2. Single-layer nested example:
 *
 * Setup:
 * ┌───────┐┌───────┐┌───────┐┌───────┐
 * │Signer1││Signer2││Signer3││Signer4│
 * └┬──────┘└┬──────┘└┬──────┘└┬──────┘
 * ┌▽────────▽┐┌──────▽────────▽┐
 * │Safe1     ││Safe2           │
 * └┬─────────┘└┬───────────────┘
 * ┌▽───────────▽┐
 * │Safe3        │
 * └┬────────────┘
 * ┌▽─────────┐
 * │ProxyAdmin│
 * └──────────┘
 *
 * Sequence:
 * ┌───────┐┌───────┐┌───────┐┌───────┐┌───────────┐          ┌──────────────┐
 * │Signer1││Signer2││Signer3││Signer4││Facilitator│          │MultisigScript│
 * └───┬───┘└───┬───┘└───┬───┘└───┬───┘└─────┬─────┘          └───────┬──────┘
 *     │        │        │       sign(Safe1) │                        │
 *     │─────────────────────────────────────────────────────────────>│
 *     │        │      <sig1>     │          │                        │
 *     │────────────────────────────────────>│                        │
 *     │        │        │        │   sign(Safe1)                     │
 *     │        │────────────────────────────────────────────────────>│
 *     │        │        │  <sig2>│          │                        │
 *     │        │───────────────────────────>│                        │
 *     │        │        │        │          │approve(Safe1,sig1|sig2)│
 *     │        │        │        │          │───────────────────────>│
 *     │        │        │        │       sign(Safe2)                 │
 *     │        │        │───────────────────────────────────────────>│
 *     │        │        │      <sig3>       │                        │
 *     │        │        │──────────────────>│                        │
 *     │        │        │        │          │ sign(Safe2)            │
 *     │        │        │        │──────────────────────────────────>│
 *     │        │        │        │  <sig4>  │                        │
 *     │        │        │        │─────────>│                        │
 *     │        │        │        │          │approve(Safe2,sig3|sig4)│
 *     │        │        │        │          │───────────────────────>│
 *     │        │        │        │          │         run()          │
 *     │        │        │        │          │───────────────────────>│
 *
 *
 * 3. Multi-layer nested example:
 *
 * Setup:
 * ┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐
 * │Signer1││Signer2││Signer3││Signer4││Signer5││Signer6│
 * └┬──────┘└┬──────┘└┬──────┘└┬──────┘└┬──────┘└┬──────┘
 * ┌▽────────▽┐┌──────▽────────▽┐┌──────▽────────▽┐
 * │Safe1     ││Safe2           ││Safe3           │
 * └┬─────────┘└┬───────────────┘└┬───────────────┘
 * ┌▽───────────▽┐                │
 * │Safe4        │                │
 * └┬────────────┘                │
 * ┌▽─────────────────────────────▽┐
 * │Safe5                          │
 * └┬──────────────────────────────┘
 * ┌▽─────────┐
 * │ProxyAdmin│
 * └──────────┘
 *
 * Sequence:
 * ┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐┌───────┐┌───────────┐                ┌──────────────┐
 * │Signer1││Signer2││Signer3││Signer4││Signer5││Signer6││Facilitator│                │MultisigScript│
 * └───┬───┘└───┬───┘└───┬───┘└───┬───┘└───┬───┘└───┬───┘└─────┬─────┘                └───────┬──────┘
 *     │        │        │        │       sign(Safe1,Safe4)    │                              │
 *     │─────────────────────────────────────────────────────────────────────────────────────>│
 *     │        │        │      <sig1>     │        │          │                              │
 *     │──────────────────────────────────────────────────────>│                              │
 *     │        │        │        │        │   sign(Safe1,Safe4)                              │
 *     │        │────────────────────────────────────────────────────────────────────────────>│
 *     │        │        │        │  <sig2>│        │          │                              │
 *     │        │─────────────────────────────────────────────>│                              │
 *     │        │        │        │        │        │          │approve(Safe1,Safe4,sig1|sig2)│
 *     │        │        │        │        │        │          │─────────────────────────────>│
 *     │        │        │        │        │       sign(Safe2,Safe4)                          │
 *     │        │        │───────────────────────────────────────────────────────────────────>│
 *     │        │        │        │      <sig3>     │          │                              │
 *     │        │        │────────────────────────────────────>│                              │
 *     │        │        │        │        │        │   sign(Safe2,Safe4)                     │
 *     │        │        │        │──────────────────────────────────────────────────────────>│
 *     │        │        │        │        │  <sig4>│          │                              │
 *     │        │        │        │───────────────────────────>│                              │
 *     │        │        │        │        │        │          │approve(Safe2,Safe4,sig3|sig4)│
 *     │        │        │        │        │        │          │─────────────────────────────>│
 *     │        │        │        │        │        │          │        approve(Safe4)        │
 *     │        │        │        │        │        │          │─────────────────────────────>│
 *     │        │        │        │        │        │          sign(Safe3)                    │
 *     │        │        │        │        │─────────────────────────────────────────────────>│
 *     │        │        │        │        │      <sig5>       │                              │
 *     │        │        │        │        │──────────────────>│                              │
 *     │        │        │        │        │        │          │    sign(Safe3)               │
 *     │        │        │        │        │        │────────────────────────────────────────>│
 *     │        │        │        │        │        │  <sig6>  │                              │
 *     │        │        │        │        │        │─────────>│                              │
 *     │        │        │        │        │        │          │   approve(Safe3,sig5|sig6)   │
 *     │        │        │        │        │        │          │─────────────────────────────>│
 *     │        │        │        │        │        │          │            run()             │
 *     │        │        │        │        │        │          │─────────────────────────────>│
 */
abstract contract MultisigScript is Script {
    bytes32 internal constant SAFE_NONCE_SLOT = bytes32(uint256(5));

    /*
     * @dev Event emitted from a `sign()` call containing the data to sign. Used in testing.
     */
    event DataToSign(bytes);

    /**
     * -----------------------------------------------------------
     * Virtual Functions
     * -----------------------------------------------------------
     */

    /**
     * @notice Returns the safe address to execute the final transaction from
     */
    function _ownerSafe() internal view virtual returns (address);

    /**
     * @notice Creates the calldata for signatures (`sign`), approvals (`approve`), and execution (`run`)
     */
    function _buildCalls() internal view virtual returns (IMulticall3.Call3Value[] memory);

    /**
     * @notice Follow up assertions to ensure that the script ran to completion.
     * @dev Called after `sign` and `run`, but not `approve`.
     */
    function _postCheck(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual;

    /**
     * @notice Follow up assertions on state and simulation after a `sign` call.
     */
    function _postSign(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual {}

    /**
     * @notice Follow up assertions on state and simulation after a `approve` call.
     */
    function _postApprove(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual {}

    /**
     * @notice Follow up assertions on state and simulation after a `run` call.
     */
    function _postRun(Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) internal virtual {}

    // Tenderly simulations can accept generic state overrides. This hook enables this functionality.
    // By default, an empty (no-op) override is returned.
    function _simulationOverrides() internal view virtual returns (Simulation.StateOverride[] memory overrides_) {}

    /**
     * -----------------------------------------------------------
     * Public Functions
     * -----------------------------------------------------------
     */

    /**
     * Step 1
     * ======
     * Generate a transaction approval data to sign. This method should be called by a threshold of
     * multisig owners.
     *
     * For non-nested multisigs, the signatures can then be used to execute the transaction (see step 3).
     *
     * For nested multisigs, the signatures can be used to execute an approval transaction for each
     * multisig (see step 2).
     *
     * @param _safes A list of nested safes (excluding the executing safe returned by `_ownerSafe`).
     */
    function sign(address[] memory _safes) public {
        _safes = _appendOwnerSafe(_safes);

        // Snapshot and restore Safe nonce after simulation, otherwise the data logged to sign
        // would not match the actual data we need to sign, because the simulation
        // would increment the nonce.
        uint256[] memory originalNonces = new uint256[](_safes.length);
        for (uint256 i = 0; i < _safes.length; i++) {
            originalNonces[i] = _getNonce(_safes[i]);
        }

        (bytes[] memory datas, uint256 value) = _transactionDatas(_safes);

        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _simulateForSigner(_safes, datas, value);

        _postSign(accesses, simPayload);
        _postCheck(accesses, simPayload);

        // Restore the original nonce.
        for (uint256 i = 0; i < _safes.length; i++) {
            vm.store(_safes[i], SAFE_NONCE_SLOT, bytes32(originalNonces[i]));
        }
        _printDataToSign(_safes[0], datas[0], value);
    }

    /**
     * Step 1.1 (optional)
     * ======
     * Verify the signatures generated from step 1 are valid.
     * This allows transactions to be pre-signed and stored safely before execution.
     *
     * @param _safes A list of nested safes (excluding the executing safe returned by `_ownerSafe`).
     * @param _signatures The signatures to verify (concatenated, 65-bytes per sig).
     */
    function verify(address[] memory _safes, bytes memory _signatures) public view {
        _safes = _appendOwnerSafe(_safes);
        (bytes[] memory datas, uint256 value) = _transactionDatas(_safes);
        _checkSignatures(_safes[0], datas[0], value, _signatures);
    }

    /**
     * Step 2 (optional for non-nested setups)
     * ======
     * Execute an approval transaction. This method should be called by a facilitator
     * (non-signer), once for each of the multisigs involved in the nested multisig,
     * after collecting a threshold of signatures for each multisig (see step 1).
     *
     * For multiple layers of nesting, this should be called for each layer of nesting (once
     * the inner multisigs have registered their approval). The array of safes passed to
     * `_safes` should get smaller by one for each layer of nesting.
     *
     * @param _safes A list of nested safes (excluding the executing safe returned by `_ownerSafe`).
     * @param _signatures The signatures from step 1 (concatenated, 65-bytes per sig)
     */
    function approve(address[] memory _safes, bytes memory _signatures) public {
        _safes = _appendOwnerSafe(_safes);
        (bytes[] memory datas, uint256 value) = _transactionDatas(_safes);
        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _executeTransaction(_safes[0], datas[0], value, _signatures, true);
        _postApprove(accesses, simPayload);
    }

    /**
     * Step 2.1 (optional)
     * ======
     * Simulate the transaction. This method should be called by a facilitator (non-signer), after all of the
     * signatures have been collected (non-nested case, see step 1), or the approval transactions have been
     * submitted onchain (nested case, see step 2, in which case `_signatures` can be empty).
     *
     * Differs from `run` in that you can override the safe nonce for simulation purposes.
     */
    function simulate(bytes memory _signatures) public {
        address ownerSafe = _ownerSafe();
        (bytes[] memory datas, uint256 value) = _transactionDatas(_toArray(ownerSafe));

        vm.store(ownerSafe, SAFE_NONCE_SLOT, bytes32(_getNonce(ownerSafe)));

        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _executeTransaction(ownerSafe, datas[0], value, _signatures, false);

        _postRun(accesses, simPayload);
        _postCheck(accesses, simPayload);
    }

    /**
     * Step 3
     * ======
     * Execute the transaction. This method should be called by a facilitator (non-signer), after all of the
     * signatures have been collected (non-nested case, see step 1), or the approval transactions have been
     * submitted onchain (nested case, see step 2, in which case `_signatures` can be empty).
     */
    function run(bytes memory _signatures) public {
        address ownerSafe = _ownerSafe();
        (bytes[] memory datas, uint256 value) = _transactionDatas(_toArray(ownerSafe));

        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _executeTransaction(ownerSafe, datas[0], value, _signatures, true);

        _postRun(accesses, simPayload);
        _postCheck(accesses, simPayload);
    }

    /**
     * -----------------------------------------------------------
     * Internal Functions
     * -----------------------------------------------------------
     */
    function _appendOwnerSafe(address[] memory _safes) internal view returns (address[] memory) {
        address[] memory safes = new address[](_safes.length + 1);
        for (uint256 i = 0; i < _safes.length; i++) {
            safes[i] = _safes[i];
        }
        safes[safes.length - 1] = _ownerSafe();
        return safes;
    }

    function _transactionDatas(address[] memory _safes) internal view returns (bytes[] memory datas, uint256 value) {
        // Build the calls and sum the values
        IMulticall3.Call3Value[] memory calls = _buildCalls();
        for (uint256 i = 0; i < calls.length; i++) {
            value += calls[i].value;
        }

        // The very last call is the actual (aggregated) call to execute
        datas = new bytes[](_safes.length);
        datas[datas.length - 1] = abi.encodeCall(IMulticall3.aggregate3Value, (calls));

        // The first n-1 calls are the nested approval calls
        uint256 valueForCallToApprove = value;
        for (uint256 i = _safes.length - 1; i > 0; i--) {
            address targetSafe = _safes[i];
            bytes memory callToApprove = datas[i];

            IMulticall3.Call3[] memory approvalCall = new IMulticall3.Call3[](1);
            approvalCall[0] = _generateApproveCall(targetSafe, callToApprove, valueForCallToApprove);
            datas[i - 1] = abi.encodeCall(IMulticall3.aggregate3, (approvalCall));

            valueForCallToApprove = 0;
        }
    }

    function _generateApproveCall(address _safe, bytes memory _data, uint256 _value)
        internal
        view
        returns (IMulticall3.Call3 memory)
    {
        bytes32 hash = _getTransactionHash(_safe, _data, _value);

        console.log("---\nNested hash for safe %s:", _safe);
        console.logBytes32(hash);

        return IMulticall3.Call3({
            target: _safe,
            allowFailure: false,
            callData: abi.encodeCall(IGnosisSafe(_safe).approveHash, (hash))
        });
    }

    function _printDataToSign(address _safe, bytes memory _data, uint256 _value) internal {
        bytes memory txData =
            _printDataHashes() ? _encodeTransactionData(_safe, _data, _value) : _encodeEIP712Json(_safe, _data, _value);
        bytes32 hash = _getTransactionHash(_safe, _data, _value);

        emit DataToSign(txData);

        console.log("---\nIf submitting onchain, call Safe.approveHash on %s with the following hash:", _safe);
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

    // Controls whether the safe tx is printed as structured EIP-712 data, or just hashes.
    //
    // If you want to print and sign hashed EIP-712 data (domain + message hash) rather than the
    // typed EIP-712 data struct, override this function and return `true`.
    function _printDataHashes() internal view virtual returns (bool) {
        return false;
    }

    function _executeTransaction(
        address _safe,
        bytes memory _data,
        uint256 _value,
        bytes memory _signatures,
        bool _broadcast
    ) internal returns (Vm.AccountAccess[] memory, Simulation.Payload memory) {
        bytes32 hash = _getTransactionHash(_safe, _data, _value);
        _signatures = Signatures.prepareSignatures(_safe, hash, _signatures);

        bytes memory simData = _execTransactionCalldata(_safe, _data, _value, _signatures);
        Simulation.logSimulationLink({_to: _safe, _data: simData, _from: msg.sender});

        vm.startStateDiffRecording();
        bool success = _execTransaction(_safe, _data, _value, _signatures, _broadcast);
        Vm.AccountAccess[] memory accesses = vm.stopAndReturnStateDiff();
        require(success, "MultisigBase::_executeTransaction: Transaction failed");
        require(accesses.length > 0, "MultisigBase::_executeTransaction: No state changes");

        // This can be used to e.g. call out to the Tenderly API and get additional
        // data about the state diff before broadcasting the transaction.
        Simulation.Payload memory simPayload = Simulation.Payload({
            from: msg.sender,
            to: _safe,
            data: simData,
            stateOverrides: new Simulation.StateOverride[](0)
        });
        return (accesses, simPayload);
    }

    function _simulateForSigner(address[] memory _safes, bytes[] memory _datas, uint256 _value)
        internal
        returns (Vm.AccountAccess[] memory, Simulation.Payload memory)
    {
        IMulticall3.Call3[] memory calls = _simulateForSignerCalls(_safes, _datas, _value);

        bytes32 firstCallDataHash = _getTransactionHash(_safes[0], _datas[0], _value);

        // Now define the state overrides for the simulation.
        Simulation.StateOverride[] memory overrides = _overrides(_safes, firstCallDataHash);

        bytes memory txData = abi.encodeCall(IMulticall3.aggregate3, (calls));
        console.log("---\nSimulation link:");
        // solhint-disable max-line-length
        Simulation.logSimulationLink({_to: MULTICALL3_ADDRESS, _data: txData, _from: msg.sender, _overrides: overrides});

        // Forge simulation of the data logged in the link. If the simulation fails
        // we revert to make it explicit that the simulation failed.
        Simulation.Payload memory simPayload =
            Simulation.Payload({to: MULTICALL3_ADDRESS, data: txData, from: msg.sender, stateOverrides: overrides});
        Vm.AccountAccess[] memory accesses = Simulation.simulateFromSimPayload(simPayload);
        return (accesses, simPayload);
    }

    function _simulateForSignerCalls(address[] memory _safes, bytes[] memory _datas, uint256 _value)
        private
        view
        returns (IMulticall3.Call3[] memory)
    {
        IMulticall3.Call3[] memory calls = new IMulticall3.Call3[](_safes.length);
        for (uint256 i; i < _safes.length; i++) {
            address signer = i == 0 ? msg.sender : _safes[i - 1];

            calls[i] = IMulticall3.Call3({
                target: _safes[i],
                allowFailure: false,
                callData: _execTransactionCalldata(
                    _safes[i], _datas[i], _value, Signatures.genPrevalidatedSignature(signer)
                )
            });
        }

        return calls;
    }

    // The state change simulation can set the threshold, owner address and/or nonce.
    // This allows simulation of the final transaction by overriding the threshold to 1.
    // State changes reflected in the simulation as a result of these overrides will
    // not be reflected in the prod execution.
    function _overrides(address[] memory _safes, bytes32 firstCallDataHash)
        internal
        view
        returns (Simulation.StateOverride[] memory)
    {
        Simulation.StateOverride[] memory simOverrides = _simulationOverrides();
        Simulation.StateOverride[] memory overrides =
            new Simulation.StateOverride[](_safes.length + simOverrides.length);

        uint256 nonce = _getNonce(_safes[0]);
        overrides[0] = Simulation.overrideSafeThresholdApprovalAndNonce(_safes[0], nonce, msg.sender, firstCallDataHash);

        for (uint256 i = 1; i < _safes.length; i++) {
            overrides[i] = Simulation.overrideSafeThresholdAndNonce(_safes[i], _getNonce(_safes[i]));
        }

        for (uint256 i = 0; i < simOverrides.length; i++) {
            overrides[i + _safes.length] = simOverrides[i];
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
    function _getNonce(address _safe) internal view virtual returns (uint256 nonce) {
        uint256 safeNonce = IGnosisSafe(_safe).nonce();
        nonce = safeNonce;

        // first try SAFE_NONCE
        try vm.envUint("SAFE_NONCE") {
            nonce = vm.envUint("SAFE_NONCE");
        } catch {}

        // then try SAFE_NONCE_{UPPERCASE_SAFE_ADDRESS}
        string memory envVarName = string.concat("SAFE_NONCE_", vm.toUppercase(vm.toString(_safe)));
        try vm.envUint(envVarName) {
            nonce = vm.envUint(envVarName);
        } catch {}

        // print if any override
        if (nonce != safeNonce) {
            console.log("Overriding nonce for safe %s: %d -> %d", _safe, safeNonce, nonce);
        }
    }

    function _checkSignatures(address _safe, bytes memory _data, uint256 _value, bytes memory _signatures)
        internal
        view
    {
        bytes32 hash = _getTransactionHash(_safe, _data, _value);
        _signatures = Signatures.prepareSignatures(_safe, hash, _signatures);
        IGnosisSafe(_safe).checkSignatures({dataHash: hash, data: _data, signatures: _signatures});
    }

    function _getTransactionHash(address _safe, bytes memory _data, uint256 _value) internal view returns (bytes32) {
        return keccak256(_encodeTransactionData(_safe, _data, _value));
    }

    function _encodeTransactionData(address _safe, bytes memory _data, uint256 _value)
        internal
        view
        returns (bytes memory)
    {
        return IGnosisSafe(_safe).encodeTransactionData({
            to: MULTICALL3_ADDRESS,
            value: _value,
            data: _data,
            operation: _value == 0 ? Enum.Operation.DelegateCall : Enum.Operation.Call,
            safeTxGas: 0,
            baseGas: 0,
            gasPrice: 0,
            gasToken: address(0),
            refundReceiver: address(0),
            _nonce: _getNonce(_safe)
        });
    }

    function _encodeEIP712Json(address _safe, bytes memory _data, uint256 _value) internal returns (bytes memory) {
        string memory types = '{"EIP712Domain":[' '{"name":"chainId","type":"uint256"},'
            '{"name":"verifyingContract","type":"address"}],' '"SafeTx":[' '{"name":"to","type":"address"},'
            '{"name":"value","type":"uint256"},' '{"name":"data","type":"bytes"},'
            '{"name":"operation","type":"uint8"},' '{"name":"safeTxGas","type":"uint256"},'
            '{"name":"baseGas","type":"uint256"},' '{"name":"gasPrice","type":"uint256"},'
            '{"name":"gasToken","type":"address"},' '{"name":"refundReceiver","type":"address"},'
            '{"name":"nonce","type":"uint256"}]}';

        string memory domain = stdJson.serialize("domain", "chainId", uint256(block.chainid));
        domain = stdJson.serialize("domain", "verifyingContract", address(_safe));

        string memory message = stdJson.serialize("message", "to", MULTICALL3_ADDRESS);
        message = stdJson.serialize("message", "value", _value);
        message = stdJson.serialize("message", "data", _data);
        message = stdJson.serialize(
            "message", "operation", uint256(_value == 0 ? Enum.Operation.DelegateCall : Enum.Operation.Call)
        );
        message = stdJson.serialize("message", "safeTxGas", uint256(0));
        message = stdJson.serialize("message", "baseGas", uint256(0));
        message = stdJson.serialize("message", "gasPrice", uint256(0));
        message = stdJson.serialize("message", "gasToken", address(0));
        message = stdJson.serialize("message", "refundReceiver", address(0));
        message = stdJson.serialize("message", "nonce", _getNonce(_safe));

        string memory json = stdJson.serialize("", "primaryType", string("SafeTx"));
        json = stdJson.serialize("", "types", types);
        json = stdJson.serialize("", "domain", domain);
        json = stdJson.serialize("", "message", message);

        return abi.encodePacked(json);
    }

    function _execTransactionCalldata(address _safe, bytes memory _data, uint256 _value, bytes memory _signatures)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodeCall(
            IGnosisSafe(_safe).execTransaction,
            (
                MULTICALL3_ADDRESS,
                _value,
                _data,
                _value == 0 ? Enum.Operation.DelegateCall : Enum.Operation.Call,
                0,
                0,
                0,
                address(0),
                payable(address(0)),
                _signatures
            )
        );
    }

    function _execTransaction(
        address _safe,
        bytes memory _data,
        uint256 _value,
        bytes memory _signatures,
        bool _broadcast
    ) internal returns (bool) {
        if (_broadcast) {
            vm.broadcast();
        }
        return IGnosisSafe(_safe).execTransaction({
            to: MULTICALL3_ADDRESS,
            value: _value,
            data: _data,
            operation: _value == 0 ? Enum.Operation.DelegateCall : Enum.Operation.Call,
            safeTxGas: 0,
            baseGas: 0,
            gasPrice: 0,
            gasToken: address(0),
            refundReceiver: payable(address(0)),
            signatures: _signatures
        });
    }

    function _toArray(address _address) internal pure returns (address[] memory) {
        address[] memory array = new address[](1);
        array[0] = _address;
        return array;
    }

    function _toArray(address _address1, address _address2) internal pure returns (address[] memory) {
        address[] memory array = new address[](2);
        array[0] = _address1;
        array[1] = _address2;
        return array;
    }
}
