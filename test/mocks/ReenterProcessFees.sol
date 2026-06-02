// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { BalanceTracker } from "src/L1/BalanceTracker.sol";

contract ReenterProcessFees {
    receive() external payable {
        BalanceTracker(payable(msg.sender)).processFees();
    }
}
