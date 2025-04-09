// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";

import {NestedMultisigBuilder} from "./NestedMultisigBuilder.sol";

abstract contract NestedMultisigBuilderNoValue is NestedMultisigBuilder {
    function _buildCallsNoValue() internal view virtual returns (IMulticall3.Call3[] memory);

    function _buildCalls() internal view override returns (IMulticall3.Call3Value[] memory) {
        IMulticall3.Call3[] memory calls = _buildCallsNoValue();

        IMulticall3.Call3Value[] memory callsWithValue = new IMulticall3.Call3Value[](calls.length);
        for (uint256 i; i < calls.length; i++) {
            callsWithValue[i] = IMulticall3.Call3Value({
                target: calls[i].target,
                callData: calls[i].callData,
                allowFailure: calls[i].allowFailure,
                value: 0
            });
        }

        return callsWithValue;
    }
}
