// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title EOA
/// @notice A library for detecting if an address is an EOA.
library EOA {
    /// @notice Returns true if sender address is an EOA.
    /// @return isEOA_ True if the sender address is an EOA.
    function isSenderEOA() internal view returns (bool isEOA_) {
        // If the sender is not the origin, check for 7702 delegated EOAs.
        isEOA_ = msg.sender == tx.origin || isDelegated(msg.sender);
    }

    /// @notice Returns true if the account is a 7702 delegated EOA.
    /// @param _account Address of the account to check.
    /// @return isDelegated_ True if the account is a 7702 delegated EOA.
    function isDelegated(address _account) internal view returns (bool isDelegated_) {
        // A 7702 delegated EOA has exactly 23 bytes of code, starting with 0xEF0100.
        if (_account.code.length == 23) {
            assembly {
                let ptr := mload(0x40)
                mstore(0x40, add(ptr, 0x20))
                extcodecopy(_account, ptr, 0, 0x20)
                isDelegated_ := eq(shr(232, mload(ptr)), 0xEF0100)
            }
        }
    }
}
