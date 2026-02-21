// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/* Testing utilities */
import {Test} from "forge-std/Test.sol";

contract CommonTest is Test {
    address alice = address(128);
    address bob = address(256);
    address admin = address(512);
    address deployer = address(1024);

    address constant ZERO_ADDRESS = address(0);
    address constant NON_ZERO_ADDRESS = address(1);
    address constant CONTRACT_MOCK = address(2);
    uint256 constant NON_ZERO_VALUE = 100;
    uint256 constant ZERO_VALUE = 0;
    uint64 constant NON_ZERO_GASLIMIT = 50000;

    string constant EMPTY_STRING = "";
    string constant NON_EMPTY_STRING = "non-empty";
    bytes constant NULL_BYTES = bytes("");
    bytes constant NON_NULL_BYTES = abi.encodePacked(uint256(1));

    function setUp() public virtual {
        // Give alice and bob some ETH
        vm.deal(alice, 1 << 16);
        vm.deal(bob, 1 << 16);
        vm.deal(admin, 1 << 16);

        vm.label(alice, "alice");
        vm.label(bob, "bob");
        vm.label(admin, "admin");

        // Make sure we have a non-zero base fee
        vm.fee(1000000000);
    }
}
