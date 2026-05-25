// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";
import { Reverter, GasBurner } from "test/mocks/Callers.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";
import { stdError } from "lib/forge-std/src/StdError.sol";

// Libraries
import { Hashing } from "src/libraries/Hashing.sol";
import { Encoding } from "src/libraries/Encoding.sol";
import { Types } from "src/libraries/Types.sol";
import { AddressAliasHelper } from "src/vendor/AddressAliasHelper.sol";

// Interfaces
import { IL2CrossDomainMessenger } from "interfaces/L2/IL2CrossDomainMessenger.sol";
import { IL2ToL1MessagePasser } from "interfaces/L2/IL2ToL1MessagePasser.sol";

/// @title L2CrossDomainMessenger_TestInit
/// @notice Reusable test initialization for `L2CrossDomainMessenger` tests.
abstract contract L2CrossDomainMessenger_TestInit is CommonTest {
    address internal constant recipient = address(0xabbaacdc);
}

/// @title L2CrossDomainMessenger_Constructor_Test
/// @notice Tests the `constructor` of the `L2CrossDomainMessenger` contract.
contract L2CrossDomainMessenger_Constructor_Test is L2CrossDomainMessenger_TestInit {
    /// @notice Tests that the implementation is initialized correctly.
    function test_constructor_succeeds() external view {
        IL2CrossDomainMessenger impl = IL2CrossDomainMessenger(
            EIP1967Helper.getImplementation(artifacts.mustGetAddress("L2CrossDomainMessenger"))
        );
        assertEq(address(impl.OTHER_MESSENGER()), address(0));
        assertEq(address(impl.otherMessenger()), address(0));
        assertEq(address(impl.l1CrossDomainMessenger()), address(0));
        assertGt(bytes(impl.version()).length, 0);
        assertEq(impl.MESSAGE_VERSION(), 1);
    }
}

/// @title L2CrossDomainMessenger_Initialize_Test
/// @notice Tests the `initialize` function of the `L2CrossDomainMessenger` contract.
contract L2CrossDomainMessenger_Initialize_Test is L2CrossDomainMessenger_TestInit {
    /// @notice Tests that the proxy is initialized correctly.
    function test_initialize_succeeds() external view {
        assertEq(address(l2CrossDomainMessenger.OTHER_MESSENGER()), address(l1CrossDomainMessenger));
        assertEq(address(l2CrossDomainMessenger.otherMessenger()), address(l1CrossDomainMessenger));
        assertEq(address(l2CrossDomainMessenger.l1CrossDomainMessenger()), address(l1CrossDomainMessenger));
        assertGt(bytes(l2CrossDomainMessenger.version()).length, 0);
        assertEq(l2CrossDomainMessenger.MESSAGE_VERSION(), 1);
        assertGt(l2CrossDomainMessenger.messageNonce(), 0);
    }
}

/// @title L2CrossDomainMessenger_SendMessage_Test
/// @notice Tests the `sendMessage` function of the `L2CrossDomainMessenger` contract.
contract L2CrossDomainMessenger_SendMessage_Test is L2CrossDomainMessenger_TestInit {
    /// @notice Tests that `sendMessage` executes successfully with various target addresses and gas limits.
    function testFuzz_sendMessage_withValidTargetAndGasLimit_succeeds(address _target, uint32 _minGasLimit) external {
        vm.assume(_target != address(0));
        _minGasLimit = uint32(bound(_minGasLimit, 21000, 30_000_000));

        uint256 initialNonce = l2CrossDomainMessenger.messageNonce();

        vm.prank(alice);
        l2CrossDomainMessenger.sendMessage(_target, hex"1234", _minGasLimit);

        assertEq(l2CrossDomainMessenger.messageNonce(), initialNonce + 1);
    }

    /// @notice Tests that `sendMessage` executes successfully with the original test case.
    function test_sendMessage_succeeds() external {
        uint32 minGasLimit = 100;
        bytes memory message = hex"ff";
        uint256 nonce = l2CrossDomainMessenger.messageNonce();
        uint256 withdrawalNonce = l2ToL1MessagePasser.messageNonce();
        uint64 baseGas = l2CrossDomainMessenger.baseGas(message, minGasLimit);
        bytes memory xDomainCallData =
            Encoding.encodeCrossDomainMessage(nonce, alice, recipient, 0, minGasLimit, message);
        vm.expectCall(
            address(l2ToL1MessagePasser),
            abi.encodeCall(
                IL2ToL1MessagePasser.initiateWithdrawal, (address(l1CrossDomainMessenger), baseGas, xDomainCallData)
            )
        );

        vm.expectEmit(true, true, true, true);
        emit MessagePassed(
            withdrawalNonce,
            address(l2CrossDomainMessenger),
            address(l1CrossDomainMessenger),
            0,
            baseGas,
            xDomainCallData,
            Hashing.hashWithdrawal(
                Types.WithdrawalTransaction({
                    nonce: withdrawalNonce,
                    sender: address(l2CrossDomainMessenger),
                    target: address(l1CrossDomainMessenger),
                    value: 0,
                    gasLimit: baseGas,
                    data: xDomainCallData
                })
            )
        );

        vm.prank(alice);
        l2CrossDomainMessenger.sendMessage(recipient, message, minGasLimit);
    }

    /// @notice Tests that `sendMessage` can be called twice and that the nonce increments correctly.
    function test_sendMessage_twice_succeeds() external {
        uint256 nonce = l2CrossDomainMessenger.messageNonce();
        l2CrossDomainMessenger.sendMessage(recipient, hex"aa", uint32(500_000));
        l2CrossDomainMessenger.sendMessage(recipient, hex"aa", uint32(500_000));
        assertEq(nonce + 2, l2CrossDomainMessenger.messageNonce());
    }
}

/// @title L2CrossDomainMessenger_Uncategorized_Test
/// @notice General tests that are not testing any function directly of the
///         `L2CrossDomainMessenger` contract.
contract L2CrossDomainMessenger_Uncategorized_Test is L2CrossDomainMessenger_TestInit {
    uint256 internal constant l2SenderSlotIndex = 50;

    function _versionedNonce(uint16 _version) internal pure returns (uint256) {
        return Encoding.encodeVersionedNonce({ _nonce: 0, _version: _version });
    }

    function _l1MessengerAlias() internal view returns (address) {
        return AddressAliasHelper.applyL1ToL2Alias(address(l1CrossDomainMessenger));
    }

    function _setPortalL2Sender(address _sender) internal {
        vm.store(address(optimismPortal2), bytes32(l2SenderSlotIndex), bytes32(abi.encode(_sender)));
    }

    function _assertMessageStatus(bytes32 _hash, bool _successful, bool _failed) internal view {
        assertEq(l2CrossDomainMessenger.successfulMessages(_hash), _successful);
        assertEq(l2CrossDomainMessenger.failedMessages(_hash), _failed);
    }

    /// @notice Tests that `messageNonce` can be decoded correctly.
    function test_messageVersion_succeeds() external view {
        (, uint16 version) = Encoding.decodeVersionedNonce(l2CrossDomainMessenger.messageNonce());
        assertEq(version, l2CrossDomainMessenger.MESSAGE_VERSION());
    }

    /// @notice Tests that `sendMessage` reverts if the recipient is the zero address.
    function test_xDomainSender_senderNotSet_reverts() external {
        vm.expectRevert("CrossDomainMessenger: xDomainMessageSender is not set");
        l2CrossDomainMessenger.xDomainMessageSender();
    }

    /// @notice Tests that `sendMessage` reverts if the message version is not supported.
    function test_relayMessage_v2_reverts() external {
        address target = address(0xabcd);
        address sender = address(l1CrossDomainMessenger);
        address caller = _l1MessengerAlias();

        vm.expectRevert("CrossDomainMessenger: only version 0 or 1 messages are supported at this time");

        vm.prank(caller);
        l2CrossDomainMessenger.relayMessage(_versionedNonce(2), sender, target, 0, 0, hex"1111");
    }

    /// @notice Tests that `relayMessage` executes successfully.
    function test_relayMessage_succeeds() external {
        address target = address(0xabcd);
        address sender = address(l1CrossDomainMessenger);
        address caller = _l1MessengerAlias();
        uint256 nonce = _versionedNonce(1);

        vm.expectCall(target, hex"1111");

        vm.prank(caller);

        vm.expectEmit(true, true, true, true);

        bytes32 hash = Hashing.hashCrossDomainMessage(nonce, sender, target, 0, 0, hex"1111");

        emit RelayedMessage(hash);

        l2CrossDomainMessenger.relayMessage(nonce, sender, target, 0, 0, hex"1111");

        _assertMessageStatus(hash, true, false);
    }

    /// @notice Tests that `relayMessage` reverts if the value sent does not match the amount
    function test_relayMessage_fromOtherMessengerValueMismatch_reverts() external {
        address target = alice;
        address sender = address(l1CrossDomainMessenger);
        address caller = _l1MessengerAlias();
        bytes memory message = hex"1111";

        vm.deal(caller, 10 ether);
        vm.prank(caller);
        vm.expectRevert(stdError.assertionError);
        l2CrossDomainMessenger.relayMessage{ value: 10 ether }(_versionedNonce(1), sender, target, 9 ether, 0, message);
    }

    /// @notice Tests that `relayMessage` reverts if a failed message is attempted to be replayed
    ///         and the caller is the other messenger
    function test_relayMessage_fromOtherMessengerFailedMessageReplay_reverts() external {
        address target = alice;
        address sender = address(l1CrossDomainMessenger);
        address caller = _l1MessengerAlias();
        uint256 nonce = _versionedNonce(1);
        bytes memory message = hex"1111";

        vm.etch(target, hex"fe");
        vm.prank(caller);
        l2CrossDomainMessenger.relayMessage(nonce, sender, target, 0, 0, message);

        vm.prank(caller);
        vm.expectRevert(stdError.assertionError);
        l2CrossDomainMessenger.relayMessage(nonce, sender, target, 0, 0, message);
    }

    /// @notice Tests that `relayMessage` reverts if attempting to relay a message sent to self
    function test_relayMessage_toSelf_reverts() external {
        address sender = address(l1CrossDomainMessenger);
        address caller = _l1MessengerAlias();
        bytes memory message = hex"1111";

        vm.prank(caller);
        vm.expectRevert("CrossDomainMessenger: cannot send message to blocked system address");
        l2CrossDomainMessenger.relayMessage(_versionedNonce(1), sender, address(l2CrossDomainMessenger), 0, 0, message);
    }

    /// @notice Tests that `relayMessage` reverts if attempting to relay a message sent to the
    ///         `l2ToL1MessagePasser` address
    function test_relayMessage_toL2ToL1MessagePasser_reverts() external {
        address sender = address(l1CrossDomainMessenger);
        address caller = _l1MessengerAlias();
        bytes memory message = hex"1111";

        vm.prank(caller);
        vm.expectRevert("CrossDomainMessenger: cannot send message to blocked system address");
        l2CrossDomainMessenger.relayMessage(_versionedNonce(1), sender, address(l2ToL1MessagePasser), 0, 0, message);
    }

    /// @notice Tests that `relayMessage` reverts if the message called by non-`optimismPortal2`
    ///         but not a failed message
    function test_relayMessage_relayingNewMessageByExternalUser_reverts() external {
        address target = address(alice);
        address sender = address(l1CrossDomainMessenger);
        bytes memory message = hex"1111";

        vm.prank(bob);
        vm.expectRevert("CrossDomainMessenger: message cannot be replayed");
        l2CrossDomainMessenger.relayMessage(_versionedNonce(1), sender, target, 0, 0, message);
    }

    /// @notice Tests that `relayMessage` on L2 will always succeed for any potential message,
    ///         regardless of the size of the message, the minimum gas limit, or the amount of gas
    ///         used by the target contract.
    function testFuzz_relayMessage_baseGasSufficient_succeeds(
        uint24 _messageLength,
        uint32 _minGasLimit,
        uint32 _gasToUse
    )
        external
    {
        // TODO(#14609): Update this test to use default.isolate = true once a new stable Foundry
        // release is available that includes #9904. That will allow us to use this test to check
        // for changes to the EVM itself that might cause our gas formula to be incorrect.

        // Skip if this is a fork test, won't work.
        skipIfForkTest("L2CrossDomainMessenger doesn't exist on L1 in forked test");

        // Using smaller uint so the fuzzer doesn't give as many massive values that get bounded.
        // TODO: Known issue, messages above 34k can actually OOG on the receiving side if the
        // target uses all available gas. Can be resolved by capping data sizes on the XDM or by
        // increasing the amount of available relay gas to ~100k. If increasing relay gas, should
        // have the relay gas only increase when the calldata size is large to avoid disrupting
        // in-flight L2 -> L1 messages.
        _messageLength = uint24(bound(_messageLength, 0, 34_000));

        // Need more than 500 since GasBurner requires it.
        // It's ok to try to use more than minGasLimit since the L2CrossDomainMessenger should
        // catch the revert and store the message hash in the failedMessages mapping.
        _gasToUse = uint32(bound(_gasToUse, 500, type(uint32).max));

        // Generate random bytes, more useful than having a _message parameter.
        bytes memory _message = vm.randomBytes(_messageLength);

        // Compute the base gas.
        // Base gas should really be computed on the fully encoded message but that would break the
        // expected API, so we instead just add the encoding overhead to the message length inside
        // of the baseGas function.
        uint64 baseGas = l2CrossDomainMessenger.baseGas(_message, _minGasLimit);

        // Deploy a gas burner.
        address target = address(new GasBurner(_gasToUse));

        // Encode the message.
        bytes memory encoded = Encoding.encodeCrossDomainMessage(
            _versionedNonce(1),
            alice, // Sender doesn't matter
            target,
            0, // Value doesn't matter
            _minGasLimit,
            _message
        );

        // Count calldata bytes so the EIP-7623 floor can be checked against actual encoded data.
        uint256 nonzeroBytesInCalldata = 0;
        for (uint256 i = 0; i < encoded.length; i++) {
            if (encoded[i] != bytes1(0)) {
                nonzeroBytesInCalldata++;
            }
        }

        uint256 zeroBytesInCalldata = encoded.length - nonzeroBytesInCalldata;
        uint256 calldataTokens = zeroBytesInCalldata + nonzeroBytesInCalldata * 4;
        uint256 floorCost = l2CrossDomainMessenger.TX_BASE_GAS() + calldataTokens
            * (l2CrossDomainMessenger.FLOOR_CALLDATA_OVERHEAD() / 4);

        // Base gas must always be sufficient to cover the floor cost from EIP-7623.
        assertGt(baseGas, floorCost);

        _setPortalL2Sender(address(l2CrossDomainMessenger));

        vm.prank(address(optimismPortal2));

        // In the L2 => L1 direction we actually get all of the base gas supplied, nothing is
        // deducted. This is an advantage over the L1 => L2 direction because it means the base
        // gas goes a lot further.
        (bool success,) = address(l1CrossDomainMessenger).call{ gas: baseGas }(encoded);
        assertTrue(success, "L1CrossDomainMessenger call should not fail");

        // Message should either be in the failed or successful messages mapping.
        bytes32 encodedHash = keccak256(encoded);
        bool inFailedMessages = l1CrossDomainMessenger.failedMessages(encodedHash);
        bool inSuccessfulMessages = l1CrossDomainMessenger.successfulMessages(encodedHash);
        assertTrue(
            inFailedMessages || inSuccessfulMessages, "message should be in either failed or successful messages"
        );
    }

    /// @notice Tests that `relayMessage` correctly resets the `xDomainMessageSender` to the
    ///         original value after a message is relayed.
    function test_xDomainMessageSender_reset_succeeds() external {
        vm.expectRevert("CrossDomainMessenger: xDomainMessageSender is not set");
        l2CrossDomainMessenger.xDomainMessageSender();

        address caller = _l1MessengerAlias();
        vm.prank(caller);
        l2CrossDomainMessenger.relayMessage(_versionedNonce(1), address(0), address(0), 0, 0, hex"");

        vm.expectRevert("CrossDomainMessenger: xDomainMessageSender is not set");
        l2CrossDomainMessenger.xDomainMessageSender();
    }

    /// @notice Tests that `relayMessage` is able to send a successful call to the target contract
    ///         after the first message fails and ETH gets stuck, but the second message succeeds.
    function test_relayMessage_retry_succeeds() external {
        address target = address(0xabcd);
        address sender = address(l1CrossDomainMessenger);
        address caller = _l1MessengerAlias();
        uint256 nonce = _versionedNonce(1);
        uint256 value = 100;

        bytes32 hash = Hashing.hashCrossDomainMessage(nonce, sender, target, value, 0, hex"1111");

        vm.etch(target, address(new Reverter()).code);
        vm.deal(address(caller), value);
        vm.prank(caller);
        l2CrossDomainMessenger.relayMessage{ value: value }(nonce, sender, target, value, 0, hex"1111");

        assertEq(address(l2CrossDomainMessenger).balance, value);
        assertEq(address(target).balance, 0);
        _assertMessageStatus(hash, false, true);

        vm.expectEmit(true, true, true, true);

        emit RelayedMessage(hash);

        vm.etch(target, address(0).code);
        vm.prank(address(sender));
        l2CrossDomainMessenger.relayMessage(nonce, sender, target, value, 0, hex"1111");

        assertEq(address(l2CrossDomainMessenger).balance, 0);
        assertEq(address(target).balance, value);
        _assertMessageStatus(hash, true, true);
    }
}
