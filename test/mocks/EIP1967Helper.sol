// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Vm } from "lib/forge-std/src/Vm.sol";
import { Constants } from "src/libraries/Constants.sol";

/// @title EIP1967Helper
/// @dev Testing library to help with reading EIP 1967 variables from state
library EIP1967Helper {
    Vm internal constant vm = Vm(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);

    function getAdmin(address _proxy) internal view returns (address) {
        return address(uint160(uint256(vm.load(_proxy, Constants.PROXY_OWNER_ADDRESS))));
    }

    function setAdmin(address _proxy, address _admin) internal {
        vm.store(_proxy, Constants.PROXY_OWNER_ADDRESS, bytes32(uint256(uint160(_admin))));
    }

    function getImplementation(address _proxy) internal view returns (address) {
        return address(uint160(uint256(vm.load(_proxy, Constants.PROXY_IMPLEMENTATION_ADDRESS))));
    }

    function setImplementation(address _proxy, address _impl) internal {
        vm.store(_proxy, Constants.PROXY_IMPLEMENTATION_ADDRESS, bytes32(uint256(uint160(_impl))));
    }
}
