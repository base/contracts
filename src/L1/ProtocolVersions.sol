// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Contracts
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";

// Interfaces
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

/// @title ProtocolVersions
/// @notice Security Council-controlled upgrade activation schedule contract.
/// @dev Maintains an ordered registry of upgrades and their L2 activation timestamps.
///      The canonical schedule commitment (`scheduleId`) is the tail of a per-upgrade hash chain:
///
///        seed                 = keccak256(abi.encode(l2ChainId, address(this)))
///        upgradeScheduleId[0] = keccak256(abi.encode(seed,                 key_0, timestamp_0))
///        upgradeScheduleId[i] = keccak256(abi.encode(upgradeScheduleId[i-1], key_i, timestamp_i))
///        scheduleId           = upgradeScheduleId[n-1]  (or seed when n == 0)
///
///      where timestamp_i is the upgrade's current activation timestamp (0 = not yet scheduled).
///      Changing any upgrade's timestamp recomputes its link and all subsequent links, making
///      scheduleId fully reproducible from (l2ChainId, address, ordered keys, current timestamps).
///      Proof journals bind to `scheduleId`, pinning every proof in a dispute game to the schedule
///      that was in effect at the game's L1 origin block.
///
///      An `upgradeId` is a human-readable string (e.g. "canyon"); each is packed into a
///      bytes32 storage `key`. The registry starts empty; all upgrades are added via
///      `registerUpgrade` owner calls.
contract ProtocolVersions is ProxyAdminOwnedBase, IProtocolVersions {
    /// @notice Semantic version.
    /// @custom:semver 1.0.0
    string public constant version = "1.0.0";

    /// @notice Minimum notice period required when scheduling or modifying an activation timestamp.
    uint64 public constant MIN_NOTICE = 1 hours;

    /// @notice Hash chain seed: keccak256(abi.encode(l2ChainId, address(this))). Cached to avoid
    ///         recomputing on every _refreshScheduleId call.
    bytes32 internal immutable _seed;

    /// @notice Ordered list of registered upgrade keys.
    bytes32[] internal _upgradeKeys;

    /// @notice Cumulative schedule hash up to and including each registered upgrade.
    ///         upgradeScheduleId[i] = keccak256(abi.encode(upgradeScheduleId[i-1], key_i, timestamps[key_i])).
    ///         Stored per-key so that changing upgrade j's timestamp requires recomputing only
    ///         j..n-1 links rather than the entire chain.
    mapping(bytes32 => bytes32) internal _upgradeScheduleId;

    /// @notice Activation timestamp for each registered upgrade key (0 = not scheduled).
    mapping(bytes32 => uint64) internal _timestamps;

    /// @notice Registration sentinel for each upgrade key.
    mapping(bytes32 => bool) internal _registered;

    /// @notice The protocol version set at the most recent `registerUpgrade` call.
    uint256 public latestProtocolVersion;

    /// @notice Address allowed to delay (push out) already-scheduled activation timestamps.
    /// @dev Appointed and revocable by the owner. This is a restricted secondary role: it can
    ///      only move an already-scheduled, not-yet-activated upgrade further into the future via
    ///      `delayTimestamp`. It cannot register upgrades, clear timestamps, pull an activation
    ///      earlier, or schedule a brand-new activation. Unset (zero) by default.
    address public chainTeam;

    /// @notice Deploys the contract and sets the chain ID.
    /// @param _l2ChainId L2 chain ID whose schedule is being committed.
    constructor(uint256 _l2ChainId) {
        if (_l2ChainId == 0) revert ProtocolVersions_InvalidL2ChainId();
        _seed = keccak256(abi.encode(_l2ChainId, address(this)));
    }

    /// @notice Restricts a call to the ProxyAdmin owner.
    modifier onlyProxyAdminOwner() {
        _assertOnlyProxyAdminOwner();
        _;
    }

    /// @notice Returns the canonical schedule commitment.
    /// @return The current scheduleId hash.
    function scheduleId() external view returns (bytes32) {
        uint256 n = _upgradeKeys.length;
        return n == 0 ? _seed : _upgradeScheduleId[_upgradeKeys[n - 1]];
    }

    /// @notice Returns the activation timestamp for `upgradeId` (0 = not scheduled).
    /// @param upgradeId The human-readable upgrade identifier string.
    /// @return The scheduled L2 activation timestamp, or 0 if unscheduled.
    function getTimestamp(string calldata upgradeId) external view returns (uint256) {
        return _timestamps[_registeredKey(upgradeId)];
    }

    /// @notice Returns the full ordered schedule: every registered upgrade with its current
    ///         activation timestamp and cumulative schedule hash.
    /// @dev Calling via eth_call is gas-free; no transaction is submitted.
    /// @return schedule_ Ordered array of Upgrade structs, one per registered upgrade.
    function getSchedule() external view returns (Upgrade[] memory schedule_) {
        uint256 n = _upgradeKeys.length;
        schedule_ = new Upgrade[](n);
        for (uint256 i = 0; i < n; i++) {
            bytes32 key = _upgradeKeys[i];
            schedule_[i] = Upgrade({
                name: _nameFromKey(key),
                timestamp: _timestamps[key],
                scheduleId: _upgradeScheduleId[key]
            });
        }
    }

    /// @notice Registers a new upgrade by upgradeId with its protocol version. Owner only.
    /// @dev The upgradeId is packed into a bytes32 key and `latestProtocolVersion` is updated.
    ///      Registration extends the scheduleId chain with the new (key, timestamp=0) link.
    /// @param upgradeId       Human-readable upgrade name (1–32 bytes, e.g. "canyon").
    /// @param protocolVersion Packed semver uint256 for this upgrade (must be non-zero).
    function registerUpgrade(string calldata upgradeId, uint256 protocolVersion) external onlyProxyAdminOwner {
        if (protocolVersion == 0) revert ProtocolVersions_InvalidProtocolVersion();
        bytes32 key = _keyFromUpgradeId(upgradeId);
        if (_registered[key]) revert ProtocolVersions_UpgradeAlreadyRegistered(key);

        uint256 index = _upgradeKeys.length;
        _upgradeKeys.push(key);
        _registered[key] = true;
        latestProtocolVersion = protocolVersion;
        emit UpgradeRegistered(key, index, upgradeId, protocolVersion);
        _refreshScheduleId(key);
    }

    /// @notice Sets the activation timestamp for one upgrade by upgradeId. Pass 0 to clear.
    /// @dev The activation timestamp must be at least MIN_NOTICE seconds in the future and the
    ///      upgrade must not have already activated. Pass 0 to remove a not-yet-activated scheduled
    ///      timestamp; reverts if the upgrade has already passed its activation time.
    /// @param upgradeId  The upgrade to schedule.
    /// @param timestamp  Future Unix timestamp for L2 activation (must be >= block.timestamp + MIN_NOTICE), or 0 to
    /// clear.
    function setTimestamp(string calldata upgradeId, uint64 timestamp) external onlyProxyAdminOwner {
        bytes32 key = _registeredKey(upgradeId);
        uint64 current = _timestamps[key];
        // Cannot modify a timestamp that has already activated.
        if (current != 0 && uint64(block.timestamp) >= current) {
            revert ProtocolVersions_ActivationAlreadyPassed(key, current);
        }
        // Non-zero timestamps must be at least MIN_NOTICE seconds in the future.
        if (timestamp != 0 && timestamp < uint64(block.timestamp) + MIN_NOTICE) {
            revert ProtocolVersions_DelayMustBeLater(current, timestamp);
        }
        if (current == timestamp) return;
        _timestamps[key] = timestamp;
        emit TimestampSet(key, timestamp);
        _refreshScheduleId(key);
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
    /// @param upgradeId     The upgrade whose activation to delay.
    /// @param newTimestamp  New activation timestamp, must be strictly later than the current one.
    function delayTimestamp(string calldata upgradeId, uint64 newTimestamp) external {
        if (msg.sender != chainTeam) revert ProtocolVersions_NotChainTeam();
        bytes32 key = _registeredKey(upgradeId);
        uint64 current = _timestamps[key];

        // The upgrade must already have a scheduled activation to delay.
        if (current == 0) revert ProtocolVersions_NotScheduled(key);
        // Cannot modify an activation that has already passed.
        if (uint64(block.timestamp) >= current) {
            revert ProtocolVersions_ActivationAlreadyPassed(key, current);
        }
        // The role can only push the activation later and must be greater than the MIN_NOTICE buffer, never earlier or to the same time.
        if (newTimestamp <= current || newTimestamp < uint64(block.timestamp) + MIN_NOTICE) revert ProtocolVersions_DelayMustBeLater(current, newTimestamp);

        _timestamps[key] = newTimestamp;
        emit TimestampSet(key, newTimestamp);
        _refreshScheduleId(key);
    }

    /// @dev Recomputes the per-upgrade cumulative hash chain starting from `changedKey` and bubbles
    ///      the result through all subsequent registered upgrades. Cost is O(n-j) where j is the
    ///      0-based registration index of `changedKey`.
    function _refreshScheduleId(bytes32 changedKey) internal {
        uint256 n = _upgradeKeys.length;

        // Locate changedKey and initialise prev to the hash of the preceding link (or seed).
        uint256 startIndex;
        for (startIndex = 0; startIndex < n; startIndex++) {
            if (_upgradeKeys[startIndex] == changedKey) break;
        }
        bytes32 prev = startIndex == 0 ? _seed : _upgradeScheduleId[_upgradeKeys[startIndex - 1]];
        // Recompute from startIndex onward, storing each link.
        for (uint256 i = startIndex; i < n; i++) {
            bytes32 k = _upgradeKeys[i];
            prev = keccak256(abi.encode(prev, k, _timestamps[k]));
            _upgradeScheduleId[k] = prev;
        }
    }

    /// @dev Resolves an upgradeId string to its storage key, reverting if not registered.
    function _registeredKey(string calldata upgradeId) internal view returns (bytes32 key) {
        key = _keyFromUpgradeId(upgradeId);
        if (!_registered[key]) revert ProtocolVersions_UnknownUpgradeName(upgradeId);
    }

    /// @dev Packs a UTF-8 upgradeId string (1–32 bytes) into a bytes32 key. Shorter strings are
    ///      zero-padded on the right; strings differing only by trailing null bytes map to the same
    ///      key. Callers are expected to use printable ASCII names only.
    function _keyFromUpgradeId(string calldata upgradeId) internal pure returns (bytes32 key) {
        bytes memory raw = bytes(upgradeId);
        if (raw.length == 0 || raw.length > 32) revert ProtocolVersions_InvalidUpgradeId();
        key = bytes32(raw);
    }

    /// @dev Recovers the original upgradeId string from its bytes32 key by stripping trailing
    ///      zero bytes. Correct for printable ASCII names, which contain no embedded null bytes.
    function _nameFromKey(bytes32 key) internal pure returns (string memory) {
        uint256 len = 32;
        while (len > 0 && key[len - 1] == 0) len--;
        bytes memory b = new bytes(len);
        for (uint256 i = 0; i < len; i++) {
            b[i] = key[i];
        }
        return string(b);
    }
}
