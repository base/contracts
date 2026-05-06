// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { DeployImplementations } from "scripts/deploy/DeployImplementations.s.sol";
import { DeploySuperchain } from "scripts/deploy/DeploySuperchain.s.sol";
import { StandardConstants } from "scripts/deploy/StandardConstants.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Types } from "scripts/libraries/Types.sol";

import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { LibGameArgs } from "src/libraries/bridge/LibGameArgs.sol";
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
        _assertUpgradedProxyImplementations(output);
    }

    function test_upgrade_existingCannonKonaFallsBackToCurrentPrestate_succeeds() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_defaultDeployInput());

        Claim currentCannonPrestate = Claim.wrap(bytes32(uint256(3)));
        Claim currentCannonKonaPrestate = Claim.wrap(bytes32(uint256(4)));
        output.opChain.disputeGameFactoryProxy
            .setImplementation(
                GameTypes.CANNON,
                IDisputeGame(output.implementationOutput.implementations.permissionedDisputeGameV2Impl),
                _permissionlessGameArgs(output, currentCannonPrestate)
            );
        output.opChain.disputeGameFactoryProxy
            .setImplementation(
                GameTypes.CANNON_KONA,
                IDisputeGame(output.implementationOutput.implementations.permissionedDisputeGameV2Impl),
                _permissionlessGameArgs(output, currentCannonKonaPrestate)
            );
        output.opChain.disputeGameFactoryProxy.setInitBond(GameTypes.CANNON, 1 ether);

        Types.OpChainConfig[] memory opChainConfigs = new Types.OpChainConfig[](1);
        opChainConfigs[0] = Types.OpChainConfig({
            systemConfigProxy: output.opChain.systemConfigProxy,
            cannonPrestate: Claim.wrap(bytes32(0)),
            cannonKonaPrestate: Claim.wrap(bytes32(0))
        });

        systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: output.superchain.superchainConfigProxy,
                implementations: output.implementationOutput.implementations,
                opChainConfigs: opChainConfigs
            })
        );

        assertEq(
            address(output.opChain.disputeGameFactoryProxy.gameImpls(GameTypes.CANNON)),
            output.implementationOutput.implementations.faultDisputeGameV2Impl,
            "cannon impl"
        );
        assertEq(
            address(output.opChain.disputeGameFactoryProxy.gameImpls(GameTypes.CANNON_KONA)),
            output.implementationOutput.implementations.faultDisputeGameV2Impl,
            "cannon kona impl"
        );

        LibGameArgs.GameArgs memory cannonArgs =
            LibGameArgs.decode(output.opChain.disputeGameFactoryProxy.gameArgs(GameTypes.CANNON));
        LibGameArgs.GameArgs memory cannonKonaArgs =
            LibGameArgs.decode(output.opChain.disputeGameFactoryProxy.gameArgs(GameTypes.CANNON_KONA));
        assertEq(cannonArgs.absolutePrestate, currentCannonPrestate.raw(), "cannon prestate");
        assertEq(cannonKonaArgs.absolutePrestate, currentCannonKonaPrestate.raw(), "cannon kona prestate");
        assertEq(cannonKonaArgs.weth, cannonArgs.weth, "shared weth");
        assertEq(cannonKonaArgs.anchorStateRegistry, cannonArgs.anchorStateRegistry, "shared asr");
        assertEq(
            output.opChain.disputeGameFactoryProxy.initBonds(GameTypes.CANNON_KONA),
            output.opChain.disputeGameFactoryProxy.initBonds(GameTypes.CANNON),
            "cannon kona bond"
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

    function _permissionlessGameArgs(
        SystemDeploy.DeployOutput memory _output,
        Claim _absolutePrestate
    )
        internal
        view
        returns (bytes memory)
    {
        return LibGameArgs.encode(
            LibGameArgs.GameArgs({
                absolutePrestate: _absolutePrestate.raw(),
                vm: _output.implementationOutput.implementations.mipsImpl,
                anchorStateRegistry: address(_output.opChain.anchorStateRegistryProxy),
                weth: address(_output.opChain.delayedWETHPermissionlessGameProxy),
                l2ChainId: l2ChainId,
                proposer: address(0),
                challenger: address(0)
            })
        );
    }

    function _assertUpgradedProxyImplementations(SystemDeploy.DeployOutput memory _output) internal view {
        IProxyAdmin superchainProxyAdmin = _output.superchain.superchainProxyAdmin;
        IProxyAdmin opChainProxyAdmin = _output.opChain.opChainProxyAdmin;
        Types.Implementations memory impls = _output.implementationOutput.implementations;

        assertEq(
            superchainProxyAdmin.getProxyImplementation(address(_output.superchain.superchainConfigProxy)),
            impls.superchainConfigImpl,
            "superchain config impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.systemConfigProxy)),
            impls.systemConfigImpl,
            "system config impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.optimismPortalProxy)),
            impls.optimismPortalImpl,
            "portal impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.anchorStateRegistryProxy)),
            impls.anchorStateRegistryImpl,
            "anchor state registry impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.optimismMintableERC20FactoryProxy)),
            impls.optimismMintableERC20FactoryImpl,
            "erc20 factory impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.disputeGameFactoryProxy)),
            impls.disputeGameFactoryImpl,
            "dispute game factory impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.l1CrossDomainMessengerProxy)),
            impls.l1CrossDomainMessengerImpl,
            "messenger impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.l1StandardBridgeProxy)),
            impls.l1StandardBridgeImpl,
            "standard bridge impl"
        );
        assertEq(
            opChainProxyAdmin.getProxyImplementation(address(_output.opChain.l1ERC721BridgeProxy)),
            impls.l1ERC721BridgeImpl,
            "erc721 bridge impl"
        );
    }
}
