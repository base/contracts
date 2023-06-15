// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import "./MultisigBase.sol";

import { console } from "forge-std/console.sol";
import { IMulticall3 } from "forge-std/interfaces/IMulticall3.sol";

import { IGnosisSafe, Enum } from "@eth-optimism-bedrock/scripts/interfaces/IGnosisSafe.sol";
import { EnhancedScript } from "@eth-optimism-bedrock/scripts/universal/EnhancedScript.sol";
import { GlobalConstants } from "@eth-optimism-bedrock/scripts/universal/GlobalConstants.sol";

/**
 * @title NestedMultisigBuilder
 * @notice Modeled from Optimism's SafeBuilder, but built for nested safes.
 */
abstract contract NestedMultisigBuilder is EnhancedScript, GlobalConstants, MultisigBase {
    /**
     * -----------------------------------------------------------
     * Virtual Functions
     * -----------------------------------------------------------
     */

    /**
     * @notice Follow up assertions to ensure that the script ran to completion.
     */
    function _postCheck(address _target) internal virtual view;

    /**
     * @notice Creates the calldata
     */
    function buildCalldata(address _target) internal virtual view returns (bytes memory);

    /**
     * -----------------------------------------------------------
     * Implemented Functions
     * -----------------------------------------------------------
     */

    /**
     * Step 1
     * ======
     * Generate a transaction approval data to sign. This method should be called by a threshold-1
     * of members of each of the multisigs involved in the nested multisig, except the final
     * multisig that will execute. Signers will pass their signature to the final signer of their
     * repsective multisig.
     *
     * Example:
     * --------
     * Given:
     *  - 3-of-3 multisig <m1>, signers <s1>, <s2>, <s3>
     *  - 2-of-3 multisig <m2>, signers <s4>, <s5>, <s6>
     *  - 3-of-3 multisig <m3>, signers <s7>, <s8>, <s9>
     *  - 3-of-3 nested multisig <n>, signers <m1>, <m2>, <m3>
     * The following signers should run this function:
        - <s1> => send signature to <s3>
        - <s2> => send signature to <s3>
        - <s4> => send signature to <s5>
     */
    function signApproval(address _safe, address _nestedSafe, address _target) public returns (bool) {
        IMulticall3.Call3 memory call = _generateApproveCall(_safe, _nestedSafe, _target);
        _printDataToSign(_safe, call);
        return true;
    }

    /**
     * Step 2
     * ======
     * Execute a transaction approval. This method should be called by the final member of each of
     * the multisigs involved in the nested multisig, except the final multisig that will execute.
     * Signatures from step 1 are required.
     *
     * Example:
     * --------
     * Given:
     *  - 3-of-3 multisig <m1>, signers <s1>, <s2>, <s3>
     *  - 2-of-3 multisig <m2>, signers <s4>, <s5>, <s6>
     *  - 3-of-3 multisig <m3>, signers <s7>, <s8>, <s9>
     *  - 3-of-3 nested multisig <n>, signers <m1>, <m2>, <m3>
     * The following signers should run this function:
        - <s3> => using signatures from <s1>, <s2>
        - <s5> => using signature from <s4>
     */
    function runApproval(address _safe, address _nestedSafe, address _target, bytes memory _signatures) public returns (bool) {
        vm.startBroadcast();
        IMulticall3.Call3 memory call = _generateApproveCall(_safe, _nestedSafe, _target);
        return _executeTransaction(_safe, call, _signatures);
    }

    /**
     * Step 3
     * ======
     * Generate a transaction execution data to sign. This method should be called by a threshold-1
     * of members of the final multisig that will execute the transaction. Signers will pass their
     * signature to the final signer of this multisig.
     *
     * Example:
     * --------
     * Given:
     *  - 3-of-3 multisig <m1>, signers <s1>, <s2>, <s3>
     *  - 2-of-3 multisig <m2>, signers <s4>, <s5>, <s6>
     *  - 3-of-3 multisig <m3>, signers <s7>, <s8>, <s9>
     *  - 3-of-3 nested multisig <n>, signers <m1>, <m2>, <m3>
     * The following signers should run this function:
        - <s7> => send signature to <s9>
        - <s8> => send signature to <s9>
     */
    function signTransaction(address _safe, address _nestedSafe, address _target, address[] memory _otherSigners) public returns (bool) {
        IMulticall3.Call3 memory call = _generateExecuteCall(_safe, _nestedSafe, _target, _otherSigners);
        _printDataToSign(_safe, call);
        return true;
    }

    /**
     * Step 4
     * ======
     * Execute the transaction. This method should be called by the final member of the final
     * multisig that will execute the transaction. Signatures from step 3 are required.
     *
     * Example:
     * --------
     * Given:
     *  - 3-of-3 multisig <m1>, signers <s1>, <s2>, <s3>
     *  - 2-of-3 multisig <m2>, signers <s4>, <s5>, <s6>
     *  - 3-of-3 multisig <m3>, signers <s7>, <s8>, <s9>
     *  - 3-of-3 nested multisig <n>, signers <m1>, <m2>, <m3>
     * The following signer should run this function:
        - <s9> => using signatures from <s7>, <s8>
     */
    function runTransaction(address _safe, address _nestedSafe, address _target, address[] memory _otherSigners, bytes memory _signatures) public returns (bool) {
        vm.startBroadcast();
        IMulticall3.Call3 memory call = _generateExecuteCall(_safe, _nestedSafe, _target, _otherSigners);
        bool success = _executeTransaction(_safe, call, _signatures);
        if (success) _postCheck(_target);
        return success;
    }

    function _generateApproveCall(address _safe, address _nestedSafe, address _target) internal returns (IMulticall3.Call3 memory) {
        IGnosisSafe safe = IGnosisSafe(payable(_safe));
        IGnosisSafe nestedSafe = IGnosisSafe(payable(_nestedSafe));
        bytes memory nestedData = buildCalldata(_target);
        bytes32 nestedHash = _getTransactionHash(_nestedSafe, nestedData);
        console.log("Nested hash:");
        console.logBytes32(nestedHash);

        return IMulticall3.Call3({
            target: _nestedSafe,
            allowFailure: false,
            callData: abi.encodeCall(
                nestedSafe.approveHash,
                (nestedHash)
            )
        });
    }

    function _generateExecuteCall(address _safe, address _nestedSafe, address _target, address[] memory _signers) internal returns (IMulticall3.Call3 memory) {
        IGnosisSafe safe = IGnosisSafe(payable(_safe));
        IGnosisSafe nestedSafe = IGnosisSafe(payable(_nestedSafe));
        bytes memory nestedData = buildCalldata(_target);

        address[] memory allSigners = new address[](_signers.length + 1);
        for (uint256 i; i < _signers.length; i++) {
            allSigners[i] = _signers[i];
        }
        allSigners[_signers.length] = _safe;

        return IMulticall3.Call3({
            target: _nestedSafe,
            allowFailure: false,
            callData: abi.encodeCall(
                nestedSafe.execTransaction, (
                    address(multicall),           // to
                    0,                            // value
                    nestedData,                   // data
                    Enum.Operation.DelegateCall,  // operation
                    0,                            // safeTxGas
                    0,                            // baseGas
                    0,                            // gasPrice
                    address(0),                   // gasToken
                    payable(address(0)),          // refundReceiver
                    addressSignatures(allSigners) // signatures
                )
            )
        });
    }
}
