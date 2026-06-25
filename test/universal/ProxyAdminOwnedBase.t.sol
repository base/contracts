// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

// Contracts
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";

/// @title ProxyAdminOwnedBase_Harness
/// @notice Contract implementing the abstract `ProxyAdminOwnedBase` contract so we can write unit
///         tests for the `ProxyAdminOwnedBase` contract.
contract ProxyAdminOwnedBase_Harness is ProxyAdminOwnedBase {
    /// @notice Slot 0, matching the legacy ResolvedDelegateProxy storage layout.
    mapping(address => string) public slot0;

    /// @notice Slot 1, matching the legacy ResolvedDelegateProxy storage layout.
    mapping(address => address) public slot1;

    /// @notice Assert that the proxy admin owner of the current contract is the same as the proxy
    ///         admin owner of the other Proxy address provided.
    function assertSharedProxyAdminOwner(address _proxy) public view {
        _assertSharedProxyAdminOwner(_proxy);
    }

    /// @notice Assert that the caller is the ProxyAdmin.
    function assertOnlyProxyAdmin() public view {
        _assertOnlyProxyAdmin();
    }

    /// @notice Assert that the caller is the ProxyAdmin owner.
    function assertOnlyProxyAdminOwner() public view {
        _assertOnlyProxyAdminOwner();
    }

    /// @notice Assert that the caller is the ProxyAdmin or the ProxyAdmin owner.
    function assertOnlyProxyAdminOrProxyAdminOwner() public view {
        _assertOnlyProxyAdminOrProxyAdminOwner();
    }
}

abstract contract ProxyAdminOwnedBase_TestInit is CommonTest {
    /// @notice Stored name value that identifies the legacy ResolvedDelegateProxy.
    bytes32 internal constant RESOLVED_DELEGATE_PROXY_NAME = bytes32("OVM_L1CrossDomainMessenger");

    /// @notice Length of the stored ResolvedDelegateProxy name.
    uint256 internal constant RESOLVED_DELEGATE_PROXY_NAME_LENGTH = 26;

    /// @notice Harness for the `ProxyAdminOwnedBase` contract.
    ProxyAdminOwnedBase_Harness public harness;

    /// @notice Sets up the test.
    function setUp() public override {
        super.setUp();

        harness = new ProxyAdminOwnedBase_Harness();
        EIP1967Helper.setAdmin(address(harness), address(proxyAdmin));
    }

    /// @notice Clears the EIP-1967 admin slot and sets the legacy ResolvedDelegateProxy slots.
    function setResolvedDelegateProxy(address _addressManager) internal {
        EIP1967Helper.setAdmin(address(harness), address(0));
        vm.store(address(harness), resolvedDelegateProxyNameSlot(), resolvedDelegateProxyNameSlotValue());
        vm.store(
            address(harness), resolvedDelegateProxyAddressManagerSlot(), bytes32(uint256(uint160(_addressManager)))
        );
    }

    /// @notice Stores a raw value in the ResolvedDelegateProxy name slot.
    function setResolvedDelegateProxyNameSlot(bytes32 _value) internal {
        EIP1967Helper.setAdmin(address(harness), address(0));
        vm.store(address(harness), resolvedDelegateProxyNameSlot(), _value);
    }

    /// @notice Returns the storage slot for `slot0[address(harness)]`.
    function resolvedDelegateProxyNameSlot() internal view returns (bytes32) {
        return keccak256(abi.encode(address(harness), uint256(0)));
    }

    /// @notice Returns the storage value Solidity uses for the short string name.
    function resolvedDelegateProxyNameSlotValue() internal pure returns (bytes32) {
        return bytes32(uint256(RESOLVED_DELEGATE_PROXY_NAME) | uint256(RESOLVED_DELEGATE_PROXY_NAME_LENGTH * 2));
    }

    /// @notice Returns the storage slot for `slot1[address(harness)]`.
    function resolvedDelegateProxyAddressManagerSlot() internal view returns (bytes32) {
        return keccak256(abi.encode(address(harness), uint256(1)));
    }
}

contract ProxyAdminOwnedBase_proxyAdminOwner_Test is ProxyAdminOwnedBase_TestInit {
    /// @notice Tests that the proxyAdminOwner function returns the correct owner.
    function test_proxyAdminOwner_succeeds() public view {
        assertEq(harness.proxyAdminOwner(), proxyAdminOwner);
    }
}

contract ProxyAdminOwnedBase_proxyAdmin_Test is ProxyAdminOwnedBase_TestInit {
    /// @notice Tests that the proxyAdmin function returns the correct proxy.
    function test_proxyAdmin_succeeds() public view {
        assertEq(address(harness.proxyAdmin()), address(proxyAdmin));
    }

    /// @notice Tests that the proxyAdmin function returns the correct proxy when the current
    ///         contract is a full ResolvedDelegateProxy.
    function test_proxyAdmin_fullResolvedDelegateProxy_succeeds() public {
        setResolvedDelegateProxy(address(addressManager));
        assertEq(address(harness.proxyAdmin()), address(proxyAdmin));
    }

    /// @notice Tests that the proxyAdmin function reverts if the current contract is not a
    ///         ResolvedDelegateProxy.
    /// @param _slot0Value The raw value to store in the ResolvedDelegateProxy name slot.
    function test_proxyAdmin_notResolvedDelegateProxy_reverts(bytes32 _slot0Value) public {
        vm.assume(_slot0Value != resolvedDelegateProxyNameSlotValue());
        setResolvedDelegateProxyNameSlot(_slot0Value);

        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotResolvedDelegateProxy.selector);
        harness.proxyAdmin();
    }

    /// @notice Tests that the proxyAdmin function reverts if the proxy admin is not found.
    function test_proxyAdmin_proxyAdminNotFound_reverts() public {
        setResolvedDelegateProxy(address(0));

        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_ProxyAdminNotFound.selector);
        harness.proxyAdmin();
    }
}

contract ProxyAdminOwnedBase_assertSharedProxyAdminOwner_Test is ProxyAdminOwnedBase_TestInit {
    /// @notice Tests that the assertSharedProxyAdminOwner function does not revert if the provided
    ///         proxy has the same owner as the current contract.
    function test_assertSharedProxyAdminOwner_sameOwner_succeeds(address _proxy) public {
        assumeNotForgeAddress(_proxy);

        vm.mockCall(_proxy, abi.encodeCall(ProxyAdminOwnedBase.proxyAdminOwner, ()), abi.encode(proxyAdminOwner));

        harness.assertSharedProxyAdminOwner(_proxy);
    }

    /// @notice Tests that the assertSharedProxyAdminOwner function reverts if the proxy admin
    ///         owner of both proxies is different.
    function testFuzz_assertSharedProxyAdminOwner_differentOwner_reverts(
        address _proxy,
        address _otherProxyOwner
    )
        public
    {
        assumeNotForgeAddress(_proxy);
        vm.assume(_otherProxyOwner != proxyAdminOwner);

        vm.mockCall(_proxy, abi.encodeCall(ProxyAdminOwnedBase.proxyAdminOwner, ()), abi.encode(_otherProxyOwner));

        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotSharedProxyAdminOwner.selector);
        harness.assertSharedProxyAdminOwner(_proxy);
    }
}

contract ProxyAdminOwnedBase_assertOnlyProxyAdmin_Test is ProxyAdminOwnedBase_TestInit {
    /// @notice Tests that the assertOnlyProxyAdmin function does not revert if the caller is the
    ///         ProxyAdmin.
    function test_assertOnlyProxyAdmin_proxyAdmin_succeeds() public {
        vm.prank(address(proxyAdmin));
        harness.assertOnlyProxyAdmin();
    }

    /// @notice Tests that the assertOnlyProxyAdmin function reverts if the caller is not the
    ///         ProxyAdmin.
    /// @param _sender The address of the sender to test.
    function test_assertOnlyProxyAdmin_notProxyAdmin_reverts(address _sender) public {
        vm.assume(_sender != address(proxyAdmin));
        vm.prank(_sender);

        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdmin.selector);
        harness.assertOnlyProxyAdmin();
    }
}

contract ProxyAdminOwnedBase_assertOnlyProxyAdminOwner_Test is ProxyAdminOwnedBase_TestInit {
    /// @notice Tests that the assertOnlyProxyAdminOwner function does not revert if the caller is
    ///         the ProxyAdmin owner.
    function test_assertOnlyProxyAdminOwner_proxyAdminOwner_succeeds() public {
        vm.prank(proxyAdminOwner);
        harness.assertOnlyProxyAdminOwner();
    }

    /// @notice Tests that the assertOnlyProxyAdminOwner function reverts if the caller is not the
    ///         ProxyAdmin owner.
    /// @param _sender The address of the sender to test.
    function test_assertOnlyProxyAdminOwner_notProxyAdminOwner_reverts(address _sender) public {
        vm.assume(_sender != proxyAdminOwner);
        vm.prank(_sender);

        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        harness.assertOnlyProxyAdminOwner();
    }
}

contract ProxyAdminOwnedBase_assertOnlyProxyAdminOrProxyAdminOwner_Test is ProxyAdminOwnedBase_TestInit {
    /// @notice Tests that the assertOnlyProxyAdminOrProxyAdminOwner function does not revert if
    ///         the caller is the ProxyAdmin or the ProxyAdmin owner.
    function test_assertOnlyProxyAdminOrProxyAdminOwner_proxyAdmin_succeeds() public {
        vm.prank(address(proxyAdmin));
        harness.assertOnlyProxyAdminOrProxyAdminOwner();
    }

    /// @notice Tests that the assertOnlyProxyAdminOrProxyAdminOwner function does not revert if
    ///         the caller is the ProxyAdmin owner.
    function test_assertOnlyProxyAdminOrProxyAdminOwner_proxyAdminOwner_succeeds() public {
        vm.prank(proxyAdminOwner);
        harness.assertOnlyProxyAdminOrProxyAdminOwner();
    }

    /// @notice Tests that the assertOnlyProxyAdminOrProxyAdminOwner function reverts if the caller
    ///         is not the ProxyAdmin or the ProxyAdmin owner.
    /// @param _sender The address of the sender to test.
    function test_assertOnlyProxyAdminOrProxyAdminOwner_notProxyAdminOrProxyAdminOwner_reverts(address _sender) public {
        vm.assume(_sender != address(proxyAdmin) && _sender != proxyAdminOwner);
        vm.prank(_sender);

        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner.selector);
        harness.assertOnlyProxyAdminOrProxyAdminOwner();
    }
}
