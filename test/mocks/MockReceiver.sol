// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

contract MockReceiver {
    function bump(uint256 x) external pure returns (uint256) {
        return x + 1;
    }

    function payAndEcho(uint256 x) external payable returns (uint256, uint256) {
        return (x, msg.value);
    }

    function willRevert() external pure {
        revert("revert");
    }
}
