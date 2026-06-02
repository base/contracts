// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";
import { IProxy } from "interfaces/universal/IProxy.sol";
import { Proxy } from "src/universal/Proxy.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

contract Proxy_SimpleStorage_Harness {
    mapping(uint256 => uint256) internal store;

    function get(uint256 key) external payable returns (uint256) {
        return store[key];
    }

    function set(uint256 key, uint256 value) external payable {
        store[key] = value;
    }
}

contract Proxy_Clasher_Harness {
    function upgradeTo(address) external pure {
        revert("Clasher: upgradeTo");
    }
}

/// @title Proxy_TestInit
/// @notice Reusable test initialization for `Proxy` tests.
abstract contract Proxy_TestInit is Test {
    event Upgraded(address indexed implementation);
    event AdminChanged(address previousAdmin, address newAdmin);

    address internal constant ADMIN = address(64);

    IProxy proxy;
    Proxy_SimpleStorage_Harness simpleStorage;

    function setUp() external {
        proxy = IProxy(payable(address(new Proxy(ADMIN))));
        simpleStorage = new Proxy_SimpleStorage_Harness();

        _upgradeTo(address(simpleStorage));
    }

    function _proxyStorage() internal view returns (Proxy_SimpleStorage_Harness) {
        return Proxy_SimpleStorage_Harness(address(proxy));
    }

    function _upgradeTo(address _newImplementation) internal {
        vm.prank(ADMIN);
        proxy.upgradeTo(_newImplementation);
    }

    function _getImplementation() internal returns (address) {
        vm.prank(ADMIN);
        return proxy.implementation();
    }

    function _adminAs(address _caller) internal returns (address) {
        vm.prank(_caller);
        return proxy.admin();
    }
}

/// @title Proxy_UpgradeTo_Test
/// @notice Tests the `upgradeTo` function of the `Proxy` contract.
contract Proxy_UpgradeTo_Test is Proxy_TestInit {
    function test_upgradeTo_notAdmin_succeeds() external {
        address newImplementation = address(128);

        vm.expectRevert(bytes(""));
        proxy.upgradeTo(newImplementation);

        vm.expectEmit(true, true, true, true);
        emit Upgraded(newImplementation);
        _upgradeTo(newImplementation);

        assertEq(_getImplementation(), newImplementation);
    }

    function test_upgradeTo_clashingFunctionSignatures_succeeds() external {
        Proxy_Clasher_Harness clasher = new Proxy_Clasher_Harness();

        _upgradeTo(address(clasher));
        assertEq(_getImplementation(), address(clasher));

        // Call the clashing function on the proxy not as the owner so that the call passes
        // through. The implementation will revert so we can be sure that the call passed through.
        vm.expectRevert(bytes("Clasher: upgradeTo"));
        proxy.upgradeTo(address(0));

        _upgradeTo(address(0));
        assertEq(_getImplementation(), address(0));
    }
}

/// @title Proxy_UpgradeToAndCall_Test
/// @notice Tests the `upgradeToAndCall` function of the `Proxy` contract.
contract Proxy_UpgradeToAndCall_Test is Proxy_TestInit {
    function test_upgradeToAndCall_succeeds() external {
        assertEq(_proxyStorage().get(1), 0);

        Proxy_SimpleStorage_Harness newSimpleStorage = new Proxy_SimpleStorage_Harness();

        vm.expectEmit(true, true, true, true);
        emit Upgraded(address(newSimpleStorage));
        vm.prank(ADMIN);
        proxy.upgradeToAndCall(address(newSimpleStorage), abi.encodeCall(Proxy_SimpleStorage_Harness.set, (1, 1)));

        assertEq(_proxyStorage().get(1), 1);
    }

    function test_upgradeToAndCall_functionDoesNotExist_reverts() external {
        address initialImplementation = _getImplementation();
        assertEq(initialImplementation, address(simpleStorage));

        Proxy_SimpleStorage_Harness newSimpleStorage = new Proxy_SimpleStorage_Harness();

        // Set the new SimpleStorage as the implementation and call. This reverts because the
        // calldata doesn't match a function on the implementation.
        vm.expectRevert("Proxy: delegatecall to new implementation contract failed");
        vm.prank(ADMIN);
        proxy.upgradeToAndCall(address(newSimpleStorage), hex"");

        assertEq(_getImplementation(), initialImplementation);

        vm.expectRevert(bytes(""));
        proxy.upgradeToAndCall(address(newSimpleStorage), abi.encodeCall(Proxy_SimpleStorage_Harness.set, (1, 1)));
    }

    function test_upgradeToAndCall_isPayable_succeeds() external {
        uint256 value = 1 ether;

        vm.deal(ADMIN, value);
        vm.prank(ADMIN);
        proxy.upgradeToAndCall{ value: value }(
            address(simpleStorage), abi.encodeCall(Proxy_SimpleStorage_Harness.set, (1, 1))
        );

        assertEq(_getImplementation(), address(simpleStorage));
        assertEq(address(proxy).balance, value);
    }
}

/// @title Proxy_ChangeAdmin_Test
/// @notice Tests the `changeAdmin` function of the `Proxy` contract.
contract Proxy_ChangeAdmin_Test is Proxy_TestInit {
    function test_changeAdmin_ownerKey_succeeds() external {
        address newAdmin = address(6);

        vm.prank(ADMIN);
        proxy.changeAdmin(newAdmin);

        assertEq(EIP1967Helper.getAdmin(address(proxy)), newAdmin);
        assertEq(_adminAs(newAdmin), newAdmin);
    }
}

/// @title Proxy_Admin_Test
/// @notice Tests the `admin` function of the `Proxy` contract.
contract Proxy_Admin_Test is Proxy_TestInit {
    function test_admin_notAdmin_succeeds() external {
        address newAdmin = address(1);

        vm.expectRevert(bytes(""));
        proxy.changeAdmin(newAdmin);

        vm.expectEmit(true, true, true, true);
        emit AdminChanged(ADMIN, newAdmin);
        vm.prank(ADMIN);
        proxy.changeAdmin(newAdmin);

        vm.expectRevert(bytes(""));
        proxy.admin();

        assertEq(_adminAs(newAdmin), newAdmin);
    }
}

/// @title Proxy_Implementation_Test
/// @notice Tests the `implementation` function of the `Proxy` contract.
contract Proxy_Implementation_Test is Proxy_TestInit {
    function test_implementation_key_succeeds() external {
        address newImplementation = address(6);

        _upgradeTo(newImplementation);

        assertEq(EIP1967Helper.getImplementation(address(proxy)), newImplementation);
        assertEq(_getImplementation(), newImplementation);
    }

    // Allow for `eth_call` to call proxy methods by setting "from" to `address(0)`.
    function test_implementation_zeroAddressCaller_succeeds() external {
        vm.prank(address(0));
        address impl = proxy.implementation();
        assertEq(impl, address(simpleStorage));
    }

    function test_implementation_isZeroAddress_reverts() external {
        _upgradeTo(address(0));

        vm.expectRevert("Proxy: implementation not initialized");
        _proxyStorage().get(1);
    }
}

/// @title Proxy_Delegation_Test
/// @notice Tests proxy delegation behavior.
contract Proxy_Delegation_Test is Proxy_TestInit {
    function test_delegatesToImpl_succeeds() external {
        _proxyStorage().set(1, 1);

        assertEq(simpleStorage.get(1), 0);
        assertEq(_proxyStorage().get(1), 1);

        // The owner should be able to call through the proxy when there is no selector clash.
        vm.prank(ADMIN);
        assertEq(_proxyStorage().get(1), 1);
    }
}
