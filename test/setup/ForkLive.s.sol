// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { console2 as console } from "lib/forge-std/src/console2.sol";
import { Script } from "lib/forge-std/src/Script.sol";
import { StdAssertions } from "lib/forge-std/src/StdAssertions.sol";

import { FeatureFlags } from "test/setup/FeatureFlags.sol";

// Scripts
import { Artifacts } from "scripts/Artifacts.s.sol";
import { DeployConfig } from "scripts/deploy/DeployConfig.s.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Config } from "scripts/libraries/Config.sol";

// Libraries
import { GameTypes } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";
import { Types } from "scripts/libraries/Types.sol";

// Interfaces
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IAggregateVerifier } from "interfaces/L1/proofs/IAggregateVerifier.sol";
import { IAggregateVerifier } from "interfaces/L1/proofs/IAggregateVerifier.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IETHLockbox } from "interfaces/L1/IETHLockbox.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";

/// @title ForkLive
/// @notice This script is called by Setup.sol as a preparation step for the foundry test suite, and is run as an
///         alternative to SystemDeploy.s.sol, when `FORK_TEST=true` is set in the env.
///         Like SystemDeploy.s.sol this script saves the system addresses to the Artifacts contract so that they can be
///         read by other contracts. However, rather than deploying new contracts from the local source code, it seeds
///         the fork with a small set of production entrypoint addresses and derives the rest onchain.
///         Therefore this script can only be run against a fork of a production network which is listed in
///         `forkSystemAddresses`.
///         This contract must not have constructor logic because it is set into state using `etch`.

contract ForkLive is Script, StdAssertions, FeatureFlags {
    DeployConfig internal constant cfg =
        DeployConfig(address(uint160(uint256(keccak256(abi.encode("optimism.deployconfig"))))));

    Artifacts internal constant artifacts =
        Artifacts(address(uint160(uint256(keccak256(abi.encode("optimism.artifacts"))))));

    bool public useOpsRepo;

    struct SystemAddresses {
        address systemConfig;
        address superchainConfig;
    }

    struct GameAddresses {
        address anchorStateRegistry;
        address weth;
    }

    /// @notice Thrown when testing with an unsupported chain ID.
    error UnsupportedChainId();

    /// @notice Returns the production entrypoints for the current L1 fork.
    function forkSystemAddresses() internal view returns (SystemAddresses memory system_) {
        if (block.chainid == 1) {
            system_ = SystemAddresses({
                systemConfig: 0x73a79Fab69143498Ed3712e519A88a918e1f4072,
                superchainConfig: 0xb535ff7F118260a952CE65e7fF41B1743De8EE6c
            });
        } else if (block.chainid == 11155111) {
            system_ = SystemAddresses({
                systemConfig: 0xf272670eb55e895584501d564AfEB048bEd26194,
                superchainConfig: 0xE4401EB53AE90a5335a51fe1828d7BeCf7a63508
            });
        } else if (block.chainid == 560048) {
            system_ = SystemAddresses({
                systemConfig: 0xcC7c76564bea74A963A0Bd75E0bC9BcE3FF0EA80, superchainConfig: address(0)
            });
        } else {
            revert UnsupportedChainId();
        }
    }

    /// @dev This function sets up the system to test it as follows:
    ///      1. Check if the SUPERCHAIN_OPS_ALLOCS_PATH environment variable was set from superchain ops.
    ///      2. If set, load the state from the given path.
    ///      3. Derive the live system addresses from the configured fork entrypoints.
    ///      4. If the environment variable wasn't set, deploy the updated implementations of the contracts.
    ///      5. Upgrade the system using SystemDeploy.upgrade() if useUpgradedFork is true.
    function run() public {
        string memory superchainOpsAllocsPath = Config.superchainOpsAllocsPath();

        useOpsRepo = bytes(superchainOpsAllocsPath).length > 0;
        if (useOpsRepo) {
            console.log("ForkLive: loading state from %s", superchainOpsAllocsPath);
            // Set the resultant state from the superchain ops repo upgrades.
            // The allocs are generated when simulating an upgrade task that runs vm.dumpState.
            // These allocs represent the state of the EVM after the upgrade has been simulated.
            vm.loadAllocs(superchainOpsAllocsPath);
            // Next, fetch the addresses from the configured fork entrypoints. This function uses a local EVM
            // to retrieve implementation addresses by reading from the live proxy addresses.
            // Setting the allocs first ensures the correct implementation addresses are retrieved.
            _readForkAddresses();
        } else {
            // Read the live system and save the addresses to the Artifacts contract.
            _readForkAddresses();
            // Now deploy the updated implementations of the contracts.
            _deployNewImplementations();
        }

        // Now upgrade the contracts (if the config is set to do so)
        if (useOpsRepo) {
            console.log("ForkLive: using ops repo to upgrade");
        } else if (cfg.useUpgradedFork()) {
            console.log("ForkLive: upgrading");
            _upgrade();
        }
    }

    /// @notice Reads the live fork system and saves the addresses to disk.
    /// @dev During development of an upgrade which adds a new contract, the contract will not yet be derivable from
    ///      the live fork. In this case, the contract will be deployed by the upgrade process, and will need to be
    ///      stored by artifacts.save() after the call to SystemDeploy.upgrade().
    function _readForkAddresses() internal {
        SystemAddresses memory system = forkSystemAddresses();
        ISystemConfig systemConfig = ISystemConfig(system.systemConfig);
        ISystemConfig.Addresses memory systemConfigAddresses = systemConfig.getAddresses();

        console.log("ForkLive: loading addresses from SystemConfig %s", address(systemConfig));

        // Slightly hacky, we encode the uint chainId as an address to save it in Artifacts
        artifacts.save("L2ChainId", address(uint160(systemConfig.l2ChainId())));
        // Superchain shared contracts
        address superchainConfig = system.superchainConfig;
        if (superchainConfig == address(0)) {
            try systemConfig.superchainConfig() returns (ISuperchainConfig superchainConfig_) {
                superchainConfig = address(superchainConfig_);
            } catch { }
        }
        if (superchainConfig != address(0)) {
            _saveProxyAndImpl("SuperchainConfig", superchainConfig);
        }
        // Core contracts
        artifacts.save("ProxyAdmin", EIP1967Helper.getAdmin(address(systemConfig)));
        _saveProxyAndImpl("SystemConfig", address(systemConfig));

        // Bridge contracts
        address optimismPortal = systemConfigAddresses.optimismPortal;
        artifacts.save("OptimismPortalProxy", optimismPortal);
        artifacts.save("OptimismPortal2Impl", EIP1967Helper.getImplementation(optimismPortal));

        // Get the lockbox address from the portal, and save it
        /// NOTE: Using try catch because this function could be called before or after the upgrade.
        try IOptimismPortal2(payable(optimismPortal)).ethLockbox() returns (IETHLockbox ethLockbox_) {
            console.log("ForkLive: ETHLockboxProxy found: %s", address(ethLockbox_));
            artifacts.save("ETHLockboxProxy", address(ethLockbox_));
        } catch {
            console.log("ForkLive: ETHLockboxProxy not found");
        }

        address l1CrossDomainMessenger = systemConfigAddresses.l1CrossDomainMessenger;
        address addressManager = _legacyAddressManager(l1CrossDomainMessenger);
        artifacts.save("AddressManager", addressManager);
        artifacts.save(
            "L1CrossDomainMessengerImpl", IAddressManager(addressManager).getAddress("OVM_L1CrossDomainMessenger")
        );
        artifacts.save("L1CrossDomainMessengerProxy", l1CrossDomainMessenger);
        _saveProxyAndImpl("OptimismMintableERC20Factory", systemConfigAddresses.optimismMintableERC20Factory);
        _saveProxyAndImpl("L1StandardBridge", systemConfigAddresses.l1StandardBridge);
        _saveProxyAndImpl("L1ERC721Bridge", systemConfigAddresses.l1ERC721Bridge);

        IDisputeGameFactory disputeGameFactory = IDisputeGameFactory(systemConfig.disputeGameFactory());
        IAggregateVerifier aggregateVerifier =
            IAggregateVerifier(address(disputeGameFactory.gameImpls(GameTypes.AGGREGATE_VERIFIER)));
        GameAddresses memory gameAddresses = _aggregateVerifierAddresses(aggregateVerifier);
        _saveProxyAndImpl("AnchorStateRegistry", gameAddresses.anchorStateRegistry);
        _saveProxyAndImpl("DisputeGameFactory", address(disputeGameFactory));

        // Pull DelayedWETH from the AggregateVerifier so stale local data cannot break this.
        artifacts.save("DelayedWETHProxy", gameAddresses.weth);
        artifacts.save("DelayedWETHImpl", EIP1967Helper.getImplementation(gameAddresses.weth));
    }

    /// @notice Calls to the SystemDeploy.s.sol contract etched by Setup.sol to a deterministic address, sets up the
    /// environment, and deploys new implementations.
    function _deployNewImplementations() internal {
        SystemDeploy deploy = SystemDeploy(address(uint160(uint256(keccak256(abi.encode("optimism.deploy"))))));
        deploy.deployImplementations();
    }

    /// @notice Performs a script-level upgrade without a manager delegatecall.
    /// @param _upgrader The address of the OP Chain ProxyAdmin owner to use for the chain upgrade.
    function _doUpgrade(address _upgrader) internal {
        SystemDeploy systemDeploy = new SystemDeploy();
        Types.Implementations memory implementations = _latestImplementations();

        ISystemConfig systemConfigProxy = ISystemConfig(artifacts.mustGetAddress("SystemConfigProxy"));

        ISuperchainConfig superchainConfig = ISuperchainConfig(artifacts.mustGetAddress("SuperchainConfigProxy"));
        IProxyAdmin superchainProxyAdmin = IProxyAdmin(EIP1967Helper.getAdmin(address(superchainConfig)));
        address superchainPAO = superchainProxyAdmin.owner();

        // Run the shared SuperchainConfig upgrade as the Superchain ProxyAdmin owner. The script
        // skips this step when the proxy is already at or above the target implementation version.
        vm.prank(superchainPAO);
        systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: superchainConfig,
                implementations: implementations,
                systemConfigProxy: ISystemConfig(address(0))
            })
        );

        // Run the per-chain upgrade as the OP Chain ProxyAdmin owner.
        vm.prank(_upgrader);
        systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: ISuperchainConfig(address(0)),
                implementations: implementations,
                systemConfigProxy: systemConfigProxy
            })
        );
    }

    /// @notice Upgrades the contracts using the script-level upgrade API.
    function _upgrade() internal {
        ISystemConfig systemConfig = ISystemConfig(artifacts.mustGetAddress("SystemConfigProxy"));
        IProxyAdmin proxyAdmin = IProxyAdmin(EIP1967Helper.getAdmin(address(systemConfig)));

        address upgrader = proxyAdmin.owner();
        vm.label(upgrader, "ProxyAdmin Owner");

        // Run past upgrades depending on network.
        if (block.chainid == 1) {
            // Mainnet
            // This is empty because the block number in the justfile is after the most recent upgrade so there are no
            // past upgrades to run.
        } else {
            revert UnsupportedChainId();
        }

        // Current upgrade.
        _doUpgrade(upgrader);

        console.log("ForkLive: Saving newly deployed contracts");

        // A new ASR and new dispute games were deployed, so we need to update them
        IDisputeGameFactory disputeGameFactory =
            IDisputeGameFactory(artifacts.mustGetAddress("DisputeGameFactoryProxy"));
        IDisputeGame av = disputeGameFactory.gameImpls(GameTypes.AGGREGATE_VERIFIER);
        artifacts.save("AggregateVerifier", address(av));

        IAnchorStateRegistry newAnchorStateRegistry = av.anchorStateRegistry();
        artifacts.save("AnchorStateRegistryProxy", address(newAnchorStateRegistry));

        // Get the lockbox address from the portal, and save it
        IOptimismPortal2 portal = IOptimismPortal2(artifacts.mustGetAddress("OptimismPortalProxy"));
        address lockboxAddress = address(portal.ethLockbox());
        artifacts.save("ETHLockboxProxy", lockboxAddress);

        // Get the new DelayedWETH address and save it (might be a new proxy).
        IDelayedWETH newDelayedWeth = IAggregateVerifier(address(av)).DELAYED_WETH();
        artifacts.save("DelayedWETHProxy", address(newDelayedWeth));
        artifacts.save("DelayedWETHImpl", EIP1967Helper.getImplementation(address(newDelayedWeth)));
    }

    /// @notice Returns the latest implementation set saved by deployImplementations.
    function _latestImplementations() internal view returns (Types.Implementations memory) {
        SystemDeploy deploy = SystemDeploy(address(uint160(uint256(keccak256(abi.encode("optimism.deploy"))))));
        return deploy.getImplementations();
    }

    /// @notice Saves the proxy and implementation addresses for a contract name.
    function _saveProxyAndImpl(string memory _contractName, address _proxy) internal {
        artifacts.save(string.concat(_contractName, "Proxy"), _proxy);

        address impl = EIP1967Helper.getImplementation(_proxy);
        require(impl != address(0), "Upgrade: Implementation address is zero");
        artifacts.save(string.concat(_contractName, "Impl"), impl);
    }

    /// @notice Returns the AddressManager backing the legacy L1CrossDomainMessenger proxy.
    function _legacyAddressManager(address _proxy) internal view returns (address) {
        return address(uint160(uint256(vm.load(_proxy, keccak256(abi.encode(_proxy, uint256(1)))))));
    }

    /// @notice Returns the addresses used by the AggregateVerifier.
    function _aggregateVerifierAddresses(IAggregateVerifier _aggregateVerifier)
        internal
        view
        returns (GameAddresses memory game_)
    {
        return GameAddresses({
            anchorStateRegistry: address(_aggregateVerifier.anchorStateRegistry()),
            weth: address(_aggregateVerifier.DELAYED_WETH())
        });
    }
}
