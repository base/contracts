// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Initializable } from "lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol";

/// @title NextImpl
/// @dev Mock future implementation used to verify storage access after an upgrade.
contract NextImpl is Initializable {
    // Initializable stores its flags in slot 0, so these fields place slot21 at storage slot 21.
    bytes32 private slot1;
    uint256[19] private __gap;
    bytes32 private slot21;

    bytes32 public constant slot21Init = bytes32(hex"1337");

    function initialize(uint8 _init) public reinitializer(_init) {
        // Slot 21 is unused by any current upgrade target and proves the new implementation can write to it.
        slot21 = slot21Init;
    }
}
