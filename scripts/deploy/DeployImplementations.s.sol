// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Script } from "lib/forge-std/src/Script.sol";

// Libraries
import { Chains } from "scripts/libraries/Chains.sol";

// Interfaces
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IPreimageOracle } from "interfaces/cannon/IPreimageOracle.sol";
import { IMIPS64 } from "interfaces/cannon/IMIPS64.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IFaultDisputeGameV2 } from "interfaces/L1/proofs/v2/IFaultDisputeGameV2.sol";
import { IPermissionedDisputeGameV2 } from "interfaces/L1/proofs/v2/IPermissionedDisputeGameV2.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { Duration, GameTypes, Hash } from "src/libraries/bridge/Types.sol";
import { IOptimismPortal2 as IOptimismPortal } from "interfaces/L1/IOptimismPortal2.sol";
import { IETHLockbox } from "interfaces/L1/IETHLockbox.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { IResourceMetering } from "interfaces/L1/IResourceMetering.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { IL1ERC721Bridge } from "interfaces/L1/IL1ERC721Bridge.sol";
import { IL1StandardBridge } from "interfaces/L1/IL1StandardBridge.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { IOptimismMintableERC20Factory } from "interfaces/universal/IOptimismMintableERC20Factory.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";
import { Solarray } from "scripts/libraries/Solarray.sol";

contract DeployImplementations is Script {
    struct Input {
        uint256 withdrawalDelaySeconds;
        uint256 minProposalSizeBytes;
        uint256 challengePeriodSeconds;
        uint256 proofMaturityDelaySeconds;
        uint256 disputeGameFinalityDelaySeconds;
        uint256 mipsVersion;
        bytes32 devFeatureBitmap;
        // Super and V2 Dispute Game parameters
        uint256 faultGameV2MaxGameDepth;
        uint256 faultGameV2SplitDepth;
        uint256 faultGameV2ClockExtension;
        uint256 faultGameV2MaxClockDuration;
        // Multiproof parameters
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
        // Outputs from the shared superchain deployment.
        ISuperchainConfig superchainConfigProxy;
        IProxyAdmin superchainProxyAdmin;
        address l1ProxyAdminOwner;
        address challenger;
        address guardian;
        address incidentResponder;
    }

    struct Output {
        IDelayedWETH delayedWETHImpl;
        IOptimismPortal optimismPortalImpl;
        IETHLockbox ethLockboxImpl;
        IPreimageOracle preimageOracleSingleton;
        IMIPS64 mipsSingleton;
        ISystemConfig systemConfigImpl;
        IL1CrossDomainMessenger l1CrossDomainMessengerImpl;
        IL1ERC721Bridge l1ERC721BridgeImpl;
        IL1StandardBridge l1StandardBridgeImpl;
        IOptimismMintableERC20Factory optimismMintableERC20FactoryImpl;
        IDisputeGameFactory disputeGameFactoryImpl;
        IAnchorStateRegistry anchorStateRegistryImpl;
        ISuperchainConfig superchainConfigImpl;
        IFaultDisputeGameV2 faultDisputeGameV2Impl;
        IPermissionedDisputeGameV2 permissionedDisputeGameV2Impl;
    }

    bytes32 internal _salt = DeployUtils.DEFAULT_SALT;

    // -------- Core Deployment Methods --------

    function runWithBytes(bytes memory _input) public returns (bytes memory) {
        Input memory input = abi.decode(_input, (Input));
        Output memory output = run(input);
        return abi.encode(output);
    }

    function run(Input memory _input) public returns (Output memory output_) {
        assertValidInput(_input);

        // Deploy the implementations.
        deploySuperchainConfigImpl(_input, output_);
        deploySystemConfigImpl(output_);
        deployL1CrossDomainMessengerImpl(output_);
        deployL1ERC721BridgeImpl(output_);
        deployL1StandardBridgeImpl(output_);
        deployOptimismMintableERC20FactoryImpl(output_);
        deployOptimismPortalImpl(_input, output_);
        deployETHLockboxImpl(output_);
        deployDelayedWETHImpl(_input, output_);
        deployPreimageOracleSingleton(_input, output_);
        deployMipsSingleton(_input, output_);
        deployDisputeGameFactoryImpl(output_);
        deployAnchorStateRegistryImpl(_input, output_);
        deployFaultDisputeGameV2Impl(_input, output_);
        deployPermissionedDisputeGameV2Impl(_input, output_);

        assertValidOutput(_input, output_);
    }

    // -------- Deployment Steps --------

    // --- Core Contracts ---

    function deploySuperchainConfigImpl(Input memory _input, Output memory _output) private {
        ISuperchainConfig impl = ISuperchainConfig(
            DeployUtils.createDeterministic({
                _name: "SuperchainConfig",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(ISuperchainConfig.__constructor__, (_input.guardian, _input.incidentResponder))
                ),
                _salt: _salt
            })
        );
        vm.label(address(impl), "SuperchainConfigImpl");
        _output.superchainConfigImpl = impl;
    }

    function deploySystemConfigImpl(Output memory _output) private {
        ISystemConfig impl = ISystemConfig(
            DeployUtils.createDeterministic({
                _name: "SystemConfig",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(ISystemConfig.__constructor__, ())),
                _salt: _salt
            })
        );
        vm.label(address(impl), "SystemConfigImpl");
        _output.systemConfigImpl = impl;
    }

    function deployL1CrossDomainMessengerImpl(Output memory _output) private {
        IL1CrossDomainMessenger impl = IL1CrossDomainMessenger(
            DeployUtils.createDeterministic({
                _name: "L1CrossDomainMessenger",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IL1CrossDomainMessenger.__constructor__, ())),
                _salt: _salt
            })
        );
        vm.label(address(impl), "L1CrossDomainMessengerImpl");
        _output.l1CrossDomainMessengerImpl = impl;
    }

    function deployL1ERC721BridgeImpl(Output memory _output) private {
        IL1ERC721Bridge impl = IL1ERC721Bridge(
            DeployUtils.createDeterministic({
                _name: "L1ERC721Bridge",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IL1ERC721Bridge.__constructor__, ())),
                _salt: _salt
            })
        );
        vm.label(address(impl), "L1ERC721BridgeImpl");
        _output.l1ERC721BridgeImpl = impl;
    }

    function deployL1StandardBridgeImpl(Output memory _output) private {
        IL1StandardBridge impl = IL1StandardBridge(
            DeployUtils.createDeterministic({
                _name: "L1StandardBridge",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IL1StandardBridge.__constructor__, ())),
                _salt: _salt
            })
        );
        vm.label(address(impl), "L1StandardBridgeImpl");
        _output.l1StandardBridgeImpl = impl;
    }

    function deployOptimismMintableERC20FactoryImpl(Output memory _output) private {
        IOptimismMintableERC20Factory impl = IOptimismMintableERC20Factory(
            DeployUtils.createDeterministic({
                _name: "OptimismMintableERC20Factory",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IOptimismMintableERC20Factory.__constructor__, ())),
                _salt: _salt
            })
        );
        vm.label(address(impl), "OptimismMintableERC20FactoryImpl");
        _output.optimismMintableERC20FactoryImpl = impl;
    }

    function deployETHLockboxImpl(Output memory _output) private {
        IETHLockbox impl = IETHLockbox(
            DeployUtils.createDeterministic({
                _name: "ETHLockbox",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IETHLockbox.__constructor__, ())),
                _salt: _salt
            })
        );
        vm.label(address(impl), "ETHLockboxImpl");
        _output.ethLockboxImpl = impl;
    }

    // --- Fault Proofs Contracts ---

    // The fault proofs contracts are configured as follows:
    // | Contract                | Proxied | Deployment                        | MCP Ready  |
    // |-------------------------|---------|-----------------------------------|------------|
    // | DisputeGameFactory      | Yes     | Bespoke                           | Yes        |
    // | AnchorStateRegistry     | Yes     | Bespoke                           | Yes         |
    // | FaultDisputeGame        | No      | Bespoke                           | No         | Not yet supported by
    // scripts | PermissionedDisputeGame | No      | Bespoke                           | No         |
    // | DelayedWETH             | Yes     | Two bespoke (one per DisputeGame) | Yes *️⃣     |
    // | PreimageOracle          | No      | Shared                            | N/A        |
    // | MIPS                    | No      | Shared                            | N/A        |
    // | OptimismPortal2         | Yes     | Shared                            | Yes *️⃣     |
    //
    // - *️⃣ These contracts have immutable values which are intended to be constant for all contracts within a
    //   Superchain, and are therefore MCP ready for any chain using the Standard Configuration.
    //
    // This script only deploys the shared contracts. The bespoke contracts are deployed by
    // `SystemDeploy.s.sol`. When the shared contracts are proxied, the contracts deployed here are
    // "implementations", and when shared contracts are not proxied, they are "singletons". So
    // here we deploy:
    //
    //   - DisputeGameFactory (implementation)
    //   - AnchorStateRegistry (implementation)
    //   - OptimismPortal2 (implementation)
    //   - DelayedWETH (implementation)
    //   - PreimageOracle (singleton)
    //   - MIPS (singleton)
    //
    // For contracts which are not MCP ready neither the Proxy nor the implementation can be shared, therefore they
    // are deployed by `DeployOpChain.s.sol`.
    // These are:
    // - FaultDisputeGame (not proxied)
    // - PermissionedDisputeGame (not proxied)

    function deployOptimismPortalImpl(Input memory _input, Output memory _output) private {
        uint256 proofMaturityDelaySeconds = _input.proofMaturityDelaySeconds;
        IOptimismPortal impl = IOptimismPortal(
            DeployUtils.createDeterministic({
                _name: "OptimismPortal2",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IOptimismPortal.__constructor__, (proofMaturityDelaySeconds))
                ),
                _salt: _salt
            })
        );
        vm.label(address(impl), "OptimismPortalImpl");
        _output.optimismPortalImpl = impl;
    }

    function deployDelayedWETHImpl(Input memory _input, Output memory _output) private {
        uint256 withdrawalDelaySeconds = _input.withdrawalDelaySeconds;
        IDelayedWETH impl = IDelayedWETH(
            DeployUtils.createDeterministic({
                _name: "DelayedWETH",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IDelayedWETH.__constructor__, (withdrawalDelaySeconds))
                ),
                _salt: _salt
            })
        );
        vm.label(address(impl), "DelayedWETHImpl");
        _output.delayedWETHImpl = impl;
    }

    function deployPreimageOracleSingleton(Input memory _input, Output memory _output) private {
        uint256 minProposalSizeBytes = _input.minProposalSizeBytes;
        uint256 challengePeriodSeconds = _input.challengePeriodSeconds;
        IPreimageOracle singleton = IPreimageOracle(
            DeployUtils.createDeterministic({
                _name: "PreimageOracle",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IPreimageOracle.__constructor__, (minProposalSizeBytes, challengePeriodSeconds))
                ),
                _salt: _salt
            })
        );
        vm.label(address(singleton), "PreimageOracleSingleton");
        _output.preimageOracleSingleton = singleton;
    }

    function deployMipsSingleton(Input memory _input, Output memory _output) private {
        uint256 mipsVersion = _input.mipsVersion;
        IPreimageOracle preimageOracle = IPreimageOracle(address(_output.preimageOracleSingleton));

        // We want to ensure that upgrade 13 uses MIPS64 on production networks.
        if (mipsVersion < 2) {
            if (block.chainid == Chains.Mainnet || block.chainid == Chains.Sepolia) {
                revert("DeployImplementations: Only Mips64 should be deployed on Mainnet or Sepolia");
            }
        }

        IMIPS64 singleton = IMIPS64(
            DeployUtils.createDeterministic({
                _name: "MIPS64",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IMIPS64.__constructor__, (preimageOracle, mipsVersion))
                ),
                _salt: DeployUtils.DEFAULT_SALT
            })
        );
        vm.label(address(singleton), "MIPSSingleton");
        _output.mipsSingleton = singleton;
    }

    function deployDisputeGameFactoryImpl(Output memory _output) private {
        IDisputeGameFactory impl = IDisputeGameFactory(
            DeployUtils.createDeterministic({
                _name: "DisputeGameFactory",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IDisputeGameFactory.__constructor__, ())),
                _salt: _salt
            })
        );
        vm.label(address(impl), "DisputeGameFactoryImpl");
        _output.disputeGameFactoryImpl = impl;
    }

    function deployAnchorStateRegistryImpl(Input memory _input, Output memory _output) private {
        uint256 disputeGameFinalityDelaySeconds = _input.disputeGameFinalityDelaySeconds;
        IAnchorStateRegistry impl = IAnchorStateRegistry(
            DeployUtils.createDeterministic({
                _name: "AnchorStateRegistry",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IAnchorStateRegistry.__constructor__, (disputeGameFinalityDelaySeconds))
                ),
                _salt: _salt
            })
        );
        vm.label(address(impl), "AnchorStateRegistryImpl");
        _output.anchorStateRegistryImpl = impl;
    }

    function deployFaultDisputeGameV2Impl(Input memory _input, Output memory _output) private {
        IFaultDisputeGameV2.GameConstructorParams memory params;
        params.maxGameDepth = _input.faultGameV2MaxGameDepth;
        params.splitDepth = _input.faultGameV2SplitDepth;
        params.clockExtension = Duration.wrap(uint64(_input.faultGameV2ClockExtension));
        params.maxClockDuration = Duration.wrap(uint64(_input.faultGameV2MaxClockDuration));

        IFaultDisputeGameV2 impl = IFaultDisputeGameV2(
            DeployUtils.createDeterministic({
                _name: "FaultDisputeGameV2",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IFaultDisputeGameV2.__constructor__, (params))),
                _salt: _salt
            })
        );
        vm.label(address(impl), "FaultDisputeGameV2Impl");
        _output.faultDisputeGameV2Impl = impl;
    }

    function deployPermissionedDisputeGameV2Impl(Input memory _input, Output memory _output) private {
        IFaultDisputeGameV2.GameConstructorParams memory params;
        params.maxGameDepth = _input.faultGameV2MaxGameDepth;
        params.splitDepth = _input.faultGameV2SplitDepth;
        params.clockExtension = Duration.wrap(uint64(_input.faultGameV2ClockExtension));
        params.maxClockDuration = Duration.wrap(uint64(_input.faultGameV2MaxClockDuration));

        IPermissionedDisputeGameV2 impl = IPermissionedDisputeGameV2(
            DeployUtils.createDeterministic({
                _name: "PermissionedDisputeGameV2",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IPermissionedDisputeGameV2.__constructor__, (params))
                ),
                _salt: _salt
            })
        );
        vm.label(address(impl), "PermissionedDisputeGameV2Impl");
        _output.permissionedDisputeGameV2Impl = impl;
    }

    function assertValidInput(Input memory _input) private pure {
        // Validate V2 game depth parameters are sensible
        require(
            _input.faultGameV2MaxGameDepth > 0 && _input.faultGameV2MaxGameDepth <= 125,
            "DeployImplementations: faultGameV2MaxGameDepth out of valid range (1-125)"
        );
        // V2 contract requires splitDepth >= 2 and splitDepth + 1 < maxGameDepth
        require(
            _input.faultGameV2SplitDepth >= 2 && _input.faultGameV2SplitDepth + 1 < _input.faultGameV2MaxGameDepth,
            "DeployImplementations: faultGameV2SplitDepth must be >= 2 and splitDepth + 1 < maxGameDepth"
        );

        // Validate V2 clock parameters fit in uint64 before deployment
        require(
            _input.faultGameV2ClockExtension <= type(uint64).max,
            "DeployImplementations: faultGameV2ClockExtension too large for uint64"
        );
        require(
            _input.faultGameV2MaxClockDuration <= type(uint64).max,
            "DeployImplementations: faultGameV2MaxClockDuration too large for uint64"
        );
        require(
            _input.faultGameV2MaxClockDuration >= _input.faultGameV2ClockExtension,
            "DeployImplementations: maxClockDuration must be >= clockExtension"
        );
        require(_input.faultGameV2ClockExtension > 0, "DeployImplementations: faultGameV2ClockExtension must be > 0");
        require(_input.withdrawalDelaySeconds != 0, "DeployImplementations: withdrawalDelaySeconds not set");
        require(_input.minProposalSizeBytes != 0, "DeployImplementations: minProposalSizeBytes not set");
        require(_input.challengePeriodSeconds != 0, "DeployImplementations: challengePeriodSeconds not set");
        require(
            _input.challengePeriodSeconds <= type(uint64).max, "DeployImplementations: challengePeriodSeconds too large"
        );
        require(_input.proofMaturityDelaySeconds != 0, "DeployImplementations: proofMaturityDelaySeconds not set");
        require(
            _input.disputeGameFinalityDelaySeconds != 0,
            "DeployImplementations: disputeGameFinalityDelaySeconds not set"
        );
        require(_input.mipsVersion != 0, "DeployImplementations: mipsVersion not set");
        if (_multiproofEnabled(_input)) {
            require(_input.teeImageHash != bytes32(0), "DeployImplementations: teeImageHash not set");
            require(_input.zkRangeHash != bytes32(0), "DeployImplementations: zkRangeHash not set");
            require(_input.zkAggregationHash != bytes32(0), "DeployImplementations: zkAggregationHash not set");
            require(_input.multiproofGameType != 0, "DeployImplementations: multiproofGameType not set");
            require(_input.nitroEnclaveVerifier != address(0), "DeployImplementations: nitroEnclaveVerifier not set");
            require(address(_input.sp1Verifier) != address(0), "DeployImplementations: sp1Verifier not set");
            require(_input.l2ChainID != 0, "DeployImplementations: l2ChainID not set");
            require(_input.multiproofBlockInterval != 0, "DeployImplementations: multiproofBlockInterval not set");
            require(
                _input.multiproofIntermediateBlockInterval != 0,
                "DeployImplementations: multiproofIntermediateBlockInterval not set"
            );
            require(
                _input.multiproofBlockInterval % _input.multiproofIntermediateBlockInterval == 0,
                "DeployImplementations: invalid multiproof block intervals"
            );
            require(_input.teeProposer != address(0), "DeployImplementations: teeProposer not set");
            require(_input.teeChallenger != address(0), "DeployImplementations: teeChallenger not set");
        }
        require(
            address(_input.superchainConfigProxy) != address(0), "DeployImplementations: superchainConfigProxy not set"
        );
        require(
            address(_input.superchainProxyAdmin) != address(0), "DeployImplementations: superchainProxyAdmin not set"
        );
        require(address(_input.l1ProxyAdminOwner) != address(0), "DeployImplementations: L1ProxyAdminOwner not set");
    }

    function _multiproofEnabled(Input memory _input) private pure returns (bool) {
        return _input.multiproofConfigHash != bytes32(0);
    }

    function assertValidOutput(Input memory _input, Output memory _output) private view {
        // With many addresses, we'd get a stack too deep error if we tried to do this inline as a
        // single call to `Solarray.addresses`. So we split it into two calls.
        address[] memory addrs1 = Solarray.addresses(
            address(_output.optimismPortalImpl),
            address(_output.delayedWETHImpl),
            address(_output.preimageOracleSingleton),
            address(_output.mipsSingleton),
            address(_output.superchainConfigImpl)
        );

        address[] memory addrs2 = Solarray.addresses(
            address(_output.systemConfigImpl),
            address(_output.l1CrossDomainMessengerImpl),
            address(_output.l1ERC721BridgeImpl),
            address(_output.l1StandardBridgeImpl),
            address(_output.optimismMintableERC20FactoryImpl),
            address(_output.disputeGameFactoryImpl),
            address(_output.anchorStateRegistryImpl),
            address(_output.ethLockboxImpl),
            address(_output.faultDisputeGameV2Impl),
            address(_output.permissionedDisputeGameV2Impl)
        );

        DeployUtils.assertValidContractAddresses(Solarray.extend(addrs1, addrs2));

        checkDelayedWETHImpl(_output.delayedWETHImpl, _input.withdrawalDelaySeconds);
        checkDisputeGameFactoryImpl(_output.disputeGameFactoryImpl);
        checkL1CrossDomainMessengerImpl(_output.l1CrossDomainMessengerImpl);
        checkL1ERC721BridgeImpl(_output.l1ERC721BridgeImpl);
        checkL1StandardBridgeImpl(_output.l1StandardBridgeImpl);
        checkMIPS(_output.mipsSingleton, _output.preimageOracleSingleton);
        checkOptimismMintableERC20FactoryImpl(_output.optimismMintableERC20FactoryImpl);
        checkOptimismPortal2Impl(_output.optimismPortalImpl);
        checkETHLockboxImpl(_output.ethLockboxImpl, _output.optimismPortalImpl);
        checkSystemConfigImpl(_output.systemConfigImpl);
        checkAnchorStateRegistryImpl(_output.anchorStateRegistryImpl);
    }

    function checkProxyAdminCallFails(address _contract, bytes4 _errorSelector) private view returns (bool) {
        (bool success, bytes memory data) =
            address(_contract).staticcall(abi.encodeCall(IProxyAdminOwnedBase.proxyAdmin, ()));
        return !success && data.length == 4 && bytes4(data) == _errorSelector;
    }

    function checkSystemConfigImpl(ISystemConfig _config) private view {
        DeployUtils.assertInitialized({ _contractAddress: address(_config), _isProxy: false, _slot: 0, _offset: 0 });

        IResourceMetering.ResourceConfig memory resourceConfig = _config.resourceConfig();

        require(_config.owner() == address(0), "CHECK-SCFG-220");
        require(_config.overhead() == 0, "CHECK-SCFG-230");
        require(_config.scalar() == 0, "CHECK-SCFG-240");
        require(_config.basefeeScalar() == 0, "CHECK-SCFG-250");
        require(_config.blobbasefeeScalar() == 0, "CHECK-SCFG-260");
        require(_config.batcherHash() == bytes32(0), "CHECK-SCFG-270");
        require(_config.gasLimit() == 0, "CHECK-SCFG-280");
        require(_config.unsafeBlockSigner() == address(0), "CHECK-SCFG-290");
        require(resourceConfig.maxResourceLimit == 0, "CHECK-SCFG-300");
        require(resourceConfig.elasticityMultiplier == 0, "CHECK-SCFG-310");
        require(resourceConfig.baseFeeMaxChangeDenominator == 0, "CHECK-SCFG-320");
        require(resourceConfig.systemTxMaxGas == 0, "CHECK-SCFG-330");
        require(resourceConfig.minimumBaseFee == 0, "CHECK-SCFG-340");
        require(resourceConfig.maximumBaseFee == 0, "CHECK-SCFG-350");
        require(_config.startBlock() == type(uint256).max, "CHECK-SCFG-360");
        require(_config.batchInbox() == address(0), "CHECK-SCFG-370");
        require(_config.l1CrossDomainMessenger() == address(0), "CHECK-SCFG-380");
        require(_config.l1ERC721Bridge() == address(0), "CHECK-SCFG-390");
        require(_config.l1StandardBridge() == address(0), "CHECK-SCFG-400");
        require(_config.optimismPortal() == address(0), "CHECK-SCFG-420");
        require(_config.optimismMintableERC20Factory() == address(0), "CHECK-SCFG-430");
    }

    function checkL1CrossDomainMessengerImpl(IL1CrossDomainMessenger _messenger) private view {
        require(address(_messenger) != address(0), "CHECK-L1XDM-10");
        DeployUtils.assertInitialized({ _contractAddress: address(_messenger), _isProxy: false, _slot: 0, _offset: 20 });

        require(address(_messenger.OTHER_MESSENGER()) == address(0), "CHECK-L1XDM-80");
        require(address(_messenger.otherMessenger()) == address(0), "CHECK-L1XDM-90");
        require(address(_messenger.PORTAL()) == address(0), "CHECK-L1XDM-100");
        require(address(_messenger.portal()) == address(0), "CHECK-L1XDM-110");
        require(address(_messenger.systemConfig()) == address(0), "CHECK-L1XDM-120");
        require(
            checkProxyAdminCallFails(
                address(_messenger), IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotResolvedDelegateProxy.selector
            ),
            "CHECK-L1XDM-130"
        );
    }

    function checkL1StandardBridgeImpl(IL1StandardBridge _bridge) private view {
        require(address(_bridge) != address(0), "CHECK-L1SB-10");
        DeployUtils.assertInitialized({ _contractAddress: address(_bridge), _isProxy: false, _slot: 0, _offset: 0 });

        require(address(_bridge.MESSENGER()) == address(0), "CHECK-L1SB-70");
        require(address(_bridge.messenger()) == address(0), "CHECK-L1SB-80");
        require(address(_bridge.OTHER_BRIDGE()) == address(0), "CHECK-L1SB-90");
        require(address(_bridge.otherBridge()) == address(0), "CHECK-L1SB-100");
        require(address(_bridge.systemConfig()) == address(0), "CHECK-L1SB-110");
    }

    function checkDisputeGameFactoryImpl(IDisputeGameFactory _factory) private view {
        require(address(_factory) != address(0), "CHECK-DG-10");
        DeployUtils.assertInitialized({ _contractAddress: address(_factory), _isProxy: false, _slot: 0, _offset: 0 });

        require(address(_factory.gameImpls(GameTypes.PERMISSIONED_CANNON)) == address(0), "CHECK-DG-20");
        require(_factory.owner() == address(0), "CHECK-DG-30");
    }

    function checkMIPS(IMIPS64 _mips, IPreimageOracle _oracle) private view {
        require(address(_mips) != address(0), "CHECK-MIPS-10");
        require(_mips.oracle() == _oracle, "CHECK-MIPS-20");
    }

    function checkDelayedWETHImpl(IDelayedWETH _weth, uint256 _faultGameWithdrawalDelay) private view {
        require(address(_weth) != address(0), "CHECK-DWETH-10");
        DeployUtils.assertInitialized({ _contractAddress: address(_weth), _isProxy: false, _slot: 0, _offset: 0 });

        require(_weth.delay() == _faultGameWithdrawalDelay, "CHECK-DWETH-50");
    }

    function checkOptimismMintableERC20FactoryImpl(IOptimismMintableERC20Factory _factory) private view {
        DeployUtils.assertInitialized({ _contractAddress: address(_factory), _isProxy: false, _slot: 0, _offset: 0 });

        require(_factory.BRIDGE() == address(0), "CHECK-MERC20F-30");
        require(_factory.bridge() == address(0), "CHECK-MERC20F-40");
    }

    function checkL1ERC721BridgeImpl(IL1ERC721Bridge _bridge) private view {
        DeployUtils.assertInitialized({ _contractAddress: address(_bridge), _isProxy: false, _slot: 0, _offset: 0 });

        require(address(_bridge.OTHER_BRIDGE()) == address(0), "CHECK-L1ERC721B-60");
        require(address(_bridge.otherBridge()) == address(0), "CHECK-L1ERC721B-70");
        require(address(_bridge.MESSENGER()) == address(0), "CHECK-L1ERC721B-80");
        require(address(_bridge.messenger()) == address(0), "CHECK-L1ERC721B-90");
        require(address(_bridge.systemConfig()) == address(0), "CHECK-L1ERC721B-100");
        require(
            checkProxyAdminCallFails(
                address(_bridge), IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotResolvedDelegateProxy.selector
            ),
            "CHECK-L1XDM-130"
        );
    }

    function checkOptimismPortal2Impl(IOptimismPortal _portal) private view {
        require(address(_portal) != address(0), "CHECK-OP2-10");
        DeployUtils.assertInitialized({ _contractAddress: address(_portal), _isProxy: false, _slot: 0, _offset: 0 });

        require(address(_portal.anchorStateRegistry()) == address(0), "CHECK-OP2-80");
        require(address(_portal.systemConfig()) == address(0), "CHECK-OP2-90");
        require(_portal.l2Sender() == address(0), "CHECK-OP2-110");
        require(address(_portal.ethLockbox()) == address(0), "CHECK-OP2-120");
        require(vm.load(address(_portal), bytes32(uint256(61))) == bytes32(0), "CHECK-OP2-130");
    }

    function checkETHLockboxImpl(IETHLockbox _ethLockbox, IOptimismPortal _portal) private view {
        DeployUtils.assertInitialized({ _contractAddress: address(_ethLockbox), _isProxy: false, _slot: 0, _offset: 0 });

        require(address(_ethLockbox.systemConfig()) == address(0), "CHECK-ELB-50");
        require(_ethLockbox.authorizedPortals(_portal) == false, "CHECK-ELB-60");
    }

    function checkAnchorStateRegistryImpl(IAnchorStateRegistry _anchorStateRegistry) private view {
        DeployUtils.assertValidContractAddress(address(_anchorStateRegistry));
        DeployUtils.assertInitialized({
            _contractAddress: address(_anchorStateRegistry), _isProxy: false, _slot: 0, _offset: 0
        });

        (Hash actualRoot,) = _anchorStateRegistry.anchors(GameTypes.PERMISSIONED_CANNON);
        require(Hash.unwrap(actualRoot) == bytes32(0), "ANCHORP-40");
    }
}
