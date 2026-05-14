// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Types } from "scripts/libraries/Types.sol";

import { Constants } from "src/libraries/Constants.sol";
import { Features } from "src/libraries/Features.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { LibGameArgs } from "src/libraries/bridge/LibGameArgs.sol";
import { Claim, Duration, GameType, GameTypes, Hash } from "src/libraries/bridge/Types.sol";

import { IPreimageOracle } from "interfaces/cannon/IPreimageOracle.sol";
import { IMIPS64 } from "interfaces/cannon/IMIPS64.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IFaultDisputeGameV2 } from "interfaces/L1/proofs/v2/IFaultDisputeGameV2.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { IL1ERC721Bridge } from "interfaces/L1/IL1ERC721Bridge.sol";
import { IL1StandardBridge } from "interfaces/L1/IL1StandardBridge.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { IResourceMetering } from "interfaces/L1/IResourceMetering.sol";
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { IETHLockbox } from "interfaces/L1/IETHLockbox.sol";
import { IOptimismMintableERC20Factory } from "interfaces/universal/IOptimismMintableERC20Factory.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { ISemver } from "interfaces/universal/ISemver.sol";

abstract contract SystemDeployAssertions is Test {
    struct ExpectedSystemDeployState {
        ISystemConfig systemConfig;
        ISuperchainConfig superchainConfig;
        Types.Implementations implementations;
        IETHLockbox ethLockbox;
        address proxyAdminOwner;
        address challenger;
        address proposer;
        uint256 l2ChainId;
        Claim permissionedCannonPrestate;
        Claim cannonPrestate;
        Claim cannonKonaPrestate;
        bool expectCannon;
        bool expectCannonKona;
        uint256 withdrawalDelaySeconds;
        uint256 minProposalSizeBytes;
        uint256 challengePeriodSeconds;
        uint256 mipsVersion;
        uint256 disputeMaxGameDepth;
        uint256 disputeSplitDepth;
        Duration disputeClockExtension;
        Duration disputeMaxClockDuration;
    }

    function assertValidStandardSystem(ExpectedSystemDeployState memory _expected) internal view {
        IProxyAdmin proxyAdmin = _expected.systemConfig.proxyAdmin();

        _assertSuperchainConfig(_expected);
        _assertProxyAdmin(_expected, proxyAdmin);
        _assertSystemConfig(_expected, proxyAdmin);
        _assertBridgeAndPortalWiring(_expected, proxyAdmin);
        _assertDisputeGameFactory(_expected, proxyAdmin);
        _assertGames(_expected, proxyAdmin);
        _assertETHLockbox(_expected, proxyAdmin);
    }

    function _assertSuperchainConfig(ExpectedSystemDeployState memory _expected) private view {
        assertFalse(_expected.superchainConfig.paused(address(0)), "SPRCFG-10");
    }

    function _assertProxyAdmin(ExpectedSystemDeployState memory _expected, IProxyAdmin _proxyAdmin) private view {
        assertEq(_proxyAdmin.owner(), _expected.proxyAdminOwner, "PROXYA-10");
    }

    function _assertSystemConfig(ExpectedSystemDeployState memory _expected, IProxyAdmin _proxyAdmin) private view {
        ISystemConfig sysCfg = _expected.systemConfig;
        assertEq(_version(address(sysCfg)), _version(_expected.implementations.systemConfigImpl), "SYSCON-10");
        assertLe(sysCfg.gasLimit(), uint64(500_000_000), "SYSCON-20");
        assertNotEq(sysCfg.scalar(), 0, "SYSCON-30");
        assertEq(
            _proxyAdmin.getProxyImplementation(address(sysCfg)), _expected.implementations.systemConfigImpl, "SYSCON-40"
        );

        IResourceMetering.ResourceConfig memory outputConfig = sysCfg.resourceConfig();
        IResourceMetering.ResourceConfig memory expectedConfig = Constants.DEFAULT_RESOURCE_CONFIG();
        assertEq(outputConfig.maxResourceLimit, expectedConfig.maxResourceLimit, "SYSCON-50");
        assertEq(outputConfig.elasticityMultiplier, expectedConfig.elasticityMultiplier, "SYSCON-60");
        assertEq(outputConfig.baseFeeMaxChangeDenominator, expectedConfig.baseFeeMaxChangeDenominator, "SYSCON-70");
        assertEq(outputConfig.systemTxMaxGas, expectedConfig.systemTxMaxGas, "SYSCON-80");
        assertEq(outputConfig.minimumBaseFee, expectedConfig.minimumBaseFee, "SYSCON-90");
        assertEq(outputConfig.maximumBaseFee, expectedConfig.maximumBaseFee, "SYSCON-100");
        assertEq(sysCfg.operatorFeeScalar(), 0, "SYSCON-110");
        assertEq(sysCfg.operatorFeeConstant(), 0, "SYSCON-120");
        assertEq(address(sysCfg.superchainConfig()), address(_expected.superchainConfig), "SYSCON-130");
        assertEq(sysCfg.batchInbox(), Types.chainIdToBatchInboxAddress(_expected.l2ChainId), "SYSCON-140");
        assertEq(sysCfg.l2ChainId(), _expected.l2ChainId, "SYSCON-150");
    }

    function _assertBridgeAndPortalWiring(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin
    )
        private
        view
    {
        ISystemConfig sysCfg = _expected.systemConfig;
        IOptimismPortal2 portal = IOptimismPortal2(payable(sysCfg.optimismPortal()));
        IDisputeGameFactory dgf = IDisputeGameFactory(sysCfg.disputeGameFactory());
        IL1CrossDomainMessenger messenger = IL1CrossDomainMessenger(sysCfg.l1CrossDomainMessenger());
        IL1StandardBridge standardBridge = IL1StandardBridge(payable(sysCfg.l1StandardBridge()));
        IOptimismMintableERC20Factory erc20Factory =
            IOptimismMintableERC20Factory(sysCfg.optimismMintableERC20Factory());
        IL1ERC721Bridge erc721Bridge = IL1ERC721Bridge(sysCfg.l1ERC721Bridge());

        assertEq(
            _version(address(messenger)), _version(_expected.implementations.l1CrossDomainMessengerImpl), "L1xDM-10"
        );
        assertEq(
            _proxyAdmin.getProxyImplementation(address(messenger)),
            _expected.implementations.l1CrossDomainMessengerImpl,
            "L1xDM-20"
        );
        assertEq(address(messenger.OTHER_MESSENGER()), Predeploys.L2_CROSS_DOMAIN_MESSENGER, "L1xDM-30");
        assertEq(address(messenger.otherMessenger()), Predeploys.L2_CROSS_DOMAIN_MESSENGER, "L1xDM-40");
        assertEq(address(messenger.PORTAL()), address(portal), "L1xDM-50");
        assertEq(address(messenger.portal()), address(portal), "L1xDM-60");
        assertEq(address(messenger.systemConfig()), address(sysCfg), "L1xDM-70");
        assertEq(address(_proxyAdminFor(address(messenger))), address(_proxyAdmin), "L1xDM-80");

        assertEq(_version(address(standardBridge)), _version(_expected.implementations.l1StandardBridgeImpl), "L1SB-10");
        assertEq(
            _proxyAdmin.getProxyImplementation(address(standardBridge)),
            _expected.implementations.l1StandardBridgeImpl,
            "L1SB-20"
        );
        assertEq(address(standardBridge.MESSENGER()), address(messenger), "L1SB-30");
        assertEq(address(standardBridge.messenger()), address(messenger), "L1SB-40");
        assertEq(address(standardBridge.OTHER_BRIDGE()), Predeploys.L2_STANDARD_BRIDGE, "L1SB-50");
        assertEq(address(standardBridge.otherBridge()), Predeploys.L2_STANDARD_BRIDGE, "L1SB-60");
        assertEq(address(standardBridge.systemConfig()), address(sysCfg), "L1SB-70");
        assertEq(address(_proxyAdminFor(address(standardBridge))), address(_proxyAdmin), "L1SB-80");

        assertEq(
            _version(address(erc20Factory)),
            _version(_expected.implementations.optimismMintableERC20FactoryImpl),
            "MERC20F-10"
        );
        assertEq(
            _proxyAdmin.getProxyImplementation(address(erc20Factory)),
            _expected.implementations.optimismMintableERC20FactoryImpl,
            "MERC20F-20"
        );
        assertEq(erc20Factory.BRIDGE(), address(standardBridge), "MERC20F-30");
        assertEq(erc20Factory.bridge(), address(standardBridge), "MERC20F-40");

        assertEq(_version(address(erc721Bridge)), _version(_expected.implementations.l1ERC721BridgeImpl), "L721B-10");
        assertEq(
            _proxyAdmin.getProxyImplementation(address(erc721Bridge)),
            _expected.implementations.l1ERC721BridgeImpl,
            "L721B-20"
        );
        assertEq(address(erc721Bridge.OTHER_BRIDGE()), Predeploys.L2_ERC721_BRIDGE, "L721B-30");
        assertEq(address(erc721Bridge.otherBridge()), Predeploys.L2_ERC721_BRIDGE, "L721B-40");
        assertEq(address(erc721Bridge.MESSENGER()), address(messenger), "L721B-50");
        assertEq(address(erc721Bridge.messenger()), address(messenger), "L721B-60");
        assertEq(address(erc721Bridge.systemConfig()), address(sysCfg), "L721B-70");
        assertEq(address(_proxyAdminFor(address(erc721Bridge))), address(_proxyAdmin), "L721B-80");

        assertEq(_version(address(portal)), _version(_expected.implementations.optimismPortalImpl), "PORTAL-10");
        assertEq(
            _proxyAdmin.getProxyImplementation(address(portal)),
            _expected.implementations.optimismPortalImpl,
            "PORTAL-20"
        );
        assertEq(address(portal.disputeGameFactory()), address(dgf), "PORTAL-30");
        assertEq(address(portal.systemConfig()), address(sysCfg), "PORTAL-40");
        LibGameArgs.GameArgs memory permissionedArgs = LibGameArgs.decode(dgf.gameArgs(GameTypes.PERMISSIONED_CANNON));
        assertEq(address(portal.anchorStateRegistry()), permissionedArgs.anchorStateRegistry, "PORTAL-50");
        assertEq(portal.l2Sender(), Constants.DEFAULT_L2_SENDER, "PORTAL-80");
        assertEq(address(_proxyAdminFor(address(portal))), address(_proxyAdmin), "PORTAL-90");
    }

    function _assertDisputeGameFactory(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin
    )
        private
        view
    {
        IDisputeGameFactory factory = IDisputeGameFactory(_expected.systemConfig.disputeGameFactory());
        assertEq(_version(address(factory)), _version(_expected.implementations.disputeGameFactoryImpl), "DF-10");
        assertEq(
            _proxyAdmin.getProxyImplementation(address(factory)),
            _expected.implementations.disputeGameFactoryImpl,
            "DF-20"
        );
        assertEq(factory.owner(), _expected.proxyAdminOwner, "DF-30");
        assertEq(address(_proxyAdminFor(address(factory))), address(_proxyAdmin), "DF-40");
    }

    function _assertGames(ExpectedSystemDeployState memory _expected, IProxyAdmin _proxyAdmin) private view {
        _assertGame(_expected, _proxyAdmin, GameTypes.PERMISSIONED_CANNON, true, "PDDG");
        _assertGame(_expected, _proxyAdmin, GameTypes.CANNON, _expected.expectCannon, "PLDG");
        _assertGame(_expected, _proxyAdmin, GameTypes.CANNON_KONA, _expected.expectCannonKona, "CKDG");
    }

    function _assertGame(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin,
        GameType _gameType,
        bool _expectSet,
        string memory _prefix
    )
        private
        view
    {
        IDisputeGameFactory factory = IDisputeGameFactory(_expected.systemConfig.disputeGameFactory());
        IDisputeGame game = factory.gameImpls(_gameType);
        if (!_expectSet) {
            assertEq(address(game), address(0), string.concat(_prefix, "-10"));
            assertEq(factory.gameArgs(_gameType).length, 0, string.concat(_prefix, "-GARGS-10"));
            return;
        }

        assertNotEq(address(game), address(0), string.concat(_prefix, "-10"));
        bool permissioned = _gameType.raw() == GameTypes.PERMISSIONED_CANNON.raw();
        address expectedImpl = permissioned
            ? _expected.implementations.permissionedDisputeGameV2Impl
            : _expected.implementations.faultDisputeGameV2Impl;
        assertEq(address(game), expectedImpl, string.concat(_prefix, "-15"));
        assertEq(_version(address(game)), _version(expectedImpl), string.concat(_prefix, "-20"));

        bytes memory rawArgs = factory.gameArgs(_gameType);
        if (permissioned) {
            assertTrue(LibGameArgs.isValidPermissionedArgs(rawArgs), string.concat(_prefix, "-GARGS-10"));
        } else {
            assertTrue(LibGameArgs.isValidPermissionlessArgs(rawArgs), string.concat(_prefix, "-GARGS-10"));
        }
        _assertGameArgsAndContracts({
            _expected: _expected,
            _proxyAdmin: _proxyAdmin,
            _factory: factory,
            _faultGame: IFaultDisputeGameV2(address(game)),
            _gameType: _gameType,
            _args: LibGameArgs.decode(rawArgs),
            _permissioned: permissioned,
            _prefix: _prefix
        });
    }

    function _assertGameArgsAndContracts(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin,
        IDisputeGameFactory _factory,
        IFaultDisputeGameV2 _faultGame,
        GameType _gameType,
        LibGameArgs.GameArgs memory _args,
        bool _permissioned,
        string memory _prefix
    )
        private
        view
    {
        Claim expectedPrestate = _expectedPrestate(_expected, _gameType);
        assertEq(_args.absolutePrestate, expectedPrestate.raw(), string.concat(_prefix, "-40"));
        assertEq(_args.vm, _expected.implementations.mipsImpl, string.concat(_prefix, "-VM-10"));
        assertEq(_args.l2ChainId, _expected.l2ChainId, string.concat(_prefix, "-60"));
        _assertGameImmutableArgs(_expected, _faultGame, _prefix);

        (Hash anchorRoot,) = IAnchorStateRegistry(_args.anchorStateRegistry).getAnchorRoot();
        assertNotEq(anchorRoot.raw(), bytes32(0), string.concat(_prefix, "-120"));

        if (_permissioned) {
            assertEq(_args.challenger, _expected.challenger, "PDDG-130");
            assertEq(_args.proposer, _expected.proposer, "PDDG-140");
        } else {
            assertEq(_args.challenger, address(0), string.concat(_prefix, "-130"));
            assertEq(_args.proposer, address(0), string.concat(_prefix, "-140"));
        }

        _assertDelayedWETH(_expected, _proxyAdmin, IDelayedWETH(payable(_args.weth)), _prefix);
        _assertAnchorStateRegistry(
            _expected, _proxyAdmin, _factory, IAnchorStateRegistry(_args.anchorStateRegistry), _prefix
        );
        _assertMipsAndPreimageOracle(_expected, IMIPS64(_args.vm), _prefix);
    }

    function _assertGameImmutableArgs(
        ExpectedSystemDeployState memory _expected,
        IFaultDisputeGameV2 _faultGame,
        string memory _prefix
    )
        private
        view
    {
        assertEq(_faultGame.l2SequenceNumber(), 0, string.concat(_prefix, "-70"));
        assertEq(
            _faultGame.clockExtension().raw(), _expected.disputeClockExtension.raw(), string.concat(_prefix, "-80")
        );
        assertEq(_faultGame.splitDepth(), _expected.disputeSplitDepth, string.concat(_prefix, "-90"));
        assertEq(_faultGame.maxGameDepth(), _expected.disputeMaxGameDepth, string.concat(_prefix, "-100"));
        assertEq(
            _faultGame.maxClockDuration().raw(), _expected.disputeMaxClockDuration.raw(), string.concat(_prefix, "-110")
        );
    }

    function _assertDelayedWETH(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin,
        IDelayedWETH _weth,
        string memory _prefix
    )
        private
        view
    {
        string memory prefix = string.concat(_prefix, "-DWETH");
        assertEq(
            _version(address(_weth)), _version(_expected.implementations.delayedWETHImpl), string.concat(prefix, "-10")
        );
        assertEq(
            _proxyAdmin.getProxyImplementation(address(_weth)),
            _expected.implementations.delayedWETHImpl,
            string.concat(prefix, "-20")
        );
        assertEq(_weth.proxyAdminOwner(), _expected.proxyAdminOwner, string.concat(prefix, "-30"));
        assertEq(_weth.delay(), _expected.withdrawalDelaySeconds, string.concat(prefix, "-40"));
        assertEq(address(_weth.systemConfig()), address(_expected.systemConfig), string.concat(prefix, "-50"));
        assertEq(address(_proxyAdminFor(address(_weth))), address(_proxyAdmin), string.concat(prefix, "-60"));
    }

    function _assertAnchorStateRegistry(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin,
        IDisputeGameFactory _factory,
        IAnchorStateRegistry _asr,
        string memory _prefix
    )
        private
        view
    {
        string memory prefix = string.concat(_prefix, "-ANCHORP");
        assertEq(
            _version(address(_asr)),
            _version(_expected.implementations.anchorStateRegistryImpl),
            string.concat(prefix, "-10")
        );
        assertEq(
            _proxyAdmin.getProxyImplementation(address(_asr)),
            _expected.implementations.anchorStateRegistryImpl,
            string.concat(prefix, "-20")
        );
        assertEq(address(_asr.disputeGameFactory()), address(_factory), string.concat(prefix, "-30"));
        assertEq(address(_asr.systemConfig()), address(_expected.systemConfig), string.concat(prefix, "-40"));
        assertEq(address(_proxyAdminFor(address(_asr))), address(_proxyAdmin), string.concat(prefix, "-50"));
        assertGt(_asr.retirementTimestamp(), 0, string.concat(prefix, "-60"));
    }

    function _assertMipsAndPreimageOracle(
        ExpectedSystemDeployState memory _expected,
        IMIPS64 _mips,
        string memory _prefix
    )
        private
        view
    {
        string memory vmPrefix = string.concat(_prefix, "-VM");
        assertEq(address(_mips), _expected.implementations.mipsImpl, string.concat(vmPrefix, "-10"));
        assertEq(_version(address(_mips)), _version(_expected.implementations.mipsImpl), string.concat(vmPrefix, "-20"));
        assertEq(_mips.stateVersion(), _expected.mipsVersion, string.concat(vmPrefix, "-30"));

        IPreimageOracle oracle = _mips.oracle();
        string memory oraclePrefix = string.concat(_prefix, "-PIMGO");
        assertGt(bytes(_version(address(oracle))).length, 0, string.concat(oraclePrefix, "-10"));
        assertEq(oracle.challengePeriod(), _expected.challengePeriodSeconds, string.concat(oraclePrefix, "-20"));
        assertEq(oracle.minProposalSize(), _expected.minProposalSizeBytes, string.concat(oraclePrefix, "-30"));
    }

    function _assertETHLockbox(ExpectedSystemDeployState memory _expected, IProxyAdmin _proxyAdmin) private view {
        IOptimismPortal2 portal = IOptimismPortal2(payable(_expected.systemConfig.optimismPortal()));
        IETHLockbox lockbox = _expected.ethLockbox;

        assertNotEq(address(lockbox), address(0), "LOCKBOX-05");
        assertEq(_version(address(lockbox)), _version(_expected.implementations.ethLockboxImpl), "LOCKBOX-10");
        assertEq(
            _proxyAdmin.getProxyImplementation(address(lockbox)), _expected.implementations.ethLockboxImpl, "LOCKBOX-20"
        );
        assertEq(address(_proxyAdminFor(address(lockbox))), address(_proxyAdmin), "LOCKBOX-30");
        assertEq(address(lockbox.systemConfig()), address(_expected.systemConfig), "LOCKBOX-40");
        assertTrue(lockbox.authorizedPortals(portal), "LOCKBOX-50");

        if (_expected.systemConfig.isFeatureEnabled(Features.ETH_LOCKBOX)) {
            assertEq(address(portal.ethLockbox()), address(lockbox), "LOCKBOX-60");
        }
    }

    function _expectedPrestate(
        ExpectedSystemDeployState memory _expected,
        GameType _gameType
    )
        private
        pure
        returns (Claim)
    {
        if (_gameType.raw() == GameTypes.CANNON_KONA.raw()) {
            return _expected.cannonKonaPrestate;
        }
        if (_gameType.raw() == GameTypes.PERMISSIONED_CANNON.raw()) {
            return _expected.permissionedCannonPrestate;
        }
        return _expected.cannonPrestate;
    }

    function _proxyAdminFor(address _contract) private view returns (IProxyAdmin) {
        return IProxyAdminOwnedBase(_contract).proxyAdmin();
    }

    function _version(address _contract) private view returns (string memory) {
        return ISemver(_contract).version();
    }
}
