// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";

// Libraries
import { Encoding } from "src/libraries/Encoding.sol";

/// @title L1Block_ TestInit
/// @notice Reusable test initialization for `L1Block` tests.
abstract contract L1Block_TestInit is CommonTest {
    bytes4 internal constant NOT_DEPOSITOR_SELECTOR = 0x3cc50b45;
    bytes32 internal constant LEGACY_HIGH_BITS_MASK =
        hex"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00000000000000000000000000000000";

    address depositor;

    /// @notice Sets up the test suite.
    function setUp() public virtual override {
        super.setUp();
        depositor = l1Block.DEPOSITOR_ACCOUNT();
    }

    /// @notice Asserts that legacy high 128-bit ranges in key storage slots remain zeroed.
    function assertEmptyLegacySlotRanges() internal view {
        bytes32 scalarsSlot = vm.load(address(l1Block), bytes32(uint256(3)));
        assertEq(0, scalarsSlot & LEGACY_HIGH_BITS_MASK);

        bytes32 numberTimestampSlot = vm.load(address(l1Block), bytes32(uint256(0)));
        assertEq(0, numberTimestampSlot & LEGACY_HIGH_BITS_MASK);
    }

    function assertDepositorCallSucceeds(bytes memory callData) internal {
        vm.prank(depositor);
        (bool success,) = address(l1Block).call(callData);
        assertTrue(success, "function call failed");
    }

    function assertNotDepositorReverts(bytes memory callData) internal {
        (bool success, bytes memory data) = address(l1Block).call(callData);
        assertFalse(success, "function call should have failed");
        assertEq(data, abi.encodeWithSelector(NOT_DEPOSITOR_SELECTOR));
    }
}

/// @title L1Block_Version_Test
/// @notice Test contract for L1Block `version` function.
contract L1Block_Version_Test is L1Block_TestInit {
    /// @notice Tests that the version function returns a valid string. We avoid testing the
    ///         specific value of the string as it changes frequently.
    function test_version_succeeds() external view {
        assertGt(bytes(l1Block.version()).length, 0);
    }
}

/// @title L1Block_SetL1BlockValues_Test
/// @notice Tests the `setL1BlockValues` function of the `L1Block` contract.
contract L1Block_SetL1BlockValues_Test is L1Block_TestInit {
    /// @notice Tests that `setL1BlockValues` updates the values correctly.
    function testFuzz_setL1BlockValues_succeeds(
        uint64 n,
        uint64 t,
        uint256 b,
        bytes32 h,
        uint64 s,
        bytes32 bt,
        uint256 fo,
        uint256 fs
    )
        external
    {
        vm.prank(depositor);
        l1Block.setL1BlockValues(n, t, b, h, s, bt, fo, fs);
        assertEq(l1Block.number(), n);
        assertEq(l1Block.timestamp(), t);
        assertEq(l1Block.basefee(), b);
        assertEq(l1Block.hash(), h);
        assertEq(l1Block.sequenceNumber(), s);
        assertEq(l1Block.batcherHash(), bt);
        assertEq(l1Block.l1FeeOverhead(), fo);
        assertEq(l1Block.l1FeeScalar(), fs);
    }

    /// @notice Tests that `setL1BlockValues` succeeds with max values
    function test_setL1BlockValuesMax_succeeds() external {
        vm.prank(depositor);
        l1Block.setL1BlockValues({
            _number: type(uint64).max,
            _timestamp: type(uint64).max,
            _basefee: type(uint256).max,
            _hash: keccak256(abi.encode(1)),
            _sequenceNumber: type(uint64).max,
            _batcherHash: bytes32(type(uint256).max),
            _l1FeeOverhead: type(uint256).max,
            _l1FeeScalar: type(uint256).max
        });
    }

    /// @notice Tests that `setL1BlockValues` reverts if sender address is not the depositor
    function test_setL1BlockValues_notDepositor_reverts() external {
        vm.expectRevert("L1Block: only the depositor account can set L1 block values");
        l1Block.setL1BlockValues({
            _number: type(uint64).max,
            _timestamp: type(uint64).max,
            _basefee: type(uint256).max,
            _hash: keccak256(abi.encode(1)),
            _sequenceNumber: type(uint64).max,
            _batcherHash: bytes32(type(uint256).max),
            _l1FeeOverhead: type(uint256).max,
            _l1FeeScalar: type(uint256).max
        });
    }
}

/// @title L1Block_SetL1BlockValuesEcotone_Test
/// @notice Tests the `setL1BlockValuesEcotone` function of the `L1Block` contract.
contract L1Block_SetL1BlockValuesEcotone_Test is L1Block_TestInit {
    function encodeMaxSetL1BlockValuesEcotone() internal pure returns (bytes memory) {
        return Encoding.encodeSetL1BlockValuesEcotone(
            type(uint32).max,
            type(uint32).max,
            type(uint64).max,
            type(uint64).max,
            type(uint64).max,
            type(uint256).max,
            type(uint256).max,
            bytes32(type(uint256).max),
            bytes32(type(uint256).max)
        );
    }

    /// @notice Tests that setL1BlockValuesEcotone updates the values appropriately.
    function testFuzz_setL1BlockValuesEcotone_succeeds(
        uint32 baseFeeScalar,
        uint32 blobBaseFeeScalar,
        uint64 sequenceNumber,
        uint64 timestamp,
        uint64 number,
        uint256 baseFee,
        uint256 blobBaseFee,
        bytes32 hash,
        bytes32 batcherHash
    )
        external
    {
        bytes memory functionCallDataPacked = Encoding.encodeSetL1BlockValuesEcotone(
            baseFeeScalar, blobBaseFeeScalar, sequenceNumber, timestamp, number, baseFee, blobBaseFee, hash, batcherHash
        );

        assertDepositorCallSucceeds(functionCallDataPacked);

        assertEq(l1Block.baseFeeScalar(), baseFeeScalar);
        assertEq(l1Block.blobBaseFeeScalar(), blobBaseFeeScalar);
        assertEq(l1Block.sequenceNumber(), sequenceNumber);
        assertEq(l1Block.timestamp(), timestamp);
        assertEq(l1Block.number(), number);
        assertEq(l1Block.basefee(), baseFee);
        assertEq(l1Block.blobBaseFee(), blobBaseFee);
        assertEq(l1Block.hash(), hash);
        assertEq(l1Block.batcherHash(), batcherHash);

        assertEmptyLegacySlotRanges();
    }

    /// @notice Tests that `setL1BlockValuesEcotone` succeeds with max values
    function test_setL1BlockValuesEcotone_isDepositorMax_succeeds() external {
        assertDepositorCallSucceeds(encodeMaxSetL1BlockValuesEcotone());
    }

    /// @notice Tests that `setL1BlockValuesEcotone` reverts if sender address is not the depositor
    function test_setL1BlockValuesEcotone_notDepositor_reverts() external {
        assertNotDepositorReverts(encodeMaxSetL1BlockValuesEcotone());
    }
}

/// @title L1Block_SetL1BlockValuesIsthmus_Test
/// @notice Tests the `setL1BlockValuesIsthmus` function of the `L1Block` contract.
contract L1Block_SetL1BlockValuesIsthmus_Test is L1Block_TestInit {
    function encodeMaxSetL1BlockValuesIsthmus() internal pure returns (bytes memory) {
        return Encoding.encodeSetL1BlockValuesIsthmus(
            type(uint32).max,
            type(uint32).max,
            type(uint64).max,
            type(uint64).max,
            type(uint64).max,
            type(uint256).max,
            type(uint256).max,
            bytes32(type(uint256).max),
            bytes32(type(uint256).max),
            type(uint32).max,
            type(uint64).max
        );
    }

    /// @notice Tests that setL1BlockValuesIsthmus updates the values appropriately.
    function testFuzz_setL1BlockValuesIsthmus_succeeds(
        uint32 baseFeeScalar,
        uint32 blobBaseFeeScalar,
        uint64 sequenceNumber,
        uint64 timestamp,
        uint64 number,
        uint256 baseFee,
        uint256 blobBaseFee,
        bytes32 hash,
        bytes32 batcherHash,
        uint32 operatorFeeScalar,
        uint64 operatorFeeConstant
    )
        external
    {
        bytes memory functionCallDataPacked = Encoding.encodeSetL1BlockValuesIsthmus(
            baseFeeScalar,
            blobBaseFeeScalar,
            sequenceNumber,
            timestamp,
            number,
            baseFee,
            blobBaseFee,
            hash,
            batcherHash,
            operatorFeeScalar,
            operatorFeeConstant
        );

        assertDepositorCallSucceeds(functionCallDataPacked);

        assertEq(l1Block.baseFeeScalar(), baseFeeScalar);
        assertEq(l1Block.blobBaseFeeScalar(), blobBaseFeeScalar);
        assertEq(l1Block.sequenceNumber(), sequenceNumber);
        assertEq(l1Block.timestamp(), timestamp);
        assertEq(l1Block.number(), number);
        assertEq(l1Block.basefee(), baseFee);
        assertEq(l1Block.blobBaseFee(), blobBaseFee);
        assertEq(l1Block.hash(), hash);
        assertEq(l1Block.batcherHash(), batcherHash);
        assertEq(l1Block.operatorFeeScalar(), operatorFeeScalar);
        assertEq(l1Block.operatorFeeConstant(), operatorFeeConstant);

        assertEmptyLegacySlotRanges();
    }

    /// @notice Tests that `setL1BlockValuesIsthmus` succeeds with max values
    function test_setL1BlockValuesIsthmus_isDepositorMax_succeeds() external {
        assertDepositorCallSucceeds(encodeMaxSetL1BlockValuesIsthmus());
    }

    /// @notice Tests that `setL1BlockValuesIsthmus` reverts if sender address is not the depositor
    function test_setL1BlockValuesIsthmus_notDepositor_reverts() external {
        assertNotDepositorReverts(encodeMaxSetL1BlockValuesIsthmus());
    }
}

/// @title L1Block_SetL1BlockValuesJovian_Test
/// @notice Tests the `setL1BlockValuesJovian` function of the `L1Block` contract.
contract L1Block_SetL1BlockValuesJovian_Test is L1Block_TestInit {
    /// @notice Struct to group parameters for L1BlockValuesJovian to avoid stack too deep.
    struct L1BlockValuesJovianParams {
        uint32 baseFeeScalar;
        uint32 blobBaseFeeScalar;
        uint64 sequenceNumber;
        uint64 timestamp;
        uint64 number;
        uint256 baseFee;
        uint256 blobBaseFee;
        bytes32 hash;
        bytes32 batcherHash;
        uint32 operatorFeeScalar;
        uint64 operatorFeeConstant;
        uint16 daFootprintGasScalar;
    }

    function encodeMaxSetL1BlockValuesJovian() internal pure returns (bytes memory) {
        return Encoding.encodeSetL1BlockValuesJovian(
            type(uint32).max,
            type(uint32).max,
            type(uint64).max,
            type(uint64).max,
            type(uint64).max,
            type(uint256).max,
            type(uint256).max,
            bytes32(type(uint256).max),
            bytes32(type(uint256).max),
            type(uint32).max,
            type(uint64).max,
            type(uint16).max
        );
    }

    /// @notice Tests that setL1BlockValuesJovian updates the values appropriately.
    function testFuzz_setL1BlockValuesJovian_succeeds(L1BlockValuesJovianParams memory params) external {
        bytes memory functionCallDataPacked = Encoding.encodeSetL1BlockValuesJovian(
            params.baseFeeScalar,
            params.blobBaseFeeScalar,
            params.sequenceNumber,
            params.timestamp,
            params.number,
            params.baseFee,
            params.blobBaseFee,
            params.hash,
            params.batcherHash,
            params.operatorFeeScalar,
            params.operatorFeeConstant,
            params.daFootprintGasScalar
        );

        assertDepositorCallSucceeds(functionCallDataPacked);

        assertEq(l1Block.baseFeeScalar(), params.baseFeeScalar);
        assertEq(l1Block.blobBaseFeeScalar(), params.blobBaseFeeScalar);
        assertEq(l1Block.sequenceNumber(), params.sequenceNumber);
        assertEq(l1Block.timestamp(), params.timestamp);
        assertEq(l1Block.number(), params.number);
        assertEq(l1Block.basefee(), params.baseFee);
        assertEq(l1Block.blobBaseFee(), params.blobBaseFee);
        assertEq(l1Block.hash(), params.hash);
        assertEq(l1Block.batcherHash(), params.batcherHash);
        assertEq(l1Block.operatorFeeScalar(), params.operatorFeeScalar);
        assertEq(l1Block.operatorFeeConstant(), params.operatorFeeConstant);
        assertEq(l1Block.daFootprintGasScalar(), params.daFootprintGasScalar);

        assertEmptyLegacySlotRanges();
    }

    /// @notice Tests that `setL1BlockValuesJovian` succeeds with max values
    function test_setL1BlockValuesJovian_isDepositorMax_succeeds() external {
        assertDepositorCallSucceeds(encodeMaxSetL1BlockValuesJovian());
    }

    /// @notice Prevents the L1 attributes system transaction from approaching its gas limit.
    function test_setL1BlockValuesJovian_benchmark() external {
        skipIfCoverage();

        bytes memory functionCallDataPacked = encodeMaxSetL1BlockValuesJovian();

        assertDepositorCallSucceeds(functionCallDataPacked);
        assertLt(vm.lastCallGas().gasTotalUsed, 200_000);
    }

    /// @notice Tests that `setL1BlockValuesJovian` reverts if sender address is not the depositor
    function test_setL1BlockValuesJovian_notDepositor_reverts() external {
        assertNotDepositorReverts(encodeMaxSetL1BlockValuesJovian());
    }
}
