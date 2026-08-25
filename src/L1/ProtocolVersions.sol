// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Contracts
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";
import { ReinitializableBase } from "src/universal/ReinitializableBase.sol";
import { Initializable } from "lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol";
import { ProtocolVersionsConfig } from "src/libraries/ProtocolVersionsConfig.sol";

// Interfaces
import { ISemver } from "interfaces/universal/ISemver.sol";

/// @custom:proxied true
/// @title ProtocolVersions
/// @notice Upgrade activation schedule contract, controlled by the ProxyAdmin owner (2-of-2 multisig).
/// @dev Maintains an ordered registry of upgrades and their L2 activation timestamps.
///      Each upgrade is identified by an ascending numeric `id` equal to its registration
///      index (0, 1, 2, ...). Human-readable names are intentionally kept offchain: a client
///      maps `id` => name via its own static configuration. Because the registry is strictly
///      append-only (upgrades are never removed or reordered), an `id` permanently refers to the
///      same upgrade.
///
///      The canonical schedule commitment (`scheduleId`) is the tail of a hash chain that is
///      seeded at index 0 and extended by one link per registered upgrade:
///
///        _upgradeScheduleId[0]   = bytes32(0)                                              (seed)
///        _upgradeScheduleId[i+1] = keccak256(abi.encode(_upgradeScheduleId[i], i, timestamp_i))
///        scheduleId              = _upgradeScheduleId[last]
///
///      where timestamp_i is upgrade i's current activation timestamp (0 = not yet scheduled).
///      Changing any upgrade's timestamp recomputes its link and all subsequent links, making
///      scheduleId fully reproducible from (upgrade count, current timestamps). Keeping the seed as
///      the array's first element lets both `scheduleId` and the refresh loop avoid an
///      empty-registry special case.
///
///      `AggregateVerifier` pins `activatedScheduleId` at game initialization using the ending L2 claim
///      timestamp. Every proof journal commits to that pinned value.
///
///      `scheduleId` commits only to registry contents. `AggregateVerifier` journals pair it with
///      `CONFIG_HASH`; they do not separately encode `L2_CHAIN_ID` or the `PROTOCOL_VERSIONS` address.
///      Any deployment-level domain separation must therefore be part of `CONFIG_HASH`.
///
///      The contract is deployed behind an OP proxy: the implementation constructor disables
///      initializers, and `initialize` (run through the proxy) seeds the hash chain.
contract ProtocolVersions is ProxyAdminOwnedBase, Initializable, ReinitializableBase, ISemver {
    /// @notice Minimum notice period required when changing a preexisting activation timestamp.
    uint64 public constant MIN_NOTICE = 1 hours;

    /// @notice Window before a scheduled activation during which that timestamp can no longer change.
    /// @dev Sized to the post-Fjord maximum sequencer drift: an L2 block may carry a timestamp up to
    ///      1800 seconds ahead of its L1 origin, so from L1 time `activation - FREEZE_WINDOW` onwards
    ///      the sequencer can already have produced the block that activates the upgrade. Freezing at
    ///      that point makes L1's answer to "was this upgrade active at a given L2 timestamp" final
    ///      before any L2 block can depend on it, rather than leaving it mutable until activation.
    uint64 public constant FREEZE_WINDOW = 30 minutes;

    /// @notice Activation timestamp for each registered upgrade, indexed by upgrade id (0 = not scheduled).
    ///         An upgrade id is registered iff it is a valid index into this array.
    /// @dev Every loop in this contract iterates this array. Its length is deliberately uncapped: it only
    ///      grows via owner-gated `registerUpgrade`, one entry per hardfork, so the unbounded iteration
    ///      poses no gas exhaustion risk.
    uint64[] private _timestamps;

    /// @notice Hash chain links. Element 0 is the seed (`bytes32(0)`), pushed in `initialize`;
    ///         element `i + 1` is the cumulative hash through upgrade `i`:
    ///         _upgradeScheduleId[i + 1] = keccak256(abi.encode(_upgradeScheduleId[i], i, _timestamps[i])).
    ///         Stored per-link so that changing upgrade j's timestamp recomputes only j..n-1 links
    ///         rather than the entire chain. Non-empty iff the contract has been initialized.
    bytes32[] private _upgradeScheduleId;

    /// @notice The minimum protocol version clients must run. Settable by the owner via
    ///         `setMinimumProtocolVersion`. Informational only — read offchain by clients;
    ///         not part of the scheduleId commitment.
    uint256 public minimumProtocolVersion;

    /// @notice Address allowed to delay (push out) already-scheduled activation timestamps.
    /// @dev Appointed and revocable by the owner. This is a restricted secondary role: it can
    ///      only move an already-scheduled, not-yet-activated upgrade further into the future via
    ///      `delayTimestamp`. It cannot register upgrades, clear timestamps, pull an activation
    ///      earlier, or schedule a brand-new activation. Unset (zero) by default.
    address public incidentResponder;

    /// @notice Emitted when a new upgrade is registered.
    event UpgradeRegistered(uint256 indexed id);
    /// @notice Emitted when the minimum protocol version clients must run is updated.
    event MinimumProtocolVersionUpdated(uint256 indexed protocolVersion);
    /// @notice Emitted when an upgrade's activation timestamp is set, cleared, or delayed.
    event TimestampSet(uint256 indexed id, uint256 timestamp);
    /// @notice Emitted when the schedule commitment changes.
    event ScheduleIdUpdated(bytes32 indexed newScheduleId);
    /// @notice Emitted when the incidentResponder role changes.
    event IncidentResponderUpdated(address indexed previousIncidentResponder, address indexed newIncidentResponder);

    /// @notice Thrown when an upgrade id is not registered.
    error ProtocolVersions_UnknownUpgrade(uint256 id);
    /// @notice Thrown when a protocol version is zero.
    error ProtocolVersions_InvalidProtocolVersion();
    /// @notice Thrown when modifying a timestamp whose activation has already passed.
    error ProtocolVersions_ActivationAlreadyPassed(uint256 id, uint64 activationTimestamp);
    /// @notice Thrown when modifying a timestamp that is within FREEZE_WINDOW of its activation.
    error ProtocolVersions_ActivationFrozen(uint256 id, uint64 activationTimestamp);
    /// @notice Thrown when the caller is not the incidentResponder.
    error ProtocolVersions_NotIncidentResponder();
    /// @notice Thrown when delaying an upgrade that has no scheduled activation.
    error ProtocolVersions_NotScheduled(uint256 id);
    /// @notice Thrown when a new timestamp is not sufficiently later than the current one.
    error ProtocolVersions_DelayMustBeLater(uint64 currentTimestamp, uint64 newTimestamp);
    /// @notice Thrown when scheduling a zero-valued static hole below a scheduled successor.
    error ProtocolVersions_StaticScheduleHole(uint256 id, uint256 nextScheduledId);
    /// @notice Thrown when a non-zero timestamp is less than the previous scheduled upgrade.
    error ProtocolVersions_TimestampNotAfterPrevious(
        uint256 id, uint256 previousId, uint64 previousTimestamp, uint64 timestamp
    );
    /// @notice Thrown when a non-zero timestamp is greater than the next scheduled upgrade.
    error ProtocolVersions_TimestampNotBeforeNext(uint256 id, uint256 nextId, uint64 timestamp, uint64 nextTimestamp);
    /// @notice Thrown when scheduleId is read before initialize has been called.
    error ProtocolVersions_NotInitialized();
    /// @notice Thrown when a non-zero timestamp does not provide at least MIN_NOTICE seconds of notice.
    error ProtocolVersions_InsufficientNotice(uint64 timestamp);

    /// @notice Disables initializers on the implementation so it can only be used behind a proxy.
    constructor() ReinitializableBase(1) {
        _disableInitializers();
    }

    /// @notice Number of contract-backed upgrades a non-empty initial schedule must contain.
    function INITIAL_UPGRADE_COUNT() public pure returns (uint256) {
        return ProtocolVersionsConfig.INITIAL_UPGRADE_COUNT;
    }

    /// @notice Initializes the registry by seeding the hash chain, importing any preexisting upgrade
    ///         schedule, and appointing the initial incidentResponder. Callable only by the
    ///         ProxyAdmin or its owner.
    /// @dev `_initialSchedule` is the only path that can enter an activation which is not at least
    ///      MIN_NOTICE in the future. It exists so a chain that already has a hardfork history can be
    ///      represented faithfully at deployment, while it is still impossible for any proof game to
    ///      have pinned a commitment from this registry. Every later write goes through
    ///      `registerUpgrade`, `setTimestamp`, or `delayTimestamp`, which together guarantee that an
    ///      activation is never created or moved once L1 is within FREEZE_WINDOW of it.
    /// @param _incidentResponder Initial incidentResponder allowed to delay activations, or address(0) to leave unset.
    /// @param _initialSchedule   Activation timestamps for already-known upgrades, ordered by ascending
    ///                           upgrade id, using 0 for an upgrade that is registered but unscheduled.
    ///                           Pass an empty array for a chain with no upgrade history; otherwise the
    ///                           array must contain exactly INITIAL_UPGRADE_COUNT entries.
    function initialize(
        address _incidentResponder,
        uint64[] calldata _initialSchedule
    )
        external
        reinitializer(initVersion())
    {
        // Initialization transactions must come from the ProxyAdmin or its owner.
        _assertOnlyProxyAdminOrProxyAdminOwner();
        ProtocolVersionsConfig.assertValidInitialScheduleLength(_initialSchedule.length);

        // Seed the hash chain at index 0. Keeping the seed as the first array element lets
        // `scheduleId` and `_refreshScheduleId` avoid an empty-registry special case, and makes a
        // non-empty array double as the "initialized" flag.
        _upgradeScheduleId.push(bytes32(0));

        for (uint256 id = 0; id < _initialSchedule.length; id++) {
            uint64 timestamp = _initialSchedule[id];
            // `id` is the index about to be appended, so this reads only the entries already imported.
            _assertTimestampAfterPrevious(id, timestamp);
            _timestamps.push(timestamp);
            _upgradeScheduleId.push();
            emit UpgradeRegistered(id);
            if (timestamp != 0) emit TimestampSet(id, timestamp);
        }

        // Builds the same chain the equivalent sequence of `registerUpgrade` calls would, in one
        // pass. With an empty import this just re-emits the seed as the current commitment.
        _refreshScheduleId(0);

        incidentResponder = _incidentResponder;
        emit IncidentResponderUpdated(address(0), _incidentResponder);
    }

    /// @notice Registers a new upgrade, assigning it the next ascending id, optionally scheduling its
    ///         activation and bumping the minimum protocol version in the same call. ProxyAdmin owner only.
    /// @dev Pass `timestamp` 0 to register without scheduling (schedule later via `setTimestamp`), or
    ///      a non-zero value to register and schedule at once. Either way registration extends the
    ///      scheduleId chain with the new upgrade's link. A non-zero value must clear MIN_NOTICE, the
    ///      same floor `setTimestamp` applies: appending an activation that L1 is already within
    ///      FREEZE_WINDOW of would change `activatedScheduleId` for L2 timestamps the sequencer may
    ///      already have produced, which is exactly what the freeze exists to prevent.
    /// @param timestamp Unix activation timestamp (must be >= block.timestamp + MIN_NOTICE), or 0 to
    ///                  leave the upgrade unscheduled.
    /// @param minProtocolVersion New minimum protocol version to set at registration, or 0 to leave
    ///                  the current minimum unchanged. Must fit in 128 bits if non-zero.
    /// @return The ascending id assigned to the newly registered upgrade.
    function registerUpgrade(uint64 timestamp, uint256 minProtocolVersion) external returns (uint256) {
        _assertOnlyProxyAdminOwner();
        if (_upgradeScheduleId.length == 0) revert ProtocolVersions_NotInitialized();
        uint256 id = _timestamps.length;
        if (timestamp != 0 && timestamp < uint64(block.timestamp) + MIN_NOTICE) {
            revert ProtocolVersions_InsufficientNotice(timestamp);
        }
        _assertTimestampAfterPrevious(id, timestamp);
        _timestamps.push(0);
        // Reserve the link slot for this upgrade at index id + 1.
        _upgradeScheduleId.push();
        emit UpgradeRegistered(id);
        if (timestamp == 0) {
            // Register-only: commit the new (id, 0) link.
            _refreshScheduleId(id);
        } else {
            // Register and schedule at once via the shared pure-write helper.
            _writeTimestamp(id, timestamp);
        }
        // Optionally bump the global minimum protocol version in the same call (0 = leave unchanged).
        if (minProtocolVersion != 0) {
            if (minProtocolVersion > type(uint128).max) revert ProtocolVersions_InvalidProtocolVersion();
            _writeMinimumProtocolVersion(minProtocolVersion);
        }
        return id;
    }

    /// @notice Sets the minimum protocol version clients must run. ProxyAdmin owner only.
    /// @dev Informational signal for offchain clients; independent of the upgrade schedule and NOT
    ///      part of the scheduleId commitment, so it can be updated at any time without shifting any
    ///      proof binding.
    /// @param protocolVersion Packed semver uint256 (must be non-zero and fit in 128 bits).
    function setMinimumProtocolVersion(uint256 protocolVersion) external {
        _assertOnlyProxyAdminOwner();
        if (protocolVersion == 0) revert ProtocolVersions_InvalidProtocolVersion();
        if (protocolVersion > type(uint128).max) revert ProtocolVersions_InvalidProtocolVersion();
        _writeMinimumProtocolVersion(protocolVersion);
    }

    /// @notice Sets the activation timestamp for one upgrade by id. Pass 0 to clear.
    /// @dev The activation timestamp must be at least MIN_NOTICE seconds in the future, and any
    ///      preexisting activation must still be more than FREEZE_WINDOW away. Pass 0 to remove a
    ///      scheduled timestamp; reverts if the upgrade has already activated or is inside its
    ///      freeze window, or if a later upgrade remains scheduled.
    /// @param id         The upgrade to schedule.
    /// @param timestamp  Future Unix timestamp for L2 activation (must be >= block.timestamp + MIN_NOTICE), or 0 to
    /// clear.
    function setTimestamp(uint256 id, uint64 timestamp) external {
        _assertOnlyProxyAdminOwner();
        _assertRegistered(id);
        uint64 current = _timestamps[id];
        if (current == timestamp) return;
        if (current != 0) _assertNotFrozen(id, current);
        if (timestamp != 0 && timestamp < uint64(block.timestamp) + MIN_NOTICE) {
            revert ProtocolVersions_InsufficientNotice(timestamp);
        }
        if (timestamp == 0 || current == 0) _assertNoScheduledSuccessor(id);
        if (timestamp != 0) {
            _assertTimestampAfterPrevious(id, timestamp);
            _assertTimestampBeforeNext(id, timestamp);
        }
        _writeTimestamp(id, timestamp);
    }

    /// @notice Appoints, replaces, or clears (set to zero) the incidentResponder role. ProxyAdmin owner only.
    /// @param newIncidentResponder New incidentResponder address, or address(0) to revoke the role.
    function setIncidentResponder(address newIncidentResponder) external {
        _assertOnlyProxyAdminOwner();
        emit IncidentResponderUpdated(incidentResponder, newIncidentResponder);
        incidentResponder = newIncidentResponder;
    }

    /// @notice Pushes an already-scheduled upgrade's activation timestamp further into the future.
    ///         Can only be called by the incidentResponder.
    /// @dev The upgrade must already have a non-zero activation timestamp that is still more than
    ///      FREEZE_WINDOW away, and `newTimestamp` must be strictly later than the current value.
    ///      This role can only delay an activation; it cannot pull one earlier, clear it, or schedule
    ///      a new one — use the owner's `setTimestamp` for those. Because `current` is in the future
    ///      and `newTimestamp` is later still, the new value is always in the future.
    /// @param id            The upgrade whose activation to delay.
    /// @param newTimestamp  New activation timestamp, must be strictly later than the current one.
    function delayTimestamp(uint256 id, uint64 newTimestamp) external {
        if (msg.sender != incidentResponder) revert ProtocolVersions_NotIncidentResponder();
        _assertRegistered(id);
        uint64 current = _timestamps[id];

        // The upgrade must already have a scheduled activation to delay.
        if (current == 0) revert ProtocolVersions_NotScheduled(id);
        // Cannot delay an activation that has passed or is already inside its freeze window.
        _assertNotFrozen(id, current);
        // The role can only push the activation later, never to the same time or earlier.
        if (newTimestamp <= current) revert ProtocolVersions_DelayMustBeLater(current, newTimestamp);
        // The new timestamp must also provide at least MIN_NOTICE seconds of notice from now.
        uint64 minFloor = uint64(block.timestamp) + MIN_NOTICE;
        if (newTimestamp < minFloor) revert ProtocolVersions_InsufficientNotice(newTimestamp);
        _assertTimestampBeforeNext(id, newTimestamp);
        _writeTimestamp(id, newTimestamp);
    }

    /// @notice Returns the canonical schedule commitment.
    /// @return The current scheduleId hash.
    function scheduleId() external view returns (bytes32) {
        uint256 n = _upgradeScheduleId.length;
        if (n == 0) revert ProtocolVersions_NotInitialized();
        return _upgradeScheduleId[n - 1];
    }

    /// @notice Returns the schedule commitment as of a specific registered upgrade.
    /// @param id The upgrade id to query.
    /// @return The scheduleId hash committing to upgrades 0 through `id`.
    function scheduleId(uint256 id) external view returns (bytes32) {
        _assertRegistered(id);
        // Upgrade `id`'s cumulative hash lives at index id + 1 (index 0 is the seed).
        return _upgradeScheduleId[id + 1];
    }

    /// @notice Returns the schedule commitment through the highest upgrade active at `l2Timestamp`.
    /// @dev Searches from the newest registration downward, returning the cached cumulative link
    ///      for the first active upgrade. Entries above that upgrade are excluded, while every
    ///      registered entry through it remains committed by the prefix. Non-zero timestamps are
    ///      ordered by id, and zero-valued holes below a scheduled successor cannot be scheduled
    ///      later, preventing inactive prefix holes from moving activated schedule commitments.
    /// @param l2Timestamp Inclusive L2 activation cutoff.
    /// @return Hash-chain commitment through the highest activated upgrade.
    function activatedScheduleId(uint64 l2Timestamp) external view returns (bytes32) {
        if (_upgradeScheduleId.length == 0) revert ProtocolVersions_NotInitialized();

        for (uint256 i = _timestamps.length; i > 0; i--) {
            uint64 activationTimestamp = _timestamps[i - 1];
            if (activationTimestamp != 0 && activationTimestamp <= l2Timestamp) return _upgradeScheduleId[i];
        }

        return _upgradeScheduleId[0];
    }

    /// @notice Returns the activation timestamp for every registered upgrade, ordered by upgrade id
    ///         (0 = not scheduled). The array index equals the upgrade `id`; names are resolved
    ///         offchain, and per-upgrade schedule hashes can be reproduced from these timestamps and
    ///         the seed or read via `scheduleId(id)`.
    /// @dev Calling via eth_call is gas-free; no transaction is submitted.
    /// @return Ordered activation timestamps, one per registered upgrade.
    function getSchedule() external view returns (uint64[] memory) {
        return _timestamps;
    }

    /// @notice Semantic version.
    /// @custom:semver 1.0.0
    function version() public pure virtual returns (string memory) {
        return "1.0.0";
    }

    /// @dev Writes newTs, emits TimestampSet, and refreshes the hash chain. All validation is
    ///      the caller's responsibility.
    function _writeTimestamp(uint256 id, uint64 newTs) private {
        _timestamps[id] = newTs;
        emit TimestampSet(id, newTs);
        _refreshScheduleId(id);
    }

    /// @dev Writes the minimum protocol version and emits the update. Validation is the caller's
    ///      responsibility.
    function _writeMinimumProtocolVersion(uint256 protocolVersion) private {
        minimumProtocolVersion = protocolVersion;
        emit MinimumProtocolVersionUpdated(protocolVersion);
    }

    /// @dev Recomputes the per-upgrade cumulative hash chain starting from upgrade `startIndex` and
    ///      bubbles the result through all subsequent registered upgrades. Cost is O(n-startIndex).
    ///      Only ever called after a state change (registration, or a timestamp that actually moved),
    ///      so the resulting tail is always a new scheduleId.
    function _refreshScheduleId(uint256 startIndex) private {
        uint256 n = _timestamps.length;

        // _upgradeScheduleId[startIndex] is the link preceding upgrade `startIndex` (the seed when
        // startIndex == 0). Recompute from startIndex onward, storing each link at index i + 1.
        bytes32 prev = _upgradeScheduleId[startIndex];
        for (uint256 i = startIndex; i < n; i++) {
            prev = keccak256(abi.encode(prev, i, _timestamps[i]));
            _upgradeScheduleId[i + 1] = prev;
        }

        emit ScheduleIdUpdated(prev);
    }

    /// @dev Requires upgrade `id`'s scheduled activation to still be more than FREEZE_WINDOW away,
    ///      which is the point past which an L2 block carrying that activation may already exist.
    function _assertNotFrozen(uint256 id, uint64 current) private view {
        if (uint64(block.timestamp) >= current) revert ProtocolVersions_ActivationAlreadyPassed(id, current);
        if (uint64(block.timestamp) + FREEZE_WINDOW >= current) revert ProtocolVersions_ActivationFrozen(id, current);
    }

    /// @dev Prevents creating or filling a zero-valued hole below a scheduled successor.
    function _assertNoScheduledSuccessor(uint256 id) private view {
        for (uint256 i = id + 1; i < _timestamps.length; i++) {
            if (_timestamps[i] != 0) revert ProtocolVersions_StaticScheduleHole(id, i);
        }
    }

    /// @dev Requires `timestamp` to be greater than or equal to the closest lower-id scheduled upgrade.
    function _assertTimestampAfterPrevious(uint256 id, uint64 timestamp) private view {
        if (timestamp == 0) return;

        for (uint256 i = id; i > 0; i--) {
            uint64 previous = _timestamps[i - 1];
            if (previous != 0) {
                if (timestamp < previous) {
                    revert ProtocolVersions_TimestampNotAfterPrevious(id, i - 1, previous, timestamp);
                }
                return;
            }
        }
    }

    /// @dev Requires `timestamp` to be less than or equal to the closest higher-id scheduled upgrade.
    function _assertTimestampBeforeNext(uint256 id, uint64 timestamp) private view {
        if (timestamp == 0) return;

        for (uint256 i = id + 1; i < _timestamps.length; i++) {
            uint64 next = _timestamps[i];
            if (next != 0) {
                if (timestamp > next) revert ProtocolVersions_TimestampNotBeforeNext(id, i, timestamp, next);
                return;
            }
        }
    }

    /// @dev Reverts if `id` is not a registered upgrade.
    function _assertRegistered(uint256 id) private view {
        if (id >= _timestamps.length) revert ProtocolVersions_UnknownUpgrade(id);
    }
}
