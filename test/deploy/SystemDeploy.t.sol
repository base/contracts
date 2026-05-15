// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Artifacts } from "scripts/Artifacts.s.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Types } from "scripts/libraries/Types.sol";
import { SystemDeployAssertions } from "test/deploy/SystemDeployAssertions.sol";

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { IProxy } from "interfaces/universal/IProxy.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";
import { ZKVerifier } from "src/L1/proofs/zk/ZKVerifier.sol";
import { GameType, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

contract MockNitroEnclaveVerifier {
    address public proofSubmitter;

    function setProofSubmitter(address _proofSubmitter) external {
        proofSubmitter = _proofSubmitter;
    }
}

contract MockSP1Verifier {
    function verifyProof(bytes32, bytes calldata, bytes calldata) external pure { }
}

contract SystemDeploy_Test is Test, SystemDeployAssertions {
    Artifacts internal constant artifacts =
        Artifacts(address(uint160(uint256(keccak256(abi.encode("optimism.artifacts"))))));
    SystemDeploy internal systemDeploy;

    address internal owner = address(this);
    address internal guardian = makeAddr("guardian");
    address internal incidentResponder = makeAddr("incidentResponder");
    address internal batcher = makeAddr("batcher");
    address internal unsafeBlockSigner = makeAddr("unsafeBlockSigner");
    address internal proposer = makeAddr("proposer");
    address internal challenger = makeAddr("challenger");
    MockNitroEnclaveVerifier internal nitroEnclaveVerifier;
    MockSP1Verifier internal sp1Verifier;

    uint256 internal l2ChainId = 901;
    Claim internal absolutePrestate = Claim.wrap(0x038512e02c4c3f7bdaec27d00edf55b7155e0905301e1a88083e4e0a6764d54c);

    function setUp() public {
        systemDeploy = new SystemDeploy();
        nitroEnclaveVerifier = new MockNitroEnclaveVerifier();
        sp1Verifier = new MockSP1Verifier();
    }

    function testFuzz_deploySuperchain_succeeds(
        address _superchainProxyAdminOwner,
        address _guardian,
        address _incidentResponder
    )
        public
    {
        vm.assume(_superchainProxyAdminOwner != address(0));
        vm.assume(_guardian != address(0));

        SystemDeploy.SuperchainOutput memory output = systemDeploy.deploySuperchain(
            SystemDeploy.SuperchainInput({
                guardian: _guardian,
                incidentResponder: _incidentResponder,
                superchainProxyAdminOwner: _superchainProxyAdminOwner
            })
        );

        assertEq(output.superchainProxyAdmin.owner(), _superchainProxyAdminOwner, "proxy admin owner");
        assertEq(output.superchainConfigProxy.guardian(), _guardian, "proxy guardian");
        assertEq(output.superchainConfigImpl.guardian(), _guardian, "impl guardian");

        vm.startPrank(address(0));
        assertEq(
            IProxy(payable(address(output.superchainConfigProxy))).implementation(),
            address(output.superchainConfigImpl),
            "implementation"
        );
        assertEq(
            IProxy(payable(address(output.superchainConfigProxy))).admin(),
            address(output.superchainProxyAdmin),
            "admin"
        );
        vm.stopPrank();
    }

    function test_deploySuperchain_nullInput_reverts() public {
        SystemDeploy.SuperchainInput memory input = SystemDeploy.SuperchainInput({
            guardian: guardian, incidentResponder: address(0), superchainProxyAdminOwner: owner
        });

        input.superchainProxyAdminOwner = address(0);
        vm.expectRevert(abi.encodeWithSelector(SystemDeploy.InvalidRoleAddress.selector, "superchainProxyAdminOwner"));
        systemDeploy.deploySuperchain(input);

        input = SystemDeploy.SuperchainInput({
            guardian: address(0), incidentResponder: address(0), superchainProxyAdminOwner: owner
        });
        vm.expectRevert(abi.encodeWithSelector(SystemDeploy.InvalidRoleAddress.selector, "guardian"));
        systemDeploy.deploySuperchain(input);
    }

    function test_deploySuperchain_reuseAddresses_succeeds() public {
        SystemDeploy.SuperchainInput memory input = SystemDeploy.SuperchainInput({
            guardian: guardian, incidentResponder: address(0), superchainProxyAdminOwner: owner
        });

        SystemDeploy.SuperchainOutput memory output0 = systemDeploy.deploySuperchain(input);
        SystemDeploy.SuperchainOutput memory output1 = systemDeploy.deploySuperchain(input);

        assertEq(address(output0.superchainConfigImpl), address(output1.superchainConfigImpl), "implementation");
        assertNotEq(address(output0.superchainConfigProxy), address(output1.superchainConfigProxy), "proxy");
    }

    function test_deploy_withoutManagerAddress_succeeds() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);

        assertNotEq(address(output.opChain.opChainProxyAdmin), address(0), "proxy admin");
        assertNotEq(address(output.opChain.systemConfigProxy), address(0), "system config");
        assertNotEq(address(output.opChain.optimismPortalProxy), address(0), "portal");
        assertNotEq(address(output.opChain.ethLockboxProxy), address(0), "lockbox");
        assertNotEq(address(output.opChain.delayedWETHProxy), address(0), "delayed weth");

        assertEq(output.opChain.opChainProxyAdmin.owner(), owner, "op chain proxy admin owner");
        assertEq(output.opChain.systemConfigProxy.batchInbox(), Types.chainIdToBatchInboxAddress(l2ChainId), "inbox");
        _assertMultiproofDeployed(output, input);
        assertEq(
            address(output.opChain.systemConfigProxy.superchainConfig()),
            address(output.superchain.superchainConfigProxy),
            "superchain config"
        );
        assertValidStandardSystem(_expected(output, input));
    }

    function test_upgrade_withoutManagerDelegatecall_succeeds() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_defaultDeployInput());

        SystemDeploy.UpgradeOutput memory upgradeOutput = systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: output.superchain.superchainConfigProxy,
                implementations: output.impls,
                systemConfigProxy: output.opChain.systemConfigProxy
            })
        );

        assertFalse(upgradeOutput.superchainConfigUpgraded, "superchain already current");
        assertTrue(upgradeOutput.chainUpgraded, "chain upgraded");
        _assertUpgradedProxyImplementations(output);
        assertValidStandardSystem(_expected(output, _defaultDeployInput()));
    }

    function test_deploy_reusingImplementations_doesNotSaveZeroImplementationOnlyArtifacts() public {
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(_defaultDeployInput());

        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        input.saveArtifacts = true;
        input.superchainConfigProxy = output.superchain.superchainConfigProxy;
        input.implementations = output.impls;
        input.opChainInput.l2ChainId = l2ChainId + 1;
        input.opChainInput.saltMixer = "system-deploy-reuse-test";

        vm.mockCallRevert(
            address(artifacts),
            abi.encodeCall(Artifacts.save, ("AggregateVerifier", address(0))),
            "zero aggregate verifier"
        );
        SystemDeploy.DeployOutput memory reuseOutput = systemDeploy.deploy(input);

        _assertMultiproofDeployed(reuseOutput, input);
    }

    function _defaultDeployInput() internal view returns (SystemDeploy.DeployInput memory input_) {
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
            nitroEnclaveVerifier: address(nitroEnclaveVerifier),
            multiproofBlockInterval: 100,
            multiproofIntermediateBlockInterval: 10,
            sp1Verifier: ISP1Verifier(address(sp1Verifier)),
            teeProposer: proposer,
            teeChallenger: challenger,
            guardian: guardian,
            incidentResponder: incidentResponder
        });
        input_.opChainInput = Types.DeployInput({
            roles: Types.Roles({
                opChainProxyAdminOwner: owner,
                systemConfigOwner: owner,
                batcher: batcher,
                unsafeBlockSigner: unsafeBlockSigner
            }),
            basefeeScalar: 100,
            blobBasefeeScalar: 200,
            l2ChainId: l2ChainId,
            startingAnchorRoot: Proposal({ root: Hash.wrap(bytes32(uint256(1))), l2SequenceNumber: 0 }),
            saltMixer: "system-deploy-test",
            gasLimit: 60_000_000
        });
    }

    function _assertMultiproofDeployed(
        SystemDeploy.DeployOutput memory _output,
        SystemDeploy.DeployInput memory _input
    )
        internal
        view
    {
        GameType gameType = GameType.wrap(uint32(_input.implementationsInput.multiproofGameType));
        address aggregateVerifierAddr = address(_output.opChain.aggregateVerifier);
        address teeProverRegistryProxyAddr = address(_output.opChain.teeProverRegistryProxy);
        address teeVerifierAddr = address(_output.opChain.teeVerifier);
        address zkVerifierAddr = address(_output.opChain.zkVerifier);
        Types.Implementations memory impls = _output.impls;

        assertNotEq(aggregateVerifierAddr, address(0), "aggregate verifier");
        assertNotEq(teeProverRegistryProxyAddr, address(0), "tee prover registry proxy");
        assertNotEq(impls.teeProverRegistryImpl, address(0), "tee prover registry impl");
        assertNotEq(teeVerifierAddr, address(0), "tee verifier");
        assertNotEq(zkVerifierAddr, address(0), "zk verifier");
        assertEq(impls.aggregateVerifierImpl, aggregateVerifierAddr, "aggregate verifier impl");
        assertEq(impls.teeVerifierImpl, teeVerifierAddr, "tee verifier impl");
        assertEq(impls.zkVerifierImpl, zkVerifierAddr, "zk verifier impl");
        assertEq(address(_output.opChain.nitroEnclaveVerifier), _input.implementationsInput.nitroEnclaveVerifier);
        assertEq(address(_output.opChain.sp1Verifier), address(_input.implementationsInput.sp1Verifier));
        assertEq(
            _output.opChain.opChainProxyAdmin.getProxyImplementation(teeProverRegistryProxyAddr),
            impls.teeProverRegistryImpl,
            "tee registry proxy impl"
        );

        assertEq(
            address(_output.opChain.disputeGameFactoryProxy.gameImpls(gameType)),
            aggregateVerifierAddr,
            "multiproof game impl"
        );

        AggregateVerifier aggregateVerifier = AggregateVerifier(aggregateVerifierAddr);
        assertEq(
            address(aggregateVerifier.anchorStateRegistry()),
            address(_output.opChain.anchorStateRegistryProxy),
            "aggregate verifier asr"
        );
        assertEq(address(aggregateVerifier.DISPUTE_GAME_FACTORY()), address(_output.opChain.disputeGameFactoryProxy));
        assertEq(address(aggregateVerifier.DELAYED_WETH()), address(_output.opChain.delayedWETHProxy));
        assertEq(address(aggregateVerifier.TEE_VERIFIER()), teeVerifierAddr);
        assertEq(address(aggregateVerifier.ZK_VERIFIER()), zkVerifierAddr);
        assertEq(aggregateVerifier.TEE_IMAGE_HASH(), _input.implementationsInput.teeImageHash);
        assertEq(aggregateVerifier.ZK_RANGE_HASH(), _input.implementationsInput.zkRangeHash);
        assertEq(aggregateVerifier.ZK_AGGREGATE_HASH(), _input.implementationsInput.zkAggregationHash);
        assertEq(aggregateVerifier.CONFIG_HASH(), _input.implementationsInput.multiproofConfigHash);
        assertEq(aggregateVerifier.L2_CHAIN_ID(), _input.opChainInput.l2ChainId);

        TEEProverRegistry teeProverRegistry = TEEProverRegistry(teeProverRegistryProxyAddr);
        assertEq(teeProverRegistry.owner(), _input.opChainInput.roles.opChainProxyAdminOwner, "tee registry owner");
        assertEq(teeProverRegistry.manager(), _input.opChainInput.roles.opChainProxyAdminOwner, "tee registry manager");
        assertTrue(teeProverRegistry.isValidProposer(_input.implementationsInput.teeProposer), "tee proposer");
        assertTrue(teeProverRegistry.isValidProposer(_input.implementationsInput.teeChallenger), "tee challenger");
        assertEq(
            MockNitroEnclaveVerifier(_input.implementationsInput.nitroEnclaveVerifier).proofSubmitter(),
            teeProverRegistryProxyAddr,
            "nitro proof submitter"
        );
        assertEq(
            address(teeProverRegistry.DISPUTE_GAME_FACTORY()),
            address(_output.opChain.disputeGameFactoryProxy),
            "tee registry dgf"
        );

        assertEq(
            address(TEEVerifier(teeVerifierAddr).TEE_PROVER_REGISTRY()),
            teeProverRegistryProxyAddr,
            "tee verifier registry"
        );
        assertEq(address(ZKVerifier(zkVerifierAddr).SP1_VERIFIER()), address(_input.implementationsInput.sp1Verifier));
    }

    function _assertUpgradedProxyImplementations(SystemDeploy.DeployOutput memory _output) internal view {
        IProxyAdmin superchainProxyAdmin = _output.superchain.superchainProxyAdmin;
        IProxyAdmin opChainProxyAdmin = _output.opChain.opChainProxyAdmin;
        Types.Implementations memory impls = _output.impls;

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

    function _expected(
        SystemDeploy.DeployOutput memory _output,
        SystemDeploy.DeployInput memory _input
    )
        internal
        pure
        returns (SystemDeployAssertions.ExpectedSystemDeployState memory expected_)
    {
        expected_ = SystemDeployAssertions.ExpectedSystemDeployState({
            systemConfig: _output.opChain.systemConfigProxy,
            anchorStateRegistry: _output.opChain.anchorStateRegistryProxy,
            superchainConfig: _output.superchain.superchainConfigProxy,
            implementations: _output.impls,
            delayedWETH: _output.opChain.delayedWETHProxy,
            ethLockbox: _output.opChain.ethLockboxProxy,
            proxyAdminOwner: _input.opChainInput.roles.opChainProxyAdminOwner,
            multiproofGameType: GameType.wrap(uint32(_input.implementationsInput.multiproofGameType)),
            teeImageHash: _input.implementationsInput.teeImageHash,
            zkRangeHash: _input.implementationsInput.zkRangeHash,
            zkAggregationHash: _input.implementationsInput.zkAggregationHash,
            multiproofConfigHash: _input.implementationsInput.multiproofConfigHash,
            l2ChainId: _input.opChainInput.l2ChainId,
            multiproofBlockInterval: _input.implementationsInput.multiproofBlockInterval,
            multiproofIntermediateBlockInterval: _input.implementationsInput.multiproofIntermediateBlockInterval,
            withdrawalDelaySeconds: _input.implementationsInput.withdrawalDelaySeconds
        });
    }
}
