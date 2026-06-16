// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Artifacts } from "scripts/Artifacts.s.sol";
import { SystemDeploy } from "scripts/deploy/SystemDeploy.s.sol";
import { Types } from "scripts/libraries/Types.sol";
import { SystemDeployAssertions } from "test/deploy/SystemDeployAssertions.sol";

import { IAggregateVerifier } from "interfaces/L1/proofs/IAggregateVerifier.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";
import { ZKVerifier } from "src/L1/proofs/zk/ZKVerifier.sol";
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { GameStatus, GameType, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";
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
            output.superchain.superchainProxyAdmin
                .getProxyImplementation(address(output.superchain.superchainConfigProxy)),
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

    function test_deploy_devMultiproof_succeeds() public {
        address devSigner = 0x6cebCF805c5191BCf26602E43DECa89AEe092b5d;
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        input.implementationsInput.nitroEnclaveVerifier = address(0);
        input.implementationsInput.sp1Verifier = ISP1Verifier(address(0));
        input.implementationsInput.zkRangeHash = bytes32(0);
        input.implementationsInput.zkAggregationHash = bytes32(0);
        input.implementationsInput.devTeeSigner = devSigner;
        input.implementationsInput.proofMaturityDelaySeconds = 0;
        input.implementationsInput.withdrawalDelaySeconds = 0;
        input.implementationsInput.disputeGameFinalityDelaySeconds = 0;
        input.implementationsInput.slowFinalizationDelay = 0;
        input.implementationsInput.fastFinalizationDelay = 0;

        SystemDeploy.DeployOutput memory output = systemDeploy.deploy(input);

        _assertMultiproofDeployed(output, input);

        DevTEEProverRegistry devRegistry = DevTEEProverRegistry(address(output.opChain.teeProverRegistryProxy));
        assertTrue(devRegistry.isRegisteredSigner(devSigner), "dev signer registered at deploy time");

        IAggregateVerifier aggVerifier = IAggregateVerifier(address(output.opChain.aggregateVerifier));
        assertEq(
            address(aggVerifier.DELAYED_WETH()),
            address(output.opChain.delayedWETHProxy),
            "aggregate verifier uses real DelayedWETH"
        );

        assertEq(address(output.opChain.zkVerifier), address(0xdead), "zk verifier is bricked (0xdead)");
        assertEq(address(output.opChain.nitroEnclaveVerifier), address(0), "no nitro verifier in dev mode");
        assertEq(address(output.opChain.sp1Verifier), address(0), "no sp1 verifier in dev mode");

        IDisputeGameFactory factory = IDisputeGameFactory(address(output.opChain.disputeGameFactoryProxy));
        GameType gameType = GameType.wrap(uint32(input.implementationsInput.multiproofGameType));
        assertNotEq(address(factory.gameImpls(gameType)), address(0), "game type registered on factory");
    }

    function test_deploy_devMultiproof_onProductionChain_reverts() public {
        SystemDeploy.DeployInput memory input = _defaultDeployInput();
        input.implementationsInput.nitroEnclaveVerifier = address(0);
        input.implementationsInput.sp1Verifier = ISP1Verifier(address(0));
        input.implementationsInput.zkRangeHash = bytes32(0);
        input.implementationsInput.zkAggregationHash = bytes32(0);
        input.implementationsInput.devTeeSigner = makeAddr("devSigner");

        // Ethereum mainnet
        vm.chainId(1);
        vm.expectRevert("SystemDeploy: dev multiproof cannot be deployed on production chains");
        systemDeploy.deploy(input);

        // Base mainnet
        vm.chainId(8453);
        vm.expectRevert("SystemDeploy: dev multiproof cannot be deployed on production chains");
        systemDeploy.deploy(input);

        // Base Sepolia
        vm.chainId(84532);
        vm.expectRevert("SystemDeploy: dev multiproof cannot be deployed on production chains");
        systemDeploy.deploy(input);
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
        address teeProverRegistryProxyAddr = address(_output.opChain.teeProverRegistryProxy);
        address teeVerifierAddr = address(_output.opChain.teeVerifier);
        address zkVerifierAddr = address(_output.opChain.zkVerifier);
        Types.Implementations memory impls = _output.impls;

        assertNotEq(teeProverRegistryProxyAddr, address(0), "tee prover registry proxy");
        assertNotEq(impls.teeProverRegistryImpl, address(0), "tee prover registry impl");
        assertEq(impls.aggregateVerifierImpl, address(_output.opChain.aggregateVerifier), "aggregate verifier impl");
        assertEq(impls.teeVerifierImpl, teeVerifierAddr, "tee verifier impl");
        if (address(_input.implementationsInput.sp1Verifier) != address(0)) {
            assertEq(impls.zkVerifierImpl, zkVerifierAddr, "zk verifier impl");
        }
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

        TEEProverRegistry teeProverRegistry = TEEProverRegistry(teeProverRegistryProxyAddr);
        assertEq(teeProverRegistry.owner(), _input.opChainInput.roles.opChainProxyAdminOwner, "tee registry owner");
        assertEq(teeProverRegistry.manager(), _input.opChainInput.roles.opChainProxyAdminOwner, "tee registry manager");
        assertTrue(teeProverRegistry.isValidProposer(_input.implementationsInput.teeProposer), "tee proposer");
        assertTrue(teeProverRegistry.isValidProposer(_input.implementationsInput.teeChallenger), "tee challenger");
        if (_input.implementationsInput.nitroEnclaveVerifier != address(0)) {
            assertEq(
                MockNitroEnclaveVerifier(_input.implementationsInput.nitroEnclaveVerifier).proofSubmitter(),
                teeProverRegistryProxyAddr,
                "nitro proof submitter"
            );
        }
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
        if (address(_input.implementationsInput.sp1Verifier) != address(0)) {
            assertEq(
                address(ZKVerifier(zkVerifierAddr).SP1_VERIFIER()),
                address(_input.implementationsInput.sp1Verifier),
                "zk verifier sp1"
            );
        }
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

/// @title ZKBricking_Test
/// @notice Invariant tests proving the ZK proof path is permanently bricked when deploying
///         in dev multiproof mode (sp1Verifier == address(0) → ZK_VERIFIER = 0xdead).
contract ZKBricking_Test is Test {
    Artifacts internal constant artifacts =
        Artifacts(address(uint160(uint256(keccak256(abi.encode("optimism.artifacts"))))));
    SystemDeploy internal systemDeploy;

    uint256 internal constant TEE_SIGNER_PK = 0xA11CE;
    address internal teeSigner;

    address internal owner = address(this);
    address internal guardian = makeAddr("guardian");
    address internal incidentResponder = makeAddr("incidentResponder");
    address internal proposer = makeAddr("proposer");
    address internal challenger = makeAddr("challenger");

    uint256 internal l2ChainId = 901;
    uint256 internal constant BLOCK_INTERVAL = 100;
    uint256 internal constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 internal constant INTERMEDIATE_ROOTS_COUNT = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL;

    SystemDeploy.DeployOutput internal output;
    SystemDeploy.DeployInput internal input;
    IDisputeGameFactory internal factory;
    GameType internal gameType;
    DevTEEProverRegistry internal devRegistry;

    function setUp() public {
        teeSigner = vm.addr(TEE_SIGNER_PK);

        systemDeploy = new SystemDeploy();
        input = _devMultiproofInput();
        output = systemDeploy.deploy(input);

        factory = IDisputeGameFactory(address(output.opChain.disputeGameFactoryProxy));
        gameType = GameType.wrap(uint32(input.implementationsInput.multiproofGameType));

        devRegistry = DevTEEProverRegistry(address(output.opChain.teeProverRegistryProxy));
        vm.prank(owner);
        devRegistry.addDevSigner(teeSigner, input.implementationsInput.teeImageHash);
    }

    /// @notice Attempting to initialize a dispute game with a ZK proof hard-reverts because
    ///         ZK_VERIFIER (0xdead) is codeless and the ABI decoder fails on empty returndata.
    function test_zkProofSubmission_reverts() public {
        Claim rootClaim = _nextRootClaim();
        bytes memory extraData =
            _buildExtraData(rootClaim, BLOCK_INTERVAL, address(output.opChain.anchorStateRegistryProxy));
        bytes memory zkProof = _buildProof(AggregateVerifier.ProofType.ZK, "zk-payload");

        uint256 initBond = factory.initBonds(gameType);
        vm.deal(proposer, initBond);
        vm.prank(proposer);
        vm.expectRevert();
        factory.createWithInitData{ value: initBond }(gameType, rootClaim, extraData, zkProof);
    }

    /// @notice After a valid TEE proof, challenge() with a ZK proof hard-reverts.
    function test_zkChallenge_reverts() public {
        (AggregateVerifier game,) = _createGameWithTEEProof();

        bytes32 intermediateRoot = game.intermediateOutputRoot(0);
        bytes32 differentRoot = keccak256(abi.encodePacked(intermediateRoot, "different"));
        bytes memory zkChallengeProof =
            abi.encodePacked(uint8(AggregateVerifier.ProofType.ZK), bytes32(0), uint256(0), "fake-zk-proof");

        vm.expectRevert();
        game.challenge(zkChallengeProof, 0, differentRoot);
    }

    /// @notice A TEE-only game resolves successfully as DEFENDER_WINS (finalization delay = 0).
    function test_resolve_succeedsWithTEEOnly() public {
        (AggregateVerifier game,) = _createGameWithTEEProof();

        vm.warp(block.timestamp + 1);
        GameStatus status = game.resolve();
        assertEq(uint8(status), uint8(GameStatus.DEFENDER_WINS), "TEE-only game resolves DEFENDER_WINS");
    }

    /// @notice After TEE initialization, verifyProposalProof() with ZK hard-reverts.
    function test_zkProofViaVerifyProposalProof_reverts() public {
        (AggregateVerifier game,) = _createGameWithTEEProof();

        bytes memory zkProof = abi.encodePacked(uint8(AggregateVerifier.ProofType.ZK), "fake-zk-proof");
        vm.expectRevert();
        game.verifyProposalProof(zkProof);
    }

    // ─── Helpers
    // ───────────────────────────────────────────────────────────────

    function _createGameWithTEEProof() internal returns (AggregateVerifier game, Claim rootClaim) {
        rootClaim = _nextRootClaim();
        bytes memory extraData =
            _buildExtraData(rootClaim, BLOCK_INTERVAL, address(output.opChain.anchorStateRegistryProxy));
        bytes memory teeProof = _buildSignedTEEProof(rootClaim, extraData);

        uint256 initBond = factory.initBonds(gameType);
        vm.deal(proposer, initBond);
        vm.prank(proposer);
        game = AggregateVerifier(
            address(factory.createWithInitData{ value: initBond }(gameType, rootClaim, extraData, teeProof))
        );
    }

    function _buildSignedTEEProof(Claim rootClaim, bytes memory extraData) internal view returns (bytes memory) {
        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);

        Proposal memory startingAnchorRoot = output.opChain.anchorStateRegistryProxy.getStartingAnchorRoot();
        bytes32 startingRoot = startingAnchorRoot.root.raw();
        uint64 startingL2SeqNum = uint64(startingAnchorRoot.l2SequenceNumber);
        uint64 endingL2SeqNum = startingL2SeqNum + uint64(BLOCK_INTERVAL);

        bytes memory intermediateRoots = _extractIntermediateRoots(extraData);

        bytes32 journal = keccak256(
            abi.encodePacked(
                proposer,
                l1OriginHash,
                startingRoot,
                startingL2SeqNum,
                rootClaim.raw(),
                endingL2SeqNum,
                intermediateRoots,
                input.implementationsInput.multiproofConfigHash,
                input.implementationsInput.teeImageHash
            )
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(TEE_SIGNER_PK, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        return abi.encodePacked(uint8(AggregateVerifier.ProofType.TEE), l1OriginHash, l1OriginNumber, signature);
    }

    function _extractIntermediateRoots(bytes memory extraData) internal pure returns (bytes memory) {
        uint256 headerLen = 32 + 20;
        uint256 rootsLen = extraData.length - headerLen;
        bytes memory roots = new bytes(rootsLen);
        for (uint256 i = 0; i < rootsLen; i++) {
            roots[i] = extraData[headerLen + i];
        }
        return roots;
    }

    function _nextRootClaim() internal view returns (Claim) {
        Proposal memory anchor = output.opChain.anchorStateRegistryProxy.getStartingAnchorRoot();
        uint256 nextL2Block = anchor.l2SequenceNumber + BLOCK_INTERVAL;
        return Claim.wrap(keccak256(abi.encode(nextL2Block)));
    }

    function _buildExtraData(
        Claim rootClaim,
        uint256 blockInterval,
        address parentAddr
    )
        internal
        pure
        returns (bytes memory)
    {
        uint256 l2BlockNumber = blockInterval;
        bytes memory intermediateRoots = _generateIntermediateRoots(l2BlockNumber, rootClaim);
        return abi.encodePacked(l2BlockNumber, parentAddr, intermediateRoots);
    }

    function _generateIntermediateRoots(uint256 l2BlockNumber, Claim rootClaim) private pure returns (bytes memory) {
        bytes32[] memory roots = new bytes32[](INTERMEDIATE_ROOTS_COUNT);
        uint256 startingL2Block = l2BlockNumber - BLOCK_INTERVAL;
        for (uint256 i = 1; i < INTERMEDIATE_ROOTS_COUNT; i++) {
            roots[i - 1] = keccak256(abi.encode(startingL2Block + INTERMEDIATE_BLOCK_INTERVAL * i));
        }
        roots[INTERMEDIATE_ROOTS_COUNT - 1] = rootClaim.raw();
        return abi.encodePacked(roots);
    }

    function _buildProof(
        AggregateVerifier.ProofType proofType,
        bytes memory payload
    )
        internal
        view
        returns (bytes memory)
    {
        uint256 l1OriginNumber = block.number - 1;
        bytes32 l1OriginHash = blockhash(l1OriginNumber);
        return abi.encodePacked(uint8(proofType), l1OriginHash, l1OriginNumber, payload);
    }

    function _devMultiproofInput() internal returns (SystemDeploy.DeployInput memory input_) {
        input_.saveArtifacts = false;
        input_.superchainInput = SystemDeploy.SuperchainInput({
            guardian: guardian, incidentResponder: incidentResponder, superchainProxyAdminOwner: owner
        });
        input_.implementationsInput = SystemDeploy.ImplementationInput({
            withdrawalDelaySeconds: 0,
            proofMaturityDelaySeconds: 0,
            disputeGameFinalityDelaySeconds: 0,
            teeImageHash: bytes32(uint256(1)),
            zkRangeHash: bytes32(0),
            zkAggregationHash: bytes32(0),
            multiproofConfigHash: bytes32(uint256(4)),
            multiproofGameType: 621,
            nitroEnclaveVerifier: address(0),
            multiproofBlockInterval: BLOCK_INTERVAL,
            multiproofIntermediateBlockInterval: INTERMEDIATE_BLOCK_INTERVAL,
            sp1Verifier: ISP1Verifier(address(0)),
            teeProposer: proposer,
            teeChallenger: challenger,
            devTeeSigner: vm.addr(TEE_SIGNER_PK),
            guardian: guardian,
            incidentResponder: incidentResponder,
            slowFinalizationDelay: 0,
            fastFinalizationDelay: 0
        });
        input_.opChainInput = Types.DeployInput({
            roles: Types.Roles({
                opChainProxyAdminOwner: owner,
                systemConfigOwner: owner,
                batcher: makeAddr("batcher"),
                unsafeBlockSigner: makeAddr("unsafeBlockSigner")
            }),
            basefeeScalar: 100,
            blobBasefeeScalar: 200,
            l2ChainId: l2ChainId,
            startingAnchorRoot: Proposal({ root: Hash.wrap(bytes32(uint256(1))), l2SequenceNumber: 0 }),
            saltMixer: "zk-bricking-test",
            gasLimit: 60_000_000
        });
    }
}
