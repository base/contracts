// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test, console } from "lib/forge-std/src/Test.sol";

import { Types } from "scripts/libraries/Types.sol";

import { Constants } from "src/libraries/Constants.sol";
import { Features } from "src/libraries/Features.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { LibGameArgs } from "src/libraries/bridge/LibGameArgs.sol";
import { Claim, Duration, GameType, GameTypes, Hash } from "src/libraries/bridge/Types.sol";

import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IAggregateVerifier } from "interfaces/L1/proofs/IAggregateVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
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
        IAnchorStateRegistry anchorStateRegistry;
        ISuperchainConfig superchainConfig;
        Types.Implementations implementations;
        IETHLockbox ethLockbox;
        address proxyAdminOwner;
        uint256 l2ChainId;
        uint256 withdrawalDelaySeconds;
    }

    function assertValidStandardSystem(ExpectedSystemDeployState memory _expected) internal view {
        IProxyAdmin proxyAdmin = _expected.systemConfig.proxyAdmin();

        _assertSuperchainConfig(_expected);
        _assertProxyAdmin(_expected, proxyAdmin);
        _assertSystemConfig(_expected, proxyAdmin);
        _assertBridgeAndPortalWiring(_expected, proxyAdmin);
        _assertDisputeGameFactory(_expected, proxyAdmin);
        _assertGame(_expected, proxyAdmin, GameTypes.AGGREGATE_VERIFIER);
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
        IDisputeGame av = dgf.gameImpls(GameTypes.AGGREGATE_VERIFIER);
        assertEq(address(portal.anchorStateRegistry()), address(av.anchorStateRegistry()), "PORTAL-50");
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

    function _assertGame(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin,
        GameType _gameType
    )
        private
        view
    {
        IDisputeGameFactory factory = IDisputeGameFactory(_expected.systemConfig.disputeGameFactory());
        IDisputeGame game = factory.gameImpls(_gameType);

        assertNotEq(address(game), address(0), "AV-10");
        address expectedImpl = _expected.implementations.aggregateVerifierImpl;
        assertEq(address(game), expectedImpl, "AV-15");
        assertEq(_version(address(game)), _version(expectedImpl), "AV-20");

        _assertGameArgsAndContracts({
            _expected: _expected,
            _proxyAdmin: _proxyAdmin,
            _factory: factory,
            _faultGame: IAggregateVerifier(address(game))
        });
    }

    function _assertGameArgsAndContracts(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin,
        IDisputeGameFactory _factory,
        IAggregateVerifier _faultGame
    )
        private
        view
    {
        IAnchorStateRegistry asr = _faultGame.anchorStateRegistry();
        IDelayedWETH weth = _faultGame.DELAYED_WETH();
        _assertGameImmutableArgs(_expected, _faultGame);

        (Hash anchorRoot,) = asr.getAnchorRoot();
        assertNotEq(anchorRoot.raw(), bytes32(0), "AV-120");
        _assertDelayedWETH(_expected, _proxyAdmin, weth);
        _assertAnchorStateRegistry(
            _expected, _proxyAdmin, _factory, asr
        );
    }

    // TODO: adapt this to assert AggregateVerifier args
    // game type
    // asr
    // delayedWETH
    // tee verifier
    // zk verifier
    // tee image hash
    // zk hashes
    // config hash
    // l2 chain id
    // block interval
    // intermediate block interval
    function _assertGameImmutableArgs(
        ExpectedSystemDeployState memory _expected,
        IAggregateVerifier _faultGame
    )
        private
        view
    {
        assertEq(_faultGame.l2SequenceNumber(), 0, "AV-70");
        assertEq(address(_faultGame.anchorStateRegistry()), address(_expected.anchorStateRegistry), "AV-70");
    }

    function _assertDelayedWETH(
        ExpectedSystemDeployState memory _expected,
        IProxyAdmin _proxyAdmin,
        IDelayedWETH _weth
    )
        private
        view
    {
        string memory prefix = "AV-DWETH";
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
        IAnchorStateRegistry _asr
    )
        private
        view
    {
        string memory prefix = "AV-ANCHORP";
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

    function _proxyAdminFor(address _contract) private view returns (IProxyAdmin) {
        return IProxyAdminOwnedBase(_contract).proxyAdmin();
    }

    function _version(address _contract) private view returns (string memory) {
        return ISemver(_contract).version();
    }
}
