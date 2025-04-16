// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";
import {Vm} from "forge-std/Vm.sol";

import {IGnosisSafe} from "./IGnosisSafe.sol";
import {NestedMultisigBuilder} from "./NestedMultisigBuilder.sol";
import {Signatures} from "./Signatures.sol";
import {Simulation} from "./Simulation.sol";

import {console} from "forge-std/console.sol";

/**
 * @title DoubleNestedMultisigBuilder
 * @notice Modeled from Optimism's SafeBuilder, but built for double nested safes (Safes where
 * the signers are other Safes with signers that are other Safes).
 *
 * There are three safes involved in a double nested multisig:
 * 1. The top-level safe, which should be returned by `_ownerSafe()`.
 * 2. One or more intermediate safes, which are signers for the top-level safe.
 * 3. Signer safes, which are signers for the intermediate safes.
 * There should be at least one signer safe per intermediate safe.
 */
abstract contract DoubleNestedMultisigBuilder is NestedMultisigBuilder {
    /**
     * Step 1
     * ======
     * Generate a transaction approval data to sign. This method should be called by a threshold
     * of members of each of the signer safes involved in the nested multisig. Signers will pass
     * their signature to a facilitator, who will execute the approval transaction for each
     * signer safe (see step 2).
     */
    function sign(address signerSafe, address intermediateSafe) public {
        address topSafe = _ownerSafe();

        // Snapshot and restore Safe nonce after simulation, otherwise the data logged to sign
        // would not match the actual data we need to sign, because the simulation
        // would increment the nonce.
        uint256 originalTopNonce = _getNonce(topSafe);
        uint256 originalIntermediateNonce = _getNonce(intermediateSafe);
        uint256 originalSignerNonce = _getNonce(signerSafe);

        (Vm.AccountAccess[] memory accesses, Simulation.Payload memory simPayload) =
            _simulateForSigner(signerSafe, intermediateSafe, topSafe, _buildCalls());

        _postSign(accesses, simPayload);
        _postCheck(accesses, simPayload);

        // Restore the original nonce.
        vm.store(topSafe, SAFE_NONCE_SLOT, bytes32(originalTopNonce));
        vm.store(intermediateSafe, SAFE_NONCE_SLOT, bytes32(originalIntermediateNonce));
        vm.store(signerSafe, SAFE_NONCE_SLOT, bytes32(originalSignerNonce));

        _printDataToSign(signerSafe, _generateIntermediateSafeApprovalCall(intermediateSafe));
    }

    /**
     * Step 1.1 (optional)
     * ======
     * Verify the signatures generated from step 1 are valid.
     * This allows transactions to be pre-signed and stored safely before execution.
     */
    function verify(address signerSafe, address intermediateSafe, bytes memory signatures) public view {
        IMulticall3.Call3[] memory calls = _generateIntermediateSafeApprovalCall(intermediateSafe);
        _checkSignatures(signerSafe, calls, signatures);
    }

    /**
     * Step 2
     * ======
     * Execute an approval transaction for a signer safe. This method should be called by a facilitator
     * (non-signer), once for each of the signer safes involved in the nested multisig,
     * after collecting a threshold of signatures for each signer safe (see step 1).
     */
    function approveOnBehalfOfSignerSafe(address signerSafe, address intermediateSafe, bytes memory signatures)
        public
    {
        IMulticall3.Call3[] memory calls = _generateIntermediateSafeApprovalCall(intermediateSafe);
        _executeTransaction(signerSafe, calls, signatures, true);
        _postApprove(signerSafe, intermediateSafe);
    }

    /**
     * Step 3
     * ======
     * Execute an approval transaction for an intermediate safe. This method should be called by a
     * facilitator (non-signer), for each of the intermediate safes after all of their approval
     * transactions have been submitted onchain by their signer safes (see step 2).
     */
    function approveOnBehalfOfIntermediateSafe(address intermediateSafe) public {
        IMulticall3.Call3[] memory calls = _generateTopSafeApprovalCall();
        // signatures is empty, because `_executeTransaction` internally collects all of the approvedHash addresses
        bytes memory signatures;
        _executeTransaction(intermediateSafe, calls, signatures, true);
        _postRunInit(intermediateSafe);
    }

    /**
     * @dev Follow up assertions on state and simulation after an `approve` call.
     */
    function _postApprove(address signerSafe, address intermediateSafe) private view {
        IMulticall3.Call3 memory topSafeApprovalCall = _generateApproveCall(_ownerSafe(), _buildCalls());
        bytes memory data = abi.encodeCall(IMulticall3.aggregate3, _toArray(topSafeApprovalCall));
        bytes32 approvedHash = _getTransactionHash(intermediateSafe, data);

        uint256 isApproved = IGnosisSafe(intermediateSafe).approvedHashes(signerSafe, approvedHash);
        require(isApproved == 1, "DoubleNestedMultisigBuilder::_postApprove: Approval failed");
    }

    /**
     * @dev Follow up assertions on state and simulation after an `init` call.
     */
    function _postRunInit(address intermediateSafe) private view {
        bytes memory data = abi.encodeCall(IMulticall3.aggregate3, _buildCalls());
        bytes32 approvedHash = _getTransactionHash(_ownerSafe(), data);

        uint256 isApproved = IGnosisSafe(_ownerSafe()).approvedHashes(intermediateSafe, approvedHash);
        require(isApproved == 1, "DoubleNestedMultisigBuilder::_postRunInit: Init transaction failed");
    }

    function _generateIntermediateSafeApprovalCall(address intermediateSafe)
        private
        view
        returns (IMulticall3.Call3[] memory)
    {
        IMulticall3.Call3[] memory topCalls = _generateTopSafeApprovalCall();
        IMulticall3.Call3 memory intermediateCall = _generateApproveCall(intermediateSafe, topCalls);
        return _toArray(intermediateCall);
    }

    function _generateTopSafeApprovalCall() private view returns (IMulticall3.Call3[] memory) {
        IMulticall3.Call3[] memory dstCalls = _buildCalls();
        IMulticall3.Call3 memory topSafeApprovalCall = _generateApproveCall(_ownerSafe(), dstCalls);
        return _toArray(topSafeApprovalCall);
    }

    function _simulateForSigner(
        address _signerSafe,
        address _intermediateSafe,
        address _safe,
        IMulticall3.Call3[] memory _calls
    ) internal returns (Vm.AccountAccess[] memory, Simulation.Payload memory) {
        bytes memory data = abi.encodeCall(IMulticall3.aggregate3, (_calls));
        IMulticall3.Call3[] memory calls = _simulateForSignerCalls(_signerSafe, _intermediateSafe, _safe, data);

        // Now define the state overrides for the simulation.
        Simulation.StateOverride[] memory overrides = _overrides(_signerSafe, _intermediateSafe, _safe);

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

    function _simulateForSignerCalls(address _signerSafe, address _intermediateSafe, address _safe, bytes memory _data)
        internal
        view
        returns (IMulticall3.Call3[] memory)
    {
        IMulticall3.Call3[] memory calls = new IMulticall3.Call3[](3);
        IMulticall3.Call3[] memory firstApproval = _generateIntermediateSafeApprovalCall(_intermediateSafe);

        // simulate an approveHash, so that signer can verify the data they are signing
        bytes memory firstApprovalHashData = abi.encodeCall(IMulticall3.aggregate3, (firstApproval));
        bytes memory firstApproveHashExec = _execTransactionCalldata(
            _signerSafe, firstApprovalHashData, Signatures.genPrevalidatedSignature(MULTICALL3_ADDRESS)
        );

        calls[0] = IMulticall3.Call3({target: _signerSafe, allowFailure: false, callData: firstApproveHashExec});

        IMulticall3.Call3[] memory secondApproval = _generateTopSafeApprovalCall();
        bytes memory secondApprovalHashData = abi.encodeCall(IMulticall3.aggregate3, (secondApproval));
        bytes memory secondApproveHashExec = _execTransactionCalldata(
            _intermediateSafe, secondApprovalHashData, Signatures.genPrevalidatedSignature(_signerSafe)
        );

        calls[1] = IMulticall3.Call3({target: _intermediateSafe, allowFailure: false, callData: secondApproveHashExec});

        // simulate the final state changes tx, so that signer can verify the final results
        bytes memory finalExec =
            _execTransactionCalldata(_safe, _data, Signatures.genPrevalidatedSignature(_intermediateSafe));
        calls[2] = IMulticall3.Call3({target: _safe, allowFailure: false, callData: finalExec});

        return calls;
    }

    function _overrides(address _signerSafe, address _intermediateSafe, address _safe)
        internal
        view
        returns (Simulation.StateOverride[] memory)
    {
        Simulation.StateOverride[] memory simOverrides = _simulationOverrides();
        Simulation.StateOverride[] memory overrides = new Simulation.StateOverride[](3 + simOverrides.length);
        overrides[0] = _safeOverrides(_signerSafe, MULTICALL3_ADDRESS);
        overrides[1] = _safeOverrides(_intermediateSafe, address(0));
        overrides[2] = _safeOverrides(_safe, address(0));
        for (uint256 i = 0; i < simOverrides.length; i++) {
            overrides[i + 3] = simOverrides[i];
        }
        return overrides;
    }
}
