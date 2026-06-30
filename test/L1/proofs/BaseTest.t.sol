// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { AnchorStateRegistry } from "src/L1/proofs/AnchorStateRegistry.sol";
import { DelayedWETH } from "src/L1/proofs/DelayedWETH.sol";
import { DisputeGameFactory } from "src/L1/proofs/DisputeGameFactory.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { GameType, GameTypes, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";

import { Proxy } from "src/universal/Proxy.sol";
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";

import { MockVerifier } from "test/mocks/MockVerifier.sol";

contract BaseTest is Test {
    uint256 internal constant L2_CHAIN_ID = 8453;

    // AggregateVerifier expects evenly spaced intermediate roots.
    uint256 internal constant BLOCK_INTERVAL = 100;
    uint256 internal constant INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint256 private constant INTERMEDIATE_ROOTS_COUNT = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL;
    uint256 internal constant LAST_INTERMEDIATE_ROOT_INDEX = INTERMEDIATE_ROOTS_COUNT - 1;

    uint256 internal constant INIT_BOND = 1 ether;
    uint256 internal constant DELAYED_WETH_DELAY = 1 days;

    uint256 internal currentL2BlockNumber;

    address internal immutable TEE_PROVER = makeAddr("tee-prover");
    address internal immutable ZK_PROVER = makeAddr("zk-prover");

    ProxyAdmin internal proxyAdmin;
    ISystemConfig internal systemConfig;

    DisputeGameFactory internal factory;
    AnchorStateRegistry internal anchorStateRegistry;
    DelayedWETH internal delayedWETH;

    MockVerifier internal teeVerifier;
    MockVerifier internal zkVerifier;

    function setUp() public virtual {
        systemConfig = ISystemConfig(makeAddr("system-config"));
        vm.mockCall(address(systemConfig), abi.encodeCall(ISystemConfig.guardian, ()), abi.encode(address(this)));
        vm.mockCall(address(systemConfig), abi.encodeCall(ISystemConfig.paused, ()), abi.encode(false));

        proxyAdmin = new ProxyAdmin(address(this));

        anchorStateRegistry = AnchorStateRegistry(_deployProxy(address(new AnchorStateRegistry(0))));
        factory = DisputeGameFactory(_deployProxy(address(new DisputeGameFactory())));
        delayedWETH = DelayedWETH(payable(_deployProxy(address(new DelayedWETH(DELAYED_WETH_DELAY)))));

        teeVerifier = new MockVerifier(IAnchorStateRegistry(address(anchorStateRegistry)));
        zkVerifier = new MockVerifier(IAnchorStateRegistry(address(anchorStateRegistry)));

        anchorStateRegistry.initialize(
            systemConfig,
            IDisputeGameFactory(address(factory)),
            Proposal({
                root: Hash.wrap(keccak256(abi.encode(currentL2BlockNumber))), l2SequenceNumber: currentL2BlockNumber
            }),
            GameType.wrap(0)
        );
        factory.initialize(address(this));
        delayedWETH.initialize(systemConfig);

        factory.setImplementation(
            GameTypes.AGGREGATE_VERIFIER,
            IDisputeGame(address(_newAggregateVerifier(BLOCK_INTERVAL, INTERMEDIATE_BLOCK_INTERVAL)))
        );
        factory.setInitBond(GameTypes.AGGREGATE_VERIFIER, INIT_BOND);

        anchorStateRegistry.setRespectedGameType(GameTypes.AGGREGATE_VERIFIER);

        // Games created at or before the registry's retirement timestamp are invalid.
        vm.warp(block.timestamp + 1);
    }

    function _deployProxy(address implementation) private returns (address) {
        Proxy proxy = new Proxy(address(proxyAdmin));
        proxyAdmin.upgrade(payable(address(proxy)), implementation);
        return address(proxy);
    }

    function _newAggregateVerifier(
        uint256 blockInterval,
        uint256 intermediateBlockInterval
    )
        internal
        returns (AggregateVerifier)
    {
        return new AggregateVerifier(
            GameTypes.AGGREGATE_VERIFIER,
            IAnchorStateRegistry(address(anchorStateRegistry)),
            IDelayedWETH(payable(address(delayedWETH))),
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            keccak256("tee-nitro-image"),
            keccak256("tee-tdx-image"),
            AggregateVerifier.ZkHashes(keccak256("zk-range"), keccak256("zk-aggregate")),
            keccak256("config"),
            L2_CHAIN_ID,
            blockInterval,
            intermediateBlockInterval
        );
    }

    function _createAggregateVerifierGame(
        address creator,
        Claim rootClaim,
        uint256 l2BlockNumber,
        address parentAddress,
        bytes memory proof
    )
        internal
        returns (AggregateVerifier)
    {
        bytes memory extraData =
            abi.encodePacked(l2BlockNumber, parentAddress, _generateIntermediateRoots(l2BlockNumber, rootClaim));

        vm.deal(creator, INIT_BOND);
        vm.prank(creator);
        return AggregateVerifier(
            address(
                factory.createWithInitData{ value: INIT_BOND }(
                    GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof
                )
            )
        );
    }

    function _provideProof(AggregateVerifier game, address prover, bytes memory proofBytes) internal {
        vm.prank(prover);
        game.verifyProposalProof(proofBytes);
    }

    function _claimCreditAfterDelay(AggregateVerifier game, address recipient) internal {
        uint256 balanceBefore = recipient.balance;
        game.claimCredit();
        vm.warp(block.timestamp + DELAYED_WETH_DELAY);
        game.claimCredit();
        assertEq(recipient.balance, balanceBefore + INIT_BOND);
        assertEq(delayedWETH.balanceOf(address(game)), 0);
    }

    /// @dev Encodes proofType || l1OriginHash || l1OriginNumber || mock verifier payload.
    function _generateProof(
        bytes memory salt,
        AggregateVerifier.ProofType proofType
    )
        internal
        view
        returns (bytes memory)
    {
        uint256 l1OriginNumber = block.number - 1;
        return abi.encodePacked(
            uint8(proofType), blockhash(l1OriginNumber), l1OriginNumber, _generateProofBody(salt, proofType)
        );
    }

    function _generateProposalProof(
        bytes memory salt,
        AggregateVerifier.ProofType proofType
    )
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(uint8(proofType), _generateProofBody(salt, proofType));
    }

    function _generateProofBody(
        bytes memory salt,
        AggregateVerifier.ProofType proofType
    )
        internal
        pure
        returns (bytes memory)
    {
        if (proofType == AggregateVerifier.ProofType.TEE) {
            bytes32 saltHash = keccak256(salt);
            return abi.encodePacked(saltHash, bytes32(0), uint8(27), saltHash, bytes32(uint256(1)), uint8(28));
        }
        return abi.encodePacked(salt, bytes32(0), bytes32(0), uint8(27));
    }

    function _generateIntermediateRoots(uint256 l2BlockNumber, Claim rootClaim) internal pure returns (bytes memory) {
        bytes32[] memory intermediateRoots = new bytes32[](INTERMEDIATE_ROOTS_COUNT);
        uint256 startingL2BlockNumber = l2BlockNumber - BLOCK_INTERVAL;
        for (uint256 i = 1; i < INTERMEDIATE_ROOTS_COUNT; i++) {
            intermediateRoots[i - 1] = keccak256(abi.encode(startingL2BlockNumber + INTERMEDIATE_BLOCK_INTERVAL * i));
        }
        intermediateRoots[LAST_INTERMEDIATE_ROOT_INDEX] = rootClaim.raw();

        return abi.encodePacked(intermediateRoots);
    }
}
