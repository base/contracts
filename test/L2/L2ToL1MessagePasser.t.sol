// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing utilities
import { CommonTest } from "test/setup/CommonTest.sol";

// Libraries
import { Types } from "src/libraries/Types.sol";
import { Hashing } from "src/libraries/Hashing.sol";
import { Features } from "src/libraries/Features.sol";
import { Encoding } from "src/libraries/Encoding.sol";
import { SemverComp } from "src/libraries/SemverComp.sol";

/// @title L2ToL1MessagePasser_TestInit
/// @notice Reusable test initialization for `L2ToL1MessagePasser` tests.
abstract contract L2ToL1MessagePasser_TestInit is CommonTest {
    uint256 internal constant RECEIVE_DEFAULT_GAS_LIMIT = 100_000;

    function _hashWithdrawal(
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        internal
        pure
        returns (bytes32)
    {
        return Hashing.hashWithdrawal(
            Types.WithdrawalTransaction({
                nonce: _nonce, sender: _sender, target: _target, value: _value, gasLimit: _gasLimit, data: _data
            })
        );
    }

    function _expectMessagePassed(
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        internal
        returns (bytes32)
    {
        bytes32 withdrawalHash = _hashWithdrawal(_nonce, _sender, _target, _value, _gasLimit, _data);

        vm.expectEmit(address(l2ToL1MessagePasser));
        emit MessagePassed(_nonce, _sender, _target, _value, _gasLimit, _data, withdrawalHash);

        return withdrawalHash;
    }
}

/// @title L2ToL1MessagePasser_Version_Test
/// @notice Tests the `version` function of the `L2ToL1MessagePasser` contract.
contract L2ToL1MessagePasser_Version_Test is L2ToL1MessagePasser_TestInit {
    /// @notice Tests that the version follows valid semver format.
    function test_version_validFormat_succeeds() external view {
        SemverComp.parse(l2ToL1MessagePasser.version());
    }
}

/// @title L2ToL1MessagePasser_Receive_Test
/// @notice Tests the `receive` function of the `L2ToL1MessagePasser` contract.
contract L2ToL1MessagePasser_Receive_Test is L2ToL1MessagePasser_TestInit {
    /// @notice Tests that receive() initiates withdrawal with default gas limit.
    function testFuzz_receive_initiatesWithdrawal_succeeds(uint256 _value) external {
        skipIfSysFeatureEnabled(Features.CUSTOM_GAS_TOKEN);

        uint256 nonce = l2ToL1MessagePasser.messageNonce();
        bytes memory data = bytes("");
        bytes32 withdrawalHash =
            _expectMessagePassed(nonce, address(this), address(this), _value, RECEIVE_DEFAULT_GAS_LIMIT, data);

        vm.deal(address(this), _value);
        (bool success,) = address(l2ToL1MessagePasser).call{ value: _value }(data);

        assertTrue(success);
        assertTrue(l2ToL1MessagePasser.sentMessages(withdrawalHash));
        assertEq(l2ToL1MessagePasser.messageNonce(), nonce + 1);
    }
}

/// @title L2ToL1MessagePasser_Burn_Test
/// @notice Tests the `burn` function of the `L2ToL1MessagePasser` contract.
contract L2ToL1MessagePasser_Burn_Test is L2ToL1MessagePasser_TestInit {
    /// @notice Tests that `burn` succeeds and destroys the ETH held in the contract.
    function testFuzz_burn_succeeds(uint256 _value) external {
        skipIfSysFeatureEnabled(Features.CUSTOM_GAS_TOKEN);
        vm.deal(address(l2ToL1MessagePasser), _value);

        assertEq(address(l2ToL1MessagePasser).balance, _value);

        vm.expectEmit(address(l2ToL1MessagePasser));
        emit WithdrawerBalanceBurnt(_value);
        l2ToL1MessagePasser.burn();

        assertEq(address(l2ToL1MessagePasser).balance, 0);
    }
}

/// @title L2ToL1MessagePasser_InitiateWithdrawal_Test
/// @notice Tests the `initiateWithdrawal` function of the `L2ToL1MessagePasser` contract.
contract L2ToL1MessagePasser_InitiateWithdrawal_Test is L2ToL1MessagePasser_TestInit {
    /// @notice Tests that `initiateWithdrawal` succeeds and correctly sets the state of the
    ///         message passer for the withdrawal hash.
    function testFuzz_initiateWithdrawal_succeeds(
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
    {
        if (isSysFeatureEnabled(Features.CUSTOM_GAS_TOKEN)) {
            _value = 0;
        }
        uint256 nonce = l2ToL1MessagePasser.messageNonce();

        bytes32 withdrawalHash = _expectMessagePassed(nonce, _sender, _target, _value, _gasLimit, _data);

        vm.deal(_sender, _value);
        vm.prank(_sender);
        l2ToL1MessagePasser.initiateWithdrawal{ value: _value }(_target, _gasLimit, _data);

        assertTrue(l2ToL1MessagePasser.sentMessages(withdrawalHash));
        assertEq(l2ToL1MessagePasser.messageNonce(), nonce + 1);
    }

    /// @notice Tests that `initiateWithdrawal` succeeds when called by a contract.
    function testFuzz_initiateWithdrawal_fromContract_succeeds(
        address _target,
        uint256 _gasLimit,
        uint256 _value,
        bytes memory _data
    )
        external
    {
        skipIfSysFeatureEnabled(Features.CUSTOM_GAS_TOKEN);
        uint256 nonce = l2ToL1MessagePasser.messageNonce();
        bytes32 withdrawalHash = _expectMessagePassed(nonce, address(this), _target, _value, _gasLimit, _data);

        vm.deal(address(this), _value);
        l2ToL1MessagePasser.initiateWithdrawal{ value: _value }(_target, _gasLimit, _data);

        assertTrue(l2ToL1MessagePasser.sentMessages(withdrawalHash));
    }

    /// @notice Tests that `initiateWithdrawal` succeeds when called by an EOA.
    function testFuzz_initiateWithdrawal_fromEOA_succeeds(
        uint256 _gasLimit,
        address _target,
        uint256 _value,
        bytes memory _data
    )
        external
    {
        skipIfSysFeatureEnabled(Features.CUSTOM_GAS_TOKEN);
        uint256 nonce = l2ToL1MessagePasser.messageNonce();
        vm.deal(alice, _value);
        bytes32 withdrawalHash = _expectMessagePassed(nonce, alice, _target, _value, _gasLimit, _data);

        vm.prank(alice, alice);
        l2ToL1MessagePasser.initiateWithdrawal{ value: _value }({
            _target: _target, _gasLimit: _gasLimit, _data: _data
        });

        assertTrue(l2ToL1MessagePasser.sentMessages(withdrawalHash));
        assertEq(l2ToL1MessagePasser.messageNonce(), nonce + 1);
    }
}

/// @title L2ToL1MessagePasser_MessageNonce_Test
/// @notice Tests the `messageNonce` function of the `L2ToL1MessagePasser` contract.
contract L2ToL1MessagePasser_MessageNonce_Test is L2ToL1MessagePasser_TestInit {
    /// @notice Tests that messageNonce encodes version in upper bytes.
    function test_messageNonce_encodesVersion_succeeds() external view {
        (, uint16 version) = Encoding.decodeVersionedNonce(l2ToL1MessagePasser.messageNonce());
        assertEq(version, l2ToL1MessagePasser.MESSAGE_VERSION());
    }
}
