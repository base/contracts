// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/// @title MockDelayedWETH
/// @notice Minimal mock for testing - implements the IDelayedWETH interface.
/// @dev For testing purposes only. The real DelayedWETH handles bond deposits and withdrawals.
contract MockDelayedWETH {
    /// @notice Accepts ETH deposits (no-op for testing).
    function deposit() external payable {}

    /// @notice Mock unlock - no-op for testing.
    function unlock(address, uint256) external {}

    /// @notice Mock withdraw - transfers ETH back.
    /// @param recipient The address to send ETH to.
    /// @param amount The amount of ETH to withdraw.
    function withdraw(address recipient, uint256 amount) external {
        payable(recipient).transfer(amount);
    }

    /// @notice Allows contract to receive ETH.
    receive() external payable {}
}
