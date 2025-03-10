// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { Claim } from "src/dispute/lib/Types.sol";

/// @title IOPContractsManagerLegacyUpgrade
/// @notice Interface for the legacy OPContractsManager upgrade function.
///         This interface is used to test Upgrade 13 and 14 paths and can be safely removed
///         after those upgrades are completed. Only difference in the new struct is the added
///         disputeGameUsesSuperRoots boolean.
interface IOPContractsManagerLegacyUpgrade {
    struct OpChainConfig {
        ISystemConfig systemConfigProxy;
        IProxyAdmin proxyAdmin;
        Claim absolutePrestate;
    }

    function upgrade(OpChainConfig[] memory _opChainConfigs) external;
}
