// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { CBMulticall, Call, Call3, Call3Value, Result } from "src/universal/CBMulticall.sol";
import { Test } from "lib/forge-std/src/Test.sol";

/// @dev Invokes `aggregateDelegateCalls` through `delegatecall` to simulate multisig execution.
contract CBMulticallDelegateCaller {
    CBMulticall public mc;

    constructor(CBMulticall _mc) {
        mc = _mc;
    }

    function aggregateDelegateCalls(Call3[] calldata calls) external returns (Result[] memory) {
        (bool success, bytes memory data) =
            address(mc).delegatecall(abi.encodeWithSelector(CBMulticall.aggregateDelegateCalls.selector, calls));
        if (!success) {
            assembly ("memory-safe") {
                revert(add(data, 0x20), mload(data))
            }
        }
        return abi.decode(data, (Result[]));
    }
}

contract MockReceiver {
    function bump(uint256 x) external pure returns (uint256) {
        return x + 1;
    }

    function payAndEcho(uint256 x) external payable returns (uint256, uint256) {
        return (x, msg.value);
    }

    function willRevert() external pure {
        revert("revert");
    }
}

contract CBMulticallTest is Test {
    string internal constant MULTICALL_CALL_FAILED = "Multicall3: call failed";

    CBMulticall mc;
    MockReceiver target;

    function setUp() public {
        mc = new CBMulticall();
        target = new MockReceiver();
    }

    function _bumpCall(uint256 x) internal view returns (Call memory) {
        return Call({ target: address(target), callData: abi.encodeCall(MockReceiver.bump, (x)) });
    }

    function _revertingCall() internal view returns (Call memory) {
        return Call({ target: address(target), callData: abi.encodeCall(MockReceiver.willRevert, ()) });
    }

    function _bumpCall3(bool allowFailure, uint256 x) internal view returns (Call3 memory) {
        return Call3({
            target: address(target), allowFailure: allowFailure, callData: abi.encodeCall(MockReceiver.bump, (x))
        });
    }

    function _revertingCall3(bool allowFailure) internal view returns (Call3 memory) {
        return Call3({
            target: address(target), allowFailure: allowFailure, callData: abi.encodeCall(MockReceiver.willRevert, ())
        });
    }

    function test_aggregate_returnsBlockNumberAndData() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(41);
        calls[1] = _bumpCall(1);

        (uint256 bn, bytes[] memory rdata) = mc.aggregate(calls);
        assertEq(bn, block.number);
        assertEq(abi.decode(rdata[0], (uint256)), 42);
        assertEq(abi.decode(rdata[1], (uint256)), 2);
    }

    function test_aggregate_revertsOnFailedCall() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(0);
        calls[1] = _revertingCall();
        vm.expectRevert(bytes(MULTICALL_CALL_FAILED));
        mc.aggregate(calls);
    }

    function test_tryAggregate_noRequire_returnsResults() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(1);
        calls[1] = _revertingCall();

        Result[] memory results = mc.tryAggregate(false, calls);
        assertEq(results.length, 2);
        assertTrue(results[0].success);
        assertEq(abi.decode(results[0].returnData, (uint256)), 2);
        assertFalse(results[1].success);
    }

    function test_tryAggregate_requireSuccess_revertsOnFailure() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(0);
        calls[1] = _revertingCall();
        vm.expectRevert(bytes(MULTICALL_CALL_FAILED));
        mc.tryAggregate(true, calls);
    }

    function test_tryBlockAndAggregate_noRequire_returnsBlockInfoAndResults() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(0);
        calls[1] = _revertingCall();

        (uint256 bn, bytes32 bh, Result[] memory res) = mc.tryBlockAndAggregate(false, calls);
        assertEq(bn, block.number);
        assertEq(bh, blockhash(block.number));
        assertTrue(res[0].success);
        assertEq(abi.decode(res[0].returnData, (uint256)), 1);
        assertFalse(res[1].success);
    }

    function test_blockAndAggregate_allSuccess_returnsResults() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(1);
        calls[1] = _bumpCall(2);
        (uint256 bn, bytes32 bh, Result[] memory res) = mc.blockAndAggregate(calls);
        assertEq(bn, block.number);
        assertEq(bh, blockhash(block.number));
        assertEq(res.length, 2);
        assertEq(abi.decode(res[1].returnData, (uint256)), 3);
    }

    function test_blockAndAggregate_revertsOnFailure() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(0);
        calls[1] = _revertingCall();
        vm.expectRevert(bytes(MULTICALL_CALL_FAILED));
        mc.blockAndAggregate(calls);
    }

    function test_tryBlockAndAggregate_requireSuccess_revertsOnFailure() external {
        Call[] memory calls = new Call[](2);
        calls[0] = _bumpCall(0);
        calls[1] = _revertingCall();
        vm.expectRevert(bytes(MULTICALL_CALL_FAILED));
        mc.tryBlockAndAggregate(true, calls);
    }

    function test_aggregate3_success() external {
        Call3[] memory calls = new Call3[](1);
        calls[0] = _bumpCall3(false, 4);
        Result[] memory results = mc.aggregate3(calls);
        assertTrue(results[0].success);
        assertEq(abi.decode(results[0].returnData, (uint256)), 5);
    }

    function test_aggregate3_allowedFailure_returnsFalse() external {
        Call3[] memory calls = new Call3[](1);
        calls[0] = _revertingCall3(true);
        Result[] memory results = mc.aggregate3(calls);
        assertFalse(results[0].success);
    }

    function test_aggregate3_revertsOnNonAllowedFailure() external {
        Call3[] memory calls = new Call3[](1);
        calls[0] = _revertingCall3(false);
        vm.expectRevert(bytes(MULTICALL_CALL_FAILED));
        mc.aggregate3(calls);
    }

    function test_aggregateDelegateCalls_success() external {
        CBMulticallDelegateCaller caller = new CBMulticallDelegateCaller(mc);
        Call3[] memory calls = new Call3[](1);
        calls[0] = _bumpCall3(false, 4);
        Result[] memory results = caller.aggregateDelegateCalls(calls);
        assertTrue(results[0].success);
        assertEq(abi.decode(results[0].returnData, (uint256)), 5);
    }

    function test_aggregateDelegateCalls_allowedFailure_returnsFalse() external {
        CBMulticallDelegateCaller caller = new CBMulticallDelegateCaller(mc);
        Call3[] memory calls = new Call3[](1);
        calls[0] = _revertingCall3(true);
        Result[] memory results = caller.aggregateDelegateCalls(calls);
        assertFalse(results[0].success);
    }

    function test_aggregateDelegateCalls_revertsOnNonAllowedFailure() external {
        CBMulticallDelegateCaller caller = new CBMulticallDelegateCaller(mc);
        Call3[] memory calls = new Call3[](1);
        calls[0] = _revertingCall3(false);
        vm.expectRevert(bytes(MULTICALL_CALL_FAILED));
        caller.aggregateDelegateCalls(calls);
    }

    function test_aggregateDelegateCalls_directCall_revertsWithMustDelegateCall() external {
        Call3[] memory calls = new Call3[](1);
        calls[0] = _bumpCall3(false, 1);

        vm.expectRevert(CBMulticall.MustDelegateCall.selector);
        mc.aggregateDelegateCalls(calls);
    }

    function test_aggregate3Value_success_usesContractBalance() external {
        vm.deal(address(mc), 1 ether);
        Call3Value[] memory calls = new Call3Value[](1);
        calls[0] = Call3Value({
            target: address(target),
            allowFailure: false,
            value: 0.5 ether,
            callData: abi.encodeCall(MockReceiver.payAndEcho, (7))
        });
        Result[] memory results = mc.aggregate3Value(calls);
        (uint256 x, uint256 v) = abi.decode(results[0].returnData, (uint256, uint256));
        assertEq(x, 7);
        assertEq(v, 0.5 ether);
        assertEq(address(target).balance, 0.5 ether);
    }

    function test_aggregate3Value_revertsOnNonAllowedFailure() external {
        Call3Value[] memory calls = new Call3Value[](1);
        calls[0] = Call3Value({
            target: address(target),
            allowFailure: false,
            value: 0,
            callData: abi.encodeCall(MockReceiver.willRevert, ())
        });
        vm.expectRevert(bytes(MULTICALL_CALL_FAILED));
        mc.aggregate3Value(calls);
    }

    function test_getBlockNumber() external view {
        assertEq(mc.getBlockNumber(), block.number);
    }

    function test_getBlockHash() external view {
        assertEq(mc.getBlockHash(block.number), blockhash(block.number));
    }

    function test_getCurrentBlockCoinbase() external view {
        assertEq(mc.getCurrentBlockCoinbase(), block.coinbase);
    }

    function test_getCurrentBlockGasLimit() external view {
        assertEq(mc.getCurrentBlockGasLimit(), block.gaslimit);
    }

    function test_getCurrentBlockTimestamp() external view {
        assertEq(mc.getCurrentBlockTimestamp(), block.timestamp);
    }

    function test_getEthBalance() external view {
        assertEq(mc.getEthBalance(address(target)), address(target).balance);
    }

    function test_getLastBlockHash() external view {
        assertEq(mc.getLastBlockHash(), blockhash(block.number - 1));
    }

    function test_getBasefee() external view {
        assertEq(mc.getBasefee(), block.basefee);
    }

    function test_getChainId() external view {
        assertEq(mc.getChainId(), block.chainid);
    }
}
