// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

// Optimism
import {
    AlreadyInitialized,
    BondTransferFailed,
    ClaimAlreadyResolved,
    GameNotFinalized,
    GameNotInProgress,
    GameNotResolved,
    GamePaused,
    NoCreditToClaim
} from "src/dispute/lib/Errors.sol";
import { IDelayedWETH } from "interfaces/dispute/IDelayedWETH.sol";
import { IDisputeGame } from "interfaces/dispute/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";
import { Claim, GameStatus, GameType, Hash, Proposal, Timestamp } from "src/dispute/lib/Types.sol";

// Solady
import { Clone } from "@solady/utils/Clone.sol";
import { FixedPointMathLib } from "@solady/utils/FixedPointMathLib.sol";
import { ReentrancyGuard } from "@solady/utils/ReentrancyGuard.sol";

import { IVerifier } from "interfaces/multiproof/IVerifier.sol";

contract AggregateVerifier is Clone, ReentrancyGuard {
    ////////////////////////////////////////////////////////////////
    //                         Enums                              //
    ////////////////////////////////////////////////////////////////

    /// @notice The type of proof. Can be expanded for different types of ZK proofs.
    enum ProofType {
        TEE,
        ZK
    }

    ////////////////////////////////////////////////////////////////
    //                         Constants                          //
    ////////////////////////////////////////////////////////////////
    /// @notice The slow finalization delay.
    uint64 public constant SLOW_FINALIZATION_DELAY = 7 days;

    /// @notice The fast finalization delay.
    uint64 public constant FAST_FINALIZATION_DELAY = 1 days;

    /// @notice The EIP-2935 blockhash history contract address (deployed post-Pectra).
    /// @dev This contract stores blockhashes for the last ~8192 blocks, extending the
    ///      256-block window of the native blockhash() opcode.
    address public constant EIP2935_CONTRACT = 0x0000F90827F1C53a10cb7A02335B175320002935;

    /// @notice The maximum number of blocks that blockhash() can look back.
    uint256 public constant BLOCKHASH_WINDOW = 256;

    /// @notice The maximum number of blocks that EIP-2935 can look back (~8192).
    uint256 public constant EIP2935_WINDOW = 8191;

    ////////////////////////////////////////////////////////////////
    //                         Immutables                         //
    ////////////////////////////////////////////////////////////////
    /// @notice The anchor state registry.
    IAnchorStateRegistry internal immutable ANCHOR_STATE_REGISTRY;

    /// @notice The dispute game factory.
    IDisputeGameFactory public immutable DISPUTE_GAME_FACTORY;

    /// @notice The delayed WETH contract.
    IDelayedWETH public immutable DELAYED_WETH;

    /// @notice The TEE prover.
    IVerifier public immutable TEE_VERIFIER;

    /// @notice The hash of the TEE image.
    bytes32 public immutable TEE_IMAGE_HASH;

    /// @notice The ZK prover.
    IVerifier public immutable ZK_VERIFIER;

    /// @notice The hash of the ZK image.
    bytes32 public immutable ZK_IMAGE_HASH;

    /// @notice The hash of the rollup configuration.
    bytes32 public immutable CONFIG_HASH;

    /// @notice The chain ID of the L2 network this contract argues about.
    uint256 public immutable L2_CHAIN_ID;

    /// @notice The block interval between each proposal.
    /// @dev    The parent's block number + BLOCK_INTERVAL = this proposal's block number.
    uint256 public immutable BLOCK_INTERVAL;

    /// @notice The block interval for intermediate proposals.
    /// @dev    BLOCK_INTERVAL must be divisible by INTERMEDIATE_BLOCK_INTERVAL.
    uint256 public immutable INTERMEDIATE_BLOCK_INTERVAL;

    /// @notice The size of the initialize call data.
    uint256 internal immutable INITIALIZE_CALLDATA_SIZE;

    /// @notice The game type ID.
    GameType internal immutable GAME_TYPE;

    ////////////////////////////////////////////////////////////////
    //                         State Vars                         //
    ////////////////////////////////////////////////////////////////
    /// @notice The starting timestamp of the game.
    Timestamp public createdAt;

    /// @notice The timestamp of the game's global resolution.
    Timestamp public resolvedAt;

    /// @notice The current status of the game.
    GameStatus public status;

    /// @notice Flag for the `initialize` function to prevent re-initialization.
    bool internal initialized;

    /// @notice A boolean for whether or not the game type was respected when the game was created.
    bool public wasRespectedGameTypeWhenCreated;

    /// @notice The starting output root of the game that is proven from in case of a challenge.
    /// @dev This should match the claim root of the parent game.
    Proposal public startingOutputRoot;

    /// @notice The address that can claim the bond.
    address public bondRecipient;

    /// @notice Whether or not the bond has been unlocked.
    bool public bondUnlocked;

    /// @notice Whether or not the bond has been claimed.
    bool public bondClaimed;

    /// @notice The amount of the bond.
    uint256 public bondAmount;

    /// @notice The address of the game that countered this game.
    address public counteredByGameAddress;

    /// @notice The address that provided a proof of the given type.
    mapping(ProofType => address) internal proofTypeToProver;

    /// @notice The timestamp of the game's expected resolution.
    Timestamp public expectedResolution;

    ////////////////////////////////////////////////////////////////
    //                         Events                             //
    ////////////////////////////////////////////////////////////////

    /// @notice Emitted when the game is resolved.
    /// @param status The status of the game.
    event Resolved(GameStatus status);

    /// @notice Emitted when a proposal with a TEE proof is challenged with a ZK proof.
    /// @param challenger The address of the challenger.
    /// @param game The game used to challenge this proposal.
    event Challenged(address indexed challenger, IDisputeGame game);

    /// @notice Emitted when the game is proved.
    /// @param prover The address of the prover.
    /// @param proofType The type of proof.
    event Proved(address indexed prover, ProofType indexed proofType);

    /// @notice Emitted when the game is nullified.
    /// @param nullifier The address of the nullifier.
    /// @param intermediateRootIndex The index of the intermediate root.
    /// @param intermediateRoot The intermediate root.
    event Nullified(address indexed nullifier, uint256 intermediateRootIndex, bytes32 intermediateRoot);

    /// @notice Emitted when the credit is claimed.
    /// @param recipient The address of the recipient.
    /// @param amount The amount of credit claimed.
    event CreditClaimed(address indexed recipient, uint256 amount);

    ////////////////////////////////////////////////////////////////
    //                         Errors                             //
    ////////////////////////////////////////////////////////////////
    /// @notice When the block interval or intermediate block interval is invalid.
    error InvalidBlockInterval(uint256 blockInterval, uint256 intermediateBlockInterval);

    /// @notice When the block number is unexpected.
    error UnexpectedBlockNumber(uint256 expectedBlockNumber, uint256 actualBlockNumber);

    /// @notice When the game is over.
    error GameOver();

    /// @notice When the game is not over.
    error GameNotOver();

    /// @notice When the game is invalid.
    error InvalidGame();

    /// @notice When the parent game is invalid.
    error InvalidParentGame();

    /// @notice When the parent game has not resolved.
    error ParentGameNotResolved();

    /// @notice When there is no proof of the given type.
    error MissingProof(ProofType proofType);

    /// @notice When the proof has already been verified.
    error AlreadyProven(ProofType proofType);

    /// @notice When the proof is invalid.
    error InvalidProof();

    /// @notice When an invalid proof type is provided.
    error InvalidProofType();

    /// @notice When no proof was provided.
    error NoProofProvided();

    /// @notice When the countered by game is invalid.
    error InvalidCounteredByGame();

    /// @notice When the countered by game is not resolved.
    error CounteredByGameNotResolved();

    /// @notice When the bond recipient is empty.
    error BondRecipientEmpty();

    /// @notice When the intermediate root index is invalid.
    error InvalidIntermediateRootIndex();

    /// @notice When the intermediate root is the same as the proposed intermediate root.
    error IntermediateRootSameAsProposed();

    /// @notice When the intermediate root does not match the claim.
    error IntermediateRootMismatch(bytes32 intermediateRoot, bytes32 claim);

    /// @notice Thrown when the L1 origin block is too old to verify.
    error L1OriginTooOld(uint256 l1OriginNumber, uint256 currentBlock);

    /// @notice Thrown when the L1 origin block number is in the future.
    error L1OriginInFuture(uint256 l1OriginNumber, uint256 currentBlock);

    /// @notice Thrown when the L1 origin hash doesn't match the actual blockhash.
    error L1OriginHashMismatch(bytes32 claimed, bytes32 actual);

    /// @param gameType_ The game type.
    /// @param anchorStateRegistry_ The anchor state registry.
    /// @param delayedWETH The delayed WETH contract.
    /// @param teeVerifier The TEE verifier.
    /// @param zkVerifier The ZK verifier.
    /// @param teeImageHash The hash of the TEE image.
    /// @param zkImageHash The hash of the ZK image.
    /// @param configHash The hash of the rollup configuration.
    /// @param l2ChainId The chain ID of the L2 network.
    /// @param blockInterval The block interval.
    constructor(
        GameType gameType_,
        IAnchorStateRegistry anchorStateRegistry_,
        IDelayedWETH delayedWETH,
        IVerifier teeVerifier,
        IVerifier zkVerifier,
        bytes32 teeImageHash,
        bytes32 zkImageHash,
        bytes32 configHash,
        uint256 l2ChainId,
        uint256 blockInterval,
        uint256 intermediateBlockInterval
    ) {
        if (blockInterval == 0 || intermediateBlockInterval == 0 || blockInterval % intermediateBlockInterval != 0) {
            revert InvalidBlockInterval(blockInterval, intermediateBlockInterval);
        }

        // Set up initial game state.
        GAME_TYPE = gameType_;
        ANCHOR_STATE_REGISTRY = anchorStateRegistry_;
        DISPUTE_GAME_FACTORY = ANCHOR_STATE_REGISTRY.disputeGameFactory();
        DELAYED_WETH = delayedWETH;
        TEE_VERIFIER = teeVerifier;
        ZK_VERIFIER = zkVerifier;
        TEE_IMAGE_HASH = teeImageHash;
        ZK_IMAGE_HASH = zkImageHash;
        CONFIG_HASH = configHash;
        L2_CHAIN_ID = l2ChainId;
        BLOCK_INTERVAL = blockInterval;
        INTERMEDIATE_BLOCK_INTERVAL = intermediateBlockInterval;

        INITIALIZE_CALLDATA_SIZE = 0x7E + 0x20 * intermediateOutputRootsCount();
    }

    /// @notice Initializes the contract.
    /// @param proof The proof.
    /// @dev This function may only be called once.
    /// @dev First byte of the proof is the proof type.
    function initialize(bytes calldata proof) external payable virtual {
        // The game must not have already been initialized.
        if (initialized) revert AlreadyInitialized();

        // Revert if the calldata size is not the expected length.
        //
        // This is to prevent adding extra or omitting bytes from to `extraData` that result in a different game UUID
        // in the factory, but are not used by the game, which would allow for multiple dispute games for the same
        // output proposal to be created.
        //
        // Expected length: 0x7E
        // - 0x04 selector
        // - 0x14 creator address
        // - 0x20 root claim
        // - 0x20 l1 head
        // - 0x20 extraData (l2BlockNumber)
        // - 0x04 extraData (parentIndex)
        // - 0x20 x (BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL) extraData (intermediate roots)
        // - 0x02 CWIA bytes

        // - 0x20 proof length location
        // - 0x20 proof length
        // - ((proof.length + 32 - 1)/ 32) * 32 (round up to the nearest 32 bytes)
        uint256 proofLength = (proof.length + 32 - 1) / 32 * 32;
        uint256 expectedCallDataSize = INITIALIZE_CALLDATA_SIZE + 0x40 + proofLength;
        assembly {
            if iszero(eq(calldatasize(), expectedCallDataSize)) {
                // Store the selector for `BadExtraData()` & revert.
                mstore(0x00, 0x9824bdab)
                revert(0x1C, 0x04)
            }
        }

        // Last intermediate root has to match the proposal's claim
        if (intermediateOutputRoot(intermediateOutputRootsCount() - 1) != rootClaim().raw()) {
            revert IntermediateRootMismatch(
                intermediateOutputRoot(intermediateOutputRootsCount() - 1), rootClaim().raw()
            );
        }

        // The first game is initialized with a parent index of uint32.max.
        if (parentIndex() != type(uint32).max) {
            // For subsequent games, get the parent game's information.
            (,, IDisputeGame parentGame) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());

            // Parent game must be respected, not blacklisted, not retired, and not challenged.
            if (!_isValidGame(parentGame)) revert InvalidParentGame();

            startingOutputRoot = Proposal({
                l2SequenceNumber: parentGame.l2SequenceNumber(), root: Hash.wrap(parentGame.rootClaim().raw())
            });
        } else {
            // When there is no parent game, the starting output root is the anchor state for the game type.
            (startingOutputRoot.root, startingOutputRoot.l2SequenceNumber) = ANCHOR_STATE_REGISTRY.getAnchorRoot();
        }

        // The block number must be BLOCK_INTERVAL blocks after the starting block number.
        if (l2SequenceNumber() != startingOutputRoot.l2SequenceNumber + BLOCK_INTERVAL) {
            revert UnexpectedBlockNumber(startingOutputRoot.l2SequenceNumber + BLOCK_INTERVAL, l2SequenceNumber());
        }

        // Set the game as initialized.
        initialized = true;

        // Set the game's starting timestamp.
        createdAt = Timestamp.wrap(uint64(block.timestamp));

        // Set the game as respected if the game type is respected.
        wasRespectedGameTypeWhenCreated =
            GameType.unwrap(ANCHOR_STATE_REGISTRY.respectedGameType()) == GameType.unwrap(GAME_TYPE);

        // Set expected resolution.
        expectedResolution = Timestamp.wrap(type(uint64).max);

        // Verify the proof.
        ProofType proofType = ProofType(uint8(proof[0]));

        bytes32 l1OriginHash = bytes32(proof[1:33]);
        uint256 l1OriginNumber = uint256(bytes32(proof[33:65]));
        // Verify claimed L1 origin hash matches actual blockhash
        _verifyL1Origin(l1OriginHash, l1OriginNumber);

        _verifyProof(
            proof[65:],
            proofType,
            gameCreator(),
            l1OriginHash,
            startingOutputRoot.root.raw(),
            startingOutputRoot.l2SequenceNumber,
            rootClaim().raw(),
            l2SequenceNumber(),
            intermediateOutputRoots()
        );

        _updateProvingData(proofType, gameCreator());

        emit Proved(gameCreator(), proofType);

        // Deposit the bond.
        bondAmount = msg.value;
        DELAYED_WETH.deposit{ value: msg.value }();
    }

    /// @notice Verifies a proof for the current game.
    /// @param proofBytes The proof.
    /// @dev The first byte of the proof is the proof type.
    function verifyProposalProof(bytes calldata proofBytes) external {
        // The game must be in progress.
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        // The game must not be over.
        if (gameOver()) revert GameOver();

        ProofType proofType = ProofType(uint8(proofBytes[0]));
        if (proofTypeToProver[proofType] != address(0)) revert AlreadyProven(proofType);

        _verifyProof(
            proofBytes[1:],
            proofType,
            msg.sender,
            l1Head().raw(),
            startingOutputRoot.root.raw(),
            startingOutputRoot.l2SequenceNumber,
            rootClaim().raw(),
            l2SequenceNumber(),
            intermediateOutputRoots()
        );
        _updateProvingData(proofType, msg.sender);

        emit Proved(msg.sender, proofType);
    }

    /// @notice Resolves the game after a proof has been provided and enough time has passed.
    function resolve() external returns (GameStatus) {
        // The game must be in progress.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        GameStatus parentGameStatus = _getParentGameStatus();
        // The parent game must have resolved.
        if (parentGameStatus == GameStatus.IN_PROGRESS) revert ParentGameNotResolved();

        // If the parent game's claim is invalid, blacklisted, or retired, then the current game's claim is invalid.
        if (parentGameStatus == GameStatus.CHALLENGER_WINS) {
            status = GameStatus.CHALLENGER_WINS;
        } else {
            // Game must be completed with a valid proof.
            if (!gameOver()) revert GameNotOver();
            status = GameStatus.DEFENDER_WINS;
        }

        // Bond is refunded as no challenge was made or parent is invalid.
        bondRecipient = gameCreator();
        // Mark the game as resolved.
        resolvedAt = Timestamp.wrap(uint64(block.timestamp));
        emit Resolved(status);

        return status;
    }

    /// @notice Challenges the TEE proof with a ZK proof.
    /// @param gameIndex The index of the game used to challenge.
    /// @dev The game used to challenge must have a ZK proof for the same
    ///      block number but a different root claim as the current game.
    function challenge(uint256 gameIndex) external {
        // Can only challenge a game that has not been challenged or resolved yet.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        // This game cannot be blacklisted or retired.
        if (!_isValidGame(IDisputeGame(address(this)))) revert InvalidGame();

        // The parent game cannot have been challenged.
        if (_getParentGameStatus() == GameStatus.CHALLENGER_WINS) revert InvalidParentGame();

        // The TEE prover must not be empty.
        // You should nullify the game if a ZK proof has already been provided.
        if (proofTypeToProver[ProofType.TEE] == address(0)) revert MissingProof(ProofType.TEE);
        if (proofTypeToProver[ProofType.ZK] != address(0)) revert AlreadyProven(ProofType.ZK);

        (,, IDisputeGame game) = DISPUTE_GAME_FACTORY.gameAtIndex(gameIndex);

        // The game must be a valid game used to challenge.
        if (!_isValidChallengingGame(game)) revert InvalidGame();

        AggregateVerifier challengingGame = AggregateVerifier(address(game));

        // The ZK prover must not be empty.
        if (challengingGame.zkProver() == address(0)) revert MissingProof(ProofType.ZK);

        // Update the counteredBy address.
        counteredByGameAddress = address(challengingGame);

        // Set the game as challenged.
        status = GameStatus.CHALLENGER_WINS;

        // Set the bond recipient.
        // Bond cannot be claimed until the game used to challenge resolves as DEFENDER_WINS.
        bondRecipient = challengingGame.zkProver();

        // Emit the challenged event.
        emit Challenged(challengingGame.zkProver(), game);
    }

    /// @notice Nullifies the game if a soundness issue is found.
    /// @param proofBytes The proof.
    /// @param intermediateRootIndex Index of the intermediate root to challenge.
    /// @param intermediateRootToProve The intermediate root that the proof claims to be correct.
    /// @dev The first byte of the proof is the proof type.
    function nullify(
        bytes calldata proofBytes,
        uint256 intermediateRootIndex,
        bytes32 intermediateRootToProve
    )
        external
    {
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        if (intermediateRootIndex >= intermediateOutputRootsCount()) revert InvalidIntermediateRootIndex();

        bytes32 proposedIntermediateRoot = intermediateOutputRoot(intermediateRootIndex);
        if (proposedIntermediateRoot == intermediateRootToProve) revert IntermediateRootSameAsProposed();

        ProofType proofType = ProofType(uint8(proofBytes[0]));
        if (proofTypeToProver[proofType] == address(0)) revert MissingProof(proofType);

        bytes32 startingRoot = intermediateRootIndex == 0
            ? startingOutputRoot.root.raw()
            : intermediateOutputRoot(intermediateRootIndex - 1);
        uint256 startingL2SequenceNumber =
            startingOutputRoot.l2SequenceNumber + intermediateRootIndex * INTERMEDIATE_BLOCK_INTERVAL;
        uint256 endingL2SequenceNumber = startingL2SequenceNumber + INTERMEDIATE_BLOCK_INTERVAL;

        _verifyProof(
            proofBytes[1:],
            proofType,
            msg.sender,
            l1Head().raw(),
            startingRoot,
            startingL2SequenceNumber,
            intermediateRootToProve,
            endingL2SequenceNumber,
            abi.encodePacked(intermediateRootToProve)
        );

        // Set the game as challenged so that child games can't resolve.
        status = GameStatus.CHALLENGER_WINS;
        // Refund the bond. This can override a challenge.
        bondRecipient = gameCreator();

        emit Nullified(msg.sender, intermediateRootIndex, intermediateRootToProve);
    }

    /// @notice Claim the credit belonging to the bond recipient. Reverts if the game isn't
    ///         finalized or if the bond transfer fails.
    function claimCredit() external nonReentrant {
        // The bond must not have been claimed yet.
        if (bondClaimed) revert NoCreditToClaim();

        // The bond recipient must not be empty.
        if (bondRecipient == address(0)) revert BondRecipientEmpty();

        // If this game was challenged, the countered by game must be valid or else the bond is refunded.
        if (counteredByGameAddress != address(0)) {
            GameStatus counteredByGameStatus = IDisputeGame(counteredByGameAddress).status();
            if (counteredByGameStatus == GameStatus.IN_PROGRESS) {
                revert CounteredByGameNotResolved();
            }
            // If the countered by game is invalid or not resolved, the bond is refunded.
            if (!_isValidChallengingGame(IDisputeGame(counteredByGameAddress))) {
                bondRecipient = gameCreator();
            }
        }

        if (!bondUnlocked) {
            DELAYED_WETH.unlock(bondRecipient, bondAmount);
            bondUnlocked = true;
            return;
        }

        bondClaimed = true;
        // This can fail if this game was challenged and the countered by game is
        // blacklisted/retired after it resolved to DEFENDER_WINS.
        // The centralized functions in DELAYED_WETH will handle this as it's a already
        // a very centralized action to blacklist/retire a valid challenging game.
        DELAYED_WETH.withdraw(bondRecipient, bondAmount);

        // Transfer the credit to the bond recipient.
        (bool success,) = bondRecipient.call{ value: bondAmount }(hex"");
        if (!success) revert BondTransferFailed();

        // Emit the credit claimed event.
        emit CreditClaimed(bondRecipient, bondAmount);
    }

    /// @notice Closes the game by trying to update the anchor state.
    function closeGame() external {
        // We won't close the game if the system is currently paused.
        if (ANCHOR_STATE_REGISTRY.paused()) {
            revert GamePaused();
        }

        // Make sure that the game is resolved.
        // AnchorStateRegistry should be checking this but we're being defensive here.
        if (resolvedAt.raw() == 0) {
            revert GameNotResolved();
        }

        // Game must be finalized according to the AnchorStateRegistry.
        bool finalized = ANCHOR_STATE_REGISTRY.isGameFinalized(IDisputeGame(address(this)));
        if (!finalized) {
            revert GameNotFinalized();
        }

        // Try to update the anchor game first. Won't always succeed because delays can lead
        // to situations in which this game might not be eligible to be a new anchor game.
        // eip150-safe
        try ANCHOR_STATE_REGISTRY.setAnchorState(IDisputeGame(address(this))) { } catch { }
    }

    /// @notice The starting block number of the game.
    function startingBlockNumber() external view returns (uint256) {
        return startingOutputRoot.l2SequenceNumber;
    }

    /// @notice The starting output root of the game.
    function startingRootHash() external view returns (Hash) {
        return startingOutputRoot.root;
    }

    /// @notice A compliant implementation of this interface should return the components of the
    ///         game UUID's preimage provided in the cwia payload. The preimage of the UUID is
    ///         constructed as `keccak256(gameType . rootClaim . extraData)` where `.` denotes
    ///         concatenation.
    /// @return gameType_ The type of proof system being used.
    /// @return rootClaim_ The root claim of the DisputeGame.
    /// @return extraData_ Any extra data supplied to the dispute game contract by the creator.
    function gameData() external view returns (GameType, Claim, bytes memory) {
        return (GAME_TYPE, rootClaim(), extraData());
    }

    /// @notice Address that provided a TEE proof.
    function teeProver() external view returns (address) {
        return proofTypeToProver[ProofType.TEE];
    }

    /// @notice Address that provided a ZK proof.
    function zkProver() external view returns (address) {
        return proofTypeToProver[ProofType.ZK];
    }

    /// @notice The game type.
    /// @dev For compliance with the IDisputeGame interface.
    function gameType() external view returns (GameType) {
        return GAME_TYPE;
    }

    /// @notice The anchor state registry.
    /// @dev Needed for anchorStateRegistry.isGameRegistered()
    function anchorStateRegistry() external view returns (IAnchorStateRegistry) {
        return ANCHOR_STATE_REGISTRY;
    }

    /// @notice Determines if the game is finished.
    function gameOver() public view returns (bool) {
        return expectedResolution.raw() <= block.timestamp;
    }

    /// @notice The number of intermediate output roots.
    /// @dev At least one as the proposal's root claim is considered an intermediate root.
    function intermediateOutputRootsCount() public view returns (uint256) {
        return (BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL);
    }

    /// @notice The intermediate output roots of the game.
    function intermediateOutputRoots() public view returns (bytes memory) {
        return _getArgBytes(0x78, 0x20 * intermediateOutputRootsCount());
    }

    /// @notice The intermediate output root at the given index.
    /// @param index The index of the intermediate output root.
    function intermediateOutputRoot(uint256 index) public view returns (bytes32) {
        if (index >= intermediateOutputRootsCount()) revert InvalidIntermediateRootIndex();
        return _getArgBytes32(0x78 + 0x20 * index);
    }

    /// @notice Getter for the extra data.
    function extraData() public view returns (bytes memory) {
        // The extra data starts at the second word within the cwia calldata and
        // is 36 + 32 x intermediateRootsCount() bytes long.
        // 32 bytes are for the l2BlockNumber
        // 4 bytes are for the parentIndex
        // 32 bytes are for each intermediate root
        return _getArgBytes(0x54, 0x24 + 0x20 * intermediateOutputRootsCount());
    }

    /// @notice Getter for the creator of the dispute game.
    function gameCreator() public pure returns (address) {
        return _getArgAddress(0x00);
    }

    /// @notice Getter for the root claim.
    function rootClaim() public pure returns (Claim) {
        return Claim.wrap(_getArgBytes32(0x14));
    }

    /// @notice Getter for the parent hash of the L1 block when the dispute game was created.
    function l1Head() public pure returns (Hash) {
        return Hash.wrap(_getArgBytes32(0x34));
    }

    /// @notice The L2 sequence number for which this game is proposing an output root (in this case - the block
    /// number).
    function l2SequenceNumber() public pure returns (uint256) {
        return _getArgUint256(0x54);
    }

    /// @notice The parent index of the game.
    function parentIndex() public pure returns (uint32) {
        return _getArgUint32(0x74);
    }

    /// @notice Updates the expected resolution timestamp.
    function _updateExpectedResolution() internal {
        bool hasTee = proofTypeToProver[ProofType.TEE] != address(0);
        bool hasZk = proofTypeToProver[ProofType.ZK] != address(0);

        if (!hasTee && !hasZk) revert NoProofProvided();

        uint64 delay = (hasTee && hasZk) ? FAST_FINALIZATION_DELAY : SLOW_FINALIZATION_DELAY;
        uint64 newResolution = uint64(block.timestamp) + delay;
        expectedResolution = Timestamp.wrap(uint64(FixedPointMathLib.min(newResolution, expectedResolution.raw())));
    }

    function _updateProvingData(ProofType proofType, address prover) internal {
        proofTypeToProver[proofType] = prover;

        // Bond can be reclaimed after a ZK proof is provided.
        if (proofType == ProofType.ZK) {
            bondRecipient = gameCreator();
        }

        _updateExpectedResolution();
    }

    function _verifyProof(
        bytes calldata proofBytes,
        ProofType proofType,
        address prover,
        bytes32 l1OriginHash,
        bytes32 startingRoot,
        uint256 startingL2SequenceNumber,
        bytes32 endingRoot,
        uint256 endingL2SequenceNumber,
        bytes memory intermediateRoots
    )
        internal
        view
    {
        if (proofBytes.length < 1) revert InvalidProof();

        if (proofType == ProofType.TEE) {
            _verifyTeeProof(
                proofBytes,
                prover,
                l1OriginHash,
                startingRoot,
                startingL2SequenceNumber,
                endingRoot,
                endingL2SequenceNumber,
                intermediateRoots
            );
        } else if (proofType == ProofType.ZK) {
            _verifyZkProof(
                proofBytes,
                prover,
                l1OriginHash,
                startingRoot,
                startingL2SequenceNumber,
                endingRoot,
                endingL2SequenceNumber,
                intermediateRoots
            );
        } else {
            revert InvalidProofType();
        }
    }

    /// @notice Verifies a TEE proof for the current game.
    /// @param proofBytes The proof: prover(20) + signature (65).
    function _verifyTeeProof(
        bytes calldata proofBytes,
        address prover,
        bytes32 l1OriginHash,
        bytes32 startingRoot,
        uint256 startingL2SequenceNumber,
        bytes32 endingRoot,
        uint256 endingL2SequenceNumber,
        bytes memory intermediateRoots
    )
        internal
        view
    {
        bytes32 journal = keccak256(
            abi.encodePacked(
                prover,
                l1OriginHash,
                startingRoot,
                startingL2SequenceNumber,
                endingRoot,
                endingL2SequenceNumber,
                intermediateRoots,
                CONFIG_HASH,
                TEE_IMAGE_HASH
            )
        );

        // Validate the proof.
        bytes memory proof = abi.encodePacked(prover, proofBytes);
        if (!TEE_VERIFIER.verify(proof, TEE_IMAGE_HASH, journal)) revert InvalidProof();
    }

    /// @notice Verifies a ZK proof for the current game.
    /// @param proofBytes The proof: zkProof (variable).
    function _verifyZkProof(
        bytes calldata proofBytes,
        address prover,
        bytes32 l1OriginHash,
        bytes32 startingRoot,
        uint256 startingL2SequenceNumber,
        bytes32 endingRoot,
        uint256 endingL2SequenceNumber,
        bytes memory intermediateRoots
    )
        internal
        view
    {
        bytes32 journal = keccak256(
            abi.encodePacked(
                prover,
                l1OriginHash,
                startingRoot,
                startingL2SequenceNumber,
                endingRoot,
                endingL2SequenceNumber,
                intermediateRoots,
                CONFIG_HASH,
                ZK_IMAGE_HASH
            )
        );

        // Validate the proof.
        if (!ZK_VERIFIER.verify(proofBytes, ZK_IMAGE_HASH, journal)) revert InvalidProof();
    }

    /// @notice Returns the status of the parent game.
    /// @dev If the parent game index is `uint32.max`, then the parent game's status is considered as `DEFENDER_WINS`.
    function _getParentGameStatus() internal view returns (GameStatus) {
        if (parentIndex() != type(uint32).max) {
            (,, IDisputeGame parentGame) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());
            if (ANCHOR_STATE_REGISTRY.isGameBlacklisted(parentGame) || ANCHOR_STATE_REGISTRY.isGameRetired(parentGame))
            {
                return GameStatus.CHALLENGER_WINS;
            }
            return parentGame.status();
        }
        // If this is the first dispute game (i.e. parent game index is `uint32.max`), then the
        // parent game's status is considered as `DEFENDER_WINS`.
        return GameStatus.DEFENDER_WINS;
    }

    /// @notice Checks if the game is respected, not blacklisted, and not retired.
    /// @param game The game to check.
    function _isValidGame(IDisputeGame game) internal view returns (bool) {
        return ANCHOR_STATE_REGISTRY.isGameRespected(game) && !ANCHOR_STATE_REGISTRY.isGameBlacklisted(game)
            && !ANCHOR_STATE_REGISTRY.isGameRetired(game) && (game.status() != GameStatus.CHALLENGER_WINS);
    }

    /// @notice Checks if the game is a valid game used to challenge or nullify.
    /// @param game The game to check.
    function _isValidChallengingGame(IDisputeGame game) internal view returns (bool) {
        return
        // The game type must be the same.
        game.gameType().raw() == GAME_TYPE.raw() && 
            // The parent game must be the same.
            AggregateVerifier(address(game)).parentIndex() == parentIndex() && 
            // The block number must be the same.
            game.l2SequenceNumber() == l2SequenceNumber() && 
            // The root claim must be different.
            game.rootClaim().raw() != rootClaim().raw() && 
            // The game must be valid.
            _isValidGame(game);
    }

    /// @notice Verifies that the claimed L1 origin hash matches the actual blockhash.
    /// @param l1OriginHash The L1 block hash claimed in the proof.
    /// @param l1OriginNumber The L1 block number claimed in the proof.
    function _verifyL1Origin(bytes32 l1OriginHash, uint256 l1OriginNumber) internal view {
        // Check for future block
        if (l1OriginNumber >= block.number) {
            revert L1OriginInFuture(l1OriginNumber, block.number);
        }

        bytes32 actualHash;
        uint256 blockAge = block.number - l1OriginNumber;

        // Prefer blockhash() over EIP-2935 when possible since it's cheaper (no external call).
        if (blockAge <= BLOCKHASH_WINDOW) {
            actualHash = blockhash(l1OriginNumber);
        } else if (blockAge <= EIP2935_WINDOW) {
            // EIP-2935 expects raw calldata: exactly 32 bytes containing the block number.
            // Using a Solidity interface would add a 4-byte function selector, causing a revert.
            // We use a low-level staticcall with raw 32-byte calldata instead.
            (bool success, bytes memory result) = EIP2935_CONTRACT.staticcall(abi.encode(l1OriginNumber));
            if (!success || result.length != 32) {
                revert L1OriginTooOld(l1OriginNumber, block.number);
            }
            actualHash = abi.decode(result, (bytes32));
        } else {
            revert L1OriginTooOld(l1OriginNumber, block.number);
        }

        if (actualHash == bytes32(0)) {
            revert L1OriginTooOld(l1OriginNumber, block.number);
        }

        if (actualHash != l1OriginHash) {
            revert L1OriginHashMismatch(l1OriginHash, actualHash);
        }
    }
}
