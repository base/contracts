// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Initializable } from "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import { ISemver } from "interfaces/universal/ISemver.sol";

/// @custom:upgradeable
/// @title FlashblockIndex
/// @notice Stores the current flashblock index alongside block.number.
/// @dev The builder calls this via fallback with 1 byte of calldata (the index as uint8).
///      Both values are manually packed into a single uint256 to guarantee 1 SSTORE per write.
contract FlashblockIndex is Initializable, ISemver {
    /// @notice Thrown when the caller is not the authorized builder.
    error OnlyBuilder();

    /// @notice Thrown when calldata is not exactly 1 byte.
    error InvalidCalldata();

    /// @notice Emitted when the flashblock index is updated.
    /// @param flashblockIndex The new flashblock index.
    /// @param blockNumber The block number at which the index was set.
    event FlashblockIndexUpdated(uint8 indexed flashblockIndex, uint48 indexed blockNumber);

    /// @notice Semantic version.
    /// @custom:semver 1.0.0
    string public constant override version = "1.0.0";

    /// @notice The authorized builder address, set at deploy time.
    address public immutable BUILDER;

    /// @notice Packed storage: blockNumber (uint48) in bits [55:8] | flashblockIndex (uint8) in bits [7:0].
    /// @dev Using uint48 for block numbers is safe for the foreseeable future (~281 trillion blocks).
    uint256 private _packed;

    /// @notice Constructor.
    /// @param builder The address authorized to update the flashblock index.
    constructor(address builder) {
        BUILDER = builder;
        _disableInitializers();
    }

    /// @notice Initializer.
    function initialize() external initializer { }

    /// @notice Sets the flashblock index for the current block.
    /// @dev Calldata must be exactly 1 byte representing the flashblock index (uint8).
    ///      Stores `(block.number << 8) | index` in a single SSTORE.
    fallback() external {
        if (msg.sender != BUILDER) revert OnlyBuilder();
        if (msg.data.length != 1) revert InvalidCalldata();
        _packed = (uint256(uint48(block.number)) << 8) | uint256(uint8(msg.data[0]));
        emit FlashblockIndexUpdated(uint8(msg.data[0]), uint48(block.number));
    }

    /// @notice Returns the last stored flashblock index and its associated block number.
    /// @return flashblockIndex The flashblock index.
    /// @return blockNumber The block number at which the index was set.
    function get() external view returns (uint8 flashblockIndex, uint48 blockNumber) {
        uint256 packed = _packed;
        flashblockIndex = uint8(packed);
        blockNumber = uint48(packed >> 8);
    }
}
