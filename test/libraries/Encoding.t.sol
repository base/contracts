// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";

// Libraries
import { Encoding } from "src/libraries/Encoding.sol";
import { Types } from "src/libraries/Types.sol";

contract Encoding_Harness {
    function encodeCrossDomainMessage(
        uint256 nonce,
        address sender,
        address target,
        uint256 value,
        uint256 gasLimit,
        bytes memory data
    )
        external
        pure
        returns (bytes memory)
    {
        return Encoding.encodeCrossDomainMessage(nonce, sender, target, value, gasLimit, data);
    }

    function encodeSuperRootProof(Types.SuperRootProof memory proof) external pure returns (bytes memory) {
        return Encoding.encodeSuperRootProof(proof);
    }
}

/// @title Encoding_TestInit
/// @notice Reusable helpers for `Encoding` tests.
abstract contract Encoding_TestInit is CommonTest {
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

    function _expectedSuperRootProofEncoding(Types.SuperRootProof memory _proof)
        internal
        pure
        returns (bytes memory expected)
    {
        expected = bytes.concat(_proof.version, bytes8(_proof.timestamp));
        for (uint256 i = 0; i < _proof.outputRoots.length; i++) {
            expected = bytes.concat(expected, bytes32(_proof.outputRoots[i].chainId), _proof.outputRoots[i].root);
        }
    }
}

/// @title Encoding_EncodeDepositTransaction_Test
/// @notice Tests the `encodeDepositTransaction` function of the `Encoding` contract.
contract Encoding_EncodeDepositTransaction_Test is Encoding_TestInit {
    /// @notice Tests deposit transaction encoding.
    function testDiff_encodeDepositTransaction_succeeds(
        address _from,
        address _to,
        uint256 _mint,
        uint256 _value,
        uint64 _gas,
        bool isCreate,
        bytes memory _data,
        uint64 _logIndex
    )
        external
    {
        Types.UserDepositTransaction memory depositTx = Types.UserDepositTransaction(
            _from, _to, isCreate, _value, _mint, _gas, _data, bytes32(uint256(0)), _logIndex
        );

        bytes memory actual = Encoding.encodeDepositTransaction(depositTx);
        bytes memory expected = ffi.encodeDepositTransaction(depositTx);

        assertEq(actual, expected);
    }
}

/// @title Encoding_EncodeCrossDomainMessage_Test
/// @notice Tests the `encodeCrossDomainMessage` function of the `Encoding` contract.
contract Encoding_EncodeCrossDomainMessage_Test is Encoding_TestInit {
    /// @notice Tests cross domain message encoding.
    function testDiff_encodeCrossDomainMessage_succeeds(
        uint240 _nonce,
        uint8 _version,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
    {
        uint8 version = uint8(bound(uint256(_version), 0, 1));
        uint256 nonce = Encoding.encodeVersionedNonce(_nonce, version);

        bytes memory actual = Encoding.encodeCrossDomainMessage(nonce, _sender, _target, _value, _gasLimit, _data);
        bytes memory expected = ffi.encodeCrossDomainMessage(nonce, _sender, _target, _value, _gasLimit, _data);

        assertEq(actual, expected);
    }

    /// @notice Tests that encodeCrossDomainMessage reverts if version is greater than 1.
    function testFuzz_encodeCrossDomainMessage_versionGreaterThanOne_reverts(uint240 _nonce, uint16 _version) external {
        uint16 invalidVersion = uint16(bound(uint256(_version), 2, type(uint16).max));
        uint256 nonce = Encoding.encodeVersionedNonce(_nonce, invalidVersion);
        Encoding_Harness harness = new Encoding_Harness();

        vm.expectRevert(bytes("Encoding: unknown cross domain message version"));
        harness.encodeCrossDomainMessage(nonce, address(this), address(this), 1, 100, hex"");
    }
}

/// @title Encoding_EncodeSuperRootProof_Test
/// @notice Tests the `encodeSuperRootProof` function of the `Encoding` contract.
contract Encoding_EncodeSuperRootProof_Test is Encoding_TestInit {
    /// @notice Tests successful encoding of a valid super root proof
    /// @param _timestamp The timestamp of the super root proof
    /// @param _length The number of output roots in the super root proof
    /// @param _seed The seed used to generate the output roots
    function testFuzz_encodeSuperRootProof_succeeds(uint64 _timestamp, uint256 _length, uint256 _seed) external pure {
        Types.SuperRootProof memory proof = _superRootProof(_timestamp, _length, _seed);

        assertEq(Encoding.encodeSuperRootProof(proof), _expectedSuperRootProofEncoding(proof));
    }

    /// @notice Tests encoding with a single output root
    function test_encodeSuperRootProof_singleOutputRoot_succeeds() external pure {
        Types.OutputRootWithChainId[] memory outputRoots = new Types.OutputRootWithChainId[](1);
        outputRoots[0] = _outputRoot(10, bytes32(uint256(0xdeadbeef)));

        Types.SuperRootProof memory proof =
            Types.SuperRootProof({ version: SUPER_ROOT_VERSION, timestamp: 1234567890, outputRoots: outputRoots });

        assertEq(Encoding.encodeSuperRootProof(proof), _expectedSuperRootProofEncoding(proof));
    }

    /// @notice Tests encoding with multiple output roots
    function test_encodeSuperRootProof_multipleOutputRoots_succeeds() external pure {
        Types.OutputRootWithChainId[] memory outputRoots = new Types.OutputRootWithChainId[](3);
        outputRoots[0] = _outputRoot(10, bytes32(uint256(0xdeadbeef)));
        outputRoots[1] = _outputRoot(20, bytes32(uint256(0xbeefcafe)));
        outputRoots[2] = _outputRoot(30, bytes32(uint256(0xcafebabe)));

        Types.SuperRootProof memory proof =
            Types.SuperRootProof({ version: SUPER_ROOT_VERSION, timestamp: 1234567890, outputRoots: outputRoots });

        assertEq(Encoding.encodeSuperRootProof(proof), _expectedSuperRootProofEncoding(proof));
    }

    /// @notice Tests that the Solidity impl of encodeSuperRootProof matches the FFI impl
    /// @param _timestamp The timestamp of the super root proof
    /// @param _length The number of output roots in the super root proof
    /// @param _seed The seed used to generate the output roots
    function testDiff_encodeSuperRootProof_succeeds(uint64 _timestamp, uint256 _length, uint256 _seed) external {
        Types.SuperRootProof memory proof = _superRootProof(_timestamp, _length, _seed);

        assertEq(Encoding.encodeSuperRootProof(proof), ffi.encodeSuperRootProof(proof));
    }

    /// @notice Tests that encoding fails when version is not 0x01
    /// @param _version The version to use for the super root proof
    /// @param _timestamp The timestamp of the super root proof
    function testFuzz_encodeSuperRootProof_invalidVersion_reverts(bytes1 _version, uint64 _timestamp) external {
        if (_version == SUPER_ROOT_VERSION) {
            _version = 0x02;
        }

        Types.OutputRootWithChainId[] memory outputRoots = new Types.OutputRootWithChainId[](1);
        outputRoots[0] = _outputRoot(1, bytes32(uint256(1)));

        Types.SuperRootProof memory proof =
            Types.SuperRootProof({ version: _version, timestamp: _timestamp, outputRoots: outputRoots });
        Encoding_Harness harness = new Encoding_Harness();

        vm.expectRevert(Encoding.Encoding_InvalidSuperRootVersion.selector);
        harness.encodeSuperRootProof(proof);
    }

    /// @notice Tests that encoding fails when output roots array is empty
    /// @param _timestamp The timestamp of the super root proof
    function testFuzz_encodeSuperRootProof_emptyOutputRoots_reverts(uint64 _timestamp) external {
        Types.OutputRootWithChainId[] memory outputRoots = new Types.OutputRootWithChainId[](0);
        Types.SuperRootProof memory proof =
            Types.SuperRootProof({ version: SUPER_ROOT_VERSION, timestamp: _timestamp, outputRoots: outputRoots });
        Encoding_Harness harness = new Encoding_Harness();

        vm.expectRevert(Encoding.Encoding_EmptySuperRoot.selector);
        harness.encodeSuperRootProof(proof);
    }
}

/// @title Encoding_Uncategorized_Test
/// @notice General tests that are not testing any function directly of the `Encoding` contract or
///         are testing multiple functions at once.
contract Encoding_Uncategorized_Test is Encoding_TestInit {
    /// @notice Tests encoding and decoding a nonce and version.
    function testFuzz_nonceVersioning_succeeds(uint240 _nonce, uint16 _version) external pure {
        (uint240 nonce, uint16 version) = Encoding.decodeVersionedNonce(Encoding.encodeVersionedNonce(_nonce, _version));
        assertEq(version, _version);
        assertEq(nonce, _nonce);
    }

    /// @notice Tests decoding a versioned nonce.
    function testDiff_decodeVersionedNonce_succeeds(uint240 _nonce, uint16 _version) external {
        uint256 nonce = uint256(Encoding.encodeVersionedNonce(_nonce, _version));
        (uint256 decodedNonce, uint256 decodedVersion) = ffi.decodeVersionedNonce(nonce);

        assertEq(_version, uint16(decodedVersion));

        assertEq(_nonce, uint240(decodedNonce));
    }

    /// @notice Tests decoding and re-encoding a versioned nonce.
    function testFuzz_encodedNonceRoundTrip_succeeds(uint256 _versionedNonce) external pure {
        (uint240 nonce, uint16 version) = Encoding.decodeVersionedNonce(_versionedNonce);

        assertEq(Encoding.encodeVersionedNonce(nonce, version), _versionedNonce);
    }
}
