// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { console2 as console } from "lib/forge-std/src/console2.sol";
import { StdAssertions } from "lib/forge-std/src/StdAssertions.sol";

// Testing
import { DelegateCaller } from "test/mocks/Callers.sol";
import { FeatureFlags } from "test/setup/FeatureFlags.sol";

// Scripts
import { Deployer } from "scripts/deploy/Deployer.sol";
import { Deploy } from "scripts/deploy/Deploy.s.sol";
import { Config } from "scripts/libraries/Config.sol";

// Libraries
import { GameTypes, Claim } from "src/libraries/bridge/Types.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";
import { LibGameArgs } from "src/libraries/bridge/LibGameArgs.sol";
import { Types, IOPContractsManagerInterop } from "scripts/libraries/Types.sol";

// Interfaces
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { IFaultDisputeGameV2 } from "interfaces/L1/proofs/v2/IFaultDisputeGameV2.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IBigStepper } from "interfaces/L1/proofs/IBigStepper.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IETHLockbox } from "interfaces/L1/IETHLockbox.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";
import { IOPContractsManagerUpgrader } from "interfaces/L1/IOPContractsManager.sol";

/// @title ForkLive
/// @notice This script is called by Setup.sol as a preparation step for the foundry test suite, and is run as an
///         alternative to Deploy.s.sol, when `FORK_TEST=true` is set in the env.
///         Like Deploy.s.sol this script saves the system addresses to the Artifacts contract so that they can be
///         read by other contracts. However, rather than deploying new contracts from the local source code, it seeds
///         the fork with a small set of production entrypoint addresses and derives the rest onchain.
///         Therefore this script can only be run against a fork of a production network which is listed in
///         `forkSystemAddresses`.
///         This contract must not have constructor logic because it is set into state using `etch`.

contract ForkLive is Deployer, StdAssertions, FeatureFlags {
    bool public useOpsRepo;

    struct SystemAddresses {
        address systemConfig;
        address superchainConfig;
        address opcm;
    }

    struct GameAddresses {
        address anchorStateRegistry;
        address weth;
        address mips;
    }

    /// @notice Thrown when testing with an unsupported chain ID.
    error UnsupportedChainId();

    /// @notice Returns the production entrypoints for the current L1 fork.
    function forkSystemAddresses() internal view returns (SystemAddresses memory system_) {
        if (block.chainid == 1) {
            system_ = SystemAddresses({
                systemConfig: 0x73a79Fab69143498Ed3712e519A88a918e1f4072,
                superchainConfig: 0xb535ff7F118260a952CE65e7fF41B1743De8EE6c,
                opcm: 0x50F47B43c24F40B92C873Fa0704D4207586D0C9f
            });
        } else if (block.chainid == 11155111) {
            system_ = SystemAddresses({
                systemConfig: 0xf272670eb55e895584501d564AfEB048bEd26194,
                superchainConfig: 0xE4401EB53AE90a5335a51fe1828d7BeCf7a63508,
                opcm: 0xF0a2e224519E876979eA6B2cd15eF5CC3d6703bd
            });
        } else if (block.chainid == 560048) {
            system_ = SystemAddresses({
                systemConfig: 0xcC7c76564bea74A963A0Bd75E0bC9BcE3FF0EA80, superchainConfig: address(0), opcm: address(0)
            });
        } else {
            revert UnsupportedChainId();
        }
    }

    /// @dev This function sets up the system to test it as follows:
    ///      1. Check if the SUPERCHAIN_OPS_ALLOCS_PATH environment variable was set from superchain ops.
    ///      2. If set, load the state from the given path.
    ///      3. Derive the live system addresses from the configured fork entrypoints.
    ///      4. If the environment variable wasn't set, deploy the updated OPCM and implementations of the contracts.
    ///      5. Upgrade the system using the OPCM.upgrade() function if useUpgradedFork is true.
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
            // Now deploy the updated OPCM and implementations of the contracts.
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
    ///      stored by artifacts.save() after the call to opcm.upgrade().
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
        artifacts.save("OPContractsManager", system.opcm);

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

        // Fault proof proxied contracts
        IDisputeGameFactory disputeGameFactory = IDisputeGameFactory(systemConfig.disputeGameFactory());
        IFaultDisputeGameV2 permissionedDisputeGame =
            IFaultDisputeGameV2(address(disputeGameFactory.gameImpls(GameTypes.PERMISSIONED_CANNON)));
        GameAddresses memory gameAddresses = _permissionedGameAddresses(disputeGameFactory, permissionedDisputeGame);
        _saveProxyAndImpl("AnchorStateRegistry", gameAddresses.anchorStateRegistry);
        _saveProxyAndImpl("DisputeGameFactory", address(disputeGameFactory));

        // Fault proof non-proxied contracts
        IBigStepper mips = IBigStepper(gameAddresses.mips);
        artifacts.save("PreimageOracle", address(mips.oracle()));
        artifacts.save("MipsSingleton", address(mips));

        artifacts.save("PermissionedDisputeGame", address(permissionedDisputeGame));
        artifacts.save("PermissionedDelayedWETHProxy", gameAddresses.weth);

        // Pull the DelayedWETH addresses from the PermissionedDisputeGame so stale local data cannot break this.
        artifacts.save("DelayedWETHProxy", gameAddresses.weth);
        artifacts.save("DelayedWETHImpl", EIP1967Helper.getImplementation(gameAddresses.weth));
    }

    /// @notice Calls to the Deploy.s.sol contract etched by Setup.sol to a deterministic address, sets up the
    /// environment, and deploys new implementations.
    function _deployNewImplementations() internal {
        Deploy deploy = Deploy(address(uint160(uint256(keccak256(abi.encode("optimism.deploy"))))));
        deploy.deployImplementations({ _isInterop: false });
    }

    /// @notice Performs a single OPCM upgrade.
    /// @param _opcm The OPCM contract to upgrade.
    /// @param _delegateCaller The address of the upgrader to use for the upgrade.
    function _doUpgrade(IOPContractsManagerInterop _opcm, address _delegateCaller) internal {
        ISystemConfig systemConfig = ISystemConfig(artifacts.mustGetAddress("SystemConfigProxy"));
        Types.OpChainConfig[] memory opChains = new Types.OpChainConfig[](1);
        opChains[0] = Types.OpChainConfig({
            systemConfigProxy: systemConfig,
            cannonPrestate: Claim.wrap(bytes32(keccak256("cannonPrestate"))),
            cannonKonaPrestate: Claim.wrap(bytes32(keccak256("cannonKonaPrestate")))
        });

        // Turn the SuperchainPAO into a DelegateCaller so we can try to upgrade the
        // SuperchainConfig contract.
        ISuperchainConfig superchainConfig = ISuperchainConfig(artifacts.mustGetAddress("SuperchainConfigProxy"));
        IProxyAdmin superchainProxyAdmin = IProxyAdmin(EIP1967Helper.getAdmin(address(superchainConfig)));
        address superchainPAO = superchainProxyAdmin.owner();
        bytes memory superchainPAOCode = address(superchainPAO).code;
        vm.etch(superchainPAO, vm.getDeployedCode("test/mocks/Callers.sol:DelegateCaller"));

        // Always try to upgrade the SuperchainConfig. Not always necessary but easier to do it
        // every time rather than adding or removing this code for each upgrade.
        try DelegateCaller(superchainPAO)
            .dcForward(
                address(_opcm), abi.encodeCall(IOPContractsManagerInterop.upgradeSuperchainConfig, (superchainConfig))
            ) {
        // Great, the upgrade succeeded.
        }
        catch (bytes memory reason) {
            // Only acceptable revert reason is the SuperchainConfig already being up to date.
            assertTrue(
                bytes4(reason)
                    == IOPContractsManagerUpgrader.OPContractsManagerUpgrader_SuperchainConfigAlreadyUpToDate.selector,
                "Revert reason other than SuperchainConfigAlreadyUpToDate"
            );
        }

        // Reset the superchainPAO to the original code.
        vm.etch(superchainPAO, superchainPAOCode);

        // Temporarily replace the upgrader with a DelegateCaller so we can test the upgrade,
        // then reset its code to the original code.
        bytes memory upgraderCode = address(_delegateCaller).code;
        vm.etch(_delegateCaller, vm.getDeployedCode("test/mocks/Callers.sol:DelegateCaller"));

        // Upgrade the chain.
        DelegateCaller(_delegateCaller)
            .dcForward(address(_opcm), abi.encodeCall(IOPContractsManagerInterop.upgrade, (opChains)));

        // Reset the upgrader to the original code.
        vm.etch(_delegateCaller, upgraderCode);
    }

    /// @notice Upgrades the contracts using the OPCM.
    function _upgrade() internal {
        IOPContractsManagerInterop opcm = IOPContractsManagerInterop(artifacts.mustGetAddress("OPContractsManager"));

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
        _doUpgrade(opcm, upgrader);

        console.log("ForkLive: Saving newly deployed contracts");

        // A new ASR and new dispute games were deployed, so we need to update them
        IDisputeGameFactory disputeGameFactory =
            IDisputeGameFactory(artifacts.mustGetAddress("DisputeGameFactoryProxy"));
        address permissionedDisputeGame = address(disputeGameFactory.gameImpls(GameTypes.PERMISSIONED_CANNON));
        artifacts.save("PermissionedDisputeGame", permissionedDisputeGame);

        IAnchorStateRegistry newAnchorStateRegistry = IAnchorStateRegistry(
            LibGameArgs.decode(disputeGameFactory.gameArgs(GameTypes.PERMISSIONED_CANNON)).anchorStateRegistry
        );
        artifacts.save("AnchorStateRegistryProxy", address(newAnchorStateRegistry));

        // Get the lockbox address from the portal, and save it
        IOptimismPortal2 portal = IOptimismPortal2(artifacts.mustGetAddress("OptimismPortalProxy"));
        address lockboxAddress = address(portal.ethLockbox());
        artifacts.save("ETHLockboxProxy", lockboxAddress);

        // Get the new DelayedWETH address and save it (might be a new proxy).
        IDelayedWETH newDelayedWeth =
            IDelayedWETH(payable(LibGameArgs.decode(disputeGameFactory.gameArgs(GameTypes.PERMISSIONED_CANNON)).weth));
        artifacts.save("DelayedWETHProxy", address(newDelayedWeth));
        artifacts.save("DelayedWETHImpl", EIP1967Helper.getImplementation(address(newDelayedWeth)));
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

    /// @notice Returns the addresses encoded for the permissioned dispute game.
    function _permissionedGameAddresses(
        IDisputeGameFactory _disputeGameFactory,
        IFaultDisputeGameV2 _permissionedDisputeGame
    )
        internal
        view
        returns (GameAddresses memory game_)
    {
        bytes memory gameArgs = _disputeGameFactory.gameArgs(GameTypes.PERMISSIONED_CANNON);
        if (
            gameArgs.length == LibGameArgs.PERMISSIONED_ARGS_LENGTH
                || gameArgs.length == LibGameArgs.PERMISSIONLESS_ARGS_LENGTH
        ) {
            LibGameArgs.GameArgs memory decoded = LibGameArgs.decode(gameArgs);
            return
                GameAddresses({
                    anchorStateRegistry: decoded.anchorStateRegistry, weth: decoded.weth, mips: decoded.vm
                });
        }

        return GameAddresses({
            anchorStateRegistry: address(_permissionedDisputeGame.anchorStateRegistry()),
            weth: address(_permissionedDisputeGame.weth()),
            mips: address(_permissionedDisputeGame.vm())
        });
    }
}
