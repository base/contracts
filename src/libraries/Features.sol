// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Features is a library that stores feature name constants. Can be used alongside the
///         feature flagging functionality in the SystemConfig contract to selectively enable or
///         disable customizable features of the OP Stack.
library Features {
    /// @notice The CUSTOM_GAS_TOKEN feature determines if the system is configured to use a custom
    ///         gas token in the OptimismPortal. When the CUSTOM_GAS_TOKEN feature is active, the
    ///         deposits and withdrawals of native ETH are disabled.
    bytes32 internal constant CUSTOM_GAS_TOKEN = "CUSTOM_GAS_TOKEN";
}
