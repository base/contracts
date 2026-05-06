// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Testing
import { VmSafe } from "lib/forge-std/src/Vm.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

// Scripts
import { Deployer } from "scripts/deploy/Deployer.sol";
import { Chains } from "scripts/libraries/Chains.sol";
import { Config } from "scripts/libraries/Config.sol";
import { StateDiff } from "scripts/libraries/StateDiff.sol";
import { ChainAssertions } from "scripts/deploy/ChainAssertions.sol";
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";
import { DeploySuperchain } from "scripts/deploy/DeploySuperchain.s.sol";
import { DeployImplementations } from "scripts/deploy/DeployImplementations.s.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { StandardConstants } from "scripts/deploy/StandardConstants.sol";

// Libraries
import { Types } from "scripts/libraries/Types.sol";
import { Duration } from "src/libraries/bridge/LibUDT.sol";
import { GameType, Claim, GameTypes, Proposal, Hash } from "src/libraries/bridge/Types.sol";

// Interfaces
import { IProxy } from "interfaces/universal/IProxy.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IMIPS64 } from "interfaces/cannon/IMIPS64.sol";
import { IPreimageOracle } from "interfaces/cannon/IPreimageOracle.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { IETHLockbox } from "interfaces/L1/IETHLockbox.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";
import { IL1StandardBridge } from "interfaces/L1/IL1StandardBridge.sol";
import { IL1ERC721Bridge } from "interfaces/L1/IL1ERC721Bridge.sol";
import { IOptimismMintableERC20Factory } from "interfaces/universal/IOptimismMintableERC20Factory.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";

/// @title Deploy
/// @notice Script used to deploy a bedrock system. The entire system is deployed within the `run` function.
///         To add a new contract to the system, add a public function that deploys that individual contract.
///         Then add a call to that function inside of `run`. Be sure to call the `save` function after each
///         deployment so that hardhat-deploy style artifacts can be generated using a call to `sync()`.
///         This contract must not have constructor logic because it is set into state using `etch`.
contract Deploy is Deployer {
    using stdJson for string;

    ////////////////////////////////////////////////////////////////
    //                        Modifiers                           //
    ////////////////////////////////////////////////////////////////

    /// @notice Modifier that wraps a function in broadcasting.
    modifier broadcast() {
        vm.startBroadcast(msg.sender);
        _;
        vm.stopBroadcast();
    }

    /// @notice Modifier that will only allow a function to be called on devnet.
    modifier onlyDevnet() {
        uint256 chainid = block.chainid;
        if (chainid == Chains.LocalDevnet || chainid == Chains.GethDevnet) {
            _;
        }
    }

    /// @notice Modifier that wraps a function with statediff recording.
    ///         The returned AccountAccess[] array is then written to
    ///         the `snapshots/state-diff/<name>.json` output file.
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

    ////////////////////////////////////////////////////////////////
    //                        Accessors                           //
    ////////////////////////////////////////////////////////////////

    /// @notice The create2 salt used for deployment of the contract implementations.
    ///         Using this helps to reduce config across networks as the implementation
    ///         addresses will be the same across networks when deployed with create2.
    function _implSalt() internal view returns (bytes32) {
        return keccak256(bytes(Config.implSalt()));
    }

    /// @notice Returns the proxy addresses, not reverting if any are unset.
    function _proxies() internal view returns (Types.ContractSet memory proxies_) {
        proxies_ = Types.ContractSet({
            L1CrossDomainMessenger: artifacts.getAddress("L1CrossDomainMessengerProxy"),
            L1StandardBridge: artifacts.getAddress("L1StandardBridgeProxy"),
            L2OutputOracle: artifacts.getAddress("L2OutputOracleProxy"),
            DisputeGameFactory: artifacts.getAddress("DisputeGameFactoryProxy"),
            DelayedWETH: artifacts.getAddress("DelayedWETHProxy"),
            PermissionedDelayedWETH: artifacts.getAddress("PermissionedDelayedWETHProxy"),
            AnchorStateRegistry: artifacts.getAddress("AnchorStateRegistryProxy"),
            OptimismMintableERC20Factory: artifacts.getAddress("OptimismMintableERC20FactoryProxy"),
            OptimismPortal: artifacts.getAddress("OptimismPortalProxy"),
            ETHLockbox: artifacts.getAddress("ETHLockboxProxy"),
            SystemConfig: artifacts.getAddress("SystemConfigProxy"),
            L1ERC721Bridge: artifacts.getAddress("L1ERC721BridgeProxy"),
            SuperchainConfig: artifacts.getAddress("SuperchainConfigProxy")
        });
    }

    ////////////////////////////////////////////////////////////////
    //                    SetUp and Run                           //
    ////////////////////////////////////////////////////////////////

    /// @notice Deploy all of the L1 contracts necessary for a full Superchain with a single Op Chain.
    function run() public {
        console.log("Deploying a fresh OP Stack including SuperchainConfig");
        _run({ _needsSuperchain: true });
    }

    /// @notice Deploy a new OP Chain using an existing SuperchainConfig
    /// @param _superchainConfigProxy Address of the existing SuperchainConfig proxy
    function runWithSuperchain(address payable _superchainConfigProxy) public {
        require(_superchainConfigProxy != address(0), "Deploy: must specify address for superchain config proxy");

        vm.chainId(cfg.l1ChainID());

        console.log("Deploying a fresh OP Stack with existing SuperchainConfig");

        IProxy scProxy = IProxy(_superchainConfigProxy);
        artifacts.save("SuperchainConfigImpl", scProxy.implementation());
        artifacts.save("SuperchainConfigProxy", _superchainConfigProxy);

        _run({ _needsSuperchain: false });
    }

    /// @notice Deploy all L1 contracts and write the state diff to a file.
    ///         Used to generate kontrol tests.
    function runWithStateDiff() public stateDiff {
        _run({ _needsSuperchain: true });
    }

    /// @notice Internal function containing the deploy logic.
    function _run(bool _needsSuperchain) internal virtual {
        console.log("start of L1 Deploy!");

        // Set up the Superchain if needed.
        if (_needsSuperchain) {
            deploySuperchain();
        }

        deployImplementations({ _isInterop: cfg.useInterop() });

        // Deploy Current OPChain Contracts
        deployOpChain();

        // Set the respected game type according to the deploy config
        vm.startPrank(ISuperchainConfig(artifacts.mustGetAddress("SuperchainConfigProxy")).guardian());
        IAnchorStateRegistry(artifacts.mustGetAddress("AnchorStateRegistryProxy"))
            .setRespectedGameType(GameType.wrap(uint32(cfg.respectedGameType())));
        vm.stopPrank();

        console.log("set up op chain!");
    }

    ////////////////////////////////////////////////////////////////
    //           High Level Deployment Functions                  //
    ////////////////////////////////////////////////////////////////

    /// @notice Deploy a full system with a new SuperchainConfig
    ///         The Superchain system has 1 singleton contract which lies outside of an OP Chain:
    ///         1. The SuperchainConfig contract
    function deploySuperchain() public {
        console.log("Setting up Superchain");
        DeploySuperchain ds = new DeploySuperchain();

        // Run the deployment script.
        DeploySuperchain.Output memory dso = ds.run(
            DeploySuperchain.Input({
                guardian: cfg.superchainConfigGuardian(),
                incidentResponder: cfg.superchainConfigIncidentResponder(),
                // TODO: when DeployAuthSystem is done, finalSystemOwner should be replaced with the Foundation Upgrades
                // Safe
                superchainProxyAdminOwner: cfg.finalSystemOwner(),
                paused: false
            })
        );

        // Store the artifacts
        artifacts.save("SuperchainProxyAdmin", address(dso.superchainProxyAdmin));
        artifacts.save("SuperchainConfigProxy", address(dso.superchainConfigProxy));
        artifacts.save("SuperchainConfigImpl", address(dso.superchainConfigImpl));

        // First run assertions for the SuperchainConfig proxy contract.
        Types.ContractSet memory contracts = _proxies();
        ChainAssertions.checkSuperchainConfig({ _contracts: contracts, _cfg: cfg, _isProxy: true });

        // Finally replace the SuperchainConfig proxy with the implementation address and run assertions on it.
        contracts.SuperchainConfig = artifacts.mustGetAddress("SuperchainConfigImpl");
        ChainAssertions.checkSuperchainConfig({ _contracts: contracts, _cfg: cfg, _isProxy: false });
    }

    /// @notice Deploy all of the implementations
    /// @param _isInterop Whether to use interop
    function deployImplementations(bool _isInterop) public {
        // TODO _isInterop is no longer being used in DeployImplementations, this might no longer be necessary
        require(_isInterop == cfg.useInterop(), "Deploy: Interop setting mismatch.");

        console.log("Deploying implementations");

        DeployImplementations di = new DeployImplementations();

        ISuperchainConfig superchainConfigProxy = ISuperchainConfig(artifacts.mustGetAddress("SuperchainConfigProxy"));
        IProxyAdmin superchainProxyAdmin = IProxyAdmin(EIP1967Helper.getAdmin(address(superchainConfigProxy)));

        DeployImplementations.Output memory dio = di.run(
            DeployImplementations.Input({
                withdrawalDelaySeconds: cfg.faultGameWithdrawalDelay(),
                minProposalSizeBytes: cfg.preimageOracleMinProposalSize(),
                challengePeriodSeconds: cfg.preimageOracleChallengePeriod(),
                proofMaturityDelaySeconds: cfg.proofMaturityDelaySeconds(),
                disputeGameFinalityDelaySeconds: cfg.disputeGameFinalityDelaySeconds(),
                mipsVersion: StandardConstants.MIPS_VERSION,
                devFeatureBitmap: cfg.devFeatureBitmap(),
                faultGameV2MaxGameDepth: cfg.faultGameV2MaxGameDepth(),
                faultGameV2SplitDepth: cfg.faultGameV2SplitDepth(),
                faultGameV2ClockExtension: cfg.faultGameV2ClockExtension(),
                faultGameV2MaxClockDuration: cfg.faultGameV2MaxClockDuration(),
                teeImageHash: cfg.teeImageHash(),
                multiproofConfigHash: cfg.multiproofConfigHash(),
                multiproofGameType: cfg.multiproofGameType(),
                nitroEnclaveVerifier: cfg.nitroEnclaveVerifier(),
                l2ChainID: cfg.l2ChainID(),
                multiproofBlockInterval: cfg.multiproofBlockInterval(),
                multiproofIntermediateBlockInterval: cfg.multiproofIntermediateBlockInterval(),
                superchainConfigProxy: superchainConfigProxy,
                superchainProxyAdmin: superchainProxyAdmin,
                l1ProxyAdminOwner: superchainProxyAdmin.owner(),
                challenger: cfg.l2OutputOracleChallenger(),
                guardian: cfg.superchainConfigGuardian(),
                incidentResponder: cfg.superchainConfigIncidentResponder(),
                sp1Verifier: ISP1Verifier(cfg.sp1Verifier())
            })
        );

        // Save the implementation addresses which are needed outside of this function or script.
        // When called in a fork test, this will overwrite the existing implementations.
        artifacts.save("SuperchainConfigImpl", address(dio.superchainConfigImpl));
        artifacts.save("SystemConfigImpl", address(dio.systemConfigImpl));
        artifacts.save("L1CrossDomainMessengerImpl", address(dio.l1CrossDomainMessengerImpl));
        artifacts.save("L1ERC721BridgeImpl", address(dio.l1ERC721BridgeImpl));
        artifacts.save("L1StandardBridgeImpl", address(dio.l1StandardBridgeImpl));
        artifacts.save("OptimismMintableERC20FactoryImpl", address(dio.optimismMintableERC20FactoryImpl));
        artifacts.save("OptimismPortalImpl", address(dio.optimismPortalImpl));
        artifacts.save("ETHLockboxImpl", address(dio.ethLockboxImpl));
        artifacts.save("DisputeGameFactoryImpl", address(dio.disputeGameFactoryImpl));
        artifacts.save("AnchorStateRegistryImpl", address(dio.anchorStateRegistryImpl));
        artifacts.save("DelayedWETHImpl", address(dio.delayedWETHImpl));
        artifacts.save("MipsSingleton", address(dio.mipsSingleton));
        artifacts.save("FaultDisputeGame", address(dio.faultDisputeGameV2Impl));
        artifacts.save("PreimageOracle", address(dio.preimageOracleSingleton));
        artifacts.save("PermissionedDisputeGame", address(dio.permissionedDisputeGameV2Impl));
        artifacts.save("AggregateVerifier", address(dio.aggregateVerifierImpl));
        artifacts.save("TEEProverRegistry", address(dio.teeProverRegistryImpl));

        // Get a contract set from the implementation addresses which were just deployed.
        Types.ContractSet memory impls = ChainAssertions.dioToContractSet(dio);

        ChainAssertions.checkL1CrossDomainMessenger(IL1CrossDomainMessenger(impls.L1CrossDomainMessenger), vm, false);
        ChainAssertions.checkL1StandardBridgeImpl(IL1StandardBridge(payable(impls.L1StandardBridge)));
        ChainAssertions.checkL1ERC721BridgeImpl(IL1ERC721Bridge(impls.L1ERC721Bridge));
        ChainAssertions.checkOptimismPortal2({
            _contracts: impls,
            _superchainConfig: superchainConfigProxy,
            _opChainProxyAdminOwner: cfg.finalSystemOwner(),
            _isProxy: false
        });
        ChainAssertions.checkETHLockboxImpl(
            IETHLockbox(impls.ETHLockbox), IOptimismPortal2(payable(impls.OptimismPortal))
        );
        ChainAssertions.checkOptimismMintableERC20FactoryImpl(
            IOptimismMintableERC20Factory(impls.OptimismMintableERC20Factory)
        );
        ChainAssertions.checkDisputeGameFactory(
            IDisputeGameFactory(impls.DisputeGameFactory), address(0), address(0), false
        );
        ChainAssertions.checkDelayedWETHImpl(IDelayedWETH(payable(impls.DelayedWETH)), cfg.faultGameWithdrawalDelay());
        ChainAssertions.checkMIPS({
            _mips: IMIPS64(address(dio.mipsSingleton)), _oracle: IPreimageOracle(address(dio.preimageOracleSingleton))
        });
        ChainAssertions.checkSystemConfigImpls(impls);
        ChainAssertions.checkAnchorStateRegistryProxy(IAnchorStateRegistry(impls.AnchorStateRegistry), false);
    }

    /// @notice Deploy all of the OP Chain specific contracts
    function deployOpChain() public {
        console.log("Deploying OP Chain");

        Types.DeployInput memory deployInput = getDeployInput();
        DeploySuperchain.Input memory superchainInput;
        DeployImplementations.Input memory implementationsInput;
        SystemDeploy.DeployOutput memory systemOutput = new SystemDeploy()
            .deploy(
                SystemDeploy.DeployInput({
                    deploySuperchain: false,
                    deployImplementations: false,
                    saveArtifacts: false,
                    superchainInput: superchainInput,
                    superchainConfigProxy: ISuperchainConfig(artifacts.mustGetAddress("SuperchainConfigProxy")),
                    implementationsInput: implementationsInput,
                    implementations: getImplementations(),
                    opChainInput: deployInput
                })
            );
        Types.DeployOutput memory deployOutput = systemOutput.opChain;

        // Store code in the Final system owner address so that it can be used for prank delegatecalls
        // Store "fe" opcode so that accidental calls to this address revert
        vm.etch(cfg.finalSystemOwner(), hex"fe");

        // Save all deploy outputs, in the order they are declared in the DeployOutput struct.
        artifacts.save("ProxyAdmin", address(deployOutput.opChainProxyAdmin));
        artifacts.save("AddressManager", address(deployOutput.addressManager));
        artifacts.save("L1ERC721BridgeProxy", address(deployOutput.l1ERC721BridgeProxy));
        artifacts.save("SystemConfigProxy", address(deployOutput.systemConfigProxy));
        artifacts.save("OptimismMintableERC20FactoryProxy", address(deployOutput.optimismMintableERC20FactoryProxy));
        artifacts.save("L1StandardBridgeProxy", address(deployOutput.l1StandardBridgeProxy));
        artifacts.save("L1CrossDomainMessengerProxy", address(deployOutput.l1CrossDomainMessengerProxy));
        artifacts.save("ETHLockboxProxy", address(deployOutput.ethLockboxProxy));

        // Fault Proof contracts
        artifacts.save("DisputeGameFactoryProxy", address(deployOutput.disputeGameFactoryProxy));
        artifacts.save("PermissionedDelayedWETHProxy", address(deployOutput.delayedWETHPermissionedGameProxy));
        artifacts.save("DelayedWETHProxy", address(deployOutput.delayedWETHPermissionlessGameProxy));
        artifacts.save("AnchorStateRegistryProxy", address(deployOutput.anchorStateRegistryProxy));
        artifacts.save("OptimismPortalProxy", address(deployOutput.optimismPortalProxy));
        artifacts.save("OptimismPortal2Proxy", address(deployOutput.optimismPortalProxy));
    }

    ////////////////////////////////////////////////////////////////
    //                Proxy Deployment Functions                  //
    ////////////////////////////////////////////////////////////////

    /// @notice Deploys an ERC1967Proxy contract with a specified owner.
    /// @param _name The name of the proxy contract to be deployed.
    /// @param _proxyOwner The address of the owner of the proxy contract.
    /// @return addr_ The address of the deployed proxy contract.
    function deployERC1967ProxyWithOwner(
        string memory _name,
        address _proxyOwner
    )
        public
        broadcast
        returns (address addr_)
    {
        IProxy proxy = IProxy(
            DeployUtils.create2AndSave({
                _save: artifacts,
                _salt: keccak256(abi.encode(_implSalt(), _name)),
                _name: "src/universal/Proxy.sol:Proxy",
                _nick: _name,
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IProxy.__constructor__, (_proxyOwner)))
            })
        );
        require(EIP1967Helper.getAdmin(address(proxy)) == _proxyOwner, "Deploy: EIP1967Proxy admin not set");
        addr_ = address(proxy);
    }

    /// @notice Get the latest implementation set saved by deployImplementations.
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
            mipsImpl: artifacts.mustGetAddress("MipsSingleton"),
            faultDisputeGameV2Impl: artifacts.mustGetAddress("FaultDisputeGame"),
            permissionedDisputeGameV2Impl: artifacts.mustGetAddress("PermissionedDisputeGame")
        });
    }

    /// @notice Get the DeployInput struct to use for testing
    function getDeployInput() public view returns (Types.DeployInput memory) {
        string memory saltMixer = "salt mixer";
        return Types.DeployInput({
            roles: Types.Roles({
                opChainProxyAdminOwner: cfg.finalSystemOwner(),
                systemConfigOwner: cfg.finalSystemOwner(),
                batcher: cfg.batchSenderAddress(),
                unsafeBlockSigner: cfg.p2pSequencerAddress(),
                proposer: cfg.l2OutputOracleProposer(),
                challenger: cfg.l2OutputOracleChallenger()
            }),
            basefeeScalar: cfg.basefeeScalar(),
            blobBasefeeScalar: cfg.blobbasefeeScalar(),
            l2ChainId: cfg.l2ChainID(),
            startingAnchorRoot: abi.encode(
                Proposal({
                    root: Hash.wrap(cfg.faultGameGenesisOutputRoot()), l2SequenceNumber: cfg.faultGameGenesisBlock()
                })
            ),
            saltMixer: saltMixer,
            gasLimit: uint64(cfg.l2GenesisBlockGasLimit()),
            disputeGameType: GameTypes.PERMISSIONED_CANNON,
            disputeAbsolutePrestate: Claim.wrap(bytes32(cfg.faultGameAbsolutePrestate())),
            disputeMaxGameDepth: cfg.faultGameMaxDepth(),
            disputeSplitDepth: cfg.faultGameSplitDepth(),
            disputeClockExtension: Duration.wrap(uint64(cfg.faultGameClockExtension())),
            disputeMaxClockDuration: Duration.wrap(uint64(cfg.faultGameMaxClockDuration()))
        });
    }
}
