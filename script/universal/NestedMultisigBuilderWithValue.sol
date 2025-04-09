// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";

import {NestedMultisigBuilder} from "./NestedMultisigBuilder.sol";

abstract contract NestedMultisigBuilderWithValue is NestedMultisigBuilder {
    function _buildCallsWithValue() internal view virtual returns (IMulticall3.Call3Value[] memory);

    function _buildCalls() internal view override returns (IMulticall3.Call3Value[] memory) {
        return _buildCallsWithValue();
    }
}
