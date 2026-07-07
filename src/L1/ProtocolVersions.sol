// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Contracts
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";
import { ReinitializableBase } from "src/universal/ReinitializableBase.sol";
import { Initializable } from "lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol";

// Interfaces
import { ISemver } from "interfaces/universal/ISemver.sol";

/// @custom:proxied true
/// @title ProtocolVersions
/// @notice Security Council-controlled upgrade activation schedule contract.
/// @dev Maintains an ordered registry of upgrades and their L2 activation timestamps.
///      Each upgrade is identified by an ascending numeric `id` equal to its registration
///      index (0, 1, 2, ...). Human-readable names are intentionally kept off-chain: a client
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
///      empty-registry special case. Proof journals bind to `scheduleId`, pinning every proof in a
///      dispute game to the schedule in effect at the game's L1 origin block; cross-chain domain
///      separation is provided by the journal itself, which commits the L2 chain id and registry
///      address alongside `scheduleId`.
///
///      The contract is deployed behind an OP proxy: the implementation constructor disables
///      initializers, and `initialize` (run through the proxy) seeds the hash chain.
contract ProtocolVersions is ProxyAdminOwnedBase, Initializable, ReinitializableBase, ISemver {
    /// @notice A registered upgrade's id, current activation timestamp, and cumulative schedule hash.
    struct Upgrade {
        uint256 id;
        uint64 timestamp;
        bytes32 scheduleId;
    }

    /// @notice Thrown when an upgrade id is not registered.
    error ProtocolVersions_UnknownUpgrade(uint256 id);
    /// @notice Thrown when a protocol version is zero.
    error ProtocolVersions_InvalidProtocolVersion();
    /// @notice Thrown when modifying a timestamp whose activation has already passed.
    error ProtocolVersions_ActivationAlreadyPassed(uint256 id, uint64 activationTimestamp);
    /// @notice Thrown when the caller is not the chainTeam.
    error ProtocolVersions_NotChainTeam();
    /// @notice Thrown when delaying an upgrade that has no scheduled activation.
    error ProtocolVersions_NotScheduled(uint256 id);
    /// @notice Thrown when a new timestamp is not sufficiently later than the current one.
    error ProtocolVersions_DelayMustBeLater(uint64 currentTimestamp, uint64 newTimestamp);
    /// @notice Thrown when scheduleId is read before initialize has been called.
    error ProtocolVersions_NotInitialized();
    /// @notice Thrown when a non-zero timestamp does not provide at least MIN_NOTICE seconds of notice.
    error ProtocolVersions_InsufficientNotice(uint64 timestamp);

    /// @notice Emitted when a new upgrade is registered.
    event UpgradeRegistered(uint256 indexed id);
    /// @notice Emitted when the minimum protocol version clients must run is updated.
    event MinimumProtocolVersionUpdated(uint256 indexed protocolVersion);
    /// @notice Emitted when an upgrade's activation timestamp is set, cleared, or delayed.
    event TimestampSet(uint256 indexed id, uint256 timestamp);
    /// @notice Emitted when the schedule commitment changes.
    event ScheduleIdUpdated(bytes32 indexed newScheduleId, uint256 indexed blockNumber);
    /// @notice Emitted when the chainTeam role changes.
    event ChainTeamUpdated(address indexed previousChainTeam, address indexed newChainTeam);

    /// @notice Semantic version.
    /// @custom:semver 1.0.0
    string public constant version = "1.0.0";

    /// @notice Minimum notice period required when scheduling or modifying an activation timestamp.
    uint64 public constant MIN_NOTICE = 1 hours;

    /// @notice Activation timestamp for each registered upgrade, indexed by upgrade id (0 = not scheduled).
    ///         An upgrade id is registered iff it is a valid index into this array.
    uint64[] internal _timestamps;

    /// @notice Hash chain links. Element 0 is the seed (`bytes32(0)`), pushed in `initialize`;
    ///         element `i + 1` is the cumulative hash through upgrade `i`:
    ///         _upgradeScheduleId[i + 1] = keccak256(abi.encode(_upgradeScheduleId[i], i, _timestamps[i])).
    ///         Stored per-link so that changing upgrade j's timestamp recomputes only j..n-1 links
    ///         rather than the entire chain. Non-empty iff the contract has been initialized.
    bytes32[] internal _upgradeScheduleId;

    /// @notice The minimum protocol version clients must run. Settable by the owner via
    ///         `setMinimumProtocolVersion`. Informational only — read off-chain by clients;
    ///         not part of the scheduleId commitment.
    uint256 public minimumProtocolVersion;

    /// @notice Address allowed to delay (push out) already-scheduled activation timestamps.
    /// @dev Appointed and revocable by the owner. This is a restricted secondary role: it can
    ///      only move an already-scheduled, not-yet-activated upgrade further into the future via
    ///      `delayTimestamp`. It cannot register upgrades, clear timestamps, pull an activation
    ///      earlier, or schedule a brand-new activation. Unset (zero) by default.
    address public chainTeam;

    /// @notice Disables initializers on the implementation so it can only be used behind a proxy.
    constructor() ReinitializableBase(1) {
        _disableInitializers();
    }

    /// @notice Initializes the registry by seeding the hash chain. Callable only by the ProxyAdmin
    ///         or its owner.
    function initialize() external reinitializer(initVersion()) {
        // Initialization transactions must come from the ProxyAdmin or its owner.
        _assertOnlyProxyAdminOrProxyAdminOwner();

        // Seed the hash chain at index 0. Keeping the seed as the first array element lets
        // `scheduleId` and `_refreshScheduleId` avoid an empty-registry special case, and makes a
        // non-empty array double as the "initialized" flag.
        _upgradeScheduleId.push(bytes32(0));

        emit ScheduleIdUpdated(bytes32(0), block.number);
    }

    /// @notice Restricts a call to the ProxyAdmin owner.
    modifier onlyProxyAdminOwner() {
        _assertOnlyProxyAdminOwner();
        _;
    }

    /// @notice Returns the canonical schedule commitment.
    /// @return The current scheduleId hash.
    function scheduleId() external view returns (bytes32) {
        uint256 n = _upgradeScheduleId.length;
        if (n == 0) revert ProtocolVersions_NotInitialized();
        return _upgradeScheduleId[n - 1];
    }

    /// @notice Returns the full ordered schedule: every registered upgrade with its current
    ///         activation timestamp and cumulative schedule hash. The array index equals the
    ///         upgrade `id`; names are resolved off-chain.
    /// @dev Calling via eth_call is gas-free; no transaction is submitted.
    /// @return schedule_ Ordered array of Upgrade structs, one per registered upgrade.
    function getSchedule() external view returns (Upgrade[] memory schedule_) {
        uint256 n = _timestamps.length;
        schedule_ = new Upgrade[](n);
        for (uint256 i = 0; i < n; i++) {
            // Upgrade i's cumulative hash lives at index i + 1 (index 0 is the seed).
            schedule_[i] = Upgrade({ id: i, timestamp: _timestamps[i], scheduleId: _upgradeScheduleId[i + 1] });
        }
    }

    /// @notice Registers a new upgrade, assigning it the next ascending id. Owner only.
    /// @dev The upgrade starts unscheduled (timestamp 0). Registration extends the scheduleId
    ///      chain with the new (id, timestamp=0) link.
    /// @return id The ascending id assigned to the newly registered upgrade.
    function registerUpgrade() external onlyProxyAdminOwner returns (uint256 id) {
        if (_upgradeScheduleId.length == 0) revert ProtocolVersions_NotInitialized();
        id = _timestamps.length;
        _timestamps.push(0);
        // Reserve the link slot for this upgrade at index id + 1; _refreshScheduleId fills it.
        _upgradeScheduleId.push();
        emit UpgradeRegistered(id);
        _refreshScheduleId(id);
    }

    /// @notice Sets the minimum protocol version clients must run. Owner (Security Council) only.
    /// @dev Informational signal for off-chain clients; independent of the upgrade schedule and NOT
    ///      part of the scheduleId commitment, so it can be updated at any time without shifting any
    ///      proof binding.
    /// @param protocolVersion Packed semver uint256 (must be non-zero).
    function setMinimumProtocolVersion(uint256 protocolVersion) external onlyProxyAdminOwner {
        if (protocolVersion == 0) revert ProtocolVersions_InvalidProtocolVersion();
        minimumProtocolVersion = protocolVersion;
        emit MinimumProtocolVersionUpdated(protocolVersion);
    }

    /// @notice Sets the activation timestamp for one upgrade by id. Pass 0 to clear.
    /// @dev The activation timestamp must be at least MIN_NOTICE seconds in the future and the
    ///      upgrade must not have already activated. Pass 0 to remove a not-yet-activated scheduled
    ///      timestamp; reverts if the upgrade has already passed its activation time.
    /// @param id         The upgrade to schedule.
    /// @param timestamp  Future Unix timestamp for L2 activation (must be >= block.timestamp + MIN_NOTICE), or 0 to
    /// clear.
    function setTimestamp(uint256 id, uint64 timestamp) external onlyProxyAdminOwner {
        _assertRegistered(id);
        uint64 current = _timestamps[id];
        if (current != 0 && uint64(block.timestamp) >= current) {
            revert ProtocolVersions_ActivationAlreadyPassed(id, current);
        }
        if (timestamp != 0 && timestamp < uint64(block.timestamp) + MIN_NOTICE) {
            revert ProtocolVersions_InsufficientNotice(timestamp);
        }
        if (current == timestamp) return;
        _writeTimestamp(id, timestamp);
    }

    /// @notice Appoints, replaces, or clears (set to zero) the chainTeam role. Owner only.
    /// @param newChainTeam New chainTeam address, or address(0) to revoke the role.
    function setChainTeam(address newChainTeam) external onlyProxyAdminOwner {
        emit ChainTeamUpdated(chainTeam, newChainTeam);
        chainTeam = newChainTeam;
    }

    /// @notice Pushes an already-scheduled upgrade's activation timestamp further into the future.
    ///         Can only be called by the chainTeam.
    /// @dev The upgrade must already have a non-zero activation timestamp that has not yet passed,
    ///      and `newTimestamp` must be strictly later than the current value. This role can only
    ///      delay an activation; it cannot pull one earlier, clear it, or schedule a new one — use
    ///      the owner's `setTimestamp` for those. Because `current` is in the future and `newTimestamp`
    ///      is later still, the new value is always in the future.
    /// @param id            The upgrade whose activation to delay.
    /// @param newTimestamp  New activation timestamp, must be strictly later than the current one.
    function delayTimestamp(uint256 id, uint64 newTimestamp) external {
        if (msg.sender != chainTeam) revert ProtocolVersions_NotChainTeam();
        _assertRegistered(id);
        uint64 current = _timestamps[id];

        // The upgrade must already have a scheduled activation to delay.
        if (current == 0) revert ProtocolVersions_NotScheduled(id);
        // Cannot delay an activation that has already passed.
        if (uint64(block.timestamp) >= current) revert ProtocolVersions_ActivationAlreadyPassed(id, current);
        // The role can only push the activation later, never to the same time or earlier.
        if (newTimestamp <= current) revert ProtocolVersions_DelayMustBeLater(current, newTimestamp);
        // The new timestamp must also provide at least MIN_NOTICE seconds of notice from now.
        uint64 minFloor = uint64(block.timestamp) + MIN_NOTICE;
        if (newTimestamp < minFloor) revert ProtocolVersions_DelayMustBeLater(minFloor, newTimestamp);
        _writeTimestamp(id, newTimestamp);
    }

    /// @dev Writes newTs, emits TimestampSet, and refreshes the hash chain. All validation is
    ///      the caller's responsibility.
    function _writeTimestamp(uint256 id, uint64 newTs) internal {
        _timestamps[id] = newTs;
        emit TimestampSet(id, newTs);
        _refreshScheduleId(id);
    }

    /// @dev Recomputes the per-upgrade cumulative hash chain starting from upgrade `startIndex` and
    ///      bubbles the result through all subsequent registered upgrades. Cost is O(n-startIndex).
    ///      Only ever called after a state change (registration, or a timestamp that actually moved),
    ///      so the resulting tail is always a new scheduleId.
    function _refreshScheduleId(uint256 startIndex) internal {
        uint256 n = _timestamps.length;

        // _upgradeScheduleId[startIndex] is the link preceding upgrade `startIndex` (the seed when
        // startIndex == 0). Recompute from startIndex onward, storing each link at index i + 1.
        bytes32 prev = _upgradeScheduleId[startIndex];
        for (uint256 i = startIndex; i < n; i++) {
            prev = keccak256(abi.encode(prev, i, _timestamps[i]));
            _upgradeScheduleId[i + 1] = prev;
        }

        emit ScheduleIdUpdated(prev, block.number);
    }

    /// @dev Reverts if `id` is not a registered upgrade.
    function _assertRegistered(uint256 id) internal view {
        if (id >= _timestamps.length) revert ProtocolVersions_UnknownUpgrade(id);
    }
}
