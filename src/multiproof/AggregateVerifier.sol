// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;
// Libraries
import { Clone } from "solady/utils/Clone.sol";
import {
    Claim,
    GameType,
    Hash,
    Proposal,
    Timestamp
} from "optimism/src/dispute/lib/Types.sol";
import "./Errors.sol";

// Interfaces
import { GameStatus, IDisputeGame, IDisputeGameFactory } from "optimism/src/dispute/AnchorStateRegistry.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {IVerifier} from "./interfaces/IVerifier.sol";

import { FixedPointMathLib } from "solady/utils/FixedPointMathLib.sol";
import { ReentrancyGuard } from "solady/utils/ReentrancyGuard.sol";

contract AggregateVerifier is Clone, ReentrancyGuard, IDisputeGame {
    ////////////////////////////////////////////////////////////////
    //                         Enums                              //
    ////////////////////////////////////////////////////////////////
    
    /// @notice The type of proof. Can be expanded for different types of ZK proofs.
    enum ProofType {
        TEE,
        ZK
    }

    ////////////////////////////////////////////////////////////////
    //                         Structs                            //
    ////////////////////////////////////////////////////////////////

    /// @notice The `ProvingData` struct represents the data associated with the proofs for a claim.
    /// @param counteredByGameAddress The address of the game that countered this game.
    /// @param teeProver The address that provided a TEE proof.
    /// @param zkProver The address that provided a ZK proof.
    /// @param expectedResolution The timestamp of the game's expected resolution.
    struct ProvingData {
        address counteredByGameAddress;
        address teeProver;
        address zkProver;
        Timestamp expectedResolution;
    }

    ////////////////////////////////////////////////////////////////
    //                         Events                             //
    ////////////////////////////////////////////////////////////////

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
    /// @param game The game used to nullify this proposal.
    event Nullified(address indexed nullifier, IDisputeGame game);

    ////////////////////////////////////////////////////////////////
    //                         State Vars                         //
    ////////////////////////////////////////////////////////////////
    /// @notice The slow finalization delay.
    uint64 public constant SLOW_FINALIZATION_DELAY = 7 days;

    /// @notice The fast finalization delay.
    uint64 public constant FAST_FINALIZATION_DELAY = 1 days;

    /// @notice The size of the initialize call data.
    uint256 private constant INITIALIZE_CALLDATA_SIZE = 0x7E;

    /// @notice The anchor state registry.
    IAnchorStateRegistry internal immutable ANCHOR_STATE_REGISTRY;

    /// @notice The dispute game factory.
    IDisputeGameFactory public immutable DISPUTE_GAME_FACTORY;

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

    /// @notice The address that can submit a TEE proof.
    address public immutable TEE_PROPOSER;

    /// @notice The game type ID.
    GameType internal immutable GAME_TYPE;

    /// @notice The chain ID of the L2 network this contract argues about.
    uint256 internal immutable L2_CHAIN_ID;

    /// @notice The block interval between each proposal. 
    /// @dev    The parent's block number + BLOCK_INTERVAL = this proposal's block number.
    uint256 internal immutable BLOCK_INTERVAL;

    /// @notice The starting timestamp of the game.
    Timestamp public createdAt;

    /// @notice The timestamp of the game's global resolution.
    Timestamp public resolvedAt;

    /// @notice The current status of the game.
    GameStatus public status;

    /// @notice Flag for the `initialize` function to prevent re-initialization.
    bool internal initialized;

    /// @notice The claim made by the proposer.
    ProvingData public provingData;

    /// @notice The starting output root of the game that is proven from in case of a challenge.
    /// @dev This should match the claim root of the parent game.
    Proposal public startingOutputRoot;

    /// @notice A boolean for whether or not the game type was respected when the game was created.
    bool public wasRespectedGameTypeWhenCreated;

    address public bondRecipient;

    /// @param _gameType The game type.
    /// @param _anchorStateRegistry The anchor state registry.
    /// @param _teeVerifier The TEE verifier.
    /// @param _zkVerifier The ZK verifier.
    /// @param _teeImageHash The hash of the TEE image.
    /// @param _zkImageHash The hash of the ZK image.
    /// @param _configHash The hash of the rollup configuration.
    /// @param _teeProposer The address that can submit a TEE proof.
    /// @param _l2ChainId The chain ID of the L2 network.
    /// @param _blockInterval The block interval.
    constructor(
        GameType _gameType,
        IAnchorStateRegistry _anchorStateRegistry,
        IVerifier _teeVerifier,
        IVerifier _zkVerifier,
        bytes32 _teeImageHash,
        bytes32 _zkImageHash,
        bytes32 _configHash,
        address _teeProposer,
        uint256 _l2ChainId,
        uint256 _blockInterval
    ) {
        // Set up initial game state.
        GAME_TYPE = _gameType;
        ANCHOR_STATE_REGISTRY = _anchorStateRegistry;
        DISPUTE_GAME_FACTORY = ANCHOR_STATE_REGISTRY.disputeGameFactory();
        TEE_VERIFIER = _teeVerifier;
        ZK_VERIFIER = _zkVerifier;
        TEE_IMAGE_HASH = _teeImageHash;
        ZK_IMAGE_HASH = _zkImageHash;
        CONFIG_HASH = _configHash;
        TEE_PROPOSER = _teeProposer;
        L2_CHAIN_ID = _l2ChainId;
        BLOCK_INTERVAL = _blockInterval;
    }

    /// @notice Initializes the contract.
    /// @dev This function may only be called once.
    function initialize() external payable virtual {
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
        // - 0x02 CWIA bytes
        assembly {
            if iszero(eq(calldatasize(), INITIALIZE_CALLDATA_SIZE)) {
                // Store the selector for `BadExtraData()` & revert
                mstore(0x00, 0x9824bdab)
                revert(0x1C, 0x04)
            }
        }

        // The first game is initialized with a parent index of uint32.max
        if (parentIndex() != type(uint32).max) {
            // For subsequent games, get the parent game's information
            (,, IDisputeGame parentGame) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());

            // Parent game must be respected, not blacklisted, and not retired.
            if (
                !ANCHOR_STATE_REGISTRY.isGameRespected(parentGame) || ANCHOR_STATE_REGISTRY.isGameBlacklisted(parentGame)
                    || ANCHOR_STATE_REGISTRY.isGameRetired(parentGame)
            ) {
                revert InvalidParentGame();
            }

            // The parent game must be a valid game.
            if (parentGame.status() == GameStatus.CHALLENGER_WINS) revert InvalidParentGame();

            // The parent game must have a proof.
            if (AggregateVerifier(address(parentGame)).teeProver() == address(0) && AggregateVerifier(address(parentGame)).zkProver() == address(0)) revert InvalidParentGame();

            startingOutputRoot = Proposal({
                l2SequenceNumber: parentGame.l2SequenceNumber(),
                root: Hash.wrap(parentGame.rootClaim().raw())
            });
        } else {
            // When there is no parent game, the starting output root is the anchor state for the game type.
            (startingOutputRoot.root, startingOutputRoot.l2SequenceNumber) =
                ANCHOR_STATE_REGISTRY.getAnchorRoot();
        }

        // The block number must be BLOCK_INTERVAL blocks after the starting block number.
        if (l2SequenceNumber() != startingOutputRoot.l2SequenceNumber + BLOCK_INTERVAL) {
            revert UnexpectedBlockNumber(startingOutputRoot.l2SequenceNumber + BLOCK_INTERVAL, l2SequenceNumber());
        }

        // Set the game as initialized.
        initialized = true;

        // Set the game's starting timestamp
        createdAt = Timestamp.wrap(uint64(block.timestamp));

        // Game cannot resolve without a proof
        provingData.expectedResolution = Timestamp.wrap(type(uint64).max);

        wasRespectedGameTypeWhenCreated =
            GameType.unwrap(ANCHOR_STATE_REGISTRY.respectedGameType()) == GameType.unwrap(GAME_TYPE);
    }

    /// @notice The L2 block number for which this game is proposing an output root.
    function l2BlockNumber() public pure returns (uint256 l2BlockNumber_) {
        l2BlockNumber_ = _getArgUint256(0x54);
    }

    function l2SequenceNumber() public pure returns (uint256 l2SequenceNumber_) {
        l2SequenceNumber_ = l2BlockNumber();
    }

    /// @notice The parent index of the game.
    function parentIndex() public pure returns (uint32 parentIndex_) {
        parentIndex_ = _getArgUint32(0x74);
    }

    /// @notice The starting block number of the game.
    function startingBlockNumber() external view returns (uint256 startingBlockNumber_) {
        startingBlockNumber_ = startingOutputRoot.l2SequenceNumber;
    }

    /// @notice The starting output root of the game.
    function startingRootHash() external view returns (Hash startingRootHash_) {
        startingRootHash_ = startingOutputRoot.root;
    }

    ////////////////////////////////////////////////////////////////
    //                    Proving Methods                         //
    ////////////////////////////////////////////////////////////////

    function verifyProof(bytes calldata proofBytes, ProofType proofType) external {
        // The game must not be over.
        if (gameOver()) revert GameOver();

        if (proofType == ProofType.TEE) {
            if (msg.sender != TEE_PROPOSER) revert NotAuthorized();
            _verifyTeeProof(proofBytes);
        } else if (proofType == ProofType.ZK) {
            _verifyZkProof(proofBytes);

            // Bond can be reclaimed after a ZK proof is provided.
            bondRecipient = gameCreator();
        }
        else {
            revert InvalidProofType();
        }

        _updateExpectedResolution();

        // Emit the proved event.
        emit Proved(msg.sender, proofType);
    }

    /// @notice Verifies a TEE proof for the current game.
    /// @param proofBytes The proof bytes.
    function _verifyTeeProof(bytes calldata proofBytes) internal {
        // Only one TEE proof can be submitted.
        if (provingData.teeProver != address(0)) revert AlreadyProven();

        // The game must be in progress.
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        bytes32 journal = keccak256(abi.encodePacked(
            msg.sender, 
            l1Head(),
            startingOutputRoot.root,
            startingOutputRoot.l2SequenceNumber,
            rootClaim(),
            l2SequenceNumber(),
            CONFIG_HASH,
            TEE_IMAGE_HASH
        ));

        // Validate the proof.
        if (!TEE_VERIFIER.verify(proofBytes, journal)) revert InvalidProof();

        // Update proving data.
        provingData.teeProver = msg.sender;
    }

    /// @notice Verifies a ZK proof for the current game.
    /// @param proofBytes The proof bytes.
    function _verifyZkProof(bytes calldata proofBytes) internal {
        // Only one ZK proof can be submitted.
        if (provingData.zkProver != address(0)) revert AlreadyProven();

        // The game must be in progress or challenged (to allow nullification).
        if (status == GameStatus.DEFENDER_WINS) revert ClaimAlreadyResolved();

        bytes32 journal = keccak256(abi.encodePacked(
            msg.sender, 
            l1Head(),
            startingOutputRoot.root,
            startingOutputRoot.l2SequenceNumber,
            rootClaim(),
            l2SequenceNumber(),
            CONFIG_HASH,
            ZK_IMAGE_HASH
        ));

        // Validate the proof.
        if (!ZK_VERIFIER.verify(proofBytes, journal)) revert InvalidProof();

        // Update proving data.
        provingData.zkProver = msg.sender;
    }

    /// @notice Resolves the game after enough time has passed.
    function resolve() external returns (GameStatus) {
        // The game must be in progress.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        GameStatus parentGameStatus = getParentGameStatus();
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
        if (ANCHOR_STATE_REGISTRY.isGameBlacklisted(IDisputeGame(address(this))) || ANCHOR_STATE_REGISTRY.isGameRetired(IDisputeGame(address(this)))) revert InvalidGame();

        // The parent game cannot have been challenged
        if (getParentGameStatus() == GameStatus.CHALLENGER_WINS) revert InvalidParentGame();
        
        // The TEE prover must not be empty. You should nullify the game if you want to challenge.
        if (provingData.teeProver == address(0)) revert MissingTEEProof();
        if (provingData.zkProver != address(0)) revert AlreadyProven();
        
        (,, IDisputeGame game) = DISPUTE_GAME_FACTORY.gameAtIndex(gameIndex);

        AggregateVerifier challengingGame = AggregateVerifier(address(game));
        // The parent index must be the same.
        if (challengingGame.parentIndex() != parentIndex()) revert IncorrectParentIndex();

        // The block number must be the same.
        if (challengingGame.l2SequenceNumber() != l2SequenceNumber()) revert IncorrectBlockNumber();

        // The root claim must be different.
        // Not actually reachable as the factory prevents the same proposal from being created.
        if (challengingGame.rootClaim().raw() == rootClaim().raw()) revert IncorrectRootClaim();

        // The ZK prover must not be empty.
        if (challengingGame.zkProver() == address(0)) revert MissingZKProof();

        // The game must be respected, not blacklisted, and not retired.
        if (!ANCHOR_STATE_REGISTRY.isGameRespected(game) || ANCHOR_STATE_REGISTRY.isGameBlacklisted(game) || ANCHOR_STATE_REGISTRY.isGameRetired(game)) {
            revert InvalidGame();
        }   

        // Update the counteredBy address
        provingData.counteredByGameAddress = address(challengingGame);

        // Set the game as challenged.
        status = GameStatus.CHALLENGER_WINS;

        // Set the bond recipient.
        // Bond cannot be claimed until the game used to challenge resolves as DEFENDER_WINS.
        bondRecipient = challengingGame.zkProver();

        // Emit the challenged event
        emit Challenged(challengingGame.zkProver(), game);
    }

    /// @notice Nullifies the game if a soundness issue is found.
    /// @param gameIndex The index of the game used to nullify.
    /// @param proofType The type of proof used to nullify.
    /// @dev The game used to nullify must have a proof for the same
    ///      block number but a different root claim as the current game.
    function nullify(uint256 gameIndex, ProofType proofType) external {
        // Can only nullify a game that has not resolved yet.
        // We can nullify a challenged game in case of a soundness issue
        if (status == GameStatus.DEFENDER_WINS) revert ClaimAlreadyResolved();

        (,, IDisputeGame game) = DISPUTE_GAME_FACTORY.gameAtIndex(gameIndex);

        // Can only nullify a game that has a proof of the same type.
        if (proofType == ProofType.TEE) {
            if (provingData.teeProver == address(0) || AggregateVerifier(address(game)).teeProver() == address(0)) revert MissingTEEProof();
        } else if (proofType == ProofType.ZK) {
            if (provingData.zkProver == address(0) || AggregateVerifier(address(game)).zkProver() == address(0)) revert MissingZKProof();
        }
        else {
            revert InvalidProofType();
        }

        // The parent index must be the same.
        if (AggregateVerifier(address(game)).parentIndex() != parentIndex()) revert IncorrectParentIndex();

        // The block number must be the same.
        if (game.l2SequenceNumber() != l2SequenceNumber()) revert IncorrectBlockNumber();

        // The root claim must be different.
        // Not actually reachable as the factory prevents the same proposal from being created.
        if (game.rootClaim().raw() == rootClaim().raw()) revert IncorrectRootClaim();

        // The game must be respected, not blacklisted, and not retired.
        if (!ANCHOR_STATE_REGISTRY.isGameRespected(game) || ANCHOR_STATE_REGISTRY.isGameBlacklisted(game) || ANCHOR_STATE_REGISTRY.isGameRetired(game)) {
            revert InvalidGame();
        }

        // Set the game as challenged so that child games can't resolve.
        status = GameStatus.CHALLENGER_WINS;
        // Refund the bond. This can override a challenge.
        bondRecipient = gameCreator();
        // To allow bond to be claimed in case challenging game is nullified
        delete provingData.counteredByGameAddress;

        emit Nullified(msg.sender, game);
    }

    /// @notice Claim the credit belonging to the bond recipient. Reverts if the game isn't
    ///         finalized or if the bond transfer fails. 
    function claimCredit() nonReentrant external {
        // The bond recipient must not be empty.
        if (bondRecipient == address(0)) revert BondRecipientEmpty();

        // If this game was challenged, the countered by game must be valid.
        if (provingData.counteredByGameAddress != address(0)) {
            if (IDisputeGame(provingData.counteredByGameAddress).status() != GameStatus.DEFENDER_WINS) revert InvalidCounteredByGame();
        }

        // The game must have credit to claim.
        if (address(this).balance == 0) revert NoCreditToClaim();

        // Transfer the credit to the bond recipient.
        (bool success,) = bondRecipient.call{value: address(this).balance}(hex"");
        if (!success) revert BondTransferFailed();
    }

    function closeGame() public {

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

    /// @notice Returns the status of the parent game.
    /// @dev If the parent game index is `uint32.max`, then the parent game's status is considered as `DEFENDER_WINS`.
    function getParentGameStatus() private view returns (GameStatus) {
        if (parentIndex() != type(uint32).max) {
            (,, IDisputeGame parentGame) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());
            if (ANCHOR_STATE_REGISTRY.isGameBlacklisted(parentGame) || ANCHOR_STATE_REGISTRY.isGameRetired(parentGame)) {
                return GameStatus.CHALLENGER_WINS;
            }
            return parentGame.status();
        } else {
            // If this is the first dispute game (i.e. parent game index is `uint32.max`), then the
            // parent game's status is considered as `DEFENDER_WINS`.
            return GameStatus.DEFENDER_WINS;
        }
    }

    /// @notice Determines if the game is finished.
    function gameOver() public view returns (bool) {
        return provingData.expectedResolution.raw() <= block.timestamp;
    }

    function _updateExpectedResolution() internal {
        uint64 newResolution = uint64(block.timestamp);
        if (provingData.teeProver != address(0) && provingData.zkProver != address(0)) {
            newResolution += FAST_FINALIZATION_DELAY;
        } else if (provingData.teeProver != address(0) || provingData.zkProver != address(0)) {
            newResolution += SLOW_FINALIZATION_DELAY;
        } else {
            revert NoProofProvided();
        }
        provingData.expectedResolution = Timestamp.wrap(uint64(FixedPointMathLib.min(newResolution, provingData.expectedResolution.raw())));
    }

    /// @notice Getter for the game type.
    /// @dev The reference impl should be entirely different depending on the type (fault, validity)
    ///      i.e. The game type should indicate the security model.
    /// @return gameType_ The type of proof system being used.
    function gameType() public view returns (GameType gameType_) {
        gameType_ = GAME_TYPE;
    }

    /// @notice Getter for the creator of the dispute game.
    /// @dev `clones-with-immutable-args` argument #1
    /// @return creator_ The creator of the dispute game.
    function gameCreator() public pure returns (address creator_) {
        creator_ = _getArgAddress(0x00);
    }

    /// @notice Getter for the root claim.
    /// @dev `clones-with-immutable-args` argument #2
    /// @return rootClaim_ The root claim of the DisputeGame.
    function rootClaim() public pure returns (Claim rootClaim_) {
        rootClaim_ = Claim.wrap(_getArgBytes32(0x14));
    }

    /// @notice Getter for the parent hash of the L1 block when the dispute game was created.
    /// @dev `clones-with-immutable-args` argument #3
    /// @return l1Head_ The parent hash of the L1 block when the dispute game was created.
    function l1Head() public pure returns (Hash l1Head_) {
        l1Head_ = Hash.wrap(_getArgBytes32(0x34));
    }

    /// @notice Getter for the extra data.
    /// @dev `clones-with-immutable-args` argument #4
    /// @return extraData_ Any extra data supplied to the dispute game contract by the creator.
    function extraData() public pure returns (bytes memory extraData_) {
        // The extra data starts at the second word within the cwia calldata and
        // is 36 bytes long. 
        // 32 bytes are for the l2BlockNumber
        // 4 bytes are for the parentIndex
        extraData_ = _getArgBytes(0x54, 0x24);
    }

    /// @notice A compliant implementation of this interface should return the components of the
    ///         game UUID's preimage provided in the cwia payload. The preimage of the UUID is
    ///         constructed as `keccak256(gameType . rootClaim . extraData)` where `.` denotes
    ///         concatenation.
    /// @return gameType_ The type of proof system being used.
    /// @return rootClaim_ The root claim of the DisputeGame.
    /// @return extraData_ Any extra data supplied to the dispute game contract by the creator.
    function gameData() external view returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_) {
        gameType_ = gameType();
        rootClaim_ = rootClaim();
        extraData_ = extraData();
    }

    function teeProver() external view returns (address teeProver_) {
        teeProver_ = provingData.teeProver;
    }

    function zkProver() external view returns (address zkProver_) {
        zkProver_ = provingData.zkProver;
    }
    
    ////////////////////////////////////////////////////////////////
    //                     IMMUTABLE GETTERS                      //
    ////////////////////////////////////////////////////////////////

    function l2ChainId() external view returns (uint256 l2ChainId_) {
        l2ChainId_ = L2_CHAIN_ID;
    }

    function blockInterval() external view returns (uint256 blockInterval_) {
        blockInterval_ = BLOCK_INTERVAL;
    }

    function anchorStateRegistry() external view returns (IAnchorStateRegistry anchorStateRegistry_) {
        anchorStateRegistry_ = ANCHOR_STATE_REGISTRY;
    }
}
