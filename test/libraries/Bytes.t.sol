// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";
import { Bytes } from "src/libraries/Bytes.sol";

contract Bytes_Harness {
    function exposed_slice(bytes memory _input, uint256 _start, uint256 _length) public pure returns (bytes memory) {
        return Bytes.slice(_input, _start, _length);
    }
}

/// @title Bytes_TestInit
/// @notice Reusable test initialization for `Bytes` tests.
abstract contract Bytes_TestInit is Test {
    function manualEq(bytes memory _a, bytes memory _b) internal pure returns (bool) {
        return _a.length == _b.length && keccak256(_a) == keccak256(_b);
    }

    function freeMemoryPtr() internal pure returns (uint64 ptr_) {
        assembly {
            ptr_ := mload(0x40)
        }
    }

    function nextSliceMemoryPtr(uint64 _ptr, uint256 _length) internal pure returns (uint64) {
        if (_length == 0) {
            return _ptr + 0x20;
        }

        return uint64((_ptr + 0x20 + _length + 0x1f) & ~uint256(0x1f));
    }

    function nextBytesMemoryPtr(uint64 _ptr, uint256 _length) internal pure returns (uint64) {
        return uint64(_ptr + 0x20 + ((_length + 0x1f) & ~uint256(0x1f)));
    }
}

/// @title Bytes_Slice_Test
/// @notice Tests the `slice` function of the `Bytes` library.
contract Bytes_Slice_Test is Bytes_TestInit {
    Bytes_Harness harness;

    function setUp() public {
        harness = new Bytes_Harness();
    }

    /// @notice Tests that the `slice` function works as expected when starting from index 0.
    function test_slice_fromZeroIdx_works() public pure {
        bytes memory input = hex"11223344556677889900";

        // Exhaustively check if all possible slices starting from index 0 are correct.
        assertEq(Bytes.slice(input, 0, 0), hex"");
        assertEq(Bytes.slice(input, 0, 1), hex"11");
        assertEq(Bytes.slice(input, 0, 2), hex"1122");
        assertEq(Bytes.slice(input, 0, 3), hex"112233");
        assertEq(Bytes.slice(input, 0, 4), hex"11223344");
        assertEq(Bytes.slice(input, 0, 5), hex"1122334455");
        assertEq(Bytes.slice(input, 0, 6), hex"112233445566");
        assertEq(Bytes.slice(input, 0, 7), hex"11223344556677");
        assertEq(Bytes.slice(input, 0, 8), hex"1122334455667788");
        assertEq(Bytes.slice(input, 0, 9), hex"112233445566778899");
        assertEq(Bytes.slice(input, 0, 10), hex"11223344556677889900");
    }

    /// @notice Tests that the `slice` function works as expected when starting from indices [1, 9]
    ///         with lengths [1, 9], in reverse order.
    function test_slice_fromNonZeroIdx_works() public pure {
        bytes memory input = hex"11223344556677889900";

        // Exhaustively check correctness of slices starting from indexes [1, 9]
        // and spanning [1, 9] bytes, in reverse order
        assertEq(Bytes.slice(input, 9, 1), hex"00");
        assertEq(Bytes.slice(input, 8, 2), hex"9900");
        assertEq(Bytes.slice(input, 7, 3), hex"889900");
        assertEq(Bytes.slice(input, 6, 4), hex"77889900");
        assertEq(Bytes.slice(input, 5, 5), hex"6677889900");
        assertEq(Bytes.slice(input, 4, 6), hex"556677889900");
        assertEq(Bytes.slice(input, 3, 7), hex"44556677889900");
        assertEq(Bytes.slice(input, 2, 8), hex"3344556677889900");
        assertEq(Bytes.slice(input, 1, 9), hex"223344556677889900");
    }

    /// @notice Tests that the `slice` function works as expected when slicing between multiple
    ///         words in memory. In this case, we test that a 2 byte slice between the 32nd byte of
    ///         the first word and the 1st byte of the second word is correct.
    function test_slice_acrossWords_works() public pure {
        bytes memory input =
            hex"00000000000000000000000000000000000000000000000000000000000000112200000000000000000000000000000000000000000000000000000000000000";

        assertEq(Bytes.slice(input, 31, 2), hex"1122");
    }

    /// @notice Tests that the `slice` function works as expected when slicing between multiple
    ///         words in memory. In this case, we test that a 34 byte slice between 3 separate
    ///         words returns the correct result.
    function test_slice_acrossMultipleWords_works() public pure {
        bytes memory input =
            hex"000000000000000000000000000000000000000000000000000000000000001122FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF1100000000000000000000000000000000000000000000000000000000000000";
        bytes memory expected = hex"1122FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF11";

        assertEq(Bytes.slice(input, 31, 34), expected);
    }

    /// @notice Tests that the `slice` function correctly updates the free memory pointer depending
    ///         on the length of the slice.
    ///         The calls to `bound` are to reduce the number of times that `assume` is triggered.
    function testFuzz_slice_memorySafety_succeeds(bytes memory _input, uint256 _start, uint256 _length) public {
        vm.assume(_input.length > 0);

        _start = bound(_start, 0, _input.length - 1);
        _length = bound(_length, 0, _input.length - _start);

        uint64 initPtr = freeMemoryPtr();
        uint64 expectedPtr = nextSliceMemoryPtr(initPtr, _length);

        vm.expectSafeMemory(initPtr, expectedPtr);
        bytes memory slice = Bytes.slice(_input, _start, _length);
        vm.stopExpectSafeMemory();

        assertEq(freeMemoryPtr(), expectedPtr);
        assertEq(slice.length, _length);
    }

    /// @notice Tests that, when given an input bytes array of length `n`, the `slice` function
    ///         will always revert if `_start + _length > n`.
    function testFuzz_slice_outOfBounds_reverts(bytes memory _input, uint256 _start, uint256 _length) public {
        // We want a valid start index that will not overflow.
        if (_input.length == 0) {
            _start = 0;
        } else {
            _start = bound(_start, 0, _input.length - 1);
        }
        // And a length that will not overflow.
        // But, we want an invalid slice length.
        if (_start > 31) {
            _length = bound(_length, (_input.length - _start) + 1, type(uint256).max - _start);
        } else {
            _length = bound(_length, (_input.length - _start) + 1, type(uint256).max - 31);
        }

        vm.expectRevert("slice_outOfBounds");
        harness.exposed_slice(_input, _start, _length);
    }

    /// @notice Tests that, when given a length `n` that is greater than `type(uint256).max - 31`,
    ///         the `slice` function reverts.
    function testFuzz_slice_lengthOverflows_reverts(uint256 _length) public {
        // Ensure that the `_length` will overflow if a number >= 31 is added to it.
        _length = uint256(bound(_length, type(uint256).max - 30, type(uint256).max));

        vm.expectRevert("slice_overflow");
        harness.exposed_slice(hex"", 0, _length);
    }

    /// @notice Tests that, when given a start index `n` that is greater than
    ///         `type(uint256).max - n`, the `slice` function reverts.
    ///         The calls to `bound` are to reduce the number of times that `assume` is triggered.
    function testFuzz_slice_rangeOverflows_reverts(bytes memory _input, uint256 _start, uint256 _length) public {
        vm.assume(_input.length > 1);

        // Ensure that `_length` is a realistic length of a slice. This is to make sure we revert
        // on the correct require statement.
        _length = bound(_length, 1, _input.length - 1);

        _start = bound(_start, type(uint256).max - _length + 1, type(uint256).max);

        vm.expectRevert("slice_overflow");
        harness.exposed_slice(_input, _start, _length);
    }
}

/// @title Bytes_ToNibbles_Test
/// @notice Tests the `toNibbles` function of the `Bytes` library.
contract Bytes_ToNibbles_Test is Bytes_TestInit {
    /// @notice Tests that, given an input of 5 bytes, the `toNibbles` function returns an array of
    ///         10 nibbles corresponding to the input data.
    function test_toNibbles_expectedResult5Bytes_works() public pure {
        bytes memory input = hex"1234567890";
        bytes memory expected = hex"01020304050607080900";
        bytes memory actual = Bytes.toNibbles(input);

        assertEq(input.length * 2, actual.length);
        assertEq(expected.length, actual.length);
        assertEq(actual, expected);
    }

    /// @notice Tests that, given an input of 128 bytes, the `toNibbles` function returns an array
    ///         of 256 nibbles corresponding to the input data. This test exists to ensure that,
    ///         given a large input, the `toNibbles` function works as expected.
    function test_toNibbles_expectedResult128Bytes_works() public pure {
        bytes memory input =
            hex"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f";
        bytes memory expected =
            hex"0000000100020003000400050006000700080009000a000b000c000d000e000f0100010101020103010401050106010701080109010a010b010c010d010e010f0200020102020203020402050206020702080209020a020b020c020d020e020f0300030103020303030403050306030703080309030a030b030c030d030e030f0400040104020403040404050406040704080409040a040b040c040d040e040f0500050105020503050405050506050705080509050a050b050c050d050e050f0600060106020603060406050606060706080609060a060b060c060d060e060f0700070107020703070407050706070707080709070a070b070c070d070e070f";
        bytes memory actual = Bytes.toNibbles(input);

        assertEq(input.length * 2, actual.length);
        assertEq(expected.length, actual.length);
        assertEq(actual, expected);
    }

    /// @notice Tests that, given an input of 0 bytes, the `toNibbles` function returns a zero
    ///         length array.
    function test_toNibbles_zeroLengthInput_works() public pure {
        assertEq(Bytes.toNibbles(hex""), hex"");
    }

    /// @notice Tests that the `toNibbles` function correctly updates the free memory pointer
    ///         depending on the length of the resulting array.
    function testFuzz_toNibbles_memorySafety_succeeds(bytes memory _input) public {
        uint256 nibblesLength = _input.length * 2;
        uint64 initPtr = freeMemoryPtr();
        uint64 expectedPtr = nextBytesMemoryPtr(initPtr, nibblesLength);

        vm.expectSafeMemory(initPtr, expectedPtr);
        bytes memory nibbles = Bytes.toNibbles(_input);
        vm.stopExpectSafeMemory();

        assertEq(freeMemoryPtr(), expectedPtr);
        assertEq(nibbles.length, nibblesLength);
    }
}

/// @title Bytes_Equal_Test
/// @notice Tests the `equal` function of the `Bytes` library.
contract Bytes_Equal_Test is Bytes_TestInit {
    /// @notice Tests that the `equal` function in the `Bytes` library returns `false` if given two
    ///         non-equal byte arrays.
    function testFuzz_equal_notEqual_works(bytes memory _a, bytes memory _b) public pure {
        vm.assume(!manualEq(_a, _b));
        assertFalse(Bytes.equal(_a, _b));
    }

    /// @notice Test whether or not the `equal` function in the `Bytes` library is equivalent to
    ///         manually checking equality of the two dynamic `bytes` arrays in memory.
    function testDiff_equal_works(bytes memory _a, bytes memory _b) public pure {
        assertEq(Bytes.equal(_a, _b), manualEq(_a, _b));
    }
}
