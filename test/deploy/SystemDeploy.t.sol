// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Artifacts } from "scripts/Artifacts.s.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Types } from "scripts/libraries/Types.sol";
import { SystemDeployAssertions } from "test/deploy/SystemDeployAssertions.sol";

import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";
import { ZKVerifier } from "src/L1/proofs/zk/ZKVerifier.sol";
import { GameType, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

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
        assertEq(output.superchainConfigProxy.incidentResponder(), _incidentResponder, "proxy incident responder");
        assertEq(output.superchainConfigImpl.incidentResponder(), _incidentResponder, "impl incident responder");

        assertEq(
            EIP1967Helper.getImplementation(address(output.superchainConfigProxy)),
            address(output.superchainConfigImpl),
            "implementation"
        );
        assertEq(
            EIP1967Helper.getAdmin(address(output.superchainConfigProxy)),
            address(output.superchainProxyAdmin),
            "admin"
        );
    }

    function test_deploySuperchain_nullInput_reverts() public {
        SystemDeploy.SuperchainInput memory input = SystemDeploy.SuperchainInput({
            guardian: guardian, incidentResponder: incidentResponder, superchainProxyAdminOwner: address(0)
        });
        vm.expectRevert(abi.encodeWithSelector(SystemDeploy.InvalidRoleAddress.selector, "superchainProxyAdminOwner"));
        systemDeploy.deploySuperchain(input);

        input = SystemDeploy.SuperchainInput({
            guardian: address(0), incidentResponder: incidentResponder, superchainProxyAdminOwner: owner
        });
        vm.expectRevert(abi.encodeWithSelector(SystemDeploy.InvalidRoleAddress.selector, "guardian"));
        systemDeploy.deploySuperchain(input);
    }

    function test_deploySuperchain_reuseAddresses_succeeds() public {
        SystemDeploy.SuperchainInput memory input = SystemDeploy.SuperchainInput({
            guardian: guardian, incidentResponder: incidentResponder, superchainProxyAdminOwner: owner
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
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);

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
        assertEq(
            output.superchain.superchainProxyAdmin.getProxyImplementation(
                address(output.superchain.superchainConfigProxy)
            ),
            output.impls.superchainConfigImpl,
            "superchain config impl"
        );
        assertValidStandardSystem(_expected(output, input));
    }

    function test_deploy_reusingImplementations_doesNotSaveZeroImplementationOnlyArtifacts() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);

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
        assertEq(
            address(_output.opChain.nitroEnclaveVerifier),
            _input.implementationsInput.nitroEnclaveVerifier,
            "nitro enclave verifier"
        );
        assertEq(address(_output.opChain.sp1Verifier), address(_input.implementationsInput.sp1Verifier), "sp1 verifier");
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
        assertEq(
            address(aggregateVerifier.DISPUTE_GAME_FACTORY()),
            address(_output.opChain.disputeGameFactoryProxy),
            "aggregate verifier dgf"
        );
        assertEq(address(aggregateVerifier.DELAYED_WETH()), address(_output.opChain.delayedWETHProxy), "delayed weth");
        assertEq(address(aggregateVerifier.TEE_VERIFIER()), teeVerifierAddr, "aggregate verifier tee");
        assertEq(address(aggregateVerifier.ZK_VERIFIER()), zkVerifierAddr, "aggregate verifier zk");
        assertEq(aggregateVerifier.TEE_IMAGE_HASH(), _input.implementationsInput.teeImageHash, "tee image hash");
        assertEq(aggregateVerifier.ZK_RANGE_HASH(), _input.implementationsInput.zkRangeHash, "zk range hash");
        assertEq(
            aggregateVerifier.ZK_AGGREGATE_HASH(), _input.implementationsInput.zkAggregationHash, "zk aggregation hash"
        );
        assertEq(
            aggregateVerifier.CONFIG_HASH(), _input.implementationsInput.multiproofConfigHash, "multiproof config hash"
        );
        assertEq(aggregateVerifier.L2_CHAIN_ID(), _input.opChainInput.l2ChainId, "l2 chain id");

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
        assertEq(
            address(ZKVerifier(zkVerifierAddr).SP1_VERIFIER()),
            address(_input.implementationsInput.sp1Verifier),
            "zk verifier sp1"
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
