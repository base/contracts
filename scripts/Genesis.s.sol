// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Script } from "lib/forge-std/src/Script.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";

import { DeployUtils } from "scripts/libraries/DeployUtils.sol";
import { L2Genesis } from "scripts/L2Genesis.s.sol";
import { SetPreinstalls } from "scripts/SetPreinstalls.s.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Hash } from "src/libraries/bridge/Types.sol";

/// @notice Offline Base L1 deployment. No RPC or transaction broadcasting is used.
contract BaseL1Genesis is Script {
    function generate(string memory input, string memory output) public {
        BaseL1Deployment deployment = new BaseL1Deployment();
        deployment.generate(input, output);
    }
}

/// @dev An ordinary helper contract permits SystemDeploy's address(this) ownership checks.
contract BaseL1Deployment is SystemDeploy {
    /// @dev The typed deployment API does not need the upstream artifact registry or provenance FFI.
    function setUp() public override { }

    function generate(string memory input, string memory output) public {
        DeployUtils.etchLabelAndAllowCheatcodes(address(cfg), "DeployConfig");
        cfg.read(input);
        DeployInput memory args = _deployInput();
        args.saveArtifacts = false;
        args.opChainInput.saltMixer = vm.toString(vm.parseJsonBytes32(vm.readFile(input), ".salt"));
        DeployOutput memory deployed = deploy(args);
        (Hash root, uint256 number) = deployed.opChain.anchorStateRegistryProxy.getAnchorRoot();
        require(Hash.unwrap(root) == cfg.multiproofGenesisOutputRoot() && number == 0, "incorrect anchor");
        require(deployed.opChain.systemConfigProxy.owner() == cfg.finalSystemOwner(), "incorrect owner");
        require(
            deployed.opChain.systemConfigProxy.batcherHash() == bytes32(uint256(uint160(cfg.batchSenderAddress()))),
            "incorrect batcher"
        );
        string memory key = "addresses";
        vm.serializeAddress(key, "SuperchainConfigProxy", address(deployed.superchain.superchainConfigProxy));
        vm.serializeAddress(key, "L1CrossDomainMessengerProxy", address(deployed.opChain.l1CrossDomainMessengerProxy));
        vm.serializeAddress(key, "L1StandardBridgeProxy", address(deployed.opChain.l1StandardBridgeProxy));
        vm.serializeAddress(key, "L1ERC721BridgeProxy", address(deployed.opChain.l1ERC721BridgeProxy));
        vm.serializeAddress(key, "OptimismPortalProxy", address(deployed.opChain.optimismPortalProxy));
        vm.serializeAddress(key, "SystemConfigProxy", address(deployed.opChain.systemConfigProxy));
        vm.serializeAddress(key, "DisputeGameFactoryProxy", address(deployed.opChain.disputeGameFactoryProxy));
        vm.serializeAddress(key, "AnchorStateRegistryProxy", address(deployed.opChain.anchorStateRegistryProxy));
        vm.serializeAddress(key, "DelayedWETHProxy", address(deployed.opChain.delayedWETHProxy));
        vm.serializeAddress(key, "OpChainProxyAdmin", address(deployed.opChain.opChainProxyAdmin));
        string memory json =
            vm.serializeAddress(key, "ProtocolVersionsProxy", address(deployed.opChain.protocolVersionsProxy));
        vm.writeJson(json, string.concat(output, "/addresses.json"));
        SetPreinstalls preinstalls = new SetPreinstalls();
        preinstalls.setPreinstalls();
        // These are execution helpers, not genesis contracts. The genesis assembler removes their full accounts.
        address[] memory helpers = new address[](5);
        helpers[0] = address(cfg);
        helpers[1] = address(preinstalls);
        helpers[2] = msg.sender;
        helpers[3] = address(this);
        helpers[4] = tx.origin;
        vm.writeJson(vm.serializeAddress("cleanup", "helpers", helpers), string.concat(output, "/cleanup.json"));
        vm.dumpState(string.concat(output, "/alloc.json"));
    }
}

/// @notice Initialize predeploys using the exact same contracts build as the L1 system.
contract BaseL2Genesis is Script {
    using stdJson for string;

    function generate(string memory input, string memory output) public {
        string memory config = vm.readFile(input);
        string memory addresses = vm.readFile(config.readString(".addressesPath"));
        L2Genesis.Input memory args;
        args.l1ChainID = config.readUint(".l1ChainId");
        args.l2ChainID = config.readUint(".l2ChainId");
        args.l1CrossDomainMessengerProxy = payable(addresses.readAddress(".L1CrossDomainMessengerProxy"));
        args.l1StandardBridgeProxy = payable(addresses.readAddress(".L1StandardBridgeProxy"));
        args.l1ERC721BridgeProxy = payable(addresses.readAddress(".L1ERC721BridgeProxy"));
        args.opChainProxyAdminOwner = config.readAddress(".finalSystemOwner");
        args.sequencerFeeVaultRecipient = config.readAddress(".sequencerFeeVaultRecipient");
        args.sequencerFeeVaultMinimumWithdrawalAmount = config.readUint(".sequencerFeeVaultMinimumWithdrawalAmount");
        args.sequencerFeeVaultWithdrawalNetwork = config.readUint(".sequencerFeeVaultWithdrawalNetwork");
        args.baseFeeVaultRecipient = config.readAddress(".baseFeeVaultRecipient");
        args.baseFeeVaultMinimumWithdrawalAmount = config.readUint(".baseFeeVaultMinimumWithdrawalAmount");
        args.baseFeeVaultWithdrawalNetwork = config.readUint(".baseFeeVaultWithdrawalNetwork");
        args.l1FeeVaultRecipient = config.readAddress(".l1FeeVaultRecipient");
        args.l1FeeVaultMinimumWithdrawalAmount = config.readUint(".l1FeeVaultMinimumWithdrawalAmount");
        args.l1FeeVaultWithdrawalNetwork = config.readUint(".l1FeeVaultWithdrawalNetwork");
        args.operatorFeeVaultRecipient = config.readAddress(".operatorFeeVaultRecipient");
        args.operatorFeeVaultMinimumWithdrawalAmount = config.readUint(".operatorFeeVaultMinimumWithdrawalAmount");
        args.operatorFeeVaultWithdrawalNetwork = config.readUint(".operatorFeeVaultWithdrawalNetwork");
        args.fork = config.readUint(".fork");
        args.fundDevAccounts = true;
        L2Genesis genesis = new L2Genesis();
        genesis.run(args);
        GenesisDump dump = new GenesisDump();
        dump.exportState(output, address(genesis));
    }
}

/// @dev Exporting from a helper lets us identify the ephemeral script account without relying on it.
contract GenesisDump is Script {
    function exportState(string memory output, address genesis) public {
        address[] memory helpers = new address[](4);
        helpers[0] = msg.sender;
        helpers[1] = address(this);
        helpers[2] = genesis;
        helpers[3] = tx.origin;
        vm.writeJson(vm.serializeAddress("cleanup", "helpers", helpers), string.concat(output, "/cleanup.json"));
        vm.dumpState(string.concat(output, "/alloc.json"));
    }
}
