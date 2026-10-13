// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { console2 as console } from "lib/forge-std/src/console2.sol";
import { Script } from "lib/forge-std/src/Script.sol";

// Scripts
import { Artifacts } from "scripts/Artifacts.s.sol";
import { Config } from "scripts/libraries/Config.sol";

// Libraries
import { GameTypes } from "src/libraries/bridge/Types.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

// Interfaces
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IAggregateVerifier } from "interfaces/L1/proofs/IAggregateVerifier.sol";
import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";

/// @title ForkLive
/// @notice This script is called by Setup.sol as a preparation step for the foundry test suite, and is run as an
///         alternative to SystemDeploy.s.sol, when `FORK_TEST=true` is set in the env.
///         Like SystemDeploy.s.sol this script saves the system addresses to the Artifacts contract so that they can be
///         read by other contracts. However, rather than deploying new contracts from the local source code, it seeds
///         the fork with a small set of production entrypoint addresses and derives the rest onchain.
///         Therefore this script can only be run against a fork of a production network which is listed in
///         `forkSystemAddresses`.
///         This contract must not have constructor logic because it is set into state using `etch`.

contract ForkLive is Script {
    Artifacts internal constant artifacts =
        Artifacts(address(uint160(uint256(keccak256(abi.encode("optimism.artifacts"))))));

    bool public useOpsRepo;

    struct SystemAddresses {
        address systemConfig;
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
            system_ = SystemAddresses({ systemConfig: 0x73a79Fab69143498Ed3712e519A88a918e1f4072 });
        } else if (block.chainid == 11155111) {
            system_ = SystemAddresses({ systemConfig: 0xf272670eb55e895584501d564AfEB048bEd26194 });
        } else if (block.chainid == 560048) {
            system_ = SystemAddresses({ systemConfig: 0xcC7c76564bea74A963A0Bd75E0bC9BcE3FF0EA80 });
        } else {
            revert UnsupportedChainId();
        }
    }

    /// @notice Loads simulated superchain-ops upgrade state and derives the system addresses from it.
    function run() public {
        string memory superchainOpsAllocsPath = Config.superchainOpsAllocsPath();

        require(bytes(superchainOpsAllocsPath).length > 0, "ForkLive: SUPERCHAIN_OPS_ALLOCS_PATH must be set");
        console.log("ForkLive: loading state from %s", superchainOpsAllocsPath);
        // Load the simulated post-upgrade state before deriving implementation addresses.
        vm.loadAllocs(superchainOpsAllocsPath);
        useOpsRepo = true;

        _readForkAddresses();
    }

    /// @notice Reads the loaded fork system and saves its addresses to the artifact registry.
    function _readForkAddresses() internal {
        SystemAddresses memory system = forkSystemAddresses();
        ISystemConfig systemConfig = ISystemConfig(system.systemConfig);
        ISystemConfig.Addresses memory systemConfigAddresses = systemConfig.getAddresses();

        console.log("ForkLive: loading addresses from SystemConfig %s", address(systemConfig));

        // Slightly hacky, we encode the uint chainId as an address to save it in Artifacts
        artifacts.save("L2ChainId", address(uint160(systemConfig.l2ChainId())));
        // Core contracts
        artifacts.save("ProxyAdmin", EIP1967Helper.getAdmin(address(systemConfig)));
        _saveProxyAndImpl("SystemConfig", address(systemConfig));

        // Bridge contracts
        address optimismPortal = systemConfigAddresses.optimismPortal;
        artifacts.save("OptimismPortalProxy", optimismPortal);
        artifacts.save("OptimismPortal2Impl", EIP1967Helper.getImplementation(optimismPortal));

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

    /// @notice Saves the proxy and implementation addresses for a contract name.
    function _saveProxyAndImpl(string memory _contractName, address _proxy) internal {
        artifacts.save(string.concat(_contractName, "Proxy"), _proxy);

        address impl = EIP1967Helper.getImplementation(_proxy);
        require(impl != address(0), "ForkLive: Implementation address is zero");
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
