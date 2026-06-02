// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing utilities
import { CommonTest } from "test/setup/CommonTest.sol";

// Libraries
import { Types } from "src/libraries/Types.sol";
import { Encoding } from "src/libraries/Encoding.sol";

// Target contract
import { Hashing } from "src/libraries/Hashing.sol";

contract Hashing_Harness {
    function hashSuperRootProof(Types.SuperRootProof memory _proof) external pure returns (bytes32) {
        return Hashing.hashSuperRootProof(_proof);
    }

    function hashCrossDomainMessage(
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
        pure
        returns (bytes32)
    {
        return Hashing.hashCrossDomainMessage(_nonce, _sender, _target, _value, _gasLimit, _data);
    }
}

abstract contract Hashing_TestInit is CommonTest {
    bytes1 internal constant SUPER_ROOT_VERSION = 0x01;
    uint256 internal constant MAX_OUTPUT_ROOTS = 50;

    function _outputRoot(uint256 _chainId, bytes32 _root) internal pure returns (Types.OutputRootWithChainId memory) {
        return Types.OutputRootWithChainId({ chainId: _chainId, root: _root });
    }

    function _superRootProof(
        uint64 _timestamp,
        uint256 _length,
        uint256 _seed
    )
        internal
        pure
        returns (Types.SuperRootProof memory proof)
    {
        _length = bound(_length, 1, MAX_OUTPUT_ROOTS);

        Types.OutputRootWithChainId[] memory outputRoots = new Types.OutputRootWithChainId[](_length);
        for (uint256 i = 0; i < _length; i++) {
            outputRoots[i] = _outputRoot(
                uint256(keccak256(abi.encode(_seed, uint8(0), i))), keccak256(abi.encode(_seed, uint8(1), i))
            );
        }

        proof = Types.SuperRootProof({ version: SUPER_ROOT_VERSION, timestamp: _timestamp, outputRoots: outputRoots });
    }
}

/// @title Hashing_hashDepositTransaction_Test
/// @notice Tests the `hashDepositTransaction` function of the `Hashing` library.
contract Hashing_hashDepositTransaction_Test is CommonTest {
    /// @notice Tests that hashDepositTransaction returns the correct hash in a simple case.
    function testDiff_hashDepositTransaction_succeeds(
        address _from,
        address _to,
        uint256 _mint,
        uint256 _value,
        uint64 _gas,
        bytes memory _data,
        uint64 _logIndex
    )
        external
    {
        assertEq(
            Hashing.hashDepositTransaction(
                Types.UserDepositTransaction(
                    _from,
                    _to,
                    false, // isCreate
                    _value,
                    _mint,
                    _gas,
                    _data,
                    bytes32(0),
                    _logIndex
                )
            ),
            ffi.hashDepositTransaction(_from, _to, _mint, _value, _gas, _data, _logIndex)
        );
    }
}

/// @title Hashing_hashDepositSource_Test
/// @notice Tests the `hashDepositSource` function of the `Hashing` library.
contract Hashing_hashDepositSource_Test is CommonTest {
    /// @notice Tests that hashDepositSource returns the correct hash.
    /// @param _l1BlockHash Hash of the L1 block where the deposit was included.
    /// @param _logIndex The index of the log that created the deposit transaction.
    function testFuzz_hashDepositSource_succeeds(bytes32 _l1BlockHash, uint256 _logIndex) external pure {
        bytes32 depositId = keccak256(abi.encode(_l1BlockHash, _logIndex));
        bytes32 expected = keccak256(abi.encode(bytes32(0), depositId));
        assertEq(Hashing.hashDepositSource(_l1BlockHash, _logIndex), expected);
    }

    /// @notice Tests that hashDepositSource returns the correct hash for a known vector.
    function test_hashDepositSource_knownVector_succeeds() external pure {
        assertEq(
            Hashing.hashDepositSource(0xd25df7858efc1778118fb133ac561b138845361626dfb976699c5287ed0f4959, 0x1),
            0xf923fb07134d7d287cb52c770cc619e17e82606c21a875c92f4c63b65280a5cc
        );
    }
}

/// @title Hashing_hashCrossDomainMessage_Test
/// @notice Tests the `hashCrossDomainMessage` function of the `Hashing` library.
contract Hashing_hashCrossDomainMessage_Test is CommonTest {
    /// @notice Tests that hashCrossDomainMessage returns the correct hash in a simple case.
    function testDiff_hashCrossDomainMessage_succeeds(
        uint240 _nonce,
        uint16 _version,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
    {
        uint16 version = uint16(bound(uint256(_version), 0, 1));
        uint256 nonce = Encoding.encodeVersionedNonce(_nonce, version);

        assertEq(
            Hashing.hashCrossDomainMessage(nonce, _sender, _target, _value, _gasLimit, _data),
            ffi.hashCrossDomainMessage(nonce, _sender, _target, _value, _gasLimit, _data)
        );
    }

    /// @notice Tests that hashCrossDomainMessage reverts with unknown version.
    /// @param _nonce Message nonce base value.
    /// @param _version Invalid version number (will be bounded to 2+).
    function testFuzz_hashCrossDomainMessage_unknownVersion_reverts(uint240 _nonce, uint16 _version) external {
        uint16 invalidVersion = uint16(bound(uint256(_version), 2, type(uint16).max));
        uint256 nonce = Encoding.encodeVersionedNonce(_nonce, invalidVersion);
        Hashing_Harness harness = new Hashing_Harness();

        vm.expectRevert(bytes("Hashing: unknown cross domain message version"));
        harness.hashCrossDomainMessage(nonce, address(this), address(this), 1, 100, hex"");
    }
}

/// @title Hashing_hashWithdrawal_Test
/// @notice Tests the `hashWithdrawal` function of the `Hashing` library.
contract Hashing_hashWithdrawal_Test is CommonTest {
    /// @notice Tests that hashWithdrawal returns the correct hash in a simple case.
    function testDiff_hashWithdrawal_succeeds(
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
    {
        assertEq(
            Hashing.hashWithdrawal(Types.WithdrawalTransaction(_nonce, _sender, _target, _value, _gasLimit, _data)),
            ffi.hashWithdrawal(_nonce, _sender, _target, _value, _gasLimit, _data)
        );
    }
}

/// @title Hashing_hashOutputRootProof_Test
/// @notice Tests the `hashOutputRootProof` function of the `Hashing` library.
contract Hashing_hashOutputRootProof_Test is CommonTest {
    /// @notice Tests that hashOutputRootProof returns the correct hash in a simple case.
    function testDiff_hashOutputRootProof_succeeds(
        bytes32 _stateRoot,
        bytes32 _messagePasserStorageRoot,
        bytes32 _latestBlockhash
    )
        external
    {
        bytes32 version = 0;
        assertEq(
            Hashing.hashOutputRootProof(
                Types.OutputRootProof({
                    version: version,
                    stateRoot: _stateRoot,
                    messagePasserStorageRoot: _messagePasserStorageRoot,
                    latestBlockhash: _latestBlockhash
                })
            ),
            ffi.hashOutputRootProof(version, _stateRoot, _messagePasserStorageRoot, _latestBlockhash)
        );
    }
}

/// @title Hashing_hashL2toL2CrossDomainMessage_Test
/// @notice Tests the `hashL2toL2CrossDomainMessage` function of the `Hashing` library.
contract Hashing_hashL2toL2CrossDomainMessage_Test is CommonTest {
    /// @notice Tests that hashL2toL2CrossDomainMessage returns the correct hash.
    /// @param _destination Chain ID of the destination chain.
    /// @param _source Chain ID of the source chain.
    /// @param _nonce Unique nonce associated with the message.
    /// @param _sender Address of the user who originally sent the message.
    /// @param _target Address of the contract or wallet that the message is targeting.
    /// @param _message The message payload to be relayed to the target.
    function testFuzz_hashL2toL2CrossDomainMessage_succeeds(
        uint256 _destination,
        uint256 _source,
        uint256 _nonce,
        address _sender,
        address _target,
        bytes memory _message
    )
        external
        pure
    {
        bytes32 expected = keccak256(abi.encode(_destination, _source, _nonce, _sender, _target, _message));
        assertEq(
            Hashing.hashL2toL2CrossDomainMessage(_destination, _source, _nonce, _sender, _target, _message), expected
        );
    }
}

/// @title Hashing_hashSuperRootProof_Test
/// @notice Tests the `hashSuperRootProof` function of the `Hashing` library.
contract Hashing_hashSuperRootProof_Test is Hashing_TestInit {
    /// @notice Tests that the Solidity impl of hashSuperRootProof matches the FFI impl
    /// @param _timestamp The timestamp of the super root proof.
    /// @param _length The number of output roots in the super root proof.
    /// @param _seed The seed used to generate the output roots.
    function testDiff_hashSuperRootProof_succeeds(uint64 _timestamp, uint256 _length, uint256 _seed) external {
        Types.SuperRootProof memory proof = _superRootProof(_timestamp, _length, _seed);

        assertEq(Hashing.hashSuperRootProof(proof), ffi.hashSuperRootProof(proof), "hash mismatch");
    }

    /// @notice Tests that hashSuperRootProof reverts when the version is incorrect.
    /// @param _version The version to use for the super root proof.
    /// @param _timestamp The timestamp of the super root proof.
    function testFuzz_hashSuperRootProof_wrongVersion_reverts(bytes1 _version, uint64 _timestamp) external {
        if (_version == SUPER_ROOT_VERSION) {
            _version = 0x00;
        }

        Types.OutputRootWithChainId[] memory outputRoots = new Types.OutputRootWithChainId[](1);
        outputRoots[0] = _outputRoot(1, bytes32(uint256(1)));

        Types.SuperRootProof memory proof =
            Types.SuperRootProof({ version: _version, timestamp: _timestamp, outputRoots: outputRoots });
        Hashing_Harness harness = new Hashing_Harness();

        vm.expectRevert(Encoding.Encoding_InvalidSuperRootVersion.selector);
        harness.hashSuperRootProof(proof);
    }
}
