// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {IVerifier} from "../interfaces/IVerifier.sol";
import {Claim} from "optimism/src/dispute/lib/Types.sol";

contract MockVerifier is IVerifier {
    function verify(bytes calldata , Claim , uint256 ) external pure returns (bool) {
        return true;
    }
}