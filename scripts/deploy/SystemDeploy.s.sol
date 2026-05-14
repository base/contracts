// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { VmSafe } from "lib/forge-std/src/Vm.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";
import { Script } from "lib/forge-std/src/Script.sol";

import { Artifacts } from "scripts/Artifacts.s.sol";
import { Config } from "scripts/libraries/Config.sol";
import { DeployConfig } from "scripts/deploy/DeployConfig.s.sol";
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";
import { StateDiff } from "scripts/libraries/StateDiff.sol";
import { Types } from "scripts/libraries/Types.sol";

import { IETHLockbox } from "interfaces/L1/IETHLockbox.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { IL1ERC721Bridge } from "interfaces/L1/IL1ERC721Bridge.sol";
import { IL1StandardBridge } from "interfaces/L1/IL1StandardBridge.sol";
import { IOptimismPortal2 as IOptimismPortal } from "interfaces/L1/IOptimismPortal2.sol";
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";
import { IL1ChugSplashProxy } from "interfaces/legacy/IL1ChugSplashProxy.sol";
import { IResolvedDelegateProxy } from "interfaces/legacy/IResolvedDelegateProxy.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";
import { ITEEProverRegistry } from "interfaces/L1/proofs/tee/ITEEProverRegistry.sol";
import { IOptimismMintableERC20Factory } from "interfaces/universal/IOptimismMintableERC20Factory.sol";
import { IProxy } from "interfaces/universal/IProxy.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";

import { AddressManager } from "src/legacy/AddressManager.sol";
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { ZKVerifier } from "src/L1/proofs/zk/ZKVerifier.sol";
import { Constants } from "src/libraries/Constants.sol";
import { SemverComp } from "src/libraries/SemverComp.sol";
import { GameType, GameTypes, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { Duration, Claim } from "src/libraries/bridge/LibUDT.sol";

/// @title SystemDeploy
/// @notice Script-level API for deploying or upgrading a complete OP Stack L1 system.
contract SystemDeploy is Script {
    DeployConfig public constant cfg =
        DeployConfig(address(uint160(uint256(keccak256(abi.encode("optimism.deployconfig"))))));

    Artifacts internal constant artifacts =
        Artifacts(address(uint160(uint256(keccak256(abi.encode("optimism.artifacts"))))));

    uint256 internal constant ETH_MAINNET_CHAIN_ID = 1;
    uint256 internal constant ETH_SEPOLIA_CHAIN_ID = 11155111;

    struct SuperchainInput {
        address guardian;
        address incidentResponder;
        address superchainProxyAdminOwner;
    }

    struct SuperchainOutput {
        ISuperchainConfig superchainConfigImpl;
        ISuperchainConfig superchainConfigProxy;
        IProxyAdmin superchainProxyAdmin;
    }

    struct ImplementationInput {
        uint256 withdrawalDelaySeconds;
        uint256 proofMaturityDelaySeconds;
        uint256 disputeGameFinalityDelaySeconds;
        bytes32 teeImageHash;
        bytes32 zkRangeHash;
        bytes32 zkAggregationHash;
        bytes32 multiproofConfigHash;
        uint256 multiproofGameType;
        address nitroEnclaveVerifier;
        uint256 l2ChainID;
        uint256 multiproofBlockInterval;
        uint256 multiproofIntermediateBlockInterval;
        ISP1Verifier sp1Verifier;
        address teeProposer;
        address teeChallenger;
        ISuperchainConfig superchainConfigProxy;
        IProxyAdmin superchainProxyAdmin;
        address guardian;
        address incidentResponder;
    }

    struct DeployInput {
        bool deploySuperchain;
        bool deployImplementations;
        bool saveArtifacts;
        SuperchainInput superchainInput;
        ISuperchainConfig superchainConfigProxy;
        ImplementationInput implementationsInput;
        Types.Implementations implementations;
        Types.DeployInput opChainInput;
    }

    struct DeployOutput {
        SuperchainOutput superchain;
        Types.Implementations impls;
        Types.DeployOutput opChain;
    }

    struct UpgradeInput {
        bool saveArtifacts;
        ISuperchainConfig superchainConfigProxy;
        Types.Implementations implementations;
        ISystemConfig[] systemConfigProxies;
    }

    struct UpgradeOutput {
        bool superchainConfigUpgraded;
        uint256 chainsUpgraded;
    }

    struct AggregateVerifierInput {
        GameType multiproofGameType;
        IAnchorStateRegistry anchorStateRegistry;
        IDelayedWETH delayedWETH;
        IVerifier teeVerifier;
        IVerifier zkVerifier;
        bytes32 teeImageHash;
        bytes32 zkRangeHash;
        bytes32 zkAggregationHash;
        bytes32 multiproofConfigHash;
        uint256 l2ChainID;
        uint256 multiproofBlockInterval;
        uint256 multiproofIntermediateBlockInterval;
    }

    struct MultiproofOutput {
        IVerifier aggregateVerifier;
        TEEProverRegistry teeProverRegistryProxy;
        TEEProverRegistry teeProverRegistryImpl;
        IVerifier teeVerifier;
        IVerifier zkVerifier;
    }

    event Deployed(uint256 indexed l2ChainId, address indexed deployer, bytes deployOutput);
    event Upgraded(uint256 indexed l2ChainId, ISystemConfig indexed systemConfig, address indexed upgrader);

    error InvalidChainId();
    error InvalidRoleAddress(string role);
    error InvalidStartingAnchorRoot();
    error MissingImplementations();
    error PrestateNotSet();
    error SuperchainConfigNeedsUpgrade(uint256 index);

    /// @notice Sets up the shared deployment config and artifact registry.
    function setUp() public virtual {
        DeployUtils.etchLabelAndAllowCheatcodes({ _etchTo: address(artifacts), _cname: "Artifacts" });
        artifacts.setUp();

        console.log("Commit hash: %s", gitCommitHash());

        DeployUtils.etchLabelAndAllowCheatcodes({ _etchTo: address(cfg), _cname: "DeployConfig" });
        cfg.read(Config.deployConfigPath());
    }

    /// @notice Returns the commit hash of HEAD, or the packaged .gitcommit file when no git repository is available.
    function gitCommitHash() internal returns (string memory) {
        string[] memory command = new string[](3);
        command[0] = "bash";
        command[1] = "-c";
        command[2] = "cast abi-encode 'f(string)' $(git rev-parse HEAD || cat .gitcommit)";
        return string(vm.ffi(command));
    }

    /// @notice Records the deployment state diff to `snapshots/state-diff/<chainid>.json`.
    modifier stateDiff() {
        vm.startStateDiffRecording();
        _;
        VmSafe.AccountAccess[] memory accesses = vm.stopAndReturnStateDiff();
        console.log(
            "Writing %d state diff account accesses to snapshots/state-diff/%s.json",
            accesses.length,
            vm.toString(block.chainid)
        );
        string memory json = StateDiff.encodeAccountAccesses(accesses);
        string memory statediffPath =
            string.concat(vm.projectRoot(), "/snapshots/state-diff/", vm.toString(block.chainid), ".json");
        vm.writeJson({ json: json, path: statediffPath });
    }

    /// @notice Deploys a fresh OP Stack from the active deploy config.
    function run() public {
        console.log("Deploying a fresh OP Stack including SuperchainConfig");
        _runConfigured();
    }

    /// @notice Deploys a fresh OP Stack and writes the state diff for Kontrol summaries.
    function runWithStateDiff() public stateDiff {
        _runConfigured();
    }

    /// @notice Deploys implementation contracts from the active deploy config and saves their artifact names.
    function deployImplementations() public returns (Types.Implementations memory output_) {
        ISuperchainConfig superchainConfigProxy = ISuperchainConfig(artifacts.mustGetAddress("SuperchainConfigProxy"));
        IProxyAdmin superchainProxyAdmin = superchainConfigProxy.proxyAdmin();

        output_ = _deployImplementations(_configuredImplementationsInput(superchainConfigProxy, superchainProxyAdmin));
        _saveUpgradeArtifacts(output_);
    }

    /// @notice Deploys the shared Superchain proxy admin and SuperchainConfig proxy.
    function deploySuperchain(SuperchainInput memory _input) public returns (SuperchainOutput memory output_) {
        output_ = _deploySuperchain(_input);
    }

    /// @notice Returns the latest implementation set saved by `deployImplementations` or `run`.
    function getImplementations() public view returns (Types.Implementations memory) {
        return Types.Implementations({
            superchainConfigImpl: artifacts.mustGetAddress("SuperchainConfigImpl"),
            l1ERC721BridgeImpl: artifacts.mustGetAddress("L1ERC721BridgeImpl"),
            optimismPortalImpl: artifacts.mustGetAddress("OptimismPortalImpl"),
            ethLockboxImpl: artifacts.mustGetAddress("ETHLockboxImpl"),
            systemConfigImpl: artifacts.mustGetAddress("SystemConfigImpl"),
            optimismMintableERC20FactoryImpl: artifacts.mustGetAddress("OptimismMintableERC20FactoryImpl"),
            l1CrossDomainMessengerImpl: artifacts.mustGetAddress("L1CrossDomainMessengerImpl"),
            l1StandardBridgeImpl: artifacts.mustGetAddress("L1StandardBridgeImpl"),
            disputeGameFactoryImpl: artifacts.mustGetAddress("DisputeGameFactoryImpl"),
            anchorStateRegistryImpl: artifacts.mustGetAddress("AnchorStateRegistryImpl"),
            delayedWETHImpl: artifacts.mustGetAddress("DelayedWETHImpl"),
            aggregateVerifierImpl: artifacts.getAddress("AggregateVerifier"),
            teeProverRegistryImpl: artifacts.getAddress("TEEProverRegistryImpl"),
            teeVerifierImpl: artifacts.getAddress("TEEVerifier"),
            zkVerifierImpl: artifacts.getAddress("ZKVerifier")
        });
    }

    function _runConfigured() internal returns (DeployOutput memory output_) {
        output_ = deploy(_deployInput());

        vm.startPrank(ISuperchainConfig(artifacts.mustGetAddress("SuperchainConfigProxy")).guardian());
        IAnchorStateRegistry(artifacts.mustGetAddress("AnchorStateRegistryProxy"))
            .setRespectedGameType(GameType.wrap(uint32(cfg.respectedGameType())));
        vm.stopPrank();

        console.log("set up op chain!");
    }

    function _deployInput() internal view returns (DeployInput memory input_) {
        Types.Implementations memory emptyImpls;
        input_ = DeployInput({
            deploySuperchain: true,
            deployImplementations: true,
            saveArtifacts: true,
            superchainInput: SuperchainInput({
                guardian: cfg.superchainConfigGuardian(),
                incidentResponder: cfg.superchainConfigIncidentResponder(),
                superchainProxyAdminOwner: cfg.finalSystemOwner()
            }),
            superchainConfigProxy: ISuperchainConfig(address(0)),
            implementationsInput: _configuredImplementationsInput(
                ISuperchainConfig(address(0)), IProxyAdmin(address(0))
            ),
            implementations: emptyImpls,
            opChainInput: _configuredOPChainInput()
        });
    }

    function _configuredImplementationsInput(
        ISuperchainConfig _superchainConfigProxy,
        IProxyAdmin _superchainProxyAdmin
    )
        internal
        view
        returns (ImplementationInput memory input_)
    {
        input_ = ImplementationInput({
            withdrawalDelaySeconds: cfg.faultGameWithdrawalDelay(),
            proofMaturityDelaySeconds: cfg.proofMaturityDelaySeconds(),
            disputeGameFinalityDelaySeconds: cfg.disputeGameFinalityDelaySeconds(),
            teeImageHash: cfg.teeImageHash(),
            zkRangeHash: cfg.zkRangeHash(),
            zkAggregationHash: cfg.zkAggregationHash(),
            multiproofConfigHash: cfg.multiproofConfigHash(),
            multiproofGameType: cfg.multiproofGameType(),
            nitroEnclaveVerifier: cfg.nitroEnclaveVerifier(),
            l2ChainID: cfg.l2ChainId(),
            multiproofBlockInterval: cfg.multiproofBlockInterval(),
            multiproofIntermediateBlockInterval: cfg.multiproofIntermediateBlockInterval(),
            sp1Verifier: ISP1Verifier(cfg.sp1Verifier()),
            teeProposer: cfg.teeProposer(),
            teeChallenger: cfg.teeChallenger(),
            superchainConfigProxy: _superchainConfigProxy,
            superchainProxyAdmin: _superchainProxyAdmin,
            guardian: cfg.superchainConfigGuardian(),
            incidentResponder: cfg.superchainConfigIncidentResponder()
        });
    }

    function _configuredOPChainInput() internal view returns (Types.DeployInput memory input_) {
        input_ = Types.DeployInput({
            roles: Types.Roles({
                opChainProxyAdminOwner: cfg.finalSystemOwner(),
                systemConfigOwner: cfg.finalSystemOwner(),
                batcher: cfg.batchSenderAddress(),
                unsafeBlockSigner: cfg.p2pSequencerAddress()
            }),
            basefeeScalar: cfg.basefeeScalar(),
            blobBasefeeScalar: cfg.blobbasefeeScalar(),
            l2ChainId: cfg.l2ChainId(),
            startingAnchorRoot: Proposal({
                root: Hash.wrap(cfg.multiproofGenesisOutputRoot()), l2SequenceNumber: cfg.multiproofGenesisBlockNumber()
            }),
            saltMixer: "salt mixer",
            gasLimit: uint64(cfg.l2GenesisBlockGasLimit()),
            disputeGameType: GameTypes.AGGREGATE_VERIFIER
        });
    }

    function deploy(DeployInput memory _input) public returns (DeployOutput memory output_) {
        _assertValidDeployInput(_input);

        output_.superchain = _deployOrLoadSuperchain(_input);
        if (_input.deployImplementations) {
            output_.impls = _deployImplementations(_withSuperchainImplementationsInput(_input, output_.superchain));
        } else {
            _assertValidImplementations(_input.implementations);
            output_.impls = _input.implementations;
        }

        Types.Implementations memory implementations;
        (output_.opChain, implementations) = _deployOPChain({
            _input: _input.opChainInput,
            _superchainConfig: output_.superchain.superchainConfigProxy,
            _impls: output_.impls,
            _implementationsInput: _withSuperchainImplementationsInput(_input, output_.superchain)
        });
        output_.impls = implementations;

        if (_input.saveArtifacts) {
            _saveDeployArtifacts(output_);
        }

        emit Deployed(_input.opChainInput.l2ChainId, msg.sender, abi.encode(output_.opChain));
    }

    function upgrade(UpgradeInput memory _input) public returns (UpgradeOutput memory output_) {
        _assertValidImplementations(_input.implementations);

        if (address(_input.superchainConfigProxy) != address(0)) {
            output_.superchainConfigUpgraded =
                _upgradeSuperchainConfigIfNeeded(_input.superchainConfigProxy, _input.implementations);
        }

        for (uint256 i = 0; i < _input.systemConfigProxies.length; i++) {
            ISystemConfig systemConfigProxy = _input.systemConfigProxies[i];
            DeployUtils.assertValidContractAddress(address(systemConfigProxy));

            ISuperchainConfig superchainConfig = systemConfigProxy.superchainConfig();
            if (SemverComp.lt(
                    superchainConfig.version(), ISuperchainConfig(_input.implementations.superchainConfigImpl).version()
                )) {
                revert SuperchainConfigNeedsUpgrade(i);
            }

            _upgradeOPChain(systemConfigProxy, _input.implementations);
            output_.chainsUpgraded++;
        }

        if (_input.saveArtifacts) {
            _saveUpgradeArtifacts(_input.implementations);
        }
    }

    function _deployOrLoadSuperchain(DeployInput memory _input) internal returns (SuperchainOutput memory output_) {
        if (_input.deploySuperchain) {
            output_ = _deploySuperchain(_input.superchainInput);
        } else {
            DeployUtils.assertValidContractAddress(address(_input.superchainConfigProxy));
            output_.superchainConfigProxy = _input.superchainConfigProxy;
            output_.superchainProxyAdmin = _input.superchainConfigProxy.proxyAdmin();
        }
    }

    function _withSuperchainImplementationsInput(
        DeployInput memory _input,
        SuperchainOutput memory _superchain
    )
        internal
        pure
        returns (ImplementationInput memory input_)
    {
        input_ = _input.implementationsInput;
        input_.superchainConfigProxy = _superchain.superchainConfigProxy;
        input_.superchainProxyAdmin = _superchain.superchainProxyAdmin;
    }

    function _deploySuperchain(SuperchainInput memory _input) internal returns (SuperchainOutput memory output_) {
        _assertValidSuperchainInput(_input);

        output_.superchainProxyAdmin = _deploySuperchainProxyAdmin();
        output_.superchainConfigImpl = _deploySuperchainConfigImpl(_input.guardian, _input.incidentResponder);
        output_.superchainConfigProxy =
            _deploySuperchainConfigProxy(output_.superchainProxyAdmin, output_.superchainConfigImpl);

        DeployUtils.assertValidContractAddress(address(output_.superchainProxyAdmin));
        vm.broadcast(msg.sender);
        output_.superchainProxyAdmin.transferOwnership(_input.superchainProxyAdminOwner);

        _assertValidSuperchainOutput(_input, output_);
    }

    function _deploySuperchainProxyAdmin() internal returns (IProxyAdmin proxyAdmin_) {
        vm.broadcast(msg.sender);
        proxyAdmin_ = IProxyAdmin(
            DeployUtils.create1({
                _name: "ProxyAdmin",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IProxyAdmin.__constructor__, (msg.sender)))
            })
        );
        vm.label(address(proxyAdmin_), "SuperchainProxyAdmin");
    }

    function _deploySuperchainConfigProxy(
        IProxyAdmin _proxyAdmin,
        ISuperchainConfig _impl
    )
        internal
        returns (ISuperchainConfig proxy_)
    {
        vm.startBroadcast(msg.sender);
        proxy_ = ISuperchainConfig(
            DeployUtils.create1({
                _name: "src/universal/Proxy.sol:Proxy",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IProxy.__constructor__, (address(_proxyAdmin))))
            })
        );
        _proxyAdmin.upgrade(payable(address(proxy_)), address(_impl));
        vm.stopBroadcast();

        vm.label(address(proxy_), "SuperchainConfigProxy");
    }

    function _assertValidSuperchainInput(SuperchainInput memory _input) internal pure {
        if (_input.superchainProxyAdminOwner == address(0)) revert InvalidRoleAddress("superchainProxyAdminOwner");
        if (_input.guardian == address(0)) revert InvalidRoleAddress("guardian");
    }

    function _assertValidSuperchainOutput(SuperchainInput memory _input, SuperchainOutput memory _output) internal {
        address[] memory addrs = new address[](3);
        addrs[0] = address(_output.superchainProxyAdmin);
        addrs[1] = address(_output.superchainConfigImpl);
        addrs[2] = address(_output.superchainConfigProxy);
        DeployUtils.assertValidContractAddresses(addrs);

        vm.startPrank(address(0));
        require(
            IProxy(payable(address(_output.superchainConfigProxy))).implementation()
                == address(_output.superchainConfigImpl),
            "SUPCON-30"
        );
        require(
            IProxy(payable(address(_output.superchainConfigProxy))).admin() == address(_output.superchainProxyAdmin),
            "SUPCON-40"
        );
        vm.stopPrank();

        require(_output.superchainProxyAdmin.owner() == _input.superchainProxyAdminOwner, "SPA-10");
        require(_output.superchainConfigProxy.guardian() == _input.guardian, "SUPCON-10");
        require(_output.superchainConfigImpl.guardian() == _input.guardian, "SUPCON-50");
    }

    function _deployImplementations(ImplementationInput memory _input)
        internal
        returns (Types.Implementations memory output_)
    {
        _assertValidImplementationInput(_input);

        output_.superchainConfigImpl = address(_deploySuperchainConfigImpl(_input.guardian, _input.incidentResponder));
        output_.systemConfigImpl = address(_deploySystemConfigImpl());
        output_.l1CrossDomainMessengerImpl = address(_deployL1CrossDomainMessengerImpl());
        output_.l1ERC721BridgeImpl = address(_deployL1ERC721BridgeImpl());
        output_.l1StandardBridgeImpl = address(_deployL1StandardBridgeImpl());
        output_.optimismMintableERC20FactoryImpl = address(_deployOptimismMintableERC20FactoryImpl());
        output_.optimismPortalImpl = address(_deployOptimismPortalImpl(_input));
        output_.ethLockboxImpl = address(_deployETHLockboxImpl());
        output_.delayedWETHImpl = address(_deployDelayedWETHImpl(_input));
        output_.disputeGameFactoryImpl = address(_deployDisputeGameFactoryImpl());
        output_.anchorStateRegistryImpl = address(_deployAnchorStateRegistryImpl(_input));
    }

    function _deployOPChain(
        Types.DeployInput memory _input,
        ISuperchainConfig _superchainConfig,
        Types.Implementations memory _impls,
        ImplementationInput memory _implementationsInput
    )
        internal
        returns (Types.DeployOutput memory output_, Types.Implementations memory impls_)
    {
        _assertValidOPChainInput(_input);
        impls_ = _impls;

        output_.opChainProxyAdmin = IProxyAdmin(
            _createDeterministic(
                "ProxyAdmin",
                DeployUtils.encodeConstructor(abi.encodeCall(IProxyAdmin.__constructor__, (msg.sender))),
                _input,
                "ProxyAdmin"
            )
        );
        output_.addressManager = _deployAddressManager(_input, output_.opChainProxyAdmin);

        vm.broadcast(msg.sender);
        output_.opChainProxyAdmin.setAddressManager(output_.addressManager);

        output_.l1ERC721BridgeProxy = IL1ERC721Bridge(_deployProxy(_input, output_.opChainProxyAdmin, "L1ERC721Bridge"));
        output_.optimismPortalProxy =
            IOptimismPortal(payable(_deployProxy(_input, output_.opChainProxyAdmin, "OptimismPortal")));
        output_.ethLockboxProxy = IETHLockbox(_deployProxy(_input, output_.opChainProxyAdmin, "ETHLockbox"));
        output_.systemConfigProxy = ISystemConfig(_deployProxy(_input, output_.opChainProxyAdmin, "SystemConfig"));
        output_.optimismMintableERC20FactoryProxy = IOptimismMintableERC20Factory(
            _deployProxy(_input, output_.opChainProxyAdmin, "OptimismMintableERC20Factory")
        );
        output_.disputeGameFactoryProxy =
            IDisputeGameFactory(_deployProxy(_input, output_.opChainProxyAdmin, "DisputeGameFactory"));
        output_.anchorStateRegistryProxy =
            IAnchorStateRegistry(_deployProxy(_input, output_.opChainProxyAdmin, "AnchorStateRegistry"));

        output_.l1StandardBridgeProxy = IL1StandardBridge(
            payable(_createDeterministic(
                    "L1ChugSplashProxy",
                    DeployUtils.encodeConstructor(
                        abi.encodeCall(IL1ChugSplashProxy.__constructor__, (address(output_.opChainProxyAdmin)))
                    ),
                    _input,
                    "L1StandardBridge"
                ))
        );
        vm.broadcast(msg.sender);
        output_.opChainProxyAdmin.setProxyType(address(output_.l1StandardBridgeProxy), IProxyAdmin.ProxyType.CHUGSPLASH);

        string memory messengerName = "OVM_L1CrossDomainMessenger";
        output_.l1CrossDomainMessengerProxy = IL1CrossDomainMessenger(
            _createDeterministic(
                "ResolvedDelegateProxy",
                DeployUtils.encodeConstructor(
                    abi.encodeCall(IResolvedDelegateProxy.__constructor__, (output_.addressManager, messengerName))
                ),
                _input,
                "L1CrossDomainMessenger"
            )
        );
        vm.broadcast(msg.sender);
        output_.opChainProxyAdmin
            .setProxyType(address(output_.l1CrossDomainMessengerProxy), IProxyAdmin.ProxyType.RESOLVED);
        vm.broadcast(msg.sender);
        output_.opChainProxyAdmin.setImplementationName(address(output_.l1CrossDomainMessengerProxy), messengerName);

        _initializeOPChain(_input, _superchainConfig, impls_, output_);

        output_.delayedWETHPermissionlessGameProxy =
            IDelayedWETH(payable(_deployProxy(_input, output_.opChainProxyAdmin, "DelayedWETHPermissionlessGame")));
        _upgradeToAndCall(
            output_.opChainProxyAdmin,
            address(output_.delayedWETHPermissionlessGameProxy),
            _impls.delayedWETHImpl,
            abi.encodeCall(IDelayedWETH.initialize, (output_.systemConfigProxy))
        );

        if (_multiproofEnabled(_implementationsInput)) {
            MultiproofOutput memory multiproof = _deployMultiproofContracts(_input, _implementationsInput, output_);
            impls_.aggregateVerifierImpl = address(multiproof.aggregateVerifier);
            impls_.teeProverRegistryImpl = address(multiproof.teeProverRegistryImpl);
            impls_.teeVerifierImpl = address(multiproof.teeVerifier);
            impls_.zkVerifierImpl = address(multiproof.zkVerifier);
            output_.aggregateVerifier = multiproof.aggregateVerifier;
            output_.teeProverRegistryProxy = ITEEProverRegistry(address(multiproof.teeProverRegistryProxy));
            output_.teeVerifier = multiproof.teeVerifier;
            output_.zkVerifier = multiproof.zkVerifier;
            output_.nitroEnclaveVerifier = INitroEnclaveVerifier(_implementationsInput.nitroEnclaveVerifier);
            output_.sp1Verifier = _implementationsInput.sp1Verifier;
        }

        _transferOwnership(address(output_.disputeGameFactoryProxy), _input.roles.opChainProxyAdminOwner);
        _transferOwnership(address(output_.opChainProxyAdmin), _input.roles.opChainProxyAdminOwner);
    }

    function _initializeOPChain(
        Types.DeployInput memory _input,
        ISuperchainConfig _superchainConfig,
        Types.Implementations memory _impls,
        Types.DeployOutput memory _output
    )
        internal
    {
        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.l1ERC721BridgeProxy),
            _impls.l1ERC721BridgeImpl,
            abi.encodeCall(IL1ERC721Bridge.initialize, (_output.l1CrossDomainMessengerProxy, _output.systemConfigProxy))
        );

        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.systemConfigProxy),
            _impls.systemConfigImpl,
            _encodeSystemConfigInitializer(_input, _output, _superchainConfig)
        );

        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.optimismPortalProxy),
            _impls.optimismPortalImpl,
            abi.encodeCall(IOptimismPortal.initialize, (_output.systemConfigProxy, _output.anchorStateRegistryProxy))
        );

        IOptimismPortal[] memory portals = new IOptimismPortal[](1);
        portals[0] = _output.optimismPortalProxy;
        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.ethLockboxProxy),
            _impls.ethLockboxImpl,
            abi.encodeCall(IETHLockbox.initialize, (_output.systemConfigProxy, portals))
        );

        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.optimismMintableERC20FactoryProxy),
            _impls.optimismMintableERC20FactoryImpl,
            abi.encodeCall(IOptimismMintableERC20Factory.initialize, (address(_output.l1StandardBridgeProxy)))
        );

        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.l1CrossDomainMessengerProxy),
            _impls.l1CrossDomainMessengerImpl,
            abi.encodeCall(IL1CrossDomainMessenger.initialize, (_output.systemConfigProxy, _output.optimismPortalProxy))
        );

        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.l1StandardBridgeProxy),
            _impls.l1StandardBridgeImpl,
            abi.encodeCall(
                IL1StandardBridge.initialize, (_output.l1CrossDomainMessengerProxy, _output.systemConfigProxy)
            )
        );

        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.disputeGameFactoryProxy),
            _impls.disputeGameFactoryImpl,
            abi.encodeCall(IDisputeGameFactory.initialize, (msg.sender))
        );

        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(_output.anchorStateRegistryProxy),
            _impls.anchorStateRegistryImpl,
            _encodeAnchorStateRegistryInitializer(_input, _output)
        );
    }

    function _upgradeSuperchainConfigIfNeeded(
        ISuperchainConfig _superchainConfig,
        Types.Implementations memory _impls
    )
        internal
        returns (bool upgraded_)
    {
        if (SemverComp.gte(_superchainConfig.version(), ISuperchainConfig(_impls.superchainConfigImpl).version())) {
            return false;
        }

        IProxyAdmin superchainProxyAdmin = _superchainConfig.proxyAdmin();
        _upgradeTo(superchainProxyAdmin, address(_superchainConfig), _impls.superchainConfigImpl);
        upgraded_ = true;
    }

    // TODO: upgrade multiproofs here too
    function _upgradeOPChain(ISystemConfig _systemConfigProxy, Types.Implementations memory _impls) internal {
        IProxyAdmin proxyAdmin = _systemConfigProxy.proxyAdmin();
        uint256 l2ChainId = _systemConfigProxy.l2ChainId();

        _upgradeTo(proxyAdmin, address(_systemConfigProxy), _impls.systemConfigImpl);

        IOptimismPortal optimismPortal = IOptimismPortal(payable(_systemConfigProxy.optimismPortal()));
        _upgradeTo(proxyAdmin, address(optimismPortal), _impls.optimismPortalImpl);
        _upgradeTo(proxyAdmin, address(optimismPortal.anchorStateRegistry()), _impls.anchorStateRegistryImpl);
        _upgradeTo(
            proxyAdmin, _systemConfigProxy.optimismMintableERC20Factory(), _impls.optimismMintableERC20FactoryImpl
        );

        IDisputeGameFactory disputeGameFactory = IDisputeGameFactory(_systemConfigProxy.disputeGameFactory());
        _upgradeTo(proxyAdmin, address(disputeGameFactory), _impls.disputeGameFactoryImpl);

        ISystemConfig.Addresses memory opChainAddrs = _systemConfigProxy.getAddresses();
        _upgradeTo(proxyAdmin, opChainAddrs.l1CrossDomainMessenger, _impls.l1CrossDomainMessengerImpl);
        _upgradeTo(proxyAdmin, opChainAddrs.l1StandardBridge, _impls.l1StandardBridgeImpl);
        _upgradeTo(proxyAdmin, opChainAddrs.l1ERC721Bridge, _impls.l1ERC721BridgeImpl);

        emit Upgraded(l2ChainId, _systemConfigProxy, msg.sender);
    }

    function _encodeSystemConfigInitializer(
        Types.DeployInput memory _input,
        Types.DeployOutput memory _output,
        ISuperchainConfig _superchainConfig
    )
        internal
        pure
        returns (bytes memory)
    {
        ISystemConfig.Addresses memory opChainAddrs = ISystemConfig.Addresses({
            l1CrossDomainMessenger: address(_output.l1CrossDomainMessengerProxy),
            l1ERC721Bridge: address(_output.l1ERC721BridgeProxy),
            l1StandardBridge: address(_output.l1StandardBridgeProxy),
            optimismPortal: address(_output.optimismPortalProxy),
            optimismMintableERC20Factory: address(_output.optimismMintableERC20FactoryProxy),
            delayedWETH: address(0)
        });

        return abi.encodeCall(
            ISystemConfig.initialize,
            (
                _input.roles.systemConfigOwner,
                _input.basefeeScalar,
                _input.blobBasefeeScalar,
                bytes32(uint256(uint160(_input.roles.batcher))),
                _input.gasLimit,
                _input.roles.unsafeBlockSigner,
                Constants.DEFAULT_RESOURCE_CONFIG(),
                Types.chainIdToBatchInboxAddress(_input.l2ChainId),
                opChainAddrs,
                _input.l2ChainId,
                _superchainConfig
            )
        );
    }

    function _encodeAnchorStateRegistryInitializer(
        Types.DeployInput memory _input,
        Types.DeployOutput memory _output
    )
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodeCall(
            IAnchorStateRegistry.initialize,
            (
                _output.systemConfigProxy,
                _output.disputeGameFactoryProxy,
                _input.startingAnchorRoot,
                GameTypes.AGGREGATE_VERIFIER
            )
        );
    }

    function _deployProxy(
        Types.DeployInput memory _input,
        IProxyAdmin _proxyAdmin,
        string memory _contractName
    )
        internal
        returns (address)
    {
        return _createDeterministic(
            "src/universal/Proxy.sol:Proxy",
            DeployUtils.encodeConstructor(abi.encodeCall(IProxy.__constructor__, (address(_proxyAdmin)))),
            _input,
            _contractName
        );
    }

    function _createDeterministic(
        string memory _name,
        bytes memory _args,
        Types.DeployInput memory _input,
        string memory _contractName
    )
        internal
        returns (address payable)
    {
        return DeployUtils.createDeterministic({
            _name: _name, _args: _args, _salt: keccak256(abi.encode(_input.l2ChainId, _input.saltMixer, _contractName))
        });
    }

    function _upgradeToAndCall(
        IProxyAdmin _proxyAdmin,
        address _target,
        address _implementation,
        bytes memory _data
    )
        internal
    {
        DeployUtils.assertValidContractAddress(_implementation);
        vm.broadcast(msg.sender);
        _proxyAdmin.upgradeAndCall(payable(_target), _implementation, _data);
    }

    function _upgradeTo(IProxyAdmin _proxyAdmin, address _target, address _implementation) internal {
        DeployUtils.assertValidContractAddress(_implementation);
        vm.broadcast(msg.sender);
        _proxyAdmin.upgrade(payable(_target), _implementation);
    }

    function _deployAddressManager(
        Types.DeployInput memory _input,
        IProxyAdmin _proxyAdmin
    )
        internal
        returns (IAddressManager)
    {
        bytes32 addressManagerSalt = keccak256(abi.encode(_input.l2ChainId, _input.saltMixer, "AddressManager"));
        AddressManagerDeployer deployer = AddressManagerDeployer(
            _createDeterministic(
                "scripts/deploy/SystemDeploy.s.sol:AddressManagerDeployer",
                abi.encode(addressManagerSalt, address(_proxyAdmin)),
                _input,
                "AddressManagerDeployer"
            )
        );
        return deployer.addressManager();
    }

    function _transferOwnership(address _target, address _newOwner) internal {
        if (IAddressManager(_target).owner() == address(this)) {
            IAddressManager(_target).transferOwnership(_newOwner);
            return;
        }
        vm.broadcast(msg.sender);
        IAddressManager(_target).transferOwnership(_newOwner);
    }

    function _deploySuperchainConfigImpl(
        address _guardian,
        address _incidentResponder
    )
        internal
        returns (ISuperchainConfig)
    {
        return ISuperchainConfig(
            DeployUtils.createDeterministic({
                _name: "SuperchainConfig",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(ISuperchainConfig.__constructor__, (_guardian, _incidentResponder))
                ),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deploySystemConfigImpl() internal returns (ISystemConfig) {
        return ISystemConfig(
            DeployUtils.createDeterministic({
                _name: "SystemConfig",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(ISystemConfig.__constructor__, ())),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployL1CrossDomainMessengerImpl() internal returns (IL1CrossDomainMessenger) {
        return IL1CrossDomainMessenger(
            DeployUtils.createDeterministic({
                _name: "L1CrossDomainMessenger",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IL1CrossDomainMessenger.__constructor__, ())),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployL1ERC721BridgeImpl() internal returns (IL1ERC721Bridge) {
        return IL1ERC721Bridge(
            DeployUtils.createDeterministic({
                _name: "L1ERC721Bridge",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IL1ERC721Bridge.__constructor__, ())),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployL1StandardBridgeImpl() internal returns (IL1StandardBridge) {
        return IL1StandardBridge(
            DeployUtils.createDeterministic({
                _name: "L1StandardBridge",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IL1StandardBridge.__constructor__, ())),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployOptimismMintableERC20FactoryImpl() internal returns (IOptimismMintableERC20Factory) {
        return IOptimismMintableERC20Factory(
            DeployUtils.createDeterministic({
                _name: "OptimismMintableERC20Factory",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IOptimismMintableERC20Factory.__constructor__, ())),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployOptimismPortalImpl(ImplementationInput memory _input) internal returns (IOptimismPortal) {
        return IOptimismPortal(
            DeployUtils.createDeterministic({
                _name: "OptimismPortal2",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IOptimismPortal.__constructor__, (_input.proofMaturityDelaySeconds))
                ),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployETHLockboxImpl() internal returns (IETHLockbox) {
        return IETHLockbox(
            DeployUtils.createDeterministic({
                _name: "ETHLockbox",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IETHLockbox.__constructor__, ())),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployDelayedWETHImpl(ImplementationInput memory _input) internal returns (IDelayedWETH) {
        return IDelayedWETH(
            DeployUtils.createDeterministic({
                _name: "DelayedWETH",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IDelayedWETH.__constructor__, (_input.withdrawalDelaySeconds))
                ),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployDisputeGameFactoryImpl() internal returns (IDisputeGameFactory) {
        return IDisputeGameFactory(
            DeployUtils.createDeterministic({
                _name: "DisputeGameFactory",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IDisputeGameFactory.__constructor__, ())),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployAnchorStateRegistryImpl(ImplementationInput memory _input) internal returns (IAnchorStateRegistry) {
        return IAnchorStateRegistry(
            DeployUtils.createDeterministic({
                _name: "AnchorStateRegistry",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IAnchorStateRegistry.__constructor__, (_input.disputeGameFinalityDelaySeconds))
                ),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
    }

    function _deployMultiproofContracts(
        Types.DeployInput memory _opChainInput,
        ImplementationInput memory _input,
        Types.DeployOutput memory _output
    )
        internal
        returns (MultiproofOutput memory output_)
    {
        _assertValidMultiproofInput(_opChainInput, _input);

        GameType gameType = GameType.wrap(uint32(_input.multiproofGameType));

        vm.broadcast(msg.sender);
        output_.teeProverRegistryImpl = new TEEProverRegistry(
            INitroEnclaveVerifier(_input.nitroEnclaveVerifier), _output.disputeGameFactoryProxy
        );

        output_.teeProverRegistryProxy =
            TEEProverRegistry(_deployProxy(_opChainInput, _output.opChainProxyAdmin, "TEEProverRegistry"));
        address[] memory initialProposers = new address[](2);
        initialProposers[0] = _input.teeProposer;
        initialProposers[1] = _input.teeChallenger;
        _upgradeToAndCall(
            _output.opChainProxyAdmin,
            address(output_.teeProverRegistryProxy),
            address(output_.teeProverRegistryImpl),
            abi.encodeCall(
                TEEProverRegistry.initialize,
                (
                    _opChainInput.roles.opChainProxyAdminOwner,
                    _opChainInput.roles.opChainProxyAdminOwner,
                    initialProposers,
                    gameType
                )
            )
        );

        INitroEnclaveVerifier nitroVerifier = INitroEnclaveVerifier(_input.nitroEnclaveVerifier);
        if (nitroVerifier.proofSubmitter() != address(output_.teeProverRegistryProxy)) {
            vm.broadcast(msg.sender);
            nitroVerifier.setProofSubmitter(address(output_.teeProverRegistryProxy));
        }

        vm.broadcast(msg.sender);
        output_.teeVerifier =
            IVerifier(address(new TEEVerifier(output_.teeProverRegistryProxy, _output.anchorStateRegistryProxy)));
        vm.broadcast(msg.sender);
        output_.zkVerifier = IVerifier(address(new ZKVerifier(_input.sp1Verifier, _output.anchorStateRegistryProxy)));

        output_.aggregateVerifier = _newAggregateVerifier(
            AggregateVerifierInput({
                multiproofGameType: gameType,
                anchorStateRegistry: _output.anchorStateRegistryProxy,
                delayedWETH: _output.delayedWETHPermissionlessGameProxy,
                teeVerifier: output_.teeVerifier,
                zkVerifier: output_.zkVerifier,
                teeImageHash: _input.teeImageHash,
                zkRangeHash: _input.zkRangeHash,
                zkAggregationHash: _input.zkAggregationHash,
                multiproofConfigHash: _input.multiproofConfigHash,
                l2ChainID: _input.l2ChainID,
                multiproofBlockInterval: _input.multiproofBlockInterval,
                multiproofIntermediateBlockInterval: _input.multiproofIntermediateBlockInterval
            })
        );

        vm.broadcast(msg.sender);
        _output.disputeGameFactoryProxy.setImplementation(gameType, IDisputeGame(address(output_.aggregateVerifier)));

        vm.label(address(output_.teeProverRegistryImpl), "TEEProverRegistryImpl");
        vm.label(address(output_.teeProverRegistryProxy), "TEEProverRegistryProxy");
        vm.label(address(output_.teeVerifier), "TEEVerifier");
        vm.label(address(output_.zkVerifier), "ZKVerifier");
        vm.label(address(output_.aggregateVerifier), "AggregateVerifier");
    }

    function _newAggregateVerifier(AggregateVerifierInput memory _input) internal returns (IVerifier) {
        vm.broadcast(msg.sender);
        return IVerifier(
            address(
                new AggregateVerifier(
                    _input.multiproofGameType,
                    _input.anchorStateRegistry,
                    _input.delayedWETH,
                    _input.teeVerifier,
                    _input.zkVerifier,
                    _input.teeImageHash,
                    AggregateVerifier.ZkHashes(_input.zkRangeHash, _input.zkAggregationHash),
                    _input.multiproofConfigHash,
                    _input.l2ChainID,
                    _input.multiproofBlockInterval,
                    _input.multiproofIntermediateBlockInterval
                )
            )
        );
    }

    function _assertValidDeployInput(DeployInput memory _input) internal pure {
        require(
            _input.deploySuperchain || address(_input.superchainConfigProxy) != address(0),
            "SystemDeploy: no superchain"
        );
        require(
            _input.deployImplementations || !_implementationsEmpty(_input.implementations), "SystemDeploy: no impls"
        );
    }

    function _assertValidOPChainInput(Types.DeployInput memory _input) internal view {
        if (_input.l2ChainId == 0 || _input.l2ChainId == block.chainid) revert InvalidChainId();
        if (_input.roles.opChainProxyAdminOwner == address(0)) revert InvalidRoleAddress("opChainProxyAdminOwner");
        if (_input.roles.systemConfigOwner == address(0)) revert InvalidRoleAddress("systemConfigOwner");
        if (_input.roles.batcher == address(0)) revert InvalidRoleAddress("batcher");
        if (_input.roles.unsafeBlockSigner == address(0)) revert InvalidRoleAddress("unsafeBlockSigner");
        if (Hash.unwrap(_input.startingAnchorRoot.root) == bytes32(0)) {
            revert InvalidStartingAnchorRoot();
        }
    }

    // TODO: add multiproof checks
    function _assertValidImplementationInput(ImplementationInput memory _input) internal pure {
        require(_input.withdrawalDelaySeconds != 0, "SystemDeploy: withdrawalDelaySeconds not set");
        require(_input.proofMaturityDelaySeconds != 0, "SystemDeploy: proofMaturityDelaySeconds not set");
        require(_input.disputeGameFinalityDelaySeconds != 0, "SystemDeploy: finality delay not set");
        require(address(_input.superchainConfigProxy) != address(0), "SystemDeploy: superchainConfigProxy not set");
        require(address(_input.superchainProxyAdmin) != address(0), "SystemDeploy: superchainProxyAdmin not set");
    }

    function _multiproofEnabled(ImplementationInput memory _input) internal pure returns (bool) {
        return _input.multiproofConfigHash != bytes32(0);
    }

    function _assertValidMultiproofInput(
        Types.DeployInput memory _opChainInput,
        ImplementationInput memory _input
    )
        internal
        view
    {
        require(_input.teeImageHash != bytes32(0), "SystemDeploy: teeImageHash not set");
        require(_input.zkRangeHash != bytes32(0), "SystemDeploy: zkRangeHash not set");
        require(_input.zkAggregationHash != bytes32(0), "SystemDeploy: zkAggregationHash not set");
        require(_input.multiproofConfigHash != bytes32(0), "SystemDeploy: multiproofConfigHash not set");
        require(_input.multiproofGameType != 0, "SystemDeploy: multiproofGameType not set");
        require(_input.nitroEnclaveVerifier != address(0), "SystemDeploy: nitroEnclaveVerifier not set");
        require(address(_input.sp1Verifier) != address(0), "SystemDeploy: sp1Verifier not set");
        DeployUtils.assertValidContractAddress(_input.nitroEnclaveVerifier);
        DeployUtils.assertValidContractAddress(address(_input.sp1Verifier));
        require(_input.l2ChainID == _opChainInput.l2ChainId, "SystemDeploy: multiproof l2ChainID mismatch");
        require(_input.multiproofBlockInterval != 0, "SystemDeploy: multiproof block interval not set");
        require(
            _input.multiproofIntermediateBlockInterval != 0, "SystemDeploy: multiproof intermediate interval not set"
        );
        require(
            _input.multiproofBlockInterval % _input.multiproofIntermediateBlockInterval == 0,
            "SystemDeploy: invalid multiproof block intervals"
        );
        require(_input.teeProposer != address(0), "SystemDeploy: teeProposer not set");
        require(_input.teeChallenger != address(0), "SystemDeploy: teeChallenger not set");
    }

    function _assertValidImplementations(Types.Implementations memory _impls) internal view {
        if (_implementationsEmpty(_impls)) revert MissingImplementations();
        DeployUtils.assertValidContractAddress(_impls.superchainConfigImpl);
        DeployUtils.assertValidContractAddress(_impls.l1ERC721BridgeImpl);
        DeployUtils.assertValidContractAddress(_impls.optimismPortalImpl);
        DeployUtils.assertValidContractAddress(_impls.ethLockboxImpl);
        DeployUtils.assertValidContractAddress(_impls.systemConfigImpl);
        DeployUtils.assertValidContractAddress(_impls.optimismMintableERC20FactoryImpl);
        DeployUtils.assertValidContractAddress(_impls.l1CrossDomainMessengerImpl);
        DeployUtils.assertValidContractAddress(_impls.l1StandardBridgeImpl);
        DeployUtils.assertValidContractAddress(_impls.disputeGameFactoryImpl);
        DeployUtils.assertValidContractAddress(_impls.anchorStateRegistryImpl);
        DeployUtils.assertValidContractAddress(_impls.delayedWETHImpl);
    }

    function _implementationsEmpty(Types.Implementations memory _impls) internal pure returns (bool) {
        return _impls.superchainConfigImpl == address(0) && _impls.systemConfigImpl == address(0)
            && _impls.l1CrossDomainMessengerImpl == address(0);
    }

    function _saveDeployArtifacts(DeployOutput memory _output) internal {
        _saveUpgradeArtifacts(_output.impls);

        artifacts.save("SuperchainProxyAdmin", address(_output.superchain.superchainProxyAdmin));
        artifacts.save("SuperchainConfigProxy", address(_output.superchain.superchainConfigProxy));

        Types.DeployOutput memory chain = _output.opChain;
        artifacts.save("ProxyAdmin", address(chain.opChainProxyAdmin));
        artifacts.save("AddressManager", address(chain.addressManager));
        artifacts.save("L1ERC721BridgeProxy", address(chain.l1ERC721BridgeProxy));
        artifacts.save("SystemConfigProxy", address(chain.systemConfigProxy));
        artifacts.save("OptimismMintableERC20FactoryProxy", address(chain.optimismMintableERC20FactoryProxy));
        artifacts.save("L1StandardBridgeProxy", address(chain.l1StandardBridgeProxy));
        artifacts.save("L1CrossDomainMessengerProxy", address(chain.l1CrossDomainMessengerProxy));
        artifacts.save("ETHLockboxProxy", address(chain.ethLockboxProxy));
        artifacts.save("DisputeGameFactoryProxy", address(chain.disputeGameFactoryProxy));
        artifacts.save("DelayedWETHProxy", address(chain.delayedWETHPermissionlessGameProxy));
        artifacts.save("AnchorStateRegistryProxy", address(chain.anchorStateRegistryProxy));
        artifacts.save("OptimismPortalProxy", address(chain.optimismPortalProxy));
        artifacts.save("OptimismPortal2Proxy", address(chain.optimismPortalProxy));
        _saveIfSet("TEEProverRegistryProxy", address(chain.teeProverRegistryProxy));
        _saveIfSet("TEEProverRegistry", address(chain.teeProverRegistryProxy));
        _saveIfSet("NitroEnclaveVerifier", address(chain.nitroEnclaveVerifier));
        _saveIfSet("SP1Verifier", address(chain.sp1Verifier));
    }

    function _saveIfSet(string memory _name, address _addr) internal {
        if (_addr != address(0)) {
            artifacts.save(_name, _addr);
        }
    }

    function _saveUpgradeArtifacts(Types.Implementations memory _impls) internal {
        artifacts.save("SuperchainConfigImpl", _impls.superchainConfigImpl);
        artifacts.save("L1ERC721BridgeImpl", _impls.l1ERC721BridgeImpl);
        artifacts.save("OptimismPortalImpl", _impls.optimismPortalImpl);
        artifacts.save("ETHLockboxImpl", _impls.ethLockboxImpl);
        artifacts.save("SystemConfigImpl", _impls.systemConfigImpl);
        artifacts.save("OptimismMintableERC20FactoryImpl", _impls.optimismMintableERC20FactoryImpl);
        artifacts.save("L1CrossDomainMessengerImpl", _impls.l1CrossDomainMessengerImpl);
        artifacts.save("L1StandardBridgeImpl", _impls.l1StandardBridgeImpl);
        artifacts.save("DisputeGameFactoryImpl", _impls.disputeGameFactoryImpl);
        artifacts.save("AnchorStateRegistryImpl", _impls.anchorStateRegistryImpl);
        artifacts.save("DelayedWETHImpl", _impls.delayedWETHImpl);
        _saveIfSet("AggregateVerifier", _impls.aggregateVerifierImpl);
        _saveIfSet("TEEProverRegistryImpl", _impls.teeProverRegistryImpl);
        _saveIfSet("TEEVerifier", _impls.teeVerifierImpl);
        _saveIfSet("ZKVerifier", _impls.zkVerifierImpl);
    }
}

contract AddressManagerDeployer {
    IAddressManager public immutable addressManager;

    constructor(bytes32 _salt, address _owner) {
        IAddressManager manager = IAddressManager(address(new AddressManager{ salt: _salt }()));
        manager.transferOwnership(_owner);
        addressManager = manager;
    }
}
