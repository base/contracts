// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";
import { FlashblockIndex } from "src/L2/FlashblockIndex.sol";

contract FlashblockIndexTest is Test {
    FlashblockIndex flashblockIndex;
    address builder;

    function setUp() public {
        builder = makeAddr("builder");
        flashblockIndex = new FlashblockIndex(builder);
    }

    /// @notice Tests that the constructor correctly sets the BUILDER immutable.
    function test_constructor_setsBuilder() external view {
        assertEq(flashblockIndex.BUILDER(), builder);
    }

    /// @notice Tests that get() returns (0, 0) when no index has ever been written.
    function test_get_returnsZeros_whenNeverWritten() external view {
        (uint8 index, uint48 blockNumber) = flashblockIndex.get();
        assertEq(index, 0);
        assertEq(blockNumber, 0);
    }

    /// @notice Tests that get() returns the correct index and block number after a write.
    function test_get_returnsCorrectValues(uint8 index, uint48 blockNumber) external {
        vm.roll(blockNumber);
        (bool success,) = _callFallback({ caller: builder, index: index });
        assertTrue(success);

        (uint8 actualIndex, uint48 actualBlock) = flashblockIndex.get();
        assertEq(actualIndex, index);
        assertEq(actualBlock, blockNumber);
    }

    /// @notice Tests that the fallback reverts with OnlyBuilder when called by a non-builder address.
    function test_fallback_reverts_whenCallerIsNotBuilder(address caller, uint8 index) external {
        vm.assume(caller != builder);
        (bool success, bytes memory returnData) = _callFallback({ caller: caller, index: index });
        assertFalse(success);
        assertEq(bytes4(returnData), FlashblockIndex.OnlyBuilder.selector);
    }

    /// @notice Tests that the fallback reverts with InvalidCalldata when called with zero bytes.
    function test_fallback_reverts_whenCalldataIsEmpty() external {
        vm.prank(builder);
        (bool success, bytes memory returnData) = address(flashblockIndex).call("");
        assertFalse(success);
        assertEq(bytes4(returnData), FlashblockIndex.InvalidCalldata.selector);
    }

    /// @notice Tests that the fallback reverts with InvalidCalldata when called with more than 1 byte.
    function test_fallback_reverts_whenCalldataIsTooLong(uint8 extra) external {
        vm.prank(builder);
        (bool success, bytes memory returnData) = address(flashblockIndex).call(abi.encodePacked(uint8(1), extra));
        assertFalse(success);
        assertEq(bytes4(returnData), FlashblockIndex.InvalidCalldata.selector);
    }

    /// @notice Tests that the fallback stores the index and block number correctly when called by the builder.
    function test_fallback_setsIndex(uint8 index, uint48 blockNumber) external {
        vm.roll(blockNumber);
        (bool success,) = _callFallback({ caller: builder, index: index });
        assertTrue(success);

        (uint8 actualIndex, uint48 actualBlock) = flashblockIndex.get();
        assertEq(actualIndex, index);
        assertEq(actualBlock, blockNumber);
    }

    /// @notice Tests that a second fallback call overwrites the previous value at a different block.
    function test_fallback_overwritesPreviousValue() external {
        uint48 firstBlock = 100;
        uint8 firstIndex = 5;
        vm.roll(firstBlock);
        (bool s1,) = _callFallback({ caller: builder, index: firstIndex });
        assertTrue(s1);

        uint48 secondBlock = 200;
        uint8 secondIndex = 10;
        vm.roll(secondBlock);
        (bool s2,) = _callFallback({ caller: builder, index: secondIndex });
        assertTrue(s2);

        (uint8 actualIndex, uint48 actualBlock) = flashblockIndex.get();
        assertEq(actualIndex, secondIndex);
        assertEq(actualBlock, secondBlock);
    }

    /// @notice Tests that a second fallback call overwrites the previous value within the same block.
    function test_fallback_overwritesWithinSameBlock() external {
        uint48 blockNumber = 100;
        uint8 firstIndex = 5;
        uint8 secondIndex = 10;

        vm.roll(blockNumber);
        (bool s1,) = _callFallback({ caller: builder, index: firstIndex });
        assertTrue(s1);

        (bool s2,) = _callFallback({ caller: builder, index: secondIndex });
        assertTrue(s2);

        (uint8 actualIndex, uint48 actualBlock) = flashblockIndex.get();
        assertEq(actualIndex, secondIndex);
        assertEq(actualBlock, blockNumber);
    }

    /// @notice Tests that the fallback correctly stores the maximum uint48 block number.
    function test_fallback_storesMaxBlockNumber() external {
        uint48 maxBlock = type(uint48).max;
        uint8 index = 1;

        vm.roll(maxBlock);
        (bool success,) = _callFallback({ caller: builder, index: index });
        assertTrue(success);

        (uint8 actualIndex, uint48 actualBlock) = flashblockIndex.get();
        assertEq(actualIndex, index);
        assertEq(actualBlock, maxBlock);
    }

    /// @notice Tests that the block number is truncated to uint48 when it exceeds the max value.
    function test_fallback_truncatesBlockNumber() external {
        uint256 overflowBlock = uint256(type(uint48).max) + 1;
        uint8 index = 1;

        vm.roll(overflowBlock);
        (bool success,) = _callFallback({ caller: builder, index: index });
        assertTrue(success);

        (uint8 actualIndex, uint48 actualBlock) = flashblockIndex.get();
        assertEq(actualIndex, index);
        assertEq(actualBlock, 0);
    }

    /// @notice Helper function to call the fallback with the given caller and index.
    /// @param caller The address of the caller.
    /// @param index The index to call the fallback with.
    /// @return success True if the fallback call succeeded.
    /// @return returnData The return data from the fallback call.
    function _callFallback(address caller, uint8 index) private returns (bool success, bytes memory returnData) {
        vm.prank(caller);
        (success, returnData) = address(flashblockIndex).call(abi.encodePacked(index));
    }
}
