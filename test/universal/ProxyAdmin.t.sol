// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { Test } from "lib/forge-std/src/Test.sol";
import { Proxy_SimpleStorage_Harness } from "test/universal/Proxy.t.sol";

// Interfaces
import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";
import { IL1ChugSplashProxy } from "interfaces/legacy/IL1ChugSplashProxy.sol";
import { IResolvedDelegateProxy } from "interfaces/legacy/IResolvedDelegateProxy.sol";
import { IProxy } from "interfaces/universal/IProxy.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";

import { DeployUtils } from "scripts/libraries/DeployUtils.sol";

/// @title ProxyAdmin_TestInit
/// @notice Reusable test initialization for `ProxyAdmin` tests.
abstract contract ProxyAdmin_TestInit is Test {
    address internal constant PROXY_ADMIN_OWNER = address(64);
    address internal constant NEW_PROXY_ADMIN = address(128);

    IProxy proxy;
    IL1ChugSplashProxy chugsplash;
    IResolvedDelegateProxy resolved;

    IAddressManager addressManager;

    IProxyAdmin admin;

    Proxy_SimpleStorage_Harness implementation;

    function setUp() external {
        admin = IProxyAdmin(
            DeployUtils.create1({
                _name: "ProxyAdmin",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IProxyAdmin.__constructor__, (PROXY_ADMIN_OWNER)))
            })
        );

        proxy = IProxy(
            DeployUtils.create1({
                _name: "src/universal/Proxy.sol:Proxy",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IProxy.__constructor__, (address(admin))))
            })
        );

        chugsplash = IL1ChugSplashProxy(
            DeployUtils.create1({
                _name: "L1ChugSplashProxy",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IL1ChugSplashProxy.__constructor__, (address(admin)))
                )
            })
        );

        addressManager = IAddressManager(
            DeployUtils.create1({
                _name: "AddressManager",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IAddressManager.__constructor__, ()))
            })
        );
        addressManager.transferOwnership(address(admin));

        // Deploy a legacy ResolvedDelegateProxy with the name `a`. Whatever `a` is set to in
        // AddressManager will be the address that is used for the implementation.
        resolved = IResolvedDelegateProxy(
            DeployUtils.create1({
                _name: "ResolvedDelegateProxy",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IResolvedDelegateProxy.__constructor__, (addressManager, "a"))
                )
            })
        );

        vm.startPrank(PROXY_ADMIN_OWNER);
        // Set the address manager so the admin can resolve the
        // implementation address of legacy ResolvedDelegateProxy based proxies.
        admin.setAddressManager(addressManager);
        admin.setImplementationName(address(resolved), "a");

        admin.setProxyType(address(proxy), IProxyAdmin.ProxyType.ERC1967);
        admin.setProxyType(address(chugsplash), IProxyAdmin.ProxyType.CHUGSPLASH);
        admin.setProxyType(address(resolved), IProxyAdmin.ProxyType.RESOLVED);
        vm.stopPrank();

        implementation = new Proxy_SimpleStorage_Harness();
    }
}

/// @title ProxyAdmin_SetProxyType_Test
/// @notice Tests the `setProxyType` function of the `ProxyAdmin` contract.
contract ProxyAdmin_SetProxyType_Test is ProxyAdmin_TestInit {
    function test_setProxyType_notOwner_reverts() external {
        vm.expectRevert("Ownable: caller is not the owner");
        admin.setProxyType(address(0), IProxyAdmin.ProxyType.CHUGSPLASH);
    }
}

/// @title ProxyAdmin_SetImplementationName_Test
/// @notice Tests the `setImplementationName` function of the `ProxyAdmin` contract.
contract ProxyAdmin_SetImplementationName_Test is ProxyAdmin_TestInit {
    function test_setImplementationName_succeeds() external {
        vm.prank(PROXY_ADMIN_OWNER);
        admin.setImplementationName(address(1), "foo");
        assertEq(admin.implementationName(address(1)), "foo");
    }

    function test_setImplementationName_notOwner_reverts() external {
        vm.expectRevert("Ownable: caller is not the owner");
        admin.setImplementationName(address(0), "foo");
    }
}

/// @title ProxyAdmin_SetAddressManager_Test
/// @notice Tests the `setAddressManager` function of the `ProxyAdmin` contract.
contract ProxyAdmin_SetAddressManager_Test is ProxyAdmin_TestInit {
    function test_setAddressManager_notOwner_reverts() external {
        vm.expectRevert("Ownable: caller is not the owner");
        admin.setAddressManager(IAddressManager(address(0)));
    }
}

/// @title ProxyAdmin_IsUpgrading_Test
/// @notice Tests the `isUpgrading` function of the `ProxyAdmin` contract.
contract ProxyAdmin_IsUpgrading_Test is ProxyAdmin_TestInit {
    function test_isUpgrading_succeeds() external {
        assertFalse(admin.isUpgrading());

        vm.prank(PROXY_ADMIN_OWNER);
        admin.setUpgrading(true);
        assertTrue(admin.isUpgrading());
    }
}

/// @title ProxyAdmin_GetProxyImplementation_Test
/// @notice Tests the `getProxyImplementation` function of the `ProxyAdmin` contract.
contract ProxyAdmin_GetProxyImplementation_Test is ProxyAdmin_TestInit {
    function _assertProxyImplementation(address payable _proxy) internal {
        assertEq(admin.getProxyImplementation(_proxy), address(0));

        vm.prank(PROXY_ADMIN_OWNER);
        admin.upgrade(_proxy, address(implementation));

        assertEq(admin.getProxyImplementation(_proxy), address(implementation));
    }

    function test_getProxyImplementation_erc1967_succeeds() external {
        _assertProxyImplementation(payable(proxy));
    }

    function test_getProxyImplementation_chugsplash_succeeds() external {
        _assertProxyImplementation(payable(chugsplash));
    }

    function test_getProxyImplementation_resolved_succeeds() external {
        _assertProxyImplementation(payable(resolved));
    }
}

/// @title ProxyAdmin_GetProxyAdmin_Test
/// @notice Tests the `getProxyAdmin` function of the `ProxyAdmin` contract.
contract ProxyAdmin_GetProxyAdmin_Test is ProxyAdmin_TestInit {
    function _assertProxyAdmin(address payable _proxy) internal view {
        assertEq(admin.getProxyAdmin(_proxy), address(admin));
    }

    function test_getProxyAdmin_erc1967_succeeds() external view {
        _assertProxyAdmin(payable(proxy));
    }

    function test_getProxyAdmin_chugsplash_succeeds() external view {
        _assertProxyAdmin(payable(chugsplash));
    }

    function test_getProxyAdmin_resolved_succeeds() external view {
        _assertProxyAdmin(payable(resolved));
    }
}

/// @title ProxyAdmin_ChangeProxyAdmin_Test
/// @notice Tests the `changeProxyAdmin` function of the `ProxyAdmin` contract.
contract ProxyAdmin_ChangeProxyAdmin_Test is ProxyAdmin_TestInit {
    function _assertChangeProxyAdmin(address payable _proxy) internal {
        IProxyAdmin.ProxyType proxyType = admin.proxyType(address(_proxy));

        vm.prank(PROXY_ADMIN_OWNER);
        admin.changeProxyAdmin(_proxy, NEW_PROXY_ADMIN);

        // The proxy is no longer the admin and can
        // no longer call the proxy interface except for
        // the ResolvedDelegate type on which anybody can
        // call the admin interface.
        if (proxyType == IProxyAdmin.ProxyType.ERC1967) {
            vm.expectRevert("Proxy: implementation not initialized");
            admin.getProxyAdmin(_proxy);
        } else if (proxyType == IProxyAdmin.ProxyType.CHUGSPLASH) {
            vm.expectRevert("L1ChugSplashProxy: implementation is not set yet");
            admin.getProxyAdmin(_proxy);
        } else if (proxyType == IProxyAdmin.ProxyType.RESOLVED) {
            assertEq(admin.getProxyAdmin(_proxy), NEW_PROXY_ADMIN);
        }

        // Call the proxy contract directly to get the admin.
        // Different proxy types have different interfaces.
        vm.prank(NEW_PROXY_ADMIN);
        if (proxyType == IProxyAdmin.ProxyType.ERC1967) {
            assertEq(IProxy(payable(_proxy)).admin(), NEW_PROXY_ADMIN);
        } else if (proxyType == IProxyAdmin.ProxyType.CHUGSPLASH) {
            assertEq(IL1ChugSplashProxy(payable(_proxy)).getOwner(), NEW_PROXY_ADMIN);
        } else if (proxyType == IProxyAdmin.ProxyType.RESOLVED) {
            assertEq(addressManager.owner(), NEW_PROXY_ADMIN);
        }
    }

    function test_changeProxyAdmin_erc1967_succeeds() external {
        _assertChangeProxyAdmin(payable(proxy));
    }

    function test_changeProxyAdmin_chugsplash_succeeds() external {
        _assertChangeProxyAdmin(payable(chugsplash));
    }

    function test_changeProxyAdmin_resolved_succeeds() external {
        _assertChangeProxyAdmin(payable(resolved));
    }
}

/// @title ProxyAdmin_Upgrade_Test
/// @notice Tests the `upgrade` function of the `ProxyAdmin` contract.
contract ProxyAdmin_Upgrade_Test is ProxyAdmin_TestInit {
    function _assertUpgrade(address payable _proxy) internal {
        vm.prank(PROXY_ADMIN_OWNER);
        admin.upgrade(_proxy, address(implementation));

        assertEq(admin.getProxyImplementation(_proxy), address(implementation));
    }

    function test_upgrade_erc1967_succeeds() external {
        _assertUpgrade(payable(proxy));
    }

    function test_upgrade_chugsplash_succeeds() external {
        _assertUpgrade(payable(chugsplash));
    }

    function test_upgrade_resolved_succeeds() external {
        _assertUpgrade(payable(resolved));
    }
}

/// @title ProxyAdmin_UpgradeAndCall_Test
/// @notice Tests the `upgradeAndCall` function of the `ProxyAdmin` contract.
contract ProxyAdmin_UpgradeAndCall_Test is ProxyAdmin_TestInit {
    function _assertUpgradeAndCall(address payable _proxy) internal {
        vm.prank(PROXY_ADMIN_OWNER);
        admin.upgradeAndCall(_proxy, address(implementation), abi.encodeCall(Proxy_SimpleStorage_Harness.set, (1, 1)));

        assertEq(admin.getProxyImplementation(_proxy), address(implementation));

        assertEq(Proxy_SimpleStorage_Harness(address(_proxy)).get(1), 1);
    }

    function test_upgradeAndCall_erc1967_succeeds() external {
        _assertUpgradeAndCall(payable(proxy));
    }

    function test_upgradeAndCall_chugsplash_succeeds() external {
        _assertUpgradeAndCall(payable(chugsplash));
    }

    function test_upgradeAndCall_resolved_succeeds() external {
        _assertUpgradeAndCall(payable(resolved));
    }
}

/// @title ProxyAdmin_Uncategorized_Test
/// @notice General tests that are not testing any function directly or that test multiple
///         functions of the `ProxyAdmin` contract.
contract ProxyAdmin_Uncategorized_Test is ProxyAdmin_TestInit {
    function test_owner_succeeds() external view {
        assertEq(admin.owner(), PROXY_ADMIN_OWNER);
    }

    function test_proxyType_succeeds() external view {
        assertEq(uint256(admin.proxyType(address(proxy))), uint256(IProxyAdmin.ProxyType.ERC1967));
        assertEq(uint256(admin.proxyType(address(chugsplash))), uint256(IProxyAdmin.ProxyType.CHUGSPLASH));
        assertEq(uint256(admin.proxyType(address(resolved))), uint256(IProxyAdmin.ProxyType.RESOLVED));
    }

    function test_onlyOwner_notOwner_reverts() external {
        vm.expectRevert("Ownable: caller is not the owner");
        admin.changeProxyAdmin(payable(proxy), address(0));

        vm.expectRevert("Ownable: caller is not the owner");
        admin.upgrade(payable(proxy), address(implementation));

        vm.expectRevert("Ownable: caller is not the owner");
        admin.upgradeAndCall(payable(proxy), address(implementation), hex"");
    }
}
