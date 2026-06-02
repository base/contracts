// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/// @title MockDelayedWETH
/// @notice Minimal mock exposing only the deposit/unlock/withdraw entry points the dev
///         scripts exercise. The real DelayedWETH enforces a withdrawal delay and the
///         full ERC20/proxy surface, none of which is needed for local deploys.
contract MockDelayedWETH {
    function deposit() external payable { }

    function unlock(address, uint256) external { }

    function withdraw(address recipient, uint256 amount) external {
        (bool ok,) = payable(recipient).call{ value: amount }("");
        require(ok, "MockDelayedWETH: withdraw failed");
    }

    receive() external payable { }
}
