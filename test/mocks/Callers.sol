// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Burn } from "src/libraries/Burn.sol";

contract CallRecorder {
    struct CallInfo {
        address sender;
        bytes data;
        uint256 gas;
        uint256 value;
    }

    CallInfo public lastCall;

    function record() public payable {
        lastCall.sender = msg.sender;
        lastCall.data = msg.data;
        lastCall.gas = gasleft();
        lastCall.value = msg.value;
    }
}

/// @dev Any call will revert
contract Reverter {
    function doRevert() public pure {
        revert("Reverter: Reverter reverted");
    }

    fallback() external {
        revert();
    }
}

/// @dev Can be etched in to any address to test making a delegatecall from that address.
contract DelegateCaller {
    function dcForward(address _target, bytes calldata _data) external {
        assembly {
            calldatacopy(0x0, _data.offset, _data.length)
            let success := delegatecall(gas(), _target, 0x0, _data.length, 0x0, 0x0)

            let size := returndatasize()
            returndatacopy(0x0, 0x0, size)

            if iszero(success) { revert(0x0, size) }

            return(0x0, size)
        }
    }
}

/// @title GasBurner
/// @notice Contract that burns a specified amount of gas on receive or fallback.
contract GasBurner {
    uint256 internal constant GAS_BURN_OVERHEAD = 500;
    uint256 internal immutable GAS_TO_BURN;

    constructor(uint256 _gas) {
        GAS_TO_BURN = _gas - GAS_BURN_OVERHEAD;
    }

    receive() external payable {
        Burn.gas(GAS_TO_BURN);
    }

    fallback() external payable {
        Burn.gas(GAS_TO_BURN);
    }
}
