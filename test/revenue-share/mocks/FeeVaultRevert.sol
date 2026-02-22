// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { FeeVault } from "src/L2/FeeVault.sol";
import { Types } from "src/libraries/Types.sol";

contract FeeVaultRevert {
    address internal immutable _RECIPIENT;

    constructor(address _recipient) {
        _RECIPIENT = _recipient;
    }

    function RECIPIENT() external view returns (address) {
        return _RECIPIENT;
    }

    function WITHDRAWAL_NETWORK() external pure returns (Types.WithdrawalNetwork) {
        return Types.WithdrawalNetwork.L2;
    }

    function MIN_WITHDRAWAL_AMOUNT() external pure returns (uint256) {
        revert("revert message");
    }
}
