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
} from "optimism/src/dispute/lib/Errors.sol";
import {IDelayedWETH} from "optimism/interfaces/dispute/IDelayedWETH.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "optimism/interfaces/dispute/IDisputeGameFactory.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {Claim, GameStatus, GameType, Hash, Proposal, Timestamp} from "optimism/src/dispute/lib/Types.sol";

// Solady
import {Clone} from "solady/utils/Clone.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {ReentrancyGuard} from "solady/utils/ReentrancyGuard.sol";

import {IVerifier} from "./interfaces/IVerifier.sol";

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
    //                         Constants                          //
    ////////////////////////////////////////////////////////////////
    /// @notice The slow finalization delay.
    uint64 public constant SLOW_FINALIZATION_DELAY = 7 days;

    /// @notice The fast finalization delay.
    uint64 public constant FAST_FINALIZATION_DELAY = 1 days;

    /// @notice The size of the initialize call data.
    uint256 internal constant INITIALIZE_CALLDATA_SIZE = 0x7E;

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

    /// @notice The claim made by the proposer.
    ProvingData public provingData;

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

    /// @notice Emitted when the credit is claimed.
    /// @param recipient The address of the recipient.
    /// @param amount The amount of credit claimed.
    event CreditClaimed(address indexed recipient, uint256 amount);

    ////////////////////////////////////////////////////////////////
    //                         Errors                             //
    ////////////////////////////////////////////////////////////////
    /// @notice When the parent game is invalid.
    error InvalidParentGame();

    /// @notice When the block number is unexpected.
    error UnexpectedBlockNumber(uint256 expectedBlockNumber, uint256 actualBlockNumber);

    /// @notice When the game is over.
    error GameOver();

    /// @notice When the game is not over.
    error GameNotOver();

    /// @notice When the parent game has not resolved.
    error ParentGameNotResolved();

    /// @notice When there is no TEE proof.
    error MissingTEEProof();

    /// @notice When there is no ZK proof.
    error MissingZKProof();

    /// @notice When the game is invalid.
    error InvalidGame();

    /// @notice When the proof has already been verified.
    error AlreadyProven();

    /// @notice When the proof is invalid.
    error InvalidProof();

    /// @notice When no proof was provided.
    error NoProofProvided();

    /// @notice When an invalid proof type is provided.
    error InvalidProofType();

    /// @notice When the bond recipient is empty.
    error BondRecipientEmpty();

    /// @notice When the countered by game is invalid.
    error InvalidCounteredByGame();

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
        uint256 blockInterval
    ) {
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
        // - 0x02 CWIA bytes

        // - 0x20 proof length location
        // - 0x20 proof length
        // - ((proof.length + 32 - 1)/ 32) * 32 (round up to the nearest 32 bytes)
        uint256 proofLength = (proof.length + 32 - 1) / 32 * 32;
        assembly {
            if iszero(eq(calldatasize(), add(INITIALIZE_CALLDATA_SIZE, add(0x40, proofLength)))) {
                // Store the selector for `BadExtraData()` & revert.
                mstore(0x00, 0x9824bdab)
                revert(0x1C, 0x04)
            }
        }

        // The first game is initialized with a parent index of uint32.max.
        if (parentIndex() != type(uint32).max) {
            // For subsequent games, get the parent game's information.
            (,, IDisputeGame parentGame) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());

            // Parent game must be respected, not blacklisted, and not retired.
            if (!_isValidGame(parentGame)) revert InvalidParentGame();

            // The parent game must be a valid game.
            if (parentGame.status() == GameStatus.CHALLENGER_WINS) revert InvalidParentGame();

            // The parent game must have a proof.
            // Should not be reachable since a proof is required to initialize.
            if (
                AggregateVerifier(address(parentGame)).teeProver() == address(0)
                    && AggregateVerifier(address(parentGame)).zkProver() == address(0)
            ) revert InvalidParentGame();

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
        provingData.expectedResolution = Timestamp.wrap(type(uint64).max);

        // Deposit the bond.
        bondAmount = msg.value;
        DELAYED_WETH.deposit{value: msg.value}();

        // Verify the proof.
        _verifyProof(proof[1:], ProofType(uint8(proof[0])), gameCreator());
    }

    /// @notice Verifies a proof for the current game.
    /// @param proofBytes The proof.
    /// @dev The first byte of the proof is the proof type.
    function verifyProof(bytes calldata proofBytes) external {
        // The game must not be over.
        if (gameOver()) revert GameOver();

        _verifyProof(proofBytes[1:], ProofType(uint8(proofBytes[0])), msg.sender);
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
        if (provingData.teeProver == address(0)) revert MissingTEEProof();
        if (provingData.zkProver != address(0)) revert AlreadyProven();

        (,, IDisputeGame game) = DISPUTE_GAME_FACTORY.gameAtIndex(gameIndex);

        // The game must be a valid game used to challenge.
        if (!_isValidChallengingGame(game)) revert InvalidGame();

        AggregateVerifier challengingGame = AggregateVerifier(address(game));

        // The ZK prover must not be empty.
        if (challengingGame.zkProver() == address(0)) revert MissingZKProof();

        // Update the counteredBy address.
        provingData.counteredByGameAddress = address(challengingGame);

        // Set the game as challenged.
        status = GameStatus.CHALLENGER_WINS;

        // Set the bond recipient.
        // Bond cannot be claimed until the game used to challenge resolves as DEFENDER_WINS.
        bondRecipient = challengingGame.zkProver();

        // Emit the challenged event.
        emit Challenged(challengingGame.zkProver(), game);
    }

    /// @notice Nullifies the game if a soundness issue is found.
    /// @param gameIndex The index of the game used to nullify.
    /// @param proofType The type of proof used to nullify.
    /// @dev The game used to nullify must have a proof for the same
    ///      block number but a different root claim as the current game.
    function nullify(uint256 gameIndex, ProofType proofType) external {
        // Can only nullify a game that has not resolved yet.
        // We can nullify a challenged game in case of a soundness issue.
        if (status == GameStatus.DEFENDER_WINS) revert ClaimAlreadyResolved();

        (,, IDisputeGame game) = DISPUTE_GAME_FACTORY.gameAtIndex(gameIndex);

        // Can only nullify a game that has a proof of the same type.
        if (proofType == ProofType.TEE) {
            if (provingData.teeProver == address(0) || AggregateVerifier(address(game)).teeProver() == address(0)) {
                revert MissingTEEProof();
            }
        } else if (proofType == ProofType.ZK) {
            if (provingData.zkProver == address(0) || AggregateVerifier(address(game)).zkProver() == address(0)) {
                revert MissingZKProof();
            }
        } else {
            revert InvalidProofType();
        }

        // The game must be a valid game used to nullify.
        if (!_isValidChallengingGame(game)) revert InvalidGame();

        // Set the game as challenged so that child games can't resolve.
        status = GameStatus.CHALLENGER_WINS;
        // Refund the bond. This can override a challenge.
        bondRecipient = gameCreator();
        // To allow bond to be refunded as the challenging game is no longer valid.
        delete provingData.counteredByGameAddress;

        emit Nullified(msg.sender, game);
    }

    /// @notice Claim the credit belonging to the bond recipient. Reverts if the game isn't
    ///         finalized or if the bond transfer fails.
    function claimCredit() external nonReentrant {
        // The bond must not have been claimed yet.
        if (bondClaimed) revert NoCreditToClaim();

        // The bond recipient must not be empty.
        if (bondRecipient == address(0)) revert BondRecipientEmpty();

        // If this game was challenged, the countered by game must be valid.
        if (provingData.counteredByGameAddress != address(0)) {
            if (!_isValidChallengingGame(IDisputeGame(provingData.counteredByGameAddress))) {
                revert InvalidCounteredByGame();
            }
            if (IDisputeGame(provingData.counteredByGameAddress).status() != GameStatus.DEFENDER_WINS) {
                revert InvalidCounteredByGame();
            }
        }

        if (!bondUnlocked) {
            DELAYED_WETH.unlock(bondRecipient, bondAmount);
            bondUnlocked = true;
            return;
        }

        bondClaimed = true;
        DELAYED_WETH.withdraw(bondRecipient, bondAmount);

        // Transfer the credit to the bond recipient.
        (bool success,) = bondRecipient.call{value: bondAmount}(hex"");
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
        try ANCHOR_STATE_REGISTRY.setAnchorState(IDisputeGame(address(this))) {} catch {}
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
        return provingData.teeProver;
    }

    /// @notice Address that provided a ZK proof.
    function zkProver() external view returns (address) {
        return provingData.zkProver;
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
        return provingData.expectedResolution.raw() <= block.timestamp;
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

    /// @notice Getter for the extra data.
    function extraData() public pure returns (bytes memory) {
        // The extra data starts at the second word within the cwia calldata and
        // is 36 bytes long.
        // 32 bytes are for the l2BlockNumber
        // 4 bytes are for the parentIndex
        return _getArgBytes(0x54, 0x24);
    }

    /// @notice The L2 sequence number for which this game is proposing an output root (in this case - the block number).
    function l2SequenceNumber() public pure returns (uint256) {
        return _getArgUint256(0x54);
    }

    /// @notice The parent index of the game.
    function parentIndex() public pure returns (uint32) {
        return _getArgUint32(0x74);
    }

    function _verifyProof(bytes calldata proofBytes, ProofType proofType, address prover) internal {
        if (proofType == ProofType.TEE) {
            _verifyTeeProof(proofBytes, prover);
        } else if (proofType == ProofType.ZK) {
            _verifyZkProof(proofBytes, prover);

            // Bond can be reclaimed after a ZK proof is provided.
            bondRecipient = gameCreator();
        } else {
            revert InvalidProofType();
        }

        _updateExpectedResolution();

        // Emit the proved event.
        emit Proved(prover, proofType);
    }

    /// @notice Verifies a TEE proof for the current game.
    /// @param proofBytes The proof: prover(20) + l1OriginHash (32) + l1OriginNumber (32) + signature (65).
    function _verifyTeeProof(bytes calldata proofBytes, address prover) internal {
        // Only one TEE proof can be submitted.
        if (provingData.teeProver != address(0)) revert AlreadyProven();

        // The game must be in progress.
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        bytes32 journal = keccak256(
            abi.encodePacked(
                prover,
                bytes32(proofBytes[0:32]),
                uint256(bytes32(proofBytes[32:64])),
                startingOutputRoot.root,
                startingOutputRoot.l2SequenceNumber,
                rootClaim(),
                l2SequenceNumber(),
                CONFIG_HASH,
                TEE_IMAGE_HASH
            )
        );

        // Validate the proof.
        bytes memory proof = abi.encodePacked(prover, proofBytes);
        if (!TEE_VERIFIER.verify(proof, TEE_IMAGE_HASH, journal)) revert InvalidProof();

        // Update proving data.
        provingData.teeProver = prover;
    }

    /// @notice Verifies a ZK proof for the current game.
    /// @param proofBytes The proof: l1OriginHash (32) + l1OriginNumber (32) + zkProof (variable).
    function _verifyZkProof(bytes calldata proofBytes, address prover) internal {
        // Only one ZK proof can be submitted.
        if (provingData.zkProver != address(0)) revert AlreadyProven();

        // The game must be in progress or challenged (to allow nullification).
        if (status == GameStatus.DEFENDER_WINS) revert ClaimAlreadyResolved();

        bytes32 journal = keccak256(
            abi.encodePacked(
                prover,
                bytes32(proofBytes[0:32]),
                uint256(bytes32(proofBytes[32:64])),
                startingOutputRoot.root,
                startingOutputRoot.l2SequenceNumber,
                rootClaim(),
                l2SequenceNumber(),
                CONFIG_HASH,
                ZK_IMAGE_HASH
            )
        );

        // Validate the proof.
        if (!ZK_VERIFIER.verify(proofBytes, ZK_IMAGE_HASH, journal)) revert InvalidProof();

        // Update proving data.
        provingData.zkProver = prover;
    }

    /// @notice Updates the expected resolution timestamp.
    function _updateExpectedResolution() internal {
        uint64 newResolution = uint64(block.timestamp);
        if (provingData.teeProver != address(0) && provingData.zkProver != address(0)) {
            newResolution += FAST_FINALIZATION_DELAY;
        } else if (provingData.teeProver != address(0) || provingData.zkProver != address(0)) {
            newResolution += SLOW_FINALIZATION_DELAY;
        } else {
            revert NoProofProvided();
        }
        provingData.expectedResolution =
            Timestamp.wrap(uint64(FixedPointMathLib.min(newResolution, provingData.expectedResolution.raw())));
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
            && !ANCHOR_STATE_REGISTRY.isGameRetired(game);
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
}
