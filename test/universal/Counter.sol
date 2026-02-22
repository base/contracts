// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

contract Counter {
    address internal immutable OWNER;
    uint256 public count = 0;

    constructor(address owner) {
        OWNER = owner;
    }

    function increment() external {
        require(msg.sender == OWNER, "only owner can increment");

        count += 1;
    }

    function incrementPayable() external payable {
        require(msg.value != 0, "value must be greater than 0");
        count += 1;
    }
}
