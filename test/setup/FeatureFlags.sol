// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Vm } from "lib/forge-std/src/Vm.sol";

// Interfaces
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";

/// @notice Provides helpers for checking SystemConfig-gated test behavior.
abstract contract FeatureFlags {
    Vm private constant vm = Vm(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);

    ISystemConfig private sysCfg;

    function setSystemConfig(ISystemConfig _sysCfg) internal {
        sysCfg = _sysCfg;
    }

    function isSysFeatureEnabled(bytes32 _feature) public view returns (bool) {
        return sysCfg.isFeatureEnabled(_feature);
    }

    /// @notice Skips tests when the provided system feature is enabled.
    /// @param _feature The feature to check.
    function skipIfSysFeatureEnabled(bytes32 _feature) public {
        if (isSysFeatureEnabled(_feature)) {
            vm.skip(true);
        }
    }

    /// @notice Skips tests when the provided system feature is disabled.
    /// @param _feature The feature to check.
    function skipIfSysFeatureDisabled(bytes32 _feature) public {
        if (!isSysFeatureEnabled(_feature)) {
            vm.skip(true);
        }
    }
}
