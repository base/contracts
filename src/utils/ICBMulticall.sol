// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

struct Call {
    address target;
    bytes callData;
}

struct Call3 {
    address target;
    bool allowFailure;
    bytes callData;
}

struct Call3Value {
    address target;
    bool allowFailure;
    uint256 value;
    bytes callData;
}

struct Result {
    bool success;
    bytes returnData;
}

/// @title ICBMulticall
/// @notice Interface for the CBMulticall contract used in multisig scripts.
interface ICBMulticall {
    /// @notice Aggregate calls, ensuring each returns success if required
    /// @param calls An array of Call3 structs
    /// @return returnData An array of Result structs
    function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData);

    /// @notice Aggregate calls, ensuring each returns success if required
    /// @param calls An array of Call3 structs
    /// @return returnData An array of Result structs
    function aggregateDelegateCalls(Call3[] calldata calls) external payable returns (Result[] memory returnData);

    /// @notice Aggregate calls with a msg value
    /// @param calls An array of Call3Value structs
    /// @return returnData An array of Result structs
    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData);
}
