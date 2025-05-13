// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

interface IDoubleNestedMultisigBuilder {
    function sign(address, address) external;
}

interface INestedMultisigBuilder {
    function sign(address) external;
}

interface IMultisigBuilder {
    function sign() external;
}
