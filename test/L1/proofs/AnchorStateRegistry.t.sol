// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import { BaseTest } from "test/L1/proofs/BaseTest.t.sol";

// Libraries
import { GameType, GameStatus, Hash, Proposal } from "src/libraries/bridge/Types.sol";
import { Claim } from "src/libraries/bridge/LibUDT.sol";
import { ForgeArtifacts, StorageSlot } from "scripts/libraries/ForgeArtifacts.sol";

// Interfaces
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";

// Contracts
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";

/// @title AnchorStateRegistry_TestInit
/// @notice Reusable test initialization for `AnchorStateRegistry` tests.
abstract contract AnchorStateRegistry_TestInit is BaseTest {
    IDisputeGameFactory disputeGameFactory;
    ISuperchainConfig superchainConfig;
    IDisputeGame gameProxy;

    /// @dev A valid l2BlockNumber that comes after the current anchor root block.
    uint256 validL2BlockNumber;

    event AnchorUpdated(IDisputeGame indexed game);
    event RespectedGameTypeSet(GameType gameType);
    event RetirementTimestampSet(uint256 timestamp);
    event DisputeGameBlacklisted(IDisputeGame indexed disputeGame);

    function setUp() public virtual override {
        super.setUp();

        disputeGameFactory = IDisputeGameFactory(address(factory));
        superchainConfig = ISuperchainConfig(makeAddr("superchain-config"));
        vm.mockCall(address(superchainConfig), abi.encodeCall(superchainConfig.guardian, ()), abi.encode(address(this)));
        vm.mockCall(
            address(systemConfig), abi.encodeCall(systemConfig.superchainConfig, ()), abi.encode(superchainConfig)
        );

        // Get the actual anchor roots
        (, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();
        validL2BlockNumber = l2BlockNumber + BLOCK_INTERVAL;
        Claim rootClaim = Claim.wrap(keccak256(abi.encode(validL2BlockNumber)));
        bytes memory proof = _generateProof("tee-proof", AggregateVerifier.ProofType.TEE);
        gameProxy = IDisputeGame(
            address(
                _createAggregateVerifierGame(
                    TEE_PROVER, rootClaim, validL2BlockNumber, address(anchorStateRegistry), proof
                )
            )
        );
    }

    function _setPaused(bool _paused) internal {
        vm.mockCall(address(systemConfig), abi.encodeCall(systemConfig.paused, ()), abi.encode(_paused));
    }

    function _mockGameStatus(GameStatus _status) internal {
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.status, ()), abi.encode(_status));
    }

    function _mockGameResolvedAt(uint256 _timestamp) internal {
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.resolvedAt, ()), abi.encode(_timestamp));
    }

    function _mockGamePastFinalityWithStatus(GameStatus _status) internal {
        _mockGameResolvedAt(block.timestamp);
        vm.warp(block.timestamp + anchorStateRegistry.disputeGameFinalityDelaySeconds() + 1);
        _mockGameStatus(_status);
    }

    function _mockGameWasRespected(bool _wasRespected) internal {
        vm.mockCall(
            address(gameProxy), abi.encodeCall(gameProxy.wasRespectedGameTypeWhenCreated, ()), abi.encode(_wasRespected)
        );
    }

    function _mockGameNotRegistered() internal {
        vm.mockCall(
            address(disputeGameFactory),
            abi.encodeCall(
                disputeGameFactory.games, (gameProxy.gameType(), gameProxy.rootClaim(), gameProxy.extraData())
            ),
            abi.encode(address(0), 0)
        );
    }

    function _initializeWithDummyStartingAnchorRoot() internal {
        anchorStateRegistry.initialize(
            systemConfig,
            disputeGameFactory,
            Proposal({
                root: Hash.wrap(0xDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF), l2SequenceNumber: 0
            }),
            GameType.wrap(0)
        );
    }

    function _assumeNotGuardian(address _caller) internal view {
        vm.assume(_caller != superchainConfig.guardian());
    }

    function _updateRetirementTimestampAsGuardian() internal {
        vm.prank(superchainConfig.guardian());
        anchorStateRegistry.updateRetirementTimestamp();
    }

    function _blacklistDisputeGameAsGuardian(IDisputeGame _game) internal {
        vm.prank(superchainConfig.guardian());
        anchorStateRegistry.blacklistDisputeGame(_game);
    }

    function _mockGameCreatedAt(uint64 _createdAtTimestamp) internal {
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.createdAt, ()), abi.encode(_createdAtTimestamp));
    }

    function _mockDisputeGameBlacklist(IDisputeGame _game, bool _blacklisted) internal {
        vm.mockCall(
            address(anchorStateRegistry),
            abi.encodeCall(anchorStateRegistry.disputeGameBlacklist, (_game)),
            abi.encode(_blacklisted)
        );
    }

    function _setAnchorStateToGameProxy() internal {
        _mockGamePastFinalityWithStatus(GameStatus.DEFENDER_WINS);
        anchorStateRegistry.setAnchorState(gameProxy);
    }

    function _assertAnchorRootEqGame(IDisputeGame _game) internal view {
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), _game.rootClaim().raw());
        assertEq(l2BlockNumber, _game.l2SequenceNumber());
    }

    function _assertCurrentAnchorRootEq(Hash _root, uint256 _l2BlockNumber) internal view {
        (Hash updatedRoot, uint256 updatedL2BlockNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(updatedL2BlockNumber, _l2BlockNumber);
        assertEq(updatedRoot.raw(), _root.raw());
    }

    function _assertAnchorEq(GameType _gameType, Hash _root, uint256 _l2BlockNumber) internal view {
        (Hash updatedRoot, uint256 updatedL2BlockNumber) = anchorStateRegistry.anchors(_gameType);
        assertEq(updatedL2BlockNumber, _l2BlockNumber);
        assertEq(updatedRoot.raw(), _root.raw());
    }
}

/// @title AnchorStateRegistry_Version_Test
/// @notice Tests the `version` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_Version_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that the version function returns a string.
    function test_version_succeeds() public view {
        assert(bytes(anchorStateRegistry.version()).length > 0);
    }
}

/// @title AnchorStateRegistry_Initialize_Test
/// @notice Tests the `initialize` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_Initialize_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that initialization is successful.
    function test_initialize_succeeds() public view {
        // Verify starting anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), keccak256(abi.encode(uint256(0))));
        assertEq(l2BlockNumber, 0);

        // Verify contract addresses.
        assertEq(address(anchorStateRegistry.systemConfig()), address(systemConfig));
        assertEq(address(anchorStateRegistry.disputeGameFactory()), address(disputeGameFactory));
        assertEq(address(anchorStateRegistry.superchainConfig()), address(superchainConfig));
    }

    /// @notice Tests that the initializer value is correct. Trivial test for normal
    ///         initialization but confirms that the initValue is not incremented incorrectly if
    ///         an upgrade function is not present.
    function test_initialize_correctInitializerValue_succeeds() public view {
        // Get the slot for _initialized.
        StorageSlot memory slot = ForgeArtifacts.getSlot("AnchorStateRegistry", "_initialized");

        // Get the initializer value.
        bytes32 slotVal = vm.load(address(anchorStateRegistry), bytes32(slot.slot));
        uint8 val = uint8(uint256(slotVal) & 0xFF);

        // Assert that the initializer value matches the expected value.
        assertEq(val, anchorStateRegistry.initVersion());
    }

    /// @notice Tests that the retirement timestamp is set on the first initialization.
    function test_initialize_setsRetirementTimestamp_succeeds() public {
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        GameType startingGameType = anchorStateRegistry.respectedGameType();

        StorageSlot memory initSlot = ForgeArtifacts.getSlot("AnchorStateRegistry", "_initialized");
        StorageSlot memory retirementSlot = ForgeArtifacts.getSlot("AnchorStateRegistry", "retirementTimestamp");
        address proxyAdminOwner = anchorStateRegistry.proxyAdminOwner();

        // Reset initialization and retirement timestamp state.
        vm.store(address(anchorStateRegistry), bytes32(initSlot.slot), bytes32(0));
        vm.store(address(anchorStateRegistry), bytes32(retirementSlot.slot), bytes32(0));

        uint256 newTimestamp = block.timestamp + 100;
        vm.warp(newTimestamp);
        uint64 expectedTimestamp = uint64(newTimestamp);

        vm.prank(proxyAdminOwner);
        anchorStateRegistry.initialize(
            systemConfig,
            disputeGameFactory,
            Proposal({ root: root, l2SequenceNumber: l2SequenceNumber }),
            startingGameType
        );

        assertEq(anchorStateRegistry.retirementTimestamp(), expectedTimestamp);
    }

    /// @notice Tests that the retirement timestamp is unchanged during re-initialization.
    function test_initialize_reinitializationDoesNotChangeRetirementTimestamp_succeeds() public {
        (Hash root, uint256 l2SequenceNumber) = anchorStateRegistry.getAnchorRoot();
        GameType startingGameType = anchorStateRegistry.respectedGameType();

        StorageSlot memory initSlot = ForgeArtifacts.getSlot("AnchorStateRegistry", "_initialized");
        address proxyAdminOwner = anchorStateRegistry.proxyAdminOwner();

        uint256 initialTimestamp = block.timestamp + 200;
        vm.warp(initialTimestamp);
        _updateRetirementTimestampAsGuardian();
        uint64 originalTimestamp = anchorStateRegistry.retirementTimestamp();

        uint256 reinitTimestamp = block.timestamp + 200;
        vm.warp(reinitTimestamp);

        vm.store(address(anchorStateRegistry), bytes32(initSlot.slot), bytes32(0));

        vm.prank(proxyAdminOwner);
        anchorStateRegistry.initialize(
            systemConfig,
            disputeGameFactory,
            Proposal({ root: root, l2SequenceNumber: l2SequenceNumber }),
            startingGameType
        );

        assertEq(anchorStateRegistry.retirementTimestamp(), originalTimestamp);
    }

    /// @notice Tests that initialization cannot be done twice
    function test_initialize_twice_reverts() public {
        vm.expectRevert("Initializable: contract is already initialized");
        _initializeWithDummyStartingAnchorRoot();
    }

    /// @notice Tests that initialization reverts if called by a non-proxy admin or owner.
    /// @param _sender The address of the sender to test.
    function testFuzz_initialize_notProxyAdminOrProxyAdminOwner_reverts(address _sender) public {
        // Prank as the not ProxyAdmin or ProxyAdmin owner.
        vm.assume(
            _sender != address(anchorStateRegistry.proxyAdmin()) && _sender != anchorStateRegistry.proxyAdminOwner()
        );

        // Get the slot for _initialized.
        StorageSlot memory slot = ForgeArtifacts.getSlot("AnchorStateRegistry", "_initialized");

        // Set the initialized slot to 0.
        vm.store(address(anchorStateRegistry), bytes32(slot.slot), bytes32(0));

        // Expect the revert with `ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner` selector.
        vm.expectRevert(IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner.selector);

        // Call the `initialize` function with the sender
        vm.prank(_sender);
        _initializeWithDummyStartingAnchorRoot();
    }
}

/// @title AnchorStateRegistry_Paused_Test
/// @notice Tests the `paused` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_Paused_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that paused() will return the correct value.
    function test_paused_succeeds() public {
        // Pause the superchain.
        _setPaused(true);

        // Paused should return true.
        assertTrue(anchorStateRegistry.paused());

        // Unpause the superchain.
        _setPaused(false);

        // Paused should return false.
        assertFalse(anchorStateRegistry.paused());
    }
}

/// @title AnchorStateRegistry_SetRespectedGameType_Test
/// @notice Tests the `setRespectedGameType` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_SetRespectedGameType_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that setRespectedGameType succeeds when called by the guardian
    /// @param _gameType The game type to set as respected
    function testFuzz_setRespectedGameType_succeeds(GameType _gameType) public {
        // Call as guardian
        vm.prank(superchainConfig.guardian());
        vm.expectEmit(address(anchorStateRegistry));
        emit RespectedGameTypeSet(_gameType);
        anchorStateRegistry.setRespectedGameType(_gameType);

        // Verify the game type was set
        assertEq(anchorStateRegistry.respectedGameType().raw(), _gameType.raw());
    }

    /// @notice Tests that setRespectedGameType reverts when not called by the guardian
    /// @param _gameType The game type to attempt to set
    /// @param _caller The address attempting to call the function
    function testFuzz_setRespectedGameType_notGuardian_reverts(GameType _gameType, address _caller) public {
        _assumeNotGuardian(_caller);

        vm.prank(_caller);
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_Unauthorized.selector);
        anchorStateRegistry.setRespectedGameType(_gameType);
    }
}

/// @title AnchorStateRegistry_UpdateRetirementTimestamp_Test
/// @notice Tests the `updateRetirementTimestamp` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_UpdateRetirementTimestamp_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that updateRetirementTimestamp succeeds when called by the guardian
    function test_updateRetirementTimestamp_succeeds() public {
        vm.expectEmit(address(anchorStateRegistry));
        emit RetirementTimestampSet(block.timestamp);
        _updateRetirementTimestampAsGuardian();

        // Verify the timestamp was set
        assertEq(anchorStateRegistry.retirementTimestamp(), block.timestamp);
    }

    /// @notice Tests that updateRetirementTimestamp can be called multiple times by the guardian
    function test_updateRetirementTimestamp_multipleUpdates_succeeds() public {
        _updateRetirementTimestampAsGuardian();
        uint64 firstTimestamp = anchorStateRegistry.retirementTimestamp();

        // Warp forward and update again
        vm.warp(block.timestamp + 1000);
        vm.expectEmit(address(anchorStateRegistry));
        emit RetirementTimestampSet(block.timestamp);
        _updateRetirementTimestampAsGuardian();

        // Verify the timestamp was updated
        assertEq(anchorStateRegistry.retirementTimestamp(), block.timestamp);
        assertGt(anchorStateRegistry.retirementTimestamp(), firstTimestamp);
    }

    /// @notice Tests that updateRetirementTimestamp reverts when not called by the guardian
    /// @param _caller The address attempting to call the function
    function testFuzz_updateRetirementTimestamp_notGuardian_reverts(address _caller) public {
        _assumeNotGuardian(_caller);

        vm.prank(_caller);
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_Unauthorized.selector);
        anchorStateRegistry.updateRetirementTimestamp();
    }
}

/// @title AnchorStateRegistry_BlacklistDisputeGame_Test
/// @notice Tests the `blacklistDisputeGame` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_BlacklistDisputeGame_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that blacklistDisputeGame succeeds when called by the guardian
    function test_blacklistDisputeGame_succeeds() public {
        vm.expectEmit(address(anchorStateRegistry));
        emit DisputeGameBlacklisted(gameProxy);
        _blacklistDisputeGameAsGuardian(gameProxy);

        // Verify the game was blacklisted
        assertTrue(anchorStateRegistry.disputeGameBlacklist(gameProxy));
    }

    /// @notice Tests that multiple games can be blacklisted
    function test_blacklistDisputeGame_multipleGames_succeeds() public {
        // Create a second game proxy
        IDisputeGame secondGame = IDisputeGame(address(0x123));

        // Blacklist both games
        vm.startPrank(superchainConfig.guardian());
        anchorStateRegistry.blacklistDisputeGame(gameProxy);
        anchorStateRegistry.blacklistDisputeGame(secondGame);
        vm.stopPrank();

        // Verify both games are blacklisted
        assertTrue(anchorStateRegistry.disputeGameBlacklist(gameProxy));
        assertTrue(anchorStateRegistry.disputeGameBlacklist(secondGame));
    }

    /// @notice Tests that blacklistDisputeGame reverts when not called by the guardian
    /// @param _caller The address attempting to call the function
    function testFuzz_blacklistDisputeGame_notGuardian_reverts(address _caller) public {
        _assumeNotGuardian(_caller);

        vm.prank(_caller);
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_Unauthorized.selector);
        anchorStateRegistry.blacklistDisputeGame(gameProxy);
    }

    /// @notice Tests that blacklisting a game twice succeeds but doesn't change state
    function test_blacklistDisputeGame_twice_succeeds() public {
        // Blacklist the game
        vm.startPrank(superchainConfig.guardian());
        anchorStateRegistry.blacklistDisputeGame(gameProxy);

        // Blacklist again - should emit event but not change state
        vm.expectEmit(address(anchorStateRegistry));
        emit DisputeGameBlacklisted(gameProxy);
        anchorStateRegistry.blacklistDisputeGame(gameProxy);
        vm.stopPrank();

        // Verify the game is still blacklisted
        assertTrue(anchorStateRegistry.disputeGameBlacklist(gameProxy));
    }
}

/// @title AnchorStateRegistry_Anchors_Test
/// @notice Tests the `anchors` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_Anchors_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that the anchors() function always matches the result of the getAnchorRoot()
    ///         function regardless of the game type used.
    /// @param _gameType Game type to use as input to anchors().
    function testFuzz_anchors_matchesGetAnchorRoot_succeeds(GameType _gameType) public view {
        // Get the anchor root according to getAnchorRoot().
        (Hash root1, uint256 l2BlockNumber1) = anchorStateRegistry.getAnchorRoot();

        // Get the anchor root according to anchors().
        (Hash root2, uint256 l2BlockNumber2) = anchorStateRegistry.anchors(_gameType);

        // Assert that the two roots are the same.
        assertEq(root1.raw(), root2.raw());
        assertEq(l2BlockNumber1, l2BlockNumber2);
    }
}

/// @title AnchorStateRegistry_GetAnchorRoot_Test
/// @notice Tests the `getAnchorRoot` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_GetAnchorRoot_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that getAnchorRoot will return the value of the starting anchor root when no
    ///         anchor game exists yet.
    function test_getAnchorRoot_noAnchorGame_succeeds() public view {
        assertEq(address(anchorStateRegistry.anchorGame()), address(0));

        // We should get the starting anchor root back.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(root.raw(), keccak256(abi.encode(uint256(0))));
        assertEq(l2BlockNumber, 0);
    }

    /// @notice Tests that getAnchorRoot will return the correct anchor root if an anchor game exists.
    function test_getAnchorRoot_anchorGameExists_succeeds() public {
        _setAnchorStateToGameProxy();
        _assertAnchorRootEqGame(gameProxy);
    }

    /// @notice Tests that getAnchorRoot will return the latest anchor root even if the superchain
    ///         is paused.
    function test_getAnchorRoot_superchainPaused_succeeds() public {
        _setAnchorStateToGameProxy();

        // Pause the superchain.
        _setPaused(true);

        _assertAnchorRootEqGame(gameProxy);
    }

    /// @notice Tests that getAnchorRoot returns even if the anchor game is blacklisted.
    function test_getAnchorRoot_blacklistedGame_succeeds() public {
        _setAnchorStateToGameProxy();

        // Blacklist the game.
        _blacklistDisputeGameAsGuardian(gameProxy);

        _assertAnchorRootEqGame(gameProxy);
    }
}

/// @title AnchorStateRegistry_GetStartingAnchorRoot_Test
/// @notice Tests the `getStartingAnchorRoot` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_GetStartingAnchorRoot_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that getStartingAnchorRoot remains unchanged even if the current anchor root
    ///         changes.
    function test_getStartingAnchorRoot_afterUpdate_succeeds() public {
        _mockGamePastFinalityWithStatus(GameStatus.DEFENDER_WINS);

        // Mock the game's L2 block number to be greater than the starting anchor root block number.
        vm.mockCall(
            address(gameProxy), abi.encodeCall(gameProxy.l2SequenceNumber, ()), abi.encode(validL2BlockNumber + 1)
        );

        // Mock the game's anchor root to be different from the starting anchor root.
        vm.mockCall(
            address(gameProxy),
            abi.encodeCall(gameProxy.rootClaim, ()),
            abi.encode(Claim.wrap(keccak256(abi.encode(123))))
        );

        // Set the anchor game to the game proxy.
        anchorStateRegistry.setAnchorState(gameProxy);

        // Grab the value of the starting anchor root before the update.
        Proposal memory startingAnchorRootBeforeUpdate = anchorStateRegistry.getStartingAnchorRoot();

        // Verify the CURRENT anchor root has changed.
        (Hash currentRoot, uint256 currentL2BlockNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(currentRoot.raw(), gameProxy.rootClaim().raw());
        assertEq(currentL2BlockNumber, gameProxy.l2SequenceNumber());

        // Verify the STARTING anchor root has NOT changed.
        Proposal memory startingAnchorRootAfterUpdate = anchorStateRegistry.getStartingAnchorRoot();
        assertEq(startingAnchorRootAfterUpdate.root.raw(), startingAnchorRootBeforeUpdate.root.raw());
        assertEq(startingAnchorRootAfterUpdate.l2SequenceNumber, startingAnchorRootBeforeUpdate.l2SequenceNumber);

        // Explicitly assert they are different (assuming the new game has different values).
        assertFalse(currentRoot.raw() == startingAnchorRootAfterUpdate.root.raw());
        assertFalse(currentL2BlockNumber == startingAnchorRootAfterUpdate.l2SequenceNumber);
    }
}

/// @title AnchorStateRegistry_IsGameRegistered_Test
/// @notice Tests the `isGameRegistered` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameRegistered_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameRegistered will return true if the game is registered.
    function test_isGameRegistered_isActuallyFactoryRegistered_succeeds() public view {
        assertTrue(anchorStateRegistry.isGameRegistered(gameProxy));
    }

    /// @notice Tests that isGameRegistered will return false if the game is not registered.
    function test_isGameRegistered_isNotFactoryRegistered_succeeds() public {
        _mockGameNotRegistered();

        // Game should not be registered.
        assertFalse(anchorStateRegistry.isGameRegistered(gameProxy));
    }

    /// @notice Tests that isGameRegistered will return false if the game is not using the same
    ///         AnchorStateRegistry as the one checking the registration.
    /// @param _anchorStateRegistry The AnchorStateRegistry to use for the test.
    function test_isGameRegistered_isNotSameAnchorStateRegistry_succeeds(address _anchorStateRegistry) public {
        // Make sure the AnchorStateRegistry is different.
        vm.assume(_anchorStateRegistry != address(anchorStateRegistry));

        // Mock the gameProxy's AnchorStateRegistry to be a different address.
        vm.mockCall(
            address(gameProxy), abi.encodeCall(gameProxy.anchorStateRegistry, ()), abi.encode(_anchorStateRegistry)
        );

        // Game should not be registered.
        assertFalse(anchorStateRegistry.isGameRegistered(gameProxy));
    }
}

/// @title AnchorStateRegistry_IsGameRespected_Test
/// @notice Tests the `isGameRespected` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameRespected_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameRespected will return true if the game is of the respected game
    ///         type.
    function test_isGameRespected_isRespected_succeeds() public {
        _mockGameWasRespected(true);
        assertTrue(anchorStateRegistry.isGameRespected(gameProxy));
    }

    /// @notice Tests that isGameRespected will return false if the game is not of the respected
    ///         game type.
    function test_isGameRespected_isNotRespected_succeeds() public {
        _mockGameWasRespected(false);
        assertFalse(anchorStateRegistry.isGameRespected(gameProxy));
    }
}

/// @title AnchorStateRegistry_IsGameBlacklisted_Test
/// @notice Tests the `isGameBlacklisted` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameBlacklisted_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameBlacklisted will return true if the game is blacklisted.
    function test_isGameBlacklisted_isActuallyBlacklisted_succeeds() public {
        // Blacklist the game.
        _blacklistDisputeGameAsGuardian(gameProxy);

        // Should return true.
        assertTrue(anchorStateRegistry.isGameBlacklisted(gameProxy));
    }

    /// @notice Tests that isGameBlacklisted will return false if the game is not blacklisted.
    function test_isGameBlacklisted_isNotBlacklisted_succeeds() public {
        _mockDisputeGameBlacklist(gameProxy, false);
        assertFalse(anchorStateRegistry.isGameBlacklisted(gameProxy));
    }
}

/// @title AnchorStateRegistry_IsGameRetired_Test
/// @notice Tests the `isGameRetired` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameRetired_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameRetired will return true if the game is retired.
    /// @param _createdAtTimestamp The createdAt timestamp to use for the test.
    function testFuzz_isGameRetired_isRetired_succeeds(uint64 _createdAtTimestamp) public {
        // Set the retirement timestamp to now.
        _updateRetirementTimestampAsGuardian();

        // Make sure createdAt timestamp is less than or equal to the retirementTimestamp.
        _createdAtTimestamp = uint64(bound(_createdAtTimestamp, 0, anchorStateRegistry.retirementTimestamp()));

        // Mock the createdAt call.
        _mockGameCreatedAt(_createdAtTimestamp);

        // Game should be retired.
        assertTrue(anchorStateRegistry.isGameRetired(gameProxy));
    }

    /// @notice Tests that isGameRetired will return false if the game is not retired.
    /// @param _createdAtTimestamp The createdAt timestamp to use for the test.
    function testFuzz_isGameRetired_isNotRetired_succeeds(uint64 _createdAtTimestamp) public {
        // Set the retirement timestamp to now.
        _updateRetirementTimestampAsGuardian();

        // Make sure createdAt timestamp is greater than the retirementTimestamp.
        _createdAtTimestamp =
            uint64(bound(_createdAtTimestamp, anchorStateRegistry.retirementTimestamp() + 1, type(uint64).max));

        // Mock the call to createdAt.
        _mockGameCreatedAt(_createdAtTimestamp);

        // Game should not be retired.
        assertFalse(anchorStateRegistry.isGameRetired(gameProxy));
    }
}

/// @title AnchorStateRegistry_IsGameResolved_Test
/// @notice Tests the `isGameResolved` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameResolved_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameResolved will return true if the game is resolved.
    /// @param _resolvedAtTimestamp The resolvedAt timestamp to use for the test.
    function testFuzz_isGameResolved_challengerWins_succeeds(uint256 _resolvedAtTimestamp) public {
        // Bound resolvedAt to be less than or equal to current timestamp.
        _resolvedAtTimestamp = bound(_resolvedAtTimestamp, 1, block.timestamp);

        _mockGameResolvedAt(_resolvedAtTimestamp);
        _mockGameStatus(GameStatus.CHALLENGER_WINS);

        // Game should be resolved.
        assertTrue(anchorStateRegistry.isGameResolved(gameProxy));
    }

    /// @notice Tests that isGameResolved will return true if the game is resolved.
    /// @param _resolvedAtTimestamp The resolvedAt timestamp to use for the test.
    function testFuzz_isGameResolved_defenderWins_succeeds(uint256 _resolvedAtTimestamp) public {
        // Bound resolvedAt to be less than or equal to current timestamp.
        _resolvedAtTimestamp = bound(_resolvedAtTimestamp, 1, block.timestamp);

        _mockGameResolvedAt(_resolvedAtTimestamp);
        _mockGameStatus(GameStatus.DEFENDER_WINS);

        // Game should be resolved.
        assertTrue(anchorStateRegistry.isGameResolved(gameProxy));
    }

    /// @notice Tests that isGameResolved will return false if the game is in progress and not
    ///         resolved.
    /// @param _resolvedAtTimestamp The resolvedAt timestamp to use for the test.
    function testFuzz_isGameResolved_inProgressNotResolved_succeeds(uint256 _resolvedAtTimestamp) public {
        // Bound resolvedAt to be less than or equal to current timestamp.
        _resolvedAtTimestamp = bound(_resolvedAtTimestamp, 1, block.timestamp);

        _mockGameResolvedAt(_resolvedAtTimestamp);
        _mockGameStatus(GameStatus.IN_PROGRESS);

        // Game should not be resolved.
        assertFalse(anchorStateRegistry.isGameResolved(gameProxy));
    }
}

/// @title AnchorStateRegistry_IsGameProper_Test
/// @notice Tests the `isGameProper` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameProper_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameProper will return true if the game meets all conditions.
    function test_isGameProper_meetsAllConditions_succeeds() public view {
        // Game will meet all conditions by default.
        assertTrue(anchorStateRegistry.isGameProper(gameProxy));
    }

    /// @notice Tests that isGameProper will return false if the game is not registered.
    function test_isGameProper_isNotFactoryRegistered_succeeds() public {
        _mockGameNotRegistered();

        assertFalse(anchorStateRegistry.isGameProper(gameProxy));
    }

    /// @notice Tests that isGameProper still returns true if the game was not the respected game
    ///         type.
    function test_isGameProper_notRespected_succeeds() public {
        _mockGameWasRespected(false);
        assertTrue(anchorStateRegistry.isGameProper(gameProxy));
    }

    /// @notice Tests that isGameProper will return false if the game is blacklisted.
    function test_isGameProper_isBlacklisted_succeeds() public {
        // Blacklist the game.
        _blacklistDisputeGameAsGuardian(gameProxy);

        // Should return false.
        assertFalse(anchorStateRegistry.isGameProper(gameProxy));
    }

    /// @notice Tests that isGameProper will return false if the superchain is paused.
    function test_isGameProper_superchainPaused_succeeds() public {
        // Pause the superchain.
        _setPaused(true);

        // Game should not be proper.
        assertFalse(anchorStateRegistry.isGameProper(gameProxy));
    }

    /// @notice Tests that isGameProper will return false if the game is retired.
    /// @param _createdAtTimestamp The createdAt timestamp to use for the test.
    function testFuzz_isGameProper_isRetired_succeeds(uint64 _createdAtTimestamp) public {
        // Set the retirement timestamp to now.
        _updateRetirementTimestampAsGuardian();

        // Make sure createdAt timestamp is less than or equal to the retirementTimestamp.
        _createdAtTimestamp = uint64(bound(_createdAtTimestamp, 0, anchorStateRegistry.retirementTimestamp()));

        // Mock the call to createdAt.
        _mockGameCreatedAt(_createdAtTimestamp);

        // Game should not be proper.
        assertFalse(anchorStateRegistry.isGameProper(gameProxy));
    }
}

/// @title AnchorStateRegistry_IsGameFinalized_Test
/// @notice Tests the `isGameFinalized` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameFinalized_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameFinalized will return true if the game is finalized.
    /// @param _resolvedAtTimestamp The resolvedAt timestamp to use for the test.
    function testFuzz_isGameFinalized_isFinalized_succeeds(uint256 _resolvedAtTimestamp) public {
        // Warp forward by disputeGameFinalityDelaySeconds.
        vm.warp(block.timestamp + anchorStateRegistry.disputeGameFinalityDelaySeconds());

        // Bound resolvedAt to be at least disputeGameFinalityDelaySeconds in the past.
        // Must be greater than 0.
        _resolvedAtTimestamp =
            bound(_resolvedAtTimestamp, 1, block.timestamp - anchorStateRegistry.disputeGameFinalityDelaySeconds() - 1);

        _mockGameResolvedAt(_resolvedAtTimestamp);
        _mockGameStatus(GameStatus.DEFENDER_WINS);

        // Game should be finalized.
        assertTrue(anchorStateRegistry.isGameFinalized(gameProxy));
    }

    /// @notice Tests that isGameFinalized will return false if the game is not finalized.
    /// @param _resolvedAtTimestamp The resolvedAt timestamp to use for the test.
    function testFuzz_isGameFinalized_isNotFinalized_succeeds(uint256 _resolvedAtTimestamp) public {
        // Warp forward by disputeGameFinalityDelaySeconds.
        vm.warp(block.timestamp + anchorStateRegistry.disputeGameFinalityDelaySeconds());

        // Bound resolvedAt to be less than disputeGameFinalityDelaySeconds in the past.
        _resolvedAtTimestamp = bound(
            _resolvedAtTimestamp,
            block.timestamp - anchorStateRegistry.disputeGameFinalityDelaySeconds(),
            block.timestamp
        );

        _mockGameResolvedAt(_resolvedAtTimestamp);

        // Game should not be finalized.
        assertFalse(anchorStateRegistry.isGameFinalized(gameProxy));
    }

    /// @notice Tests that isGameFinalized will return false if the game is not resolved.
    function test_isGameFinalized_isNotResolved_succeeds() public {
        // Warp forward by disputeGameFinalityDelaySeconds.
        vm.warp(block.timestamp + anchorStateRegistry.disputeGameFinalityDelaySeconds());

        _mockGameStatus(GameStatus.IN_PROGRESS);

        // Game should not be finalized.
        assertFalse(anchorStateRegistry.isGameFinalized(gameProxy));
    }
}

/// @title AnchorStateRegistry_IsGameClaimValid_Test
/// @notice Tests the `isGameClaimValid` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_IsGameClaimValid_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that isGameClaimValid will return true if the game claim is valid.
    /// @param _resolvedAtTimestamp The resolvedAt timestamp to use for the test.
    function testFuzz_isGameClaimValid_claimIsValid_succeeds(uint256 _resolvedAtTimestamp) public {
        // Warp forward by disputeGameFinalityDelaySeconds.
        vm.warp(block.timestamp + anchorStateRegistry.disputeGameFinalityDelaySeconds());

        // Bound resolvedAt to be at least disputeGameFinalityDelaySeconds in the past.
        _resolvedAtTimestamp =
            bound(_resolvedAtTimestamp, 1, block.timestamp - anchorStateRegistry.disputeGameFinalityDelaySeconds() - 1);

        _mockGameWasRespected(true);
        _mockGameResolvedAt(_resolvedAtTimestamp);
        _mockGameStatus(GameStatus.DEFENDER_WINS);

        // Claim should be valid.
        assertTrue(anchorStateRegistry.isGameClaimValid(gameProxy));
    }

    /// @notice Tests that isGameClaimValid will return false if the game is not registered.
    function test_isGameClaimValid_notRegistered_succeeds() public {
        _mockGameNotRegistered();

        // Claim should not be valid.
        assertFalse(anchorStateRegistry.isGameClaimValid(gameProxy));
    }

    /// @notice Tests that isGameClaimValid will return false if the game is not respected.
    function test_isGameClaimValid_isNotRespected_succeeds() public {
        _mockGameWasRespected(false);

        // Claim should not be valid.
        assertFalse(anchorStateRegistry.isGameClaimValid(gameProxy));
    }

    /// @notice Tests that isGameClaimValid will return false if the game is blacklisted.
    function test_isGameClaimValid_isBlacklisted_succeeds() public {
        _mockDisputeGameBlacklist(gameProxy, true);

        // Claim should not be valid.
        assertFalse(anchorStateRegistry.isGameClaimValid(gameProxy));
    }

    /// @notice Tests that isGameClaimValid will return false if the game is retired.
    /// @param _createdAtTimestamp The createdAt timestamp to use for the test.
    function testFuzz_isGameClaimValid_isRetired_succeeds(uint64 _createdAtTimestamp) public {
        // Set the retirement timestamp to now.
        _updateRetirementTimestampAsGuardian();

        // Make sure createdAt timestamp is less than or equal to the retirementTimestamp.
        _createdAtTimestamp = uint64(bound(_createdAtTimestamp, 0, anchorStateRegistry.retirementTimestamp()));

        // Mock the call to createdAt.
        _mockGameCreatedAt(_createdAtTimestamp);

        // Claim should not be valid.
        assertFalse(anchorStateRegistry.isGameClaimValid(gameProxy));
    }

    /// @notice Tests that isGameClaimValid will return false if the game is not resolved.
    function test_isGameClaimValid_notResolved_succeeds() public {
        _mockGameStatus(GameStatus.IN_PROGRESS);

        // Claim should not be valid.
        assertFalse(anchorStateRegistry.isGameClaimValid(gameProxy));
    }

    /// @notice Tests that isGameClaimValid will return false if the game is not finalized.
    /// @param _resolvedAtTimestamp The resolvedAt timestamp to use for the test.
    function testFuzz_isGameClaimValid_notFinalized_succeeds(uint256 _resolvedAtTimestamp) public {
        // Warp forward by disputeGameFinalityDelaySeconds.
        vm.warp(block.timestamp + anchorStateRegistry.disputeGameFinalityDelaySeconds());

        // Bound resolvedAt to be less than disputeGameFinalityDelaySeconds in the past.
        _resolvedAtTimestamp = bound(
            _resolvedAtTimestamp,
            block.timestamp - anchorStateRegistry.disputeGameFinalityDelaySeconds(),
            block.timestamp
        );

        _mockGameResolvedAt(_resolvedAtTimestamp);

        // Claim should not be valid.
        assertFalse(anchorStateRegistry.isGameClaimValid(gameProxy));
    }

    /// @notice Tests that isGameClaimValid will return false if the superchain is paused.
    function test_isGameClaimValid_superchainPaused_succeeds() public {
        // Pause the superchain.
        _setPaused(true);

        // Game should not be valid.
        assertFalse(anchorStateRegistry.isGameClaimValid(gameProxy));
    }
}

/// @title AnchorStateRegistry_SetAnchorState_Test
/// @notice Tests the `setAnchorState` function of the `AnchorStateRegistry` contract.
contract AnchorStateRegistry_SetAnchorState_Test is AnchorStateRegistry_TestInit {
    /// @notice Tests that setAnchorState will succeed if the game is valid, the game block number
    ///         is greater than the current anchor root block number, and the game is the currently
    ///         respected game type.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_validNewerState_succeeds(uint256 _l2BlockNumber) public {
        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, validL2BlockNumber, type(uint256).max);

        // Mock the l2BlockNumber call.
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.l2SequenceNumber, ()), abi.encode(_l2BlockNumber));

        _mockGamePastFinalityWithStatus(GameStatus.DEFENDER_WINS);
        _mockGameWasRespected(true);

        // Update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectEmit(address(anchorStateRegistry));
        emit AnchorUpdated(gameProxy);
        anchorStateRegistry.setAnchorState(gameProxy);

        // Confirm that the anchor state is now the same as the game state.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();
        assertEq(l2BlockNumber, gameProxy.l2SequenceNumber());
        assertEq(root.raw(), gameProxy.rootClaim().raw());

        // Confirm that the anchor game is now set.
        IDisputeGame anchorGame = anchorStateRegistry.anchorGame();
        assertEq(address(anchorGame), address(gameProxy));
    }

    /// @notice Tests that setAnchorState will revert if the game is valid and the game block
    ///         number is less than or equal to the current anchor root block number.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_olderValidGameClaim_fails(uint256 _l2BlockNumber) public {
        // Grab block number of the existing anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();

        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, 0, l2BlockNumber);

        // Mock the l2BlockNumber call.
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.l2SequenceNumber, ()), abi.encode(_l2BlockNumber));

        _mockGamePastFinalityWithStatus(GameStatus.DEFENDER_WINS);
        _mockGameWasRespected(true);

        // Try to update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);

        _assertCurrentAnchorRootEq(root, l2BlockNumber);
    }

    /// @notice Tests that setAnchorState will revert if the game is not registered.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_notFactoryRegisteredGame_fails(uint256 _l2BlockNumber) public {
        // Grab block number of the existing anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();

        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, l2BlockNumber, type(uint256).max);

        // Mock the l2BlockNumber call.
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.l2SequenceNumber, ()), abi.encode(_l2BlockNumber));

        _mockGameStatus(GameStatus.DEFENDER_WINS);
        _mockGameWasRespected(true);
        _mockGameNotRegistered();

        // Try to update the anchor state.
        vm.prank(superchainConfig.guardian());
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);

        _assertCurrentAnchorRootEq(root, l2BlockNumber);
    }

    /// @notice Tests that setAnchorState will revert if the game is valid and the game status is
    ///         CHALLENGER_WINS.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_challengerWins_fails(uint256 _l2BlockNumber) public {
        // Grab block number of the existing anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();

        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, l2BlockNumber, type(uint256).max);

        // Mock the l2BlockNumber call.
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.l2SequenceNumber, ()), abi.encode(_l2BlockNumber));

        _mockGamePastFinalityWithStatus(GameStatus.CHALLENGER_WINS);
        _mockGameWasRespected(true);

        // Try to update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);

        _assertCurrentAnchorRootEq(root, l2BlockNumber);
    }

    /// @notice Tests that setAnchorState will revert if the game is valid and the game status is
    ///         IN_PROGRESS.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_inProgress_fails(uint256 _l2BlockNumber) public {
        // Grab block number of the existing anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();

        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, l2BlockNumber, type(uint256).max);

        // Mock the l2BlockNumber call.
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.l2SequenceNumber, ()), abi.encode(_l2BlockNumber));

        _mockGamePastFinalityWithStatus(GameStatus.IN_PROGRESS);
        _mockGameWasRespected(true);

        // Try to update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);

        _assertCurrentAnchorRootEq(root, l2BlockNumber);
    }

    /// @notice Tests that setAnchorState will revert if the game is not respected.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_isNotRespectedGame_fails(uint256 _l2BlockNumber) public {
        // Grab block number of the existing anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.anchors(gameProxy.gameType());

        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, l2BlockNumber, type(uint256).max);

        // Mock the l2BlockNumber call.
        vm.mockCall(address(gameProxy), abi.encodeCall(gameProxy.l2SequenceNumber, ()), abi.encode(_l2BlockNumber));

        _mockGamePastFinalityWithStatus(GameStatus.DEFENDER_WINS);
        _mockGameWasRespected(false);

        // Try to update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);

        _assertAnchorEq(gameProxy.gameType(), root, l2BlockNumber);
    }

    /// @notice Tests that setAnchorState will revert if the game is valid and the game is
    ///         blacklisted.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_blacklistedGame_fails(uint256 _l2BlockNumber) public {
        // Grab block number of the existing anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();

        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, validL2BlockNumber, type(uint256).max);

        _mockGamePastFinalityWithStatus(GameStatus.DEFENDER_WINS);
        _mockGameWasRespected(true);

        // Blacklist the game.
        _blacklistDisputeGameAsGuardian(gameProxy);

        // Update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);

        _assertAnchorEq(gameProxy.gameType(), root, l2BlockNumber);
    }

    /// @notice Tests that setAnchorState will revert if the game is retired.
    /// @param _l2BlockNumber The L2 block number to use for the game.
    function testFuzz_setAnchorState_retiredGame_fails(uint256 _l2BlockNumber) public {
        // Grab block number of the existing anchor root.
        (Hash root, uint256 l2BlockNumber) = anchorStateRegistry.getAnchorRoot();

        // Bound the new block number.
        _l2BlockNumber = bound(_l2BlockNumber, validL2BlockNumber, type(uint256).max);

        _mockGameStatus(GameStatus.DEFENDER_WINS);
        _mockGameWasRespected(true);

        // Set the retirement timestamp.
        _updateRetirementTimestampAsGuardian();

        // Mock the call to createdAt.
        _mockGameCreatedAt(anchorStateRegistry.retirementTimestamp() - 1);

        // Update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);

        _assertAnchorEq(gameProxy.gameType(), root, l2BlockNumber);
    }

    /// @notice Tests that setAnchorState will revert if the superchain is paused.
    function test_setAnchorState_superchainPaused_fails() public {
        // Pause the superchain.
        _setPaused(true);

        // Update the anchor state.
        vm.prank(address(gameProxy));
        vm.expectRevert(IAnchorStateRegistry.AnchorStateRegistry_InvalidAnchorGame.selector);
        anchorStateRegistry.setAnchorState(gameProxy);
    }
}
