// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing utilities
import { Test } from "lib/forge-std/src/Test.sol";
import { CommonTest } from "test/setup/CommonTest.sol";
import { ForgeArtifacts, StorageSlot } from "scripts/libraries/ForgeArtifacts.sol";

// Libraries
import { Math } from "lib/openzeppelin-contracts/contracts/utils/math/Math.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { Hashing } from "src/libraries/Hashing.sol";
import { Encoding } from "src/libraries/Encoding.sol";

import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";

/// @title CrossDomainMessenger_ExternalRelay_Harness
/// @notice A mock external contract called via the SafeCall inside the CrossDomainMessenger's
///         `relayMessage` function.
contract CrossDomainMessenger_ExternalRelay_Harness is Test {
    address internal immutable op;
    address internal fuzzedSender;
    IL1CrossDomainMessenger internal immutable l1CrossDomainMessenger;

    event FailedRelayedMessage(bytes32 indexed msgHash);

    constructor(IL1CrossDomainMessenger _l1Messenger, address _op) {
        l1CrossDomainMessenger = _l1Messenger;
        op = _op;
    }

    function _internalRelay(address _innerSender, bytes memory _callMessage) internal {
        address initialSender = l1CrossDomainMessenger.xDomainMessageSender();
        uint256 nonce = Encoding.encodeVersionedNonce({ _nonce: 0, _version: 1 });

        bytes32 hash = Hashing.hashCrossDomainMessage({
            _nonce: nonce, _sender: _innerSender, _target: address(this), _value: 0, _gasLimit: 0, _data: _callMessage
        });

        vm.expectEmit(true, true, true, true);
        emit FailedRelayedMessage(hash);

        vm.prank(address(op));
        l1CrossDomainMessenger.relayMessage({
            _nonce: nonce,
            _sender: _innerSender,
            _target: address(this),
            _value: 0,
            _minGasLimit: 0,
            _message: _callMessage
        });

        assertTrue(l1CrossDomainMessenger.failedMessages(hash));
        assertFalse(l1CrossDomainMessenger.successfulMessages(hash));
        assertEq(initialSender, l1CrossDomainMessenger.xDomainMessageSender());
    }

    /// @notice externalCallWithMinGas is called by the CrossDomainMessenger.
    function externalCallWithMinGas() external payable {
        bytes memory callMessage = getCallData();

        for (uint256 i = 0; i < 10; i++) {
            address _innerSender;
            unchecked {
                _innerSender = address(uint160(uint256(uint160(fuzzedSender)) + i));
            }
            _internalRelay(_innerSender, callMessage);
        }
    }

    function getCallData() public pure returns (bytes memory) {
        return abi.encodeCall(CrossDomainMessenger_ExternalRelay_Harness.externalCallWithMinGas, ());
    }

    function setFuzzedSender(address _fuzzedSender) public {
        fuzzedSender = _fuzzedSender;
    }
}

/// @title CrossDomainMessenger_TestInit
/// @notice Reusable test initialization for `CrossDomainMessenger` tests.
abstract contract CrossDomainMessenger_TestInit is CommonTest {
    CrossDomainMessenger_ExternalRelay_Harness public er;

    function setUp() public override {
        super.setUp();
        er = new CrossDomainMessenger_ExternalRelay_Harness(l1CrossDomainMessenger, address(optimismPortal2));
    }

    function _versionedNonce(uint16 _version) internal pure returns (uint256) {
        return Encoding.encodeVersionedNonce({ _nonce: 0, _version: _version });
    }

    function _setPortalL2Sender(address _sender) internal {
        StorageSlot memory senderSlot = ForgeArtifacts.getSlot("OptimismPortal2", "l2Sender");
        vm.store(address(optimismPortal2), bytes32(senderSlot.slot), bytes32(abi.encode(_sender)));
    }

    function _assertMessageStatus(bytes32 _hash, bool _successful, bool _failed) internal view {
        assertEq(l1CrossDomainMessenger.successfulMessages(_hash), _successful);
        assertEq(l1CrossDomainMessenger.failedMessages(_hash), _failed);
    }
}

/// @title CrossDomainMessenger_RelayMessage_Test
/// @notice Fuzz tests re-entrancy into the CrossDomainMessenger relayMessage function.
contract CrossDomainMessenger_RelayMessage_Test is CrossDomainMessenger_TestInit {
    /// @dev This test mocks an OptimismPortal call to the `L1CrossDomainMessenger` via the
    ///      `relayMessage` function. The `relayMessage` function will then use `SafeCall`'s
    ///      `callWithMinGas` to call the target with call data packed in the `callMessage`. For
    ///      this test, the `callWithMinGas` will call the mock `ExternalRelay` test contract
    ///      defined above, executing the `externalCallWithMinGas` function which will try to
    ///      re-enter the `CrossDomainMessenger`'s `relayMessage` function, resulting in that
    ///      message being recorded as failed.
    function testFuzz_relayMessageReenter_succeeds(address _sender, uint256 _gasLimit) external {
        vm.assume(_sender != Predeploys.L2_CROSS_DOMAIN_MESSENGER);
        address sender = Predeploys.L2_CROSS_DOMAIN_MESSENGER;

        er.setFuzzedSender(_sender);
        address target = address(er);
        bytes memory callMessage = er.getCallData();

        vm.expectCall(target, callMessage);

        uint64 gasLimit = uint64(bound(_gasLimit, 0, 30_000_000));
        uint256 nonce = _versionedNonce(1);

        bytes32 hash = Hashing.hashCrossDomainMessage({
            _nonce: nonce, _sender: sender, _target: target, _value: 0, _gasLimit: gasLimit, _data: callMessage
        });

        _setPortalL2Sender(sender);
        vm.prank(address(optimismPortal2));
        l1CrossDomainMessenger.relayMessage({
            _nonce: nonce, _sender: sender, _target: target, _value: 0, _minGasLimit: gasLimit, _message: callMessage
        });

        _assertMessageStatus(hash, true, false);

        vm.expectRevert("CrossDomainMessenger: xDomainMessageSender is not set");
        l1CrossDomainMessenger.xDomainMessageSender();
    }
}

/// @dev CrossDomainMessenger_Test is for testing functionality which is common to both the L1 and
///      L2 CrossDomainMessenger contracts. For simplicity, we use the L1 Messenger as the test
///      contract.
contract CrossDomainMessenger_BaseGas_Test is CommonTest {
    function _totalMessageSize(uint256 _messageLength) internal view returns (uint64) {
        return uint64(_messageLength + l1CrossDomainMessenger.ENCODING_OVERHEAD());
    }

    function _executionGas(uint32 _minGasLimit) internal view returns (uint64) {
        return uint64(
            l1CrossDomainMessenger.RELAY_CONSTANT_OVERHEAD() + l1CrossDomainMessenger.RELAY_CALL_OVERHEAD()
                + l1CrossDomainMessenger.RELAY_RESERVED_GAS() + l1CrossDomainMessenger.RELAY_GAS_CHECK_BUFFER()
                + ((_minGasLimit * l1CrossDomainMessenger.MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR())
                    / l1CrossDomainMessenger.MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR())
        );
    }

    function _floorGas(uint256 _messageLength) internal view returns (uint64) {
        return l1CrossDomainMessenger.TX_BASE_GAS()
            + (_totalMessageSize(_messageLength) * l1CrossDomainMessenger.FLOOR_CALLDATA_OVERHEAD());
    }

    function _executionGasWithOverhead(uint256 _messageLength, uint32 _minGasLimit) internal view returns (uint64) {
        return l1CrossDomainMessenger.TX_BASE_GAS() + _executionGas(_minGasLimit)
            + (_totalMessageSize(_messageLength) * l1CrossDomainMessenger.MIN_GAS_CALLDATA_OVERHEAD());
    }

    function _baseGasMinimum(uint256 _messageLength, uint32 _minGasLimit) internal view returns (uint64) {
        uint64 totalMessageSize = _totalMessageSize(_messageLength);
        return l1CrossDomainMessenger.TX_BASE_GAS()
            + uint64(
            Math.max(
            _executionGas(_minGasLimit) + (totalMessageSize * l1CrossDomainMessenger.MIN_GAS_CALLDATA_OVERHEAD()),
            totalMessageSize * l1CrossDomainMessenger.FLOOR_CALLDATA_OVERHEAD()
        )
        );
    }

    /// @notice Ensure that `baseGas` passes for the max value of `_minGasLimit`, this is about
    ///         4 Billion.
    function test_baseGas_succeeds() external view {
        l1CrossDomainMessenger.baseGas(hex"ff", type(uint32).max);
    }

    /// @notice Fuzz for other values which might cause a revert in `baseGas`.
    function testFuzz_baseGas_succeeds(uint32 _minGasLimit) external view {
        l1CrossDomainMessenger.baseGas(hex"ff", _minGasLimit);
    }

    /// @notice The `baseGas` function should always return a value greater than or equal to the
    ///         minimum gas limit value on the `OptimismPortal`. This guarantees that the
    ///         messengers will always pass sufficient gas to the `OptimismPortal`.
    function testFuzz_baseGas_portalMinGasLimit_succeeds(bytes calldata _data, uint32 _minGasLimit) external view {
        if (_data.length > type(uint64).max) {
            _data = _data[0:type(uint64).max];
        }

        uint64 baseGas = l1CrossDomainMessenger.baseGas(_data, _minGasLimit);
        uint64 minGasLimit = optimismPortal2.minimumGasLimit(uint64(_data.length));
        assertTrue(baseGas >= minGasLimit);
    }

    /// @notice Test that `baseGas` returns at least the floor cost for calldata
    function test_baseGas_floor_succeeds() external view {
        bytes memory largeMessage = new bytes(100_000);

        uint64 baseGasResult = l1CrossDomainMessenger.baseGas(largeMessage, 0);

        assertGe(baseGasResult, _floorGas(largeMessage.length), "baseGas should return at least the floor cost");
    }

    /// @notice Test that `baseGas` returns the execution gas when it's higher than the floor cost
    function test_baseGas_executionGas_succeeds() external view {
        bytes memory smallMessage = new bytes(10);
        uint32 highGasLimit = 1_000_000;

        uint64 baseGasResult = l1CrossDomainMessenger.baseGas(smallMessage, highGasLimit);
        uint64 expectedExecutionGasWithOverhead = _executionGasWithOverhead(smallMessage.length, highGasLimit);

        assertGe(baseGasResult, expectedExecutionGasWithOverhead, "baseGas should return at least the execution gas");
        assertGt(
            expectedExecutionGasWithOverhead,
            _floorGas(smallMessage.length),
            "Execution gas should be higher than floor cost for this test"
        );
    }

    /// @notice Fuzz test to verify the `baseGas` function correctly implements the `Math.max`
    ///         logic.
    /// @param _message The message to test
    /// @param _minGasLimit The minimum gas limit to test
    function testFuzz_baseGas_maxLogic_succeeds(bytes calldata _message, uint32 _minGasLimit) external view {
        uint64 baseGasResult = l1CrossDomainMessenger.baseGas(_message, _minGasLimit);
        uint64 expectedMinimum = _baseGasMinimum(_message.length, _minGasLimit);

        assertGe(baseGasResult, expectedMinimum, "baseGas should return at least the max gas path");
    }
}
