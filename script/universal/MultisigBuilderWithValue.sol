// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";

import {MultisigBuilder} from "./MultisigBuilder.sol";

abstract contract MultisigBuilderWithValue is MultisigBuilder {
    function _buildCallsWithValue() internal view virtual returns (IMulticall3.Call3Value[] memory);

    function _buildCalls() internal view override returns (IMulticall3.Call3Value[] memory) {
        return _buildCallsWithValue();
    }
}
