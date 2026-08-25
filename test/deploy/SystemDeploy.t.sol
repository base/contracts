// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Artifacts } from "scripts/Artifacts.s.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Types } from "scripts/libraries/Types.sol";
import { SystemDeployAssertions } from "test/deploy/SystemDeployAssertions.sol";

import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroValidator } from "interfaces/L1/proofs/tee/INitroValidator.sol";
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";
import { ProtocolVersions } from "src/L1/ProtocolVersions.sol";
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";
import { ZKVerifier } from "src/L1/proofs/zk/ZKVerifier.sol";
import { ProtocolVersionsConfig } from "src/libraries/ProtocolVersionsConfig.sol";
import { GameType, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";
import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { MockNitroValidator } from "test/mocks/MockNitroValidator.sol";

contract MockLegacyTEEProverRegistry is DevTEEProverRegistry {
    constructor(
        INitroValidator nitroValidator,
        IDisputeGameFactory factory
    )
        DevTEEProverRegistry(nitroValidator, factory)
    { }

    function version() public pure override returns (string memory) {
        return "0.5.0";
    }
}

contract MockSP1Verifier {
    function verifyProof(bytes32, bytes calldata, bytes calldata) external pure { }
}

contract MockInvalidTEEProverRegistry {
    function NITRO_VALIDATOR() external pure returns (INitroValidator) {
        return INitroValidator(address(0));
    }
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
    MockNitroValidator internal nitroValidator;
    MockSP1Verifier internal sp1Verifier;

    uint256 internal l2ChainId = 901;

    function setUp() public {
        systemDeploy = new SystemDeploy();
        nitroValidator = new MockNitroValidator();
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
            EIP1967Helper.getAdmin(address(output.superchainConfigProxy)), address(output.superchainProxyAdmin), "admin"
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
        assertNotEq(address(output.opChain.delayedWETHProxy), address(0), "delayed weth");
        assertNotEq(address(output.opChain.protocolVersionsProxy), address(0), "protocol versions");
        assertEq(
            output.opChain.protocolVersionsProxy.incidentResponder(),
            incidentResponder,
            "protocol versions incident responder"
        );

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

    /// @notice A chain that already has a hardfork history has to seed the registry at deploy time.
    ///         `registerUpgrade` cannot enter activations that are already in the past, and
    ///         `initialize` runs once, so an empty import here would be permanent.
    function test_deploy_seedsProtocolVersionsWithInitialSchedule_succeeds() public {
        vm.warp(1_800_000_000);

        uint64[] memory schedule = new uint64[](ProtocolVersionsConfig.INITIAL_UPGRADE_COUNT);
        schedule[0] = 1_686_789_347;
        schedule[1] = 1_704_992_401;
        schedule[2] = 0; // Unscheduled on this chain.
        schedule[3] = 1_710_374_401;

        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        input.opChainInput.initialUpgradeSchedule = schedule;

        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);

        uint64[] memory imported = output.opChain.protocolVersionsProxy.getSchedule();
        assertEq(imported.length, schedule.length, "schedule length");
        for (uint256 i = 0; i < schedule.length; i++) {
            assertEq(imported[i], schedule[i], "schedule entry");
        }

        assertEq(
            address(AggregateVerifier(address(output.opChain.aggregateVerifier)).PROTOCOL_VERSIONS()),
            address(output.opChain.protocolVersionsProxy),
            "verifier bound to seeded registry"
        );
    }

    function test_deploy_multiproofDisabled_allowsUnsetL2BlockTime_succeeds() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        input.implementationsInput.multiproofConfigHash = bytes32(0);
        input.implementationsInput.scheduleConfig = AggregateVerifier.ScheduleConfig({
            protocolVersions: IProtocolVersions(address(0)), genesisBlockNumber: 0, genesisTimestamp: 0, blockTime: 0
        });

        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);

        assertNotEq(address(output.opChain.protocolVersionsProxy), address(0), "protocol versions");
        assertEq(address(output.opChain.aggregateVerifier), address(0), "aggregate verifier");
        assertEq(address(output.opChain.teeProverRegistryProxy), address(0), "tee registry");
        assertEq(output.impls.aggregateVerifierImpl, address(0), "aggregate verifier impl");
    }

    function test_deploy_multiproofEnabled_withoutL2GenesisTimestamp_reverts() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        input.implementationsInput.scheduleConfig.genesisTimestamp = 0;

        vm.expectRevert("SystemDeploy: L2 genesis timestamp not set");
        systemDeploy.deploy(input);
    }

    function test_deploy_withoutNitroValidator_reverts() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        input.implementationsInput.nitroValidator = address(0);

        vm.expectRevert("SystemDeploy: nitroValidator not set");
        systemDeploy.deploy(input);
    }

    function test_upgrade_registryWithInvalidNitroValidator_reverts() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);
        Types.Implementations memory implementations = output.impls;
        implementations.teeProverRegistryImpl = address(new MockInvalidTEEProverRegistry());

        vm.expectRevert("DeployUtils: zero address");
        systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: output.superchain.superchainConfigProxy,
                implementations: implementations,
                systemConfigProxy: output.opChain.systemConfigProxy,
                protocolVersionsProxy: output.opChain.protocolVersionsProxy
            })
        );
    }

    function test_upgrade_withoutManagerDelegatecall_succeeds() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);
        Types.Implementations memory implementations = output.impls;
        ProtocolVersions protocolVersionsImpl = new ProtocolVersions();
        implementations.protocolVersionsImpl = address(protocolVersionsImpl);

        SystemDeploy.UpgradeOutput memory upgradeOutput = systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: output.superchain.superchainConfigProxy,
                implementations: implementations,
                systemConfigProxy: output.opChain.systemConfigProxy,
                protocolVersionsProxy: output.opChain.protocolVersionsProxy
            })
        );

        assertFalse(upgradeOutput.superchainConfigUpgraded, "superchain already current");
        assertTrue(upgradeOutput.chainUpgraded, "chain upgraded");
        assertEq(
            output.superchain.superchainProxyAdmin
                .getProxyImplementation(address(output.superchain.superchainConfigProxy)),
            output.impls.superchainConfigImpl,
            "superchain config impl"
        );
        assertEq(
            output.opChain.opChainProxyAdmin.getProxyImplementation(address(output.opChain.protocolVersionsProxy)),
            address(protocolVersionsImpl),
            "protocol versions impl"
        );
        assertValidStandardSystem(_expected(output, input));
    }

    function test_upgrade_predeployedTEEProverRegistryImplementation_succeeds() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);
        TEEProverRegistry registry = TEEProverRegistry(address(output.opChain.teeProverRegistryProxy));
        MockLegacyTEEProverRegistry legacyImpl = new MockLegacyTEEProverRegistry({
            nitroValidator: INitroValidator(address(nitroValidator)), factory: registry.DISPUTE_GAME_FACTORY()
        });
        output.opChain.opChainProxyAdmin.upgrade(payable(address(registry)), address(legacyImpl));

        address signer = makeAddr("existing-signer");
        bytes32 imageHash = keccak256("existing-image");
        DevTEEProverRegistry(address(registry)).addDevSigner(signer, imageHash);

        TEEProverRegistry newImpl = new TEEProverRegistry({
            nitroValidator: INitroValidator(address(nitroValidator)), factory: registry.DISPUTE_GAME_FACTORY()
        });
        Types.Implementations memory implementations = output.impls;
        implementations.teeProverRegistryImpl = address(newImpl);
        systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: output.superchain.superchainConfigProxy,
                implementations: implementations,
                systemConfigProxy: output.opChain.systemConfigProxy,
                protocolVersionsProxy: output.opChain.protocolVersionsProxy
            })
        );

        assertEq(
            output.opChain.opChainProxyAdmin.getProxyImplementation(address(registry)),
            address(newImpl),
            "tee registry impl"
        );
        assertEq(registry.version(), "0.6.1");
        assertEq(registry.owner(), owner);
        assertEq(registry.manager(), owner);
        assertEq(GameType.unwrap(registry.gameType()), uint32(input.implementationsInput.multiproofGameType));
        assertTrue(registry.isValidProposer(proposer));
        assertTrue(registry.isValidProposer(challenger));
        assertTrue(registry.isRegisteredSigner(signer));
        assertEq(registry.signerImageHash(signer), imageHash);
        assertEq(address(registry.NITRO_VALIDATOR()), address(nitroValidator));
    }

    function test_upgrade_discoversProtocolVersionsProxyFromArtifacts_succeeds() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);
        Types.Implementations memory implementations = output.impls;
        ProtocolVersions protocolVersionsImpl = new ProtocolVersions();
        implementations.protocolVersionsImpl = address(protocolVersionsImpl);

        _saveArtifact("ProtocolVersionsProxy", address(output.opChain.protocolVersionsProxy));

        SystemDeploy.UpgradeOutput memory upgradeOutput = systemDeploy.upgrade(
            SystemDeploy.UpgradeInput({
                saveArtifacts: false,
                superchainConfigProxy: output.superchain.superchainConfigProxy,
                implementations: implementations,
                systemConfigProxy: output.opChain.systemConfigProxy,
                protocolVersionsProxy: IProtocolVersions(address(0))
            })
        );

        assertFalse(upgradeOutput.superchainConfigUpgraded, "superchain already current");
        assertTrue(upgradeOutput.chainUpgraded, "chain upgraded");
        assertEq(
            output.opChain.opChainProxyAdmin.getProxyImplementation(address(output.opChain.protocolVersionsProxy)),
            address(protocolVersionsImpl),
            "protocol versions impl"
        );
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
            nitroValidator: address(nitroValidator),
            scheduleConfig: AggregateVerifier.ScheduleConfig({
                protocolVersions: IProtocolVersions(address(0)),
                genesisBlockNumber: 0,
                genesisTimestamp: 1,
                blockTime: 2
            }),
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
                unsafeBlockSigner: unsafeBlockSigner,
                incidentResponder: incidentResponder
            }),
            basefeeScalar: 100,
            blobBasefeeScalar: 200,
            l2ChainId: l2ChainId,
            startingAnchorRoot: Proposal({ root: Hash.wrap(bytes32(uint256(1))), l2SequenceNumber: 0 }),
            saltMixer: "system-deploy-test",
            gasLimit: 60_000_000,
            initialUpgradeSchedule: new uint64[](0)
        });
    }

    function _assertMultiproofDeployed(
        SystemDeploy.DeployOutput memory _output,
        SystemDeploy.DeployInput memory _input
    )
        internal
        view
    {
        address teeProverRegistryProxyAddr = address(_output.opChain.teeProverRegistryProxy);
        address teeVerifierAddr = address(_output.opChain.teeVerifier);
        address zkVerifierAddr = address(_output.opChain.zkVerifier);
        Types.Implementations memory impls = _output.impls;

        assertNotEq(teeProverRegistryProxyAddr, address(0), "tee prover registry proxy");
        assertNotEq(impls.teeProverRegistryImpl, address(0), "tee prover registry impl");
        assertEq(impls.aggregateVerifierImpl, address(_output.opChain.aggregateVerifier), "aggregate verifier impl");
        assertEq(impls.teeVerifierImpl, teeVerifierAddr, "tee verifier impl");
        assertEq(impls.zkVerifierImpl, zkVerifierAddr, "zk verifier impl");
        assertEq(address(_output.opChain.nitroValidator), _input.implementationsInput.nitroValidator, "nitro validator");
        assertEq(address(_output.opChain.sp1Verifier), address(_input.implementationsInput.sp1Verifier), "sp1 verifier");
        assertEq(
            _output.opChain.opChainProxyAdmin.getProxyImplementation(teeProverRegistryProxyAddr),
            impls.teeProverRegistryImpl,
            "tee registry proxy impl"
        );

        TEEProverRegistry teeProverRegistry = TEEProverRegistry(teeProverRegistryProxyAddr);
        assertEq(teeProverRegistry.owner(), _input.opChainInput.roles.opChainProxyAdminOwner, "tee registry owner");
        assertEq(teeProverRegistry.manager(), _input.opChainInput.roles.opChainProxyAdminOwner, "tee registry manager");
        assertTrue(teeProverRegistry.isValidProposer(_input.implementationsInput.teeProposer), "tee proposer");
        assertTrue(teeProverRegistry.isValidProposer(_input.implementationsInput.teeChallenger), "tee challenger");
        assertEq(
            address(teeProverRegistry.NITRO_VALIDATOR()),
            _input.implementationsInput.nitroValidator,
            "tee registry nitro validator"
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

    function _saveArtifact(string memory _name, address _addr) internal {
        vm.etch(address(artifacts), vm.getDeployedCode("Artifacts.s.sol:Artifacts"));
        bytes32 slot = keccak256(abi.encodePacked(_name, uint256(0)));
        vm.store(address(artifacts), slot, bytes32(uint256(uint160(_addr))));
        assertEq(artifacts.getAddress(_name), _addr, "artifact saved");
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
            proxyAdminOwner: _input.opChainInput.roles.opChainProxyAdminOwner,
            multiproofGameType: GameType.wrap(uint32(_input.implementationsInput.multiproofGameType)),
            teeImageHash: _input.implementationsInput.teeImageHash,
            zkRangeHash: _input.implementationsInput.zkRangeHash,
            zkAggregationHash: _input.implementationsInput.zkAggregationHash,
            multiproofConfigHash: _input.implementationsInput.multiproofConfigHash,
            l2ChainId: _input.opChainInput.l2ChainId,
            l2GenesisBlockNumber: _input.implementationsInput.scheduleConfig.genesisBlockNumber,
            l2GenesisTimestamp: _input.implementationsInput.scheduleConfig.genesisTimestamp,
            l2BlockTime: _input.implementationsInput.scheduleConfig.blockTime,
            multiproofBlockInterval: _input.implementationsInput.multiproofBlockInterval,
            multiproofIntermediateBlockInterval: _input.implementationsInput.multiproofIntermediateBlockInterval,
            withdrawalDelaySeconds: _input.implementationsInput.withdrawalDelaySeconds
        });
    }
}
