// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Scripts
import { Vm } from "lib/forge-std/src/Vm.sol";

// Libraries
import { LibString } from "lib/solady/src/utils/LibString.sol";
import { Bytes } from "src/libraries/Bytes.sol";

library DeployUtils {
    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    bytes32 internal constant DEFAULT_SALT = keccak256("op-stack-contract-impls-salt-v0");

    /// @notice Deploys a contract with the given name and arguments via CREATE.
    /// @param _name Name of the contract to deploy.
    /// @param _args ABI-encoded constructor arguments.
    /// @return addr_ Address of the deployed contract.
    function create1(string memory _name, bytes memory _args) internal returns (address payable addr_) {
        bytes memory bytecode = abi.encodePacked(vm.getCode(_name), _args);
        assembly {
            addr_ := create(0, add(bytecode, 0x20), mload(bytecode))
        }
        assertValidContractAddress(addr_);
    }

    function create2asm(bytes memory _initCode, bytes32 _salt) private returns (address payable addr_) {
        assembly {
            addr_ := create2(0, add(_initCode, 0x20), mload(_initCode), _salt)
            if iszero(addr_) {
                let size := returndatasize()
                returndatacopy(0, 0, size)
                revert(0, size)
            }
        }
        assertValidContractAddress(addr_);
    }

    /// @notice Deploys a contract with the given name using CREATE2. If the contract is already deployed, this method
    /// does nothing.
    /// @param _name Name of the contract to deploy.
    /// @param _args ABI-encoded constructor arguments.
    function createDeterministic(
        string memory _name,
        bytes memory _args,
        bytes32 _salt
    )
        internal
        returns (address payable addr_)
    {
        bytes memory initCode = abi.encodePacked(vm.getCode(_name), _args);
        address preComputedAddress = vm.computeCreate2Address(_salt, keccak256(initCode));
        if (preComputedAddress.code.length > 0) {
            addr_ = payable(preComputedAddress);
        } else {
            vm.broadcast(msg.sender);
            addr_ = create2asm(initCode, _salt);
        }
    }

    /// @notice Strips the first 4 bytes of `_data` and returns the remaining bytes
    ///         If `_data` is not greater than 4 bytes, it returns empty bytes type.
    /// @param _data constructor arguments prefixed with a psuedo-constructor function signature
    /// @return encodedData_ constructor arguments without the psuedo-constructor function signature prefix
    function encodeConstructor(bytes memory _data) internal pure returns (bytes memory encodedData_) {
        require(_data.length >= 4, "DeployUtils: encodeConstructor takes in _data of length >= 4");
        encodedData_ = Bytes.slice(_data, 4);
    }

    /// @notice Asserts that the given address is a valid contract address.
    /// @param _who Address to check.
    function assertValidContractAddress(address _who) internal view {
        // Foundry will set returned address to address(1) whenever a contract creation fails
        // inside of a test. If this is the case then let Foundry handle the error itself and don't
        // trigger a revert (which would probably break a test).
        if (_who == address(1)) return;
        require(_who != address(0), "DeployUtils: zero address");
        require(_who.code.length > 0, string.concat("DeployUtils: no code at ", LibString.toHexStringChecksummed(_who)));
    }

    /// @notice Asserts that the given list of addresses does not contain duplicates.
    /// @param _addrs Addresses to check.
    function assertUniqueAddresses(address[] memory _addrs) internal pure {
        // All addresses should be unique.
        for (uint256 i = 0; i < _addrs.length; i++) {
            for (uint256 j = i + 1; j < _addrs.length; j++) {
                require(
                    _addrs[i] != _addrs[j],
                    string.concat(
                        "DeployUtils: check failed, duplicates at ", LibString.toString(i), ",", LibString.toString(j)
                    )
                );
            }
        }
    }

    /// @notice Asserts that the given addresses are valid contract addresses.
    /// @param _addrs Addresses to check.
    function assertValidContractAddresses(address[] memory _addrs) internal view {
        // Assert that all addresses are non-zero and have code.
        // We use LibString to avoid the need for adding cheatcodes to this contract.
        for (uint256 i = 0; i < _addrs.length; i++) {
            address who = _addrs[i];
            assertValidContractAddress(who);
        }

        // All addresses should be unique.
        assertUniqueAddresses(_addrs);
    }

    /// @notice Etches a contract, labels it, and allows cheatcodes for it.
    /// @param _etchTo Address of the contract to etch.
    /// @param _cname The contract name (also used to label the contract). MUST be the name of both the file and the
    ///               contract.
    function etchLabelAndAllowCheatcodes(address _etchTo, string memory _cname) internal {
        vm.etch(_etchTo, vm.getDeployedCode(string.concat(_cname, ".s.sol:", _cname)));
        vm.label(_etchTo, _cname);
        vm.allowCheatcodes(_etchTo);
    }
}
