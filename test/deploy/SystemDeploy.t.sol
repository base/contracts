// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { DeployImplementations } from "scripts/deploy/DeployImplementations.s.sol";
import { DeploySuperchain } from "scripts/deploy/DeploySuperchain.s.sol";
import { StandardConstants } from "scripts/deploy/StandardConstants.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Types } from "scripts/libraries/Types.sol";

import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { Claim, Duration, GameTypes, Hash, Proposal } from "src/libraries/bridge/Types.sol";

contract SystemDeploy_Test is Test {
    SystemDeploy internal systemDeploy;

    address internal owner = address(this);
    address internal guardian = makeAddr("guardian");
    address internal incidentResponder = makeAddr("incidentResponder");
    address internal batcher = makeAddr("batcher");
    address internal unsafeBlockSigner = makeAddr("unsafeBlockSigner");
    address internal proposer = makeAddr("proposer");
    address internal challenger = makeAddr("challenger");
    address internal sp1Verifier = makeAddr("sp1Verifier");

    uint256 internal l2ChainId = 901;
    Claim internal absolutePrestate = Claim.wrap(0x038512e02c4c3f7bdaec27d00edf55b7155e0905301e1a88083e4e0a6764d54c);

    function setUp() public {
        systemDeploy = new SystemDeploy();
    }

    function test_deploy_succeeds_withoutOPCMAddress() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_defaultDeployInput());

        assertNotEq(address(output.opChain.opChainProxyAdmin), address(0), "proxy admin");
        assertNotEq(address(output.opChain.systemConfigProxy), address(0), "system config");
        assertNotEq(address(output.opChain.optimismPortalProxy), address(0), "portal");
        assertNotEq(address(output.opChain.ethLockboxProxy), address(0), "lockbox");
        assertNotEq(address(output.opChain.delayedWETHPermissionlessGameProxy), address(0), "permissionless weth");

        assertEq(output.opChain.opChainProxyAdmin.owner(), owner, "op chain proxy admin owner");
        assertEq(output.opChain.systemConfigProxy.batchInbox(), Types.chainIdToBatchInboxAddress(l2ChainId), "inbox");
        assertEq(
            address(output.opChain.disputeGameFactoryProxy.gameImpls(GameTypes.PERMISSIONED_CANNON)),
            output.implementationOutput.implementations.permissionedDisputeGameV2Impl,
            "permissioned game impl"
        );
        assertEq(
            address(output.opChain.systemConfigProxy.superchainConfig()),
            address(output.superchain.superchainConfigProxy),
            "superchain config"
        );
    }

    function test_upgrade_succeeds_withoutOPCMDelegatecall() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_defaultDeployInput());

        Types.OpChainConfig[] memory opChainConfigs = new Types.OpChainConfig[](1);
        opChainConfigs[0] = Types.OpChainConfig({
            systemConfigProxy: output.opChain.systemConfigProxy,
            cannonPrestate: absolutePrestate,
            cannonKonaPrestate: Claim.wrap(bytes32(0))
        });

        SystemDeploy.UpgradeOutput memory upgradeOutput = systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: output.superchain.superchainConfigProxy,
                implementations: output.implementationOutput.implementations,
                opChainConfigs: opChainConfigs
            })
        );

        assertFalse(upgradeOutput.superchainConfigUpgraded, "superchain already current");
        assertEq(upgradeOutput.chainsUpgraded, 1, "chains upgraded");
        assertEq(
            address(output.opChain.disputeGameFactoryProxy.gameImpls(GameTypes.PERMISSIONED_CANNON)),
            output.implementationOutput.implementations.permissionedDisputeGameV2Impl,
            "permissioned game impl after upgrade"
        );
    }

    function _defaultDeployInput() internal view returns (SystemDeploy.DeployInput memory input_) {
        input_.deploySuperchain = true;
        input_.deployImplementations = true;
        input_.saveArtifacts = false;
        input_.superchainInput = DeploySuperchain.Input({
            guardian: guardian, incidentResponder: incidentResponder, superchainProxyAdminOwner: owner, paused: false
        });
        input_.implementationsInput = DeployImplementations.Input({
            withdrawalDelaySeconds: 100,
            minProposalSizeBytes: 200,
            challengePeriodSeconds: 300,
            proofMaturityDelaySeconds: 400,
            disputeGameFinalityDelaySeconds: 500,
            mipsVersion: StandardConstants.MIPS_VERSION,
            devFeatureBitmap: bytes32(0),
            faultGameV2MaxGameDepth: 73,
            faultGameV2SplitDepth: 30,
            faultGameV2ClockExtension: 10_800,
            faultGameV2MaxClockDuration: 302_400,
            teeImageHash: bytes32(uint256(1)),
            multiproofConfigHash: bytes32(0),
            multiproofGameType: 621,
            nitroEnclaveVerifier: address(0),
            l2ChainID: l2ChainId,
            multiproofBlockInterval: 100,
            multiproofIntermediateBlockInterval: 10,
            sp1Verifier: ISP1Verifier(sp1Verifier),
            superchainConfigProxy: ISuperchainConfig(address(0)),
            superchainProxyAdmin: IProxyAdmin(address(0)),
            l1ProxyAdminOwner: owner,
            challenger: challenger,
            guardian: guardian,
            incidentResponder: incidentResponder
        });
        input_.opChainInput = Types.DeployInput({
            roles: Types.Roles({
                opChainProxyAdminOwner: owner,
                systemConfigOwner: owner,
                batcher: batcher,
                unsafeBlockSigner: unsafeBlockSigner,
                proposer: proposer,
                challenger: challenger
            }),
            basefeeScalar: 100,
            blobBasefeeScalar: 200,
            l2ChainId: l2ChainId,
            startingAnchorRoot: abi.encode(Proposal({ root: Hash.wrap(bytes32(uint256(1))), l2SequenceNumber: 0 })),
            saltMixer: "system-deploy-test",
            gasLimit: 60_000_000,
            disputeGameType: GameTypes.PERMISSIONED_CANNON,
            disputeAbsolutePrestate: absolutePrestate,
            disputeMaxGameDepth: 73,
            disputeSplitDepth: 30,
            disputeClockExtension: Duration.wrap(10_800),
            disputeMaxClockDuration: Duration.wrap(302_400)
        });
    }
}
