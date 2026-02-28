// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {IVerifier} from "interfaces/multiproof/IVerifier.sol";

contract MockVerifier is IVerifier {
    function verify(bytes calldata, bytes32, bytes32) external pure returns (bool) {
        return true;
    }
}
