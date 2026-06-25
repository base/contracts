// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Contracts
import { Ownable } from "lib/openzeppelin-contracts/contracts/access/Ownable.sol";

// Interfaces
import { ISemver } from "interfaces/universal/ISemver.sol";
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

/// @title ProtocolVersions
/// @notice Security Council-controlled upgrade activation schedule contract.
/// @dev Maintains an ordered registry of upgrades and their L2 activation timestamps.
///      The canonical schedule commitment (`scheduleId`) is a hash over all (key, timestamp)
///      pairs in registration order. Proof journals bind to `scheduleId`, pinning every proof
///      in a dispute game to the schedule that was in effect at the game's L1 origin block.
///
///      An `upgradeId` is a human-readable string (e.g. "canyon"); each is packed into a
///      bytes32 storage `key`. The registry starts empty; all upgrades are added via
///      `registerUpgrade` owner calls.
contract ProtocolVersions is Ownable, IProtocolVersions {
    /// @notice Semantic version.
    /// @custom:semver 1.0.0
    string public constant version = "1.0.0";

    /// @notice The L2 chain ID whose schedule this contract commits to.
    uint256 public immutable l2ChainId;

    /// @notice The latest block number in which `scheduleId` changed.
    uint256 public lastUpdatedAtBlock;

    bytes32 internal _scheduleId;

    /// @notice Ordered list of registered upgrade keys. Defines the iteration order used to
    ///         compute `scheduleId`, so it must be reproduced exactly by off-chain consumers.
    bytes32[] internal _upgradeKeys;

    /// @notice 1-based registration index of each upgrade key into `_upgradeKeys`. 0 = unregistered.
    ///         Provides O(1) registered/duplicate checks without iterating `_upgradeKeys`.
    mapping(bytes32 => uint256) internal _upgradeIndex;

    mapping(bytes32 => uint64) internal _timestamps;

    /// @notice Protocol version for each upgrade, chosen by the owner and set at registration via
    ///         `registerUpgrade`. Immutable once registered.
    mapping(bytes32 => uint256) internal _protocolVersions;

    /// @notice Address allowed to delay (push out) already-scheduled activation timestamps.
    /// @dev Appointed and revocable by the owner. This is a restricted secondary role: it can
    ///      only move an already-scheduled, not-yet-activated upgrade further into the future via
    ///      `delaySignal`. It cannot register upgrades, clear timestamps, pull an activation
    ///      earlier, or schedule a brand-new activation. Unset (zero) by default.
    address public chainTeam;

    /// @notice Emitted when a new upgrade is added to the registry.
    /// @param key             The canonical bytes32 storage key.
    /// @param index           The 0-based registration index of this upgrade.
    /// @param upgradeId       The upgrade identifier string.
    /// @param protocolVersion The owner-assigned protocol version for this upgrade.
    event UpgradeRegistered(bytes32 indexed key, uint256 indexed index, string upgradeId, uint256 protocolVersion);

    /// @notice Emitted when an activation timestamp is set for an upgrade.
    event TimestampSet(bytes32 indexed key, uint256 timestamp);

    /// @notice Emitted when the schedule commitment changes.
    event ScheduleIdUpdated(bytes32 indexed oldScheduleId, bytes32 indexed newScheduleId, uint256 indexed blockNumber);

    /// @notice Emitted when the chainTeam role is appointed, changed, or cleared.
    event ChainTeamUpdated(address indexed previousChainTeam, address indexed newChainTeam);

    /// @notice Thrown when the owner is unset.
    error ProtocolVersions_ZeroOwner();

    /// @notice Thrown when the L2 chain ID is zero.
    error ProtocolVersions_InvalidL2ChainId();

    /// @notice Thrown when an upgradeId does not resolve to a registered upgrade.
    error ProtocolVersions_UnknownUpgradeName(string upgradeId);

    /// @notice Thrown when registering an upgrade that is already registered.
    error ProtocolVersions_UpgradeAlreadyRegistered(bytes32 key);

    /// @notice Thrown when an upgradeId is malformed (empty, or longer than 32 bytes).
    error ProtocolVersions_InvalidUpgradeId();

    /// @notice Thrown when trying to modify an upgrade whose activation timestamp has already passed.
    error ProtocolVersions_ActivationAlreadyPassed(bytes32 key, uint64 activationTimestamp);

    /// @notice Thrown when setting an activation timestamp that is not in the future.
    error ProtocolVersions_ActivationTimestampInPast(uint64 timestamp);

    /// @notice Thrown when a chainTeam-only function is called by another account.
    error ProtocolVersions_NotChainTeam();

    /// @notice Thrown when delaying an upgrade that has no scheduled activation timestamp.
    error ProtocolVersions_NotScheduled(bytes32 key);

    /// @notice Thrown when a delay does not push the activation strictly later than its current value.
    error ProtocolVersions_DelayMustBeLater(uint64 currentTimestamp, uint64 newTimestamp);

    /// @param owner_     Initial Security Council owner.
    /// @param l2ChainId_ L2 chain ID whose schedule is being committed.
    constructor(address owner_, uint256 l2ChainId_) Ownable() {
        if (owner_ == address(0)) revert ProtocolVersions_ZeroOwner();
        if (l2ChainId_ == 0) revert ProtocolVersions_InvalidL2ChainId();

        _transferOwnership(owner_);
        l2ChainId = l2ChainId_;

        _scheduleId = _computeScheduleId();
        lastUpdatedAtBlock = block.number;
        emit ScheduleIdUpdated(bytes32(0), _scheduleId, block.number);
    }

    /// @notice Restricts a call to the appointed chainTeam role.
    modifier onlyChainTeam() {
        if (msg.sender != chainTeam) revert ProtocolVersions_NotChainTeam();
        _;
    }

    /// @notice Returns the canonical schedule commitment.
    function scheduleId() public view returns (bytes32) {
        return _scheduleId;
    }

    /// @notice Returns the number of registered upgrades.
    function upgradeCount() external view returns (uint256) {
        return _upgradeKeys.length;
    }

    /// @notice Returns the registered upgrade key at `index` (registration order).
    function upgradeIdAt(uint256 index) external view returns (bytes32) {
        return _upgradeKeys[index];
    }

    /// @notice Returns the full ordered list of registered upgrade keys.
    function upgradeIds() external view returns (bytes32[] memory) {
        return _upgradeKeys;
    }

    /// @notice Returns the activation timestamp for `upgradeId` (0 = not scheduled).
    function getTimestamp(string calldata upgradeId) external view returns (uint256) {
        return _timestamps[_registeredKey(upgradeId)];
    }

    /// @notice Returns the owner-assigned protocol version for `upgradeId`. Reverts if not registered.
    function getProtocolVersion(string calldata upgradeId) external view returns (uint256) {
        return _protocolVersions[_registeredKey(upgradeId)];
    }

    /// @notice Registers a new upgrade by upgradeId with its protocol version. Owner only.
    /// @dev The upgradeId is packed into a bytes32 key and `protocolVersion` is recorded for it.
    ///      Adding an upgrade extends the committed schedule, so `scheduleId` changes. The proof
    ///      image must already understand the new upgrade before its timestamp is activated.
    function registerUpgrade(string calldata upgradeId, uint256 protocolVersion) external onlyOwner {
        _registerUpgrade(upgradeId, protocolVersion);
        _refreshScheduleId();
    }

    /// @notice Sets the activation timestamp for one upgrade by upgradeId. Pass 0 to clear.
    /// @dev The activation timestamp must be in the future and the upgrade must not have
    ///      already activated. Pass 0 to remove a scheduled activation.
    function setTimestamp(string calldata upgradeId, uint64 timestamp) external onlyOwner {
        _applyTimestamp(_registeredKey(upgradeId), timestamp);
        _refreshScheduleId();
    }

    /// @notice Appoints, replaces, or clears (set to zero) the chainTeam role. Owner only.
    function setChainTeam(address newChainTeam) external onlyOwner {
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
    function delayTimestamp(string calldata upgradeId, uint64 newTimestamp) external onlyChainTeam {
        bytes32 key = _registeredKey(upgradeId);
        uint64 current = _timestamps[key];

        // The upgrade must already have a scheduled activation to delay.
        if (current == 0) revert ProtocolVersions_NotScheduled(key);
        // Cannot modify an activation that has already passed.
        if (uint64(block.timestamp) >= current) {
            revert ProtocolVersions_ActivationAlreadyPassed(key, current);
        }
        // The role can only push the activation later, never earlier or to the same time.
        if (newTimestamp <= current) revert ProtocolVersions_DelayMustBeLater(current, newTimestamp);

        _timestamps[key] = newTimestamp;
        emit TimestampSet(key, newTimestamp);
        _refreshScheduleId();
    }

    function _applyTimestamp(bytes32 key, uint64 timestamp) internal {
        uint64 current = _timestamps[key];
        // Cannot modify a timestamp that has already activated.
        if (current != 0 && uint64(block.timestamp) >= current) {
            revert ProtocolVersions_ActivationAlreadyPassed(key, current);
        }
        // Non-zero timestamps must be in the future.
        if (timestamp != 0 && uint64(block.timestamp) >= timestamp) {
            revert ProtocolVersions_ActivationTimestampInPast(timestamp);
        }
        if (current != timestamp) {
            _timestamps[key] = timestamp;
            emit TimestampSet(key, timestamp);
        }
    }

    function _registerUpgrade(string calldata upgradeId, uint256 protocolVersion) internal {
        bytes32 key = _keyFromUpgradeId(upgradeId);
        if (_upgradeIndex[key] != 0) revert ProtocolVersions_UpgradeAlreadyRegistered(key);

        uint256 index = _upgradeKeys.length;
        _upgradeKeys.push(key);
        _upgradeIndex[key] = index + 1;
        _protocolVersions[key] = protocolVersion;
        emit UpgradeRegistered(key, index, upgradeId, protocolVersion);
    }

    function _refreshScheduleId() internal {
        bytes32 oldScheduleId = _scheduleId;
        bytes32 newScheduleId = _computeScheduleId();
        if (newScheduleId != oldScheduleId) {
            _scheduleId = newScheduleId;
            lastUpdatedAtBlock = block.number;
            emit ScheduleIdUpdated(oldScheduleId, newScheduleId, block.number);
        }
    }

    function _computeScheduleId() internal view returns (bytes32) {
        uint256 count = _upgradeKeys.length;
        uint64[] memory timestamps = new uint64[](count);
        for (uint256 i = 0; i < count; i++) {
            timestamps[i] = _timestamps[_upgradeKeys[i]];
        }
        return keccak256(abi.encode(l2ChainId, address(this), _upgradeKeys, timestamps));
    }

    function _registeredKey(string calldata upgradeId) internal view returns (bytes32 key) {
        key = _keyFromUpgradeId(upgradeId);
        if (_upgradeIndex[key] == 0) revert ProtocolVersions_UnknownUpgradeName(upgradeId);
    }

    function _keyFromUpgradeId(string calldata upgradeId) internal pure returns (bytes32 key) {
        bytes memory raw = bytes(upgradeId);
        if (raw.length == 0 || raw.length > 32) revert ProtocolVersions_InvalidUpgradeId();
        assembly {
            key := mload(add(raw, 32))
        }
    }
}
