// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

contract MockSystemConfig {

    address public guardian;

    constructor() {
        guardian = msg.sender;
    }

    function paused() public pure returns (bool) {
        return false;
    }
}