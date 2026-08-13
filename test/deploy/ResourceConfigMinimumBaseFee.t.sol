// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Types } from "scripts/libraries/Types.sol";
import { Constants } from "src/libraries/Constants.sol";
import { Hash, Proposal } from "src/libraries/bridge/Types.sol";

import { IL1StandardBridge } from "interfaces/L1/IL1StandardBridge.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";
import { IResourceMetering } from "interfaces/L1/IResourceMetering.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";

/// @title ResourceConfigMinimumBaseFee_Test
/// @notice Tests deploy-time configuration of the deposit resource minimum base fee.
contract ResourceConfigMinimumBaseFee_Test is Test {
    SystemDeploy internal systemDeploy;

    address internal owner = address(this);
    address internal guardian = makeAddr("guardian");
    address internal incidentResponder = makeAddr("incidentResponder");
    address internal batcher = makeAddr("batcher");
    address internal unsafeBlockSigner = makeAddr("unsafeBlockSigner");
    address internal proposer = makeAddr("proposer");
    address internal challenger = makeAddr("challenger");

    uint256 internal l2ChainId = 901;
    uint32 internal constant L3_MINIMUM_BASE_FEE = 10_000_000;
    /// @dev The Fusaka EIP-7825 transaction gas cap. Chains may not enable it and future forks may change it.
    uint256 internal constant BASE_TX_GAS_CAP = 16_777_216;

    function setUp() public {
        systemDeploy = new SystemDeploy();
    }

    function test_deploy_defaultResourceConfigMinimumBaseFee_succeeds() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_deployInput(uint32(1 gwei)));

        IResourceMetering.ResourceConfig memory config = output.opChain.systemConfigProxy.resourceConfig();
        assertEq(config.minimumBaseFee, uint32(1 gwei));
        assertEq(config.maximumBaseFee, Constants.DEFAULT_RESOURCE_CONFIG().maximumBaseFee);

        (uint128 prevBaseFee,,) = IOptimismPortal2(payable(address(output.opChain.optimismPortalProxy))).params();
        assertEq(prevBaseFee, config.minimumBaseFee);
    }

    function test_deploy_overriddenResourceConfigMinimumBaseFee_succeeds() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_deployInput(L3_MINIMUM_BASE_FEE));

        IResourceMetering.ResourceConfig memory config = output.opChain.systemConfigProxy.resourceConfig();
        assertEq(config.minimumBaseFee, L3_MINIMUM_BASE_FEE);
        assertEq(config.maximumBaseFee, Constants.DEFAULT_RESOURCE_CONFIG().maximumBaseFee);

        (uint128 prevBaseFee,,) = IOptimismPortal2(payable(address(output.opChain.optimismPortalProxy))).params();
        assertEq(prevBaseFee, L3_MINIMUM_BASE_FEE);
    }

    function test_depositETH_lowParentBaseFee_belowTxGasCap_succeeds() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_deployInput(L3_MINIMUM_BASE_FEE));

        ISystemConfig systemConfig = output.opChain.systemConfigProxy;
        IOptimismPortal2 optimismPortal = IOptimismPortal2(payable(address(output.opChain.optimismPortalProxy)));
        IL1StandardBridge l1StandardBridge = IL1StandardBridge(payable(systemConfig.l1StandardBridge()));

        vm.fee(L3_MINIMUM_BASE_FEE);

        uint256 gasUsed = _depositETHAndMeasureGas(l1StandardBridge);
        assertLt(gasUsed, BASE_TX_GAS_CAP);

        (uint128 prevBaseFee, uint64 prevBoughtGas,) = optimismPortal.params();
        assertEq(prevBaseFee, L3_MINIMUM_BASE_FEE);
        assertGt(prevBoughtGas, 0);
    }

    function test_depositETH_defaultMinimum_lowParentBaseFee_exceedsFusakaTxGasCap_succeeds() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_deployInput(uint32(1 gwei)));
        IL1StandardBridge l1StandardBridge =
            IL1StandardBridge(payable(output.opChain.systemConfigProxy.l1StandardBridge()));

        vm.fee(L3_MINIMUM_BASE_FEE);

        uint256 gasUsed = _depositETHAndMeasureGas(l1StandardBridge);
        assertGe(gasUsed, BASE_TX_GAS_CAP);
    }

    function _depositETHAndMeasureGas(IL1StandardBridge _l1StandardBridge) internal returns (uint256 gasUsed_) {
        uint256 depositorKey = 0xBEEF;
        address depositor = vm.addr(depositorKey);
        vm.deal(depositor, 10 ether);

        vm.startBroadcast(depositorKey);
        uint256 gasBefore = gasleft();
        _l1StandardBridge.depositETH{ value: 1 ether }(200_000, hex"");
        gasUsed_ = gasBefore - gasleft();
        vm.stopBroadcast();
    }

    function _deployInput(uint32 _resourceConfigMinimumBaseFee)
        internal
        view
        returns (SystemDeploy.DeployInput memory input_)
    {
        input_.saveArtifacts = false;
        input_.superchainInput = SystemDeploy.SuperchainInput({
            guardian: guardian, incidentResponder: incidentResponder, superchainProxyAdminOwner: owner
        });
        input_.implementationsInput = SystemDeploy.ImplementationInput({
            withdrawalDelaySeconds: 100,
            proofMaturityDelaySeconds: 400,
            disputeGameFinalityDelaySeconds: 500,
            teeImageHash: bytes32(uint256(1)),
            zkRangeHash: bytes32(uint256(2)),
            zkAggregationHash: bytes32(uint256(3)),
            multiproofConfigHash: bytes32(uint256(4)),
            multiproofGameType: 621,
            nitroEnclaveVerifier: address(0),
            multiproofBlockInterval: 100,
            multiproofIntermediateBlockInterval: 10,
            multiproofMaxUpgradeId: 12,
            sp1Verifier: ISP1Verifier(address(0)),
            teeProposer: proposer,
            teeChallenger: challenger,
            devTeeSigner: address(0),
            guardian: guardian,
            incidentResponder: incidentResponder,
            slowFinalizationDelay: 5 days,
            fastFinalizationDelay: 1 days
        });
        input_.opChainInput = Types.DeployInput({
            roles: Types.Roles({
                opChainProxyAdminOwner: owner,
                systemConfigOwner: owner,
                batcher: batcher,
                unsafeBlockSigner: unsafeBlockSigner,
                incidentResponder: incidentResponder
            }),
            basefeeScalar: 100,
            blobBasefeeScalar: 200,
            l2ChainId: l2ChainId,
            startingAnchorRoot: Proposal({ root: Hash.wrap(bytes32(uint256(1))), l2SequenceNumber: 0 }),
            saltMixer: "resource-config-minimum-base-fee-test",
            gasLimit: 60_000_000,
            resourceConfigMinimumBaseFee: _resourceConfigMinimumBaseFee
        });
    }
}
