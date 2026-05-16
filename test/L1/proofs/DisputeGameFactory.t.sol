// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";

// Scripts
import { ForgeArtifacts, StorageSlot } from "scripts/libraries/ForgeArtifacts.sol";

// Libraries
import { Claim, Timestamp } from "src/libraries/bridge/LibUDT.sol";
import "src/libraries/bridge/Types.sol";
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";

// Interfaces
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";

// Mocks
import { MockVerifier } from "test/mocks/MockVerifier.sol";

/// @notice A fake clone used for testing the `DisputeGameFactory` contract's `create` function.
contract DisputeGameFactory_FakeClone_Harness {
    function initialize() external payable {
        // noop
    }

    function extraData() external pure returns (bytes memory) {
        return hex"FF0420";
    }

    function parentHash() external pure returns (bytes32) {
        return bytes32(0);
    }

    function rootClaim() external pure returns (Claim) {
        return Claim.wrap(bytes32(0));
    }
}

/// @title DisputeGameFactory_TestInit
/// @notice Reusable test initialization for `DisputeGameFactory` tests.
abstract contract DisputeGameFactory_TestInit is CommonTest {
    uint256 internal constant DEFAULT_INIT_BOND = 0.08 ether;
    uint256 internal constant L2_CHAIN_ID = 111;
    uint256 internal constant AGGREGATE_BLOCK_INTERVAL = 100;
    uint256 internal constant AGGREGATE_INTERMEDIATE_BLOCK_INTERVAL = 10;
    uint32 internal constant MAX_GAME_TYPE = 8;
    address internal constant NON_OWNER = address(0xBEEF);

    DisputeGameFactory_FakeClone_Harness fakeClone;

    event DisputeGameCreated(address indexed disputeProxy, GameType indexed gameType, Claim indexed rootClaim);
    event ImplementationSet(address indexed impl, GameType indexed gameType);
    event ImplementationArgsSet(GameType indexed gameType, bytes args);
    event InitBondUpdated(GameType indexed gameType, uint256 indexed newBond);

    function setUp() public virtual override {
        super.setUp();
        fakeClone = new DisputeGameFactory_FakeClone_Harness();

        // Transfer ownership of the factory to the test contract.
        vm.prank(disputeGameFactory.owner());
        disputeGameFactory.transferOwnership(address(this));
    }

    function _setGame(address _gameImpl, GameType _gameType) internal {
        _setGame(_gameImpl, _gameType, DEFAULT_INIT_BOND);
    }

    function _setGame(address _gameImpl, GameType _gameType, uint256 _initBond) internal {
        vm.startPrank(disputeGameFactory.owner());
        disputeGameFactory.setImplementation(_gameType, IDisputeGame(_gameImpl));
        disputeGameFactory.setInitBond(_gameType, _initBond);
        vm.stopPrank();
    }

    function _expectOwnableRevertFrom(address _caller) internal {
        vm.prank(_caller);
        vm.expectRevert("Ownable: caller is not the owner");
    }

    function _assertCreatedGame(
        GameType _gameType,
        Claim _rootClaim,
        bytes memory _extraData,
        IDisputeGame _proxy,
        uint256 _index
    )
        internal
        view
    {
        (IDisputeGame game, Timestamp timestamp) = disputeGameFactory.games(_gameType, _rootClaim, _extraData);

        assertEq(address(game), address(_proxy));
        assertEq(Timestamp.unwrap(timestamp), block.timestamp);
        assertEq(disputeGameFactory.gameCount(), _index + 1);

        (GameType gameType, Timestamp timestampAtIndex, IDisputeGame gameAtIndex) =
            disputeGameFactory.gameAtIndex(_index);
        assertEq(GameType.unwrap(gameType), GameType.unwrap(_gameType));
        assertEq(address(gameAtIndex), address(_proxy));
        assertEq(Timestamp.unwrap(timestampAtIndex), block.timestamp);
    }
}

/// @title DisputeGameFactory_Initialize_Test
/// @notice Tests the `initialize` function of the `DisputeGameFactory` contract.
contract DisputeGameFactory_Initialize_Test is DisputeGameFactory_TestInit {
    bytes32 internal initializedSlot;

    function setUp() public override {
        super.setUp();

        StorageSlot memory slot = ForgeArtifacts.getSlot("DisputeGameFactory", "_initialized");
        initializedSlot = bytes32(slot.slot);
    }

    /// @notice Tests that initialization reverts if called by a non-proxy admin or proxy admin
    ///         owner.
    /// @param _sender The address of the sender to test.
    function testFuzz_initialize_notProxyAdminOrProxyAdminOwner_reverts(address _sender) public {
        vm.assume(
            _sender != address(disputeGameFactory.proxyAdmin()) && _sender != disputeGameFactory.proxyAdminOwner()
        );

        vm.store(address(disputeGameFactory), initializedSlot, bytes32(0));

        vm.expectRevert(IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner.selector);
        vm.prank(_sender);
        disputeGameFactory.initialize(address(1234));
    }

    /// @notice Tests that the initializer value is correct. Trivial test for normal initialization
    ///         but confirms that the initValue is not incremented incorrectly if an upgrade
    ///         function is not present.
    function test_initialize_correctInitializerValue_succeeds() public view {
        bytes32 slotVal = vm.load(address(disputeGameFactory), initializedSlot);
        uint8 val = uint8(uint256(slotVal) & 0xFF);

        assertEq(val, disputeGameFactory.initVersion());
    }
}

/// @title DisputeGameFactory_Create_Test
/// @notice Tests the `create` function of the `DisputeGameFactory` contract.
contract DisputeGameFactory_Create_Test is DisputeGameFactory_TestInit {
    /// @notice Tests that the `create` function succeeds when creating a new dispute game with a
    ///         `GameType` that has an implementation set.
    function testFuzz_create_succeeds(
        uint32 gameType,
        Claim rootClaim,
        bytes calldata extraData,
        uint256 _value
    )
        public
    {
        GameType gt = GameType.wrap(uint8(bound(gameType, 0, MAX_GAME_TYPE)));
        _setGame(address(fakeClone), gt, _value);

        vm.deal(address(this), _value);

        uint256 gameCountBefore = disputeGameFactory.gameCount();

        vm.expectEmit(false, true, true, false);
        emit DisputeGameCreated(address(0), gt, rootClaim);
        IDisputeGame proxy = disputeGameFactory.create{ value: _value }(gt, rootClaim, extraData);

        _assertCreatedGame(gt, rootClaim, extraData, proxy, gameCountBefore);
        assertEq(address(proxy).balance, _value);
    }

    /// @notice Tests that the `create` function reverts when creating a new dispute game with an
    ///         incorrect bond amount.
    function testFuzz_create_incorrectBondAmount_reverts(
        uint32 gameType,
        Claim rootClaim,
        bytes calldata extraData
    )
        public
    {
        GameType gt = GameType.wrap(uint8(bound(gameType, 0, 2)));
        _setGame(address(fakeClone), gt, 1 ether);

        vm.expectRevert(IDisputeGameFactory.IncorrectBondAmount.selector);
        disputeGameFactory.create(gt, rootClaim, extraData);
    }

    /// @notice Tests that the `create` function reverts when there is no implementation set for
    ///         the given `GameType`.
    function testFuzz_create_noImpl_reverts(uint32 gameType, Claim rootClaim, bytes calldata extraData) public {
        GameType gt = GameType.wrap(gameType);
        vm.assume(address(disputeGameFactory.gameImpls(gt)) == address(0));

        vm.expectRevert(abi.encodeWithSelector(IDisputeGameFactory.NoImplementation.selector, gt));
        disputeGameFactory.create(gt, rootClaim, extraData);
    }

    /// @notice Tests that the `create` function reverts when there exists a dispute game with the
    ///         same UUID.
    function testFuzz_create_sameUUID_reverts(uint32 gameType, Claim rootClaim, bytes calldata extraData) public {
        GameType gt = GameType.wrap(uint8(bound(gameType, 0, MAX_GAME_TYPE)));
        disputeGameFactory.setImplementation(gt, IDisputeGame(address(fakeClone)));

        uint256 bondAmount = disputeGameFactory.initBonds(gt);

        vm.expectEmit(false, true, true, false);
        emit DisputeGameCreated(address(0), gt, rootClaim);
        uint256 gameCountBefore = disputeGameFactory.gameCount();
        IDisputeGame proxy = disputeGameFactory.create{ value: bondAmount }(gt, rootClaim, extraData);
        _assertCreatedGame(gt, rootClaim, extraData, proxy, gameCountBefore);

        vm.expectRevert(
            abi.encodeWithSelector(
                IDisputeGameFactory.GameAlreadyExists.selector, disputeGameFactory.getGameUUID(gt, rootClaim, extraData)
            )
        );
        disputeGameFactory.create{ value: bondAmount }(gt, rootClaim, extraData);
    }

    function test_create_implArgs_succeeds() public {
        MockVerifier teeVerifier = new MockVerifier(anchorStateRegistry);
        MockVerifier zkVerifier = new MockVerifier(anchorStateRegistry);
        AggregateVerifier gameImpl = new AggregateVerifier(
            GameTypes.AGGREGATE_VERIFIER,
            anchorStateRegistry,
            delayedWeth,
            IVerifier(address(teeVerifier)),
            IVerifier(address(zkVerifier)),
            bytes32(uint256(1)),
            AggregateVerifier.ZkHashes(bytes32(uint256(2)), bytes32(uint256(3))),
            bytes32(uint256(4)),
            L2_CHAIN_ID,
            AGGREGATE_BLOCK_INTERVAL,
            AGGREGATE_INTERMEDIATE_BLOCK_INTERVAL
        );
        _setGame(address(gameImpl), GameTypes.AGGREGATE_VERIFIER);

        Claim rootClaim = Claim.wrap(bytes32(hex"beef"));
        Proposal memory startingRoot = anchorStateRegistry.getStartingAnchorRoot();
        bytes memory intermediateRoots;
        for (uint256 i = 1; i < gameImpl.intermediateOutputRootsCount(); i++) {
            intermediateRoots = abi.encodePacked(
                intermediateRoots,
                keccak256(abi.encode(startingRoot.l2SequenceNumber + AGGREGATE_INTERMEDIATE_BLOCK_INTERVAL * i))
            );
        }
        intermediateRoots = abi.encodePacked(intermediateRoots, rootClaim.raw());
        bytes memory extraData = abi.encodePacked(
            startingRoot.l2SequenceNumber + AGGREGATE_BLOCK_INTERVAL, address(anchorStateRegistry), intermediateRoots
        );
        bytes memory proof = abi.encodePacked(
            uint8(AggregateVerifier.ProofType.TEE), blockhash(block.number - 1), block.number - 1, bytes32(0)
        );

        uint256 bondAmount = disputeGameFactory.initBonds(GameTypes.AGGREGATE_VERIFIER);
        vm.deal(address(this), bondAmount);

        IDisputeGame proxy = disputeGameFactory.createWithInitData{ value: bondAmount }(
            GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData, proof
        );

        (IDisputeGame game, Timestamp timestamp) =
            disputeGameFactory.games(GameTypes.AGGREGATE_VERIFIER, rootClaim, extraData);

        assertEq(address(game), address(proxy));
        assertEq(Timestamp.unwrap(timestamp), block.timestamp);

        AggregateVerifier gameV2 = AggregateVerifier(address(proxy));

        assertEq(Claim.unwrap(gameV2.rootClaim()), Claim.unwrap(rootClaim));
        assertEq(gameV2.extraData(), extraData);
        assertEq(gameV2.L2_CHAIN_ID(), L2_CHAIN_ID);
        assertEq(address(gameV2.gameCreator()), address(this));
        assertEq(gameV2.l2SequenceNumber(), startingRoot.l2SequenceNumber + AGGREGATE_BLOCK_INTERVAL);
        assertEq(gameV2.parentAddress(), address(anchorStateRegistry));
        assertEq(address(gameV2.DELAYED_WETH()), address(delayedWeth));
        assertEq(address(gameV2.anchorStateRegistry()), address(anchorStateRegistry));
        assertEq(GameType.unwrap(gameV2.gameType()), GameType.unwrap(GameTypes.AGGREGATE_VERIFIER));
        assertEq(gameV2.BLOCK_INTERVAL(), AGGREGATE_BLOCK_INTERVAL);
        assertEq(gameV2.INTERMEDIATE_BLOCK_INTERVAL(), AGGREGATE_INTERMEDIATE_BLOCK_INTERVAL);
    }
}

/// @title DisputeGameFactory_SetImplementation_Test
/// @notice Tests the `setImplementation` function of the `DisputeGameFactory` contract.
contract DisputeGameFactory_SetImplementation_Test is DisputeGameFactory_TestInit {
    /// @notice Tests that the `setImplementation` function properly sets the implementation for a
    ///         given `GameType`.
    function test_setImplementation_succeeds() public {
        vm.expectEmit(true, true, true, true, address(disputeGameFactory));
        emit ImplementationSet(address(1), GameTypes.AGGREGATE_VERIFIER);

        disputeGameFactory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(1)));

        assertEq(address(disputeGameFactory.gameImpls(GameTypes.AGGREGATE_VERIFIER)), address(1));
    }

    /// @notice Tests that the `setImplementation` function reverts when called by a non-owner.
    function test_setImplementation_notOwner_reverts() public {
        _expectOwnableRevertFrom(NON_OWNER);
        disputeGameFactory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(1)));
    }

    /// @notice Tests that the `setImplementation` function with args properly sets the implementation
    ///         and args for a given `GameType`.
    function test_setImplementation_withArgs_succeeds() public {
        address gameImpl = address(1);

        bytes memory args = abi.encodePacked(
            bytes32(hex"dead"), // 32 bytes
            address(0xbeef), // 20 bytes
            anchorStateRegistry, // 20 bytes
            delayedWeth, // 20 bytes
            L2_CHAIN_ID // 32 bytes
        );

        vm.expectEmit(true, true, true, true, address(disputeGameFactory));
        emit ImplementationSet(gameImpl, GameTypes.AGGREGATE_VERIFIER);
        vm.expectEmit(true, true, true, true, address(disputeGameFactory));
        emit ImplementationArgsSet(GameTypes.AGGREGATE_VERIFIER, args);

        disputeGameFactory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(gameImpl), args);

        assertEq(address(disputeGameFactory.gameImpls(GameTypes.AGGREGATE_VERIFIER)), gameImpl);
        assertEq(disputeGameFactory.gameArgs(GameTypes.AGGREGATE_VERIFIER), args);
    }

    /// @notice Tests that the `setImplementation` function with args reverts when called by a non-owner.
    function test_setImplementationArgs_notOwner_reverts() public {
        bytes memory args = abi.encode(uint256(123), address(0xdead));

        _expectOwnableRevertFrom(NON_OWNER);
        disputeGameFactory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(1)), args);
    }
}

/// @title DisputeGameFactory_SetInitBond_Test
/// @notice Tests the `setInitBond` function of the `DisputeGameFactory` contract.
contract DisputeGameFactory_SetInitBond_Test is DisputeGameFactory_TestInit {
    /// @notice Tests that the `setInitBond` function properly sets the init bond for a given
    ///         `GameType`.
    function test_setInitBond_succeeds() public {
        vm.expectEmit(true, true, true, true, address(disputeGameFactory));
        emit InitBondUpdated(GameTypes.AGGREGATE_VERIFIER, 1 ether);

        disputeGameFactory.setInitBond(GameTypes.AGGREGATE_VERIFIER, 1 ether);

        assertEq(disputeGameFactory.initBonds(GameTypes.AGGREGATE_VERIFIER), 1 ether);

        vm.expectEmit(true, true, true, true, address(disputeGameFactory));
        emit InitBondUpdated(GameTypes.AGGREGATE_VERIFIER, 2 ether);

        disputeGameFactory.setInitBond(GameTypes.AGGREGATE_VERIFIER, 2 ether);

        assertEq(disputeGameFactory.initBonds(GameTypes.AGGREGATE_VERIFIER), 2 ether);
    }

    /// @notice Tests that the `setInitBond` function reverts when called by a non-owner.
    function test_setInitBond_notOwner_reverts() public {
        _expectOwnableRevertFrom(NON_OWNER);
        disputeGameFactory.setInitBond(GameTypes.AGGREGATE_VERIFIER, 1 ether);
    }
}

/// @title DisputeGameFactory_GetGameUUID_Test
/// @notice Tests the `getGameUUID` function of the `DisputeGameFactory` contract.
contract DisputeGameFactory_GetGameUUID_Test is DisputeGameFactory_TestInit {
    /// @notice Tests that the `getGameUUID` function returns the correct hash when comparing
    ///         against the keccak256 hash of the abi-encoded parameters.
    function testDiff_getGameUUID_succeeds(uint32 gameType, Claim rootClaim, bytes calldata extraData) public view {
        GameType gt = GameType.wrap(uint8(bound(gameType, 0, 2)));

        assertEq(
            Hash.unwrap(disputeGameFactory.getGameUUID(gt, rootClaim, extraData)),
            keccak256(abi.encode(gt, rootClaim, extraData))
        );
    }
}

/// @title DisputeGameFactory_FindLatestGames_Test
/// @notice Tests the `findLatestGames` function of the `DisputeGameFactory` contract.
contract DisputeGameFactory_FindLatestGames_Test is DisputeGameFactory_TestInit {
    GameType internal constant FIND_LATEST_SEARCH_GAME_TYPE = GameType.wrap(type(uint32).max);
    GameType internal constant FIND_LATEST_OTHER_GAME_TYPE = GameType.wrap(type(uint32).max - 1);

    function setUp() public override {
        super.setUp();

        for (uint8 i; i < 3; i++) {
            _setGame(address(fakeClone), GameType.wrap(i), 0);
        }
    }

    /// @notice Tests that `findLatestGames` returns an empty array when the passed starting index
    ///         is greater than or equal to the game count.
    function testFuzz_findLatestGames_greaterThanLength_succeeds(uint256 _start) public view {
        uint256 gameCount = disputeGameFactory.gameCount();
        _start = bound(_start, gameCount, type(uint256).max);

        IDisputeGameFactory.GameSearchResult[] memory games =
            disputeGameFactory.findLatestGames(GameTypes.AGGREGATE_VERIFIER, _start, 1);
        assertEq(games.length, 0);
    }

    /// @notice Tests that `findLatestGames` returns the correct games.
    function test_findLatestGames_static_succeeds() public {
        for (uint256 i; i < 5; i++) {
            disputeGameFactory.create(GameType.wrap(uint8(i % 3)), Claim.wrap(bytes32(i)), abi.encode(i));
        }

        uint256 gameCount = disputeGameFactory.gameCount();

        uint256 start = gameCount - 1;

        _assertLatestGameResult(GameType.wrap(1), start, start);
        _assertLatestGameResult(GameType.wrap(0), start, start - 1);
        _assertLatestGameResult(GameType.wrap(2), start, start - 2);
    }

    /// @notice Tests that `findLatestGames` returns the correct games, if there are less than `_n`
    ///         games of the given type available.
    function test_findLatestGames_lessThanNAvailable_succeeds() public {
        _setGame(address(fakeClone), FIND_LATEST_SEARCH_GAME_TYPE, 0);
        _setGame(address(fakeClone), FIND_LATEST_OTHER_GAME_TYPE, 0);

        uint256 firstSearchGameIndex = disputeGameFactory.gameCount();
        disputeGameFactory.create(FIND_LATEST_SEARCH_GAME_TYPE, Claim.wrap(bytes32(0)), abi.encode(0));
        disputeGameFactory.create(FIND_LATEST_SEARCH_GAME_TYPE, Claim.wrap(bytes32(uint256(1))), abi.encode(1));
        for (uint256 i; i < 1 << 3; i++) {
            disputeGameFactory.create(FIND_LATEST_OTHER_GAME_TYPE, Claim.wrap(bytes32(i)), abi.encode(i));
        }

        uint256 gameCount = disputeGameFactory.gameCount();

        IDisputeGameFactory.GameSearchResult[] memory games;
        games = disputeGameFactory.findLatestGames(GameType.wrap(type(uint32).max - 2), gameCount - 1, 5);
        assertEq(games.length, 0);

        games = disputeGameFactory.findLatestGames(FIND_LATEST_SEARCH_GAME_TYPE, gameCount - 1, 5);
        assertEq(games.length, 2);
        assertEq(games[0].index, firstSearchGameIndex + 1);
        assertEq(games[1].index, firstSearchGameIndex);
    }

    /// @notice Tests that the expected number of games are returned when `findLatestGames` is
    ///         called.
    function testFuzz_findLatestGames_correctAmount_succeeds(
        uint256 _numGames,
        uint256 _numSearchedGames,
        uint256 _n
    )
        public
    {
        _numGames = bound(_numGames, 0, isForkTest() ? 5 : 32);
        _numSearchedGames = bound(_numSearchedGames, 0, _numGames);
        _n = bound(_n, 0, _numSearchedGames);

        for (uint256 i; i < _numGames; i++) {
            uint32 gameType = i < _numSearchedGames ? 0 : 1;
            disputeGameFactory.create(GameType.wrap(gameType), Claim.wrap(bytes32(i)), abi.encode(i));
        }

        uint256 start = _numGames == 0 ? 0 : _numGames - 1;
        IDisputeGameFactory.GameSearchResult[] memory games =
            disputeGameFactory.findLatestGames(GameType.wrap(0), start, _n);
        assertEq(games.length, _n);
    }

    function _assertLatestGameResult(GameType _gameType, uint256 _start, uint256 _expectedIndex) internal view {
        IDisputeGameFactory.GameSearchResult[] memory games = disputeGameFactory.findLatestGames(_gameType, _start, 1);
        assertEq(games.length, 1);

        assertEq(games[0].index, _expectedIndex);
        (GameType gameType, Timestamp createdAt,) = games[0].metadata.unpack();
        assertEq(GameType.unwrap(gameType), GameType.unwrap(_gameType));
        assertEq(Timestamp.unwrap(createdAt), block.timestamp);
    }
}

/// @title DisputeGameFactory_Uncategorized_Test
/// @notice General tests that are not testing any function directly of the `DisputeGameFactory`
///         contract or are testing multiple functions at once.
contract DisputeGameFactory_Uncategorized_Test is DisputeGameFactory_TestInit {
    /// @notice Tests that the `owner` function returns the correct address after deployment.
    function test_owner_succeeds() public view {
        assertEq(disputeGameFactory.owner(), address(this));
    }

    /// @notice Tests that the `transferOwnership` function succeeds when called by the owner.
    function test_transferOwnership_succeeds() public {
        disputeGameFactory.transferOwnership(address(1));
        assertEq(disputeGameFactory.owner(), address(1));
    }

    /// @notice Tests that the `transferOwnership` function reverts when called by a non-owner.
    function test_transferOwnership_notOwner_reverts() public {
        _expectOwnableRevertFrom(NON_OWNER);
        disputeGameFactory.transferOwnership(address(1));
    }
}
