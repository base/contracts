// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";
import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";
import { IResolvedDelegateProxy } from "interfaces/legacy/IResolvedDelegateProxy.sol";
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";

contract ResolvedDelegateProxy_SimpleImplementation_Harness {
    function foo(uint256 _x) public pure returns (uint256) {
        return _x;
    }

    function bar() public pure {
        revert("SimpleImplementation: revert");
    }
}

abstract contract ResolvedDelegateProxy_TestBase is Test {
    string internal constant IMPLEMENTATION_NAME = "SimpleImplementation";

    function _deployAddressManager() internal returns (IAddressManager) {
        return IAddressManager(
            DeployUtils.create1({
                _name: "AddressManager",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IAddressManager.__constructor__, ()))
            })
        );
    }

    function _deployProxy(IAddressManager _addressManager) internal returns (IResolvedDelegateProxy) {
        return IResolvedDelegateProxy(
            DeployUtils.create1({
                _name: "ResolvedDelegateProxy",
                _args: DeployUtils.encodeConstructor(
                    abi.encodeCall(IResolvedDelegateProxy.__constructor__, (_addressManager, IMPLEMENTATION_NAME))
                )
            })
        );
    }

    function _proxyAsImplementation(IResolvedDelegateProxy _proxy)
        internal
        pure
        returns (ResolvedDelegateProxy_SimpleImplementation_Harness)
    {
        return ResolvedDelegateProxy_SimpleImplementation_Harness(address(_proxy));
    }
}

contract ResolvedDelegateProxy_Fallback_Test is ResolvedDelegateProxy_TestBase {
    IAddressManager internal addressManager;
    ResolvedDelegateProxy_SimpleImplementation_Harness internal impl;
    IResolvedDelegateProxy internal proxy;

    function setUp() public {
        addressManager = _deployAddressManager();
        impl = new ResolvedDelegateProxy_SimpleImplementation_Harness();
        addressManager.setAddress(IMPLEMENTATION_NAME, address(impl));

        proxy = _deployProxy(addressManager);
    }

    function testFuzz_fallback_delegateCallFoo_succeeds(uint256 x) public {
        vm.expectCall(address(impl), abi.encodeCall(impl.foo, (x)));
        assertEq(_proxyAsImplementation(proxy).foo(x), x);
    }

    function test_fallback_delegateCallBar_reverts() public {
        vm.expectRevert("SimpleImplementation: revert");
        vm.expectCall(address(impl), abi.encodeCall(impl.bar, ()));
        _proxyAsImplementation(proxy).bar();
    }
}

contract ResolvedDelegateProxy_FallbackNoImplementation_Test is ResolvedDelegateProxy_TestBase {
    function test_fallback_addressManagerNotSet_reverts() public {
        IResolvedDelegateProxy proxy = _deployProxy(_deployAddressManager());

        vm.expectRevert("ResolvedDelegateProxy: target address must be initialized");
        _proxyAsImplementation(proxy).foo(0);
    }
}
