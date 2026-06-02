// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.15;

// Forge
import { Test } from "lib/forge-std/src/Test.sol";
import { VmSafe } from "lib/forge-std/src/Vm.sol";

// Scripts
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";
import { Config } from "scripts/libraries/Config.sol";

// Libraries
import { LibString } from "lib/solady/src/utils/LibString.sol";

// Interfaces
import { IL1ChugSplashProxy } from "interfaces/legacy/IL1ChugSplashProxy.sol";

contract L1ChugSplashProxy_Owner_Harness {
    bool public isUpgrading = true;
}

contract L1ChugSplashProxy_Implementation_Harness {
    function setCode(bytes memory) public pure returns (uint256) {
        return 1;
    }

    function setStorage(bytes32, bytes32) public pure returns (uint256) {
        return 2;
    }

    function setOwner(address) public pure returns (uint256) {
        return 3;
    }

    function getOwner() public pure returns (uint256) {
        return 4;
    }

    function getImplementation() public pure returns (uint256) {
        return 5;
    }
}

/// @title L1ChugSplashProxy_TestInit
/// @notice Reusable test initialization for `L1ChugSplashProxy` tests.
abstract contract L1ChugSplashProxy_TestInit is Test {
    bytes internal constant RETURN_42_BYTECODE = hex"604260005260206000f3";

    IL1ChugSplashProxy proxy;
    address owner = makeAddr("owner");
    address alice = makeAddr("alice");

    function setUp() public virtual {
        proxy = _deployProxy();
        vm.prank(owner);
        assertEq(proxy.getOwner(), owner);
    }

    function _deployProxy() internal returns (IL1ChugSplashProxy) {
        return IL1ChugSplashProxy(
            DeployUtils.create1({
                _name: "L1ChugSplashProxy",
                _args: DeployUtils.encodeConstructor(abi.encodeCall(IL1ChugSplashProxy.__constructor__, (owner)))
            })
        );
    }

    function _proxyAsImplementation() internal view returns (L1ChugSplashProxy_Implementation_Harness) {
        return L1ChugSplashProxy_Implementation_Harness(address(proxy));
    }
}

abstract contract L1ChugSplashProxy_WithImplementation_TestInit is L1ChugSplashProxy_TestInit {
    address impl;

    function setUp() public virtual override {
        super.setUp();

        vm.prank(owner);
        proxy.setCode(type(L1ChugSplashProxy_Implementation_Harness).runtimeCode);

        vm.prank(owner);
        impl = proxy.getImplementation();
    }
}

/// @title L1ChugSplashProxy_SetCode_Test
/// @notice Tests the `setCode` function of the `L1ChugSplashProxy` contract.
contract L1ChugSplashProxy_SetCode_Test is L1ChugSplashProxy_WithImplementation_TestInit {
    /// @notice Tests that the owner can deploy a new implementation with a given runtime code.
    function test_setCode_whenOwner_succeeds() public {
        vm.prank(owner);
        proxy.setCode(RETURN_42_BYTECODE);

        vm.prank(owner);
        assertNotEq(proxy.getImplementation(), impl);
    }

    /// @notice Tests that when not the owner, `setCode` delegatecalls the implementation.
    function test_setCode_whenNotOwner_works() public view {
        uint256 ret = _proxyAsImplementation().setCode(RETURN_42_BYTECODE);
        assertEq(ret, 1);
    }

    /// @notice Tests that when the owner deploys the same bytecode as the existing implementation,
    ///         it does not deploy a new implementation
    function test_setCode_whenOwnerSameBytecode_works() public {
        vm.prank(owner);
        proxy.setCode(type(L1ChugSplashProxy_Implementation_Harness).runtimeCode);

        vm.prank(owner);
        assertEq(proxy.getImplementation(), impl);
    }

    /// @notice Tests that when the owner calls `setCode` with insufficient gas to complete the
    ///         implementation contract's deployment, it reverts.
    /// @dev    If this solc version/settings change and modifying this proves time consuming, we
    ///         can just remove it.
    function test_setCode_whenOwnerAndDeployOutOfGas_reverts() public {
        // The values below are best gotten by removing the gas limit parameter from the call and
        // running the test with a verbosity of `-vvvv` then setting the value to a few thousand
        // gas lower than the gas used by the call. A faster way to do this for forge coverage
        // cases, is to comment out the optimizer and optimizer runs in the foundry.toml file and
        // then run forge test. This is faster because forge test only compiles modified contracts
        // unlike forge coverage.
        uint256 gasLimit;

        // Because forge coverage always runs with the optimizer disabled,
        // if forge coverage is run before testing this with forge test or forge snapshot, forge
        // clean should be run first so that it recompiles the contracts using the foundry.toml
        // optimizer settings.
        if (vm.isContext(VmSafe.ForgeContext.Coverage) || LibString.eq(Config.foundryProfile(), "lite")) {
            gasLimit = 95_000;
        } else if (vm.isContext(VmSafe.ForgeContext.Test) || vm.isContext(VmSafe.ForgeContext.Snapshot)) {
            gasLimit = 65_000;
        } else {
            revert("SafeCall_Test: unknown context");
        }

        vm.prank(owner);
        vm.expectRevert(bytes("L1ChugSplashProxy: code was not correctly deployed")); // Ran out of gas
        proxy.setCode{ gas: gasLimit }(
            hex"fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe"
        );
    }
}

/// @title L1ChugSplashProxy_SetStorage_Test
/// @notice Tests the `setStorage` function of the `L1ChugSplashProxy` contract.
contract L1ChugSplashProxy_SetStorage_Test is L1ChugSplashProxy_WithImplementation_TestInit {
    /// @notice Tests that the owner can set storage of the proxy
    function test_setStorage_whenOwner_works() public {
        vm.prank(owner);
        proxy.setStorage(bytes32(0), bytes32(uint256(42)));
        assertEq(vm.load(address(proxy), bytes32(0)), bytes32(uint256(42)));
    }

    /// @notice Tests that when not the owner, `setStorage` delegatecalls the implementation
    function test_setStorage_whenNotOwner_works() public view {
        uint256 ret = _proxyAsImplementation().setStorage(bytes32(0), bytes32(uint256(42)));
        assertEq(ret, 2);
        assertEq(vm.load(address(proxy), bytes32(0)), bytes32(uint256(0)));
    }
}

/// @title L1ChugSplashProxy_SetOwner_Test
/// @notice Tests the `setOwner` function of the `L1ChugSplashProxy` contract.
contract L1ChugSplashProxy_SetOwner_Test is L1ChugSplashProxy_WithImplementation_TestInit {
    /// @notice Tests that the owner can set the owner of the proxy
    function test_setOwner_whenOwner_works() public {
        vm.prank(owner);
        proxy.setOwner(alice);

        vm.prank(alice);
        assertEq(proxy.getOwner(), alice);
    }

    /// @notice Tests that when not the owner, `setOwner` delegatecalls the implementation
    function test_setOwner_whenNotOwner_works() public {
        uint256 ret = _proxyAsImplementation().setOwner(alice);
        assertEq(ret, 3);

        vm.prank(owner);
        assertEq(proxy.getOwner(), owner);
    }
}

/// @title L1ChugSplashProxy_GetOwner_Test
/// @notice Tests the `getOwner` function of the `L1ChugSplashProxy` contract.
contract L1ChugSplashProxy_GetOwner_Test is L1ChugSplashProxy_WithImplementation_TestInit {
    /// @notice Tests that the owner can get the owner of the proxy
    function test_getOwner_whenOwner_works() public {
        vm.prank(owner);
        assertEq(proxy.getOwner(), owner);
    }

    /// @notice Tests that when not the owner, `getOwner` delegatecalls the implementation
    function test_getOwner_whenNotOwner_works() public view {
        uint256 ret = _proxyAsImplementation().getOwner();
        assertEq(ret, 4);
    }
}

/// @title L1ChugSplashProxy_GetImplementation_Test
/// @notice Tests the `getImplementation` function of the `L1ChugSplashProxy` contract.
contract L1ChugSplashProxy_GetImplementation_Test is L1ChugSplashProxy_WithImplementation_TestInit {
    /// @notice Tests that the owner can get the implementation of the proxy
    function test_getImplementation_whenOwner_works() public {
        vm.prank(owner);
        assertEq(proxy.getImplementation(), impl);
    }

    /// @notice Tests that when not the owner, `getImplementation` delegatecalls the implementation
    function test_getImplementation_whenNotOwner_works() public view {
        uint256 ret = _proxyAsImplementation().getImplementation();
        assertEq(ret, 5);
    }
}

/// @title L1ChugSplashProxy_NoImplementation_Test
/// @notice Tests calls against a proxy before its implementation has been set.
contract L1ChugSplashProxy_NoImplementation_Test is L1ChugSplashProxy_TestInit {
    /// @notice Tests that when the caller is not the owner and the implementation is not set, all
    ///         calls reverts.
    function test_calls_whenNotOwnerNoImplementation_reverts() public {
        vm.expectRevert(bytes("L1ChugSplashProxy: implementation is not set yet"));
        _proxyAsImplementation().setCode(RETURN_42_BYTECODE);
    }
}

/// @title L1ChugSplashProxy_Upgrading_Test
/// @notice Tests calls against a proxy while its owner reports an active upgrade.
contract L1ChugSplashProxy_Upgrading_Test is L1ChugSplashProxy_WithImplementation_TestInit {
    /// @notice Tests that when the caller is not the owner but the owner has marked `isUpgrading`
    ///         as true, the call reverts.
    function test_calls_whenUpgrading_reverts() public {
        L1ChugSplashProxy_Owner_Harness ownerContract = new L1ChugSplashProxy_Owner_Harness();
        vm.prank(owner);
        proxy.setOwner(address(ownerContract));

        vm.expectRevert(bytes("L1ChugSplashProxy: system is currently being upgraded"));
        _proxyAsImplementation().setCode(RETURN_42_BYTECODE);
    }
}
