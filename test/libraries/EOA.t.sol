// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Forge
import { Test } from "lib/forge-std/src/Test.sol";

// Libraries
import { EOA } from "src/libraries/EOA.sol";

/// @title EOA_Harness
/// @notice A helper contract to test the EOA library.
contract EOA_Harness {
    /// @notice Returns true if the sender is an EOA.
    /// @return isEOA_ True if the sender is an EOA.
    function isSenderEOA() external view returns (bool isEOA_) {
        return EOA.isSenderEOA();
    }

    /// @notice Returns true if the account is a 7702 delegated EOA.
    /// @param _account Address of the account to check.
    /// @return isDelegated_ True if the account is a 7702 delegated EOA.
    function isDelegated(address _account) external view returns (bool isDelegated_) {
        return EOA.isDelegated(_account);
    }
}

/// @title EOA_isSenderEOA_Test
/// @notice Tests the `isSenderEOA` function of the `EOA` library.
contract EOA_isSenderEOA_Test is Test {
    EOA_Harness harness;

    /// @notice Sets up the test.
    function setUp() public {
        harness = new EOA_Harness();
    }

    /// @notice Tests that a standard EOA is detected as an EOA.
    /// @param _privateKey The private key of the sender.
    function testFuzz_isSenderEOA_isStandardEOA_succeeds(uint256 _privateKey) external {
        address sender = _senderFromPrivateKey(_privateKey);
        vm.assume(sender.code.length == 0);

        vm.prank(sender, sender);
        assertTrue(harness.isSenderEOA());
    }

    /// @notice Tests that a 7702 EOA is detected as an EOA.
    /// @param _privateKey The private key of the sender.
    /// @param _7702Target The target of the 7702 EOA.
    function testFuzz_isSenderEOA_is7702EOA_succeeds(uint256 _privateKey, address _7702Target) external {
        // Delegating to address(0) revokes the delegation per EIP-7702, so exclude it.
        vm.assume(_7702Target != address(0));

        address sender = _senderFromPrivateKey(_privateKey);
        vm.etch(sender, abi.encodePacked(hex"EF0100", _7702Target));

        vm.prank(sender, sender);
        assertTrue(harness.isSenderEOA());

        // Should still be considered an EOA even if origin is different.
        vm.prank(sender, address(0x0420));
        assertTrue(harness.isSenderEOA());
    }

    /// @notice Tests that a contract is not detected as an EOA.
    /// @param _privateKey The private key of the sender.
    /// @param _code The code of the sender.
    function testFuzz_isSenderEOA_isContract_succeeds(uint256 _privateKey, bytes memory _code) external {
        // Avoid empty code and the 0xEF prefix space, which includes 7702 delegation code.
        if (_code.length == 0 || _code[0] == 0xEF) {
            _code = bytes.concat(hex"FFFFFF", _code);
        }

        address sender = _senderFromPrivateKey(_privateKey);
        vm.etch(sender, _code);

        vm.prank(sender);
        assertFalse(harness.isSenderEOA());
    }

    /// @notice Tests that a contract with 23 bytes of code is not detected as an EOA.
    /// @param _privateKey The private key of the sender.
    function testFuzz_isSenderEOA_isContract23BytesNot7702_succeeds(uint256 _privateKey) external {
        address sender = _senderFromPrivateKey(_privateKey);
        vm.etch(sender, abi.encodePacked(hex"FE", bytes22(0)));

        vm.prank(sender);
        assertFalse(harness.isSenderEOA());
    }

    /// @notice Returns the sender for a valid secp256k1 private key.
    /// @param _privateKey The private key of the sender.
    /// @return sender_ The sender address.
    function _senderFromPrivateKey(uint256 _privateKey) internal pure returns (address sender_) {
        sender_ = vm.addr(boundPrivateKey(_privateKey));
    }
}

/// @title EOA_isDelegated_Test
/// @notice Tests the `isDelegated` function of the `EOA` library.
contract EOA_isDelegated_Test is Test {
    EOA_Harness harness;
    address account;

    /// @notice Sets up the test.
    function setUp() public {
        harness = new EOA_Harness();
        account = makeAddr("account");
    }

    /// @notice Tests that an account without code is not detected as a 7702 delegated EOA.
    function test_isDelegated_noCode_succeeds() external view {
        assertFalse(harness.isDelegated(account));
    }

    /// @notice Tests that a 7702 EOA is detected as a 7702 delegated EOA.
    /// @param _7702Target The target of the 7702 EOA.
    function testFuzz_isDelegated_is7702EOA_succeeds(address _7702Target) external {
        // Delegating to address(0) revokes the delegation per EIP-7702, so exclude it.
        vm.assume(_7702Target != address(0));

        vm.etch(account, abi.encodePacked(hex"EF0100", _7702Target));

        assertTrue(harness.isDelegated(account));
    }

    /// @notice Tests that a contract is not detected as a 7702 delegated EOA.
    /// @param _code The code of the contract.
    function testFuzz_isDelegated_isContract_succeeds(bytes memory _code) external {
        // Avoid empty code and the 0xEF prefix space, which includes 7702 delegation code.
        if (_code.length == 0 || _code[0] == 0xEF) {
            _code = bytes.concat(hex"FFFFFF", _code);
        }

        vm.etch(account, _code);

        assertFalse(harness.isDelegated(account));
    }

    /// @notice Tests that a contract with 23 bytes of code is not detected as a 7702 delegated EOA.
    function test_isDelegated_isContract23BytesNot7702_succeeds() external {
        vm.etch(account, abi.encodePacked(hex"FE", bytes22(0)));

        assertFalse(harness.isDelegated(account));
    }
}
