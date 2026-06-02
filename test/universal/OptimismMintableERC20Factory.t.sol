// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";
import { NextImpl } from "test/mocks/NextImpl.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

// Contracts
import { OptimismMintableERC20 } from "src/universal/OptimismMintableERC20.sol";
import { OptimismMintableERC20Factory } from "src/universal/OptimismMintableERC20Factory.sol";

// Interfaces
import { IProxy } from "interfaces/universal/IProxy.sol";
import { IOptimismMintableERC20Factory } from "interfaces/universal/IOptimismMintableERC20Factory.sol";

/// @title OptimismMintableERC20Factory_TestInit
/// @notice Reusable test initialization for `OptimismMintableERC20Factory` tests.
abstract contract OptimismMintableERC20Factory_TestInit is CommonTest {
    event StandardL2TokenCreated(address indexed remoteToken, address indexed localToken);
    event OptimismMintableERC20Created(address indexed localToken, address indexed remoteToken, address deployer);

    string internal constant REMOTE_TOKEN_ZERO_REVERT =
        "OptimismMintableERC20Factory: must provide remote token address";

    /// @notice Precalculates the address of the token contract.
    function _calculateTokenAddress(
        address _remote,
        string memory _name,
        string memory _symbol,
        uint8 _decimals
    )
        internal
        view
        returns (address)
    {
        bytes memory constructorArgs = abi.encode(address(l2StandardBridge), _remote, _name, _symbol, _decimals);
        bytes memory bytecode = abi.encodePacked(type(OptimismMintableERC20).creationCode, constructorArgs);
        bytes32 salt = keccak256(abi.encode(_remote, _name, _symbol, _decimals));
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0xff), address(l2OptimismMintableERC20Factory), salt, keccak256(bytecode))
        );
        return address(uint160(uint256(hash)));
    }
}

/// @title OptimismMintableERC20Factory_Constructor_Test
/// @notice Tests the `constructor` function of the `OptimismMintableERC20Factory` contract.
contract OptimismMintableERC20Factory_Constructor_Test is OptimismMintableERC20Factory_TestInit {
    /// @notice Tests that the constructor is initialized correctly.
    function test_constructor_succeeds() external {
        IOptimismMintableERC20Factory impl = IOptimismMintableERC20Factory(address(new OptimismMintableERC20Factory()));
        assertEq(address(impl.BRIDGE()), address(0));
        assertEq(address(impl.bridge()), address(0));
    }
}

/// @title OptimismMintableERC20Factory_Initialize_Test
/// @notice Tests the `initialize` function of the `OptimismMintableERC20Factory` contract.
contract OptimismMintableERC20Factory_Initialize_Test is OptimismMintableERC20Factory_TestInit {
    /// @notice Tests that the proxy is initialized correctly.
    function test_initialize_succeeds() external view {
        assertEq(address(l1OptimismMintableERC20Factory.BRIDGE()), address(l1StandardBridge));
        assertEq(address(l1OptimismMintableERC20Factory.bridge()), address(l1StandardBridge));
    }
}

/// @title OptimismMintableERC20Factory_CreateStandardL2Token_Test
/// @notice Tests the `createStandardL2Token` function of the `OptimismMintableERC20Factory`
///         contract.
contract OptimismMintableERC20Factory_CreateStandardL2Token_Test is OptimismMintableERC20Factory_TestInit {
    /// @notice Test that calling `createStandardL2Token` with valid parameters succeeds.
    function test_createStandardL2Token_succeeds(
        address _caller,
        address _remoteToken,
        string memory _name,
        string memory _symbol
    )
        external
    {
        vm.assume(_remoteToken != address(0));

        address local = _calculateTokenAddress(_remoteToken, _name, _symbol, 18);

        vm.expectEmit(address(l2OptimismMintableERC20Factory));
        emit StandardL2TokenCreated(_remoteToken, local);

        vm.expectEmit(address(l2OptimismMintableERC20Factory));
        emit OptimismMintableERC20Created(local, _remoteToken, _caller);

        vm.prank(_caller);
        address addr = l2OptimismMintableERC20Factory.createStandardL2Token(_remoteToken, _name, _symbol);

        assertEq(addr, local);
        assertEq(OptimismMintableERC20(local).decimals(), 18);
        assertEq(l2OptimismMintableERC20Factory.deployments(local), _remoteToken);
    }

    /// @notice Test that calling `createOptimismMintableERC20WithDecimals` with valid parameters
    ///         succeeds.
    function test_createOptimismMintableERC20WithDecimals_succeeds(
        address _caller,
        address _remoteToken,
        string memory _name,
        string memory _symbol,
        uint8 _decimals
    )
        external
    {
        vm.assume(_remoteToken != address(0));

        address local = _calculateTokenAddress(_remoteToken, _name, _symbol, _decimals);

        vm.expectEmit(address(l2OptimismMintableERC20Factory));
        emit StandardL2TokenCreated(_remoteToken, local);

        vm.expectEmit(address(l2OptimismMintableERC20Factory));
        emit OptimismMintableERC20Created(local, _remoteToken, _caller);

        vm.prank(_caller);
        address addr = l2OptimismMintableERC20Factory.createOptimismMintableERC20WithDecimals(
            _remoteToken, _name, _symbol, _decimals
        );

        assertEq(addr, local);
        assertEq(OptimismMintableERC20(local).decimals(), _decimals);
        assertEq(l2OptimismMintableERC20Factory.deployments(local), _remoteToken);
    }

    /// @notice Test that calling `createStandardL2Token` with the same parameters twice reverts.
    function test_createStandardL2Token_sameTwice_reverts(
        address _caller,
        address _remoteToken,
        string memory _name,
        string memory _symbol
    )
        external
    {
        vm.assume(_remoteToken != address(0));

        vm.prank(_caller);
        l2OptimismMintableERC20Factory.createStandardL2Token(_remoteToken, _name, _symbol);

        vm.expectRevert(bytes(""));

        vm.prank(_caller);
        l2OptimismMintableERC20Factory.createStandardL2Token(_remoteToken, _name, _symbol);
    }

    /// @notice Test that calling `createOptimismMintableERC20WithDecimals` with the same parameters
    ///         twice reverts.
    function test_createOptimismMintableERC20WithDecimals_sameTwice_reverts(
        address _caller,
        address _remoteToken,
        string memory _name,
        string memory _symbol,
        uint8 _decimals
    )
        external
    {
        vm.assume(_remoteToken != address(0));

        vm.prank(_caller);
        l2OptimismMintableERC20Factory.createOptimismMintableERC20WithDecimals(_remoteToken, _name, _symbol, _decimals);

        vm.expectRevert(bytes(""));

        vm.prank(_caller);
        l2OptimismMintableERC20Factory.createOptimismMintableERC20WithDecimals(_remoteToken, _name, _symbol, _decimals);
    }

    /// @notice Test that calling `createStandardL2Token` with a zero remote token address reverts.
    function test_createStandardL2Token_remoteIsZero_reverts(
        address _caller,
        string memory _name,
        string memory _symbol
    )
        external
    {
        vm.expectRevert(bytes(REMOTE_TOKEN_ZERO_REVERT));

        vm.prank(_caller);
        l2OptimismMintableERC20Factory.createStandardL2Token(address(0), _name, _symbol);
    }

    /// @notice Test that calling `createOptimismMintableERC20WithDecimals` with a zero remote token
    ///         address reverts.
    function test_createOptimismMintableERC20WithDecimals_remoteIsZero_reverts(
        address _caller,
        string memory _name,
        string memory _symbol,
        uint8 _decimals
    )
        external
    {
        vm.expectRevert(bytes(REMOTE_TOKEN_ZERO_REVERT));

        vm.prank(_caller);
        l2OptimismMintableERC20Factory.createOptimismMintableERC20WithDecimals(address(0), _name, _symbol, _decimals);
    }
}

/// @title OptimismMintableERC20Factory_Uncategorized_Test
/// @notice General tests that are not testing any function directly of the
///         `OptimismMintableERC20Factory` contract.
contract OptimismMintableERC20Factory_Uncategorized_Test is OptimismMintableERC20Factory_TestInit {
    /// @notice Tests that the upgrade is successful.
    function test_upgrading_succeeds() external {
        IProxy proxy = IProxy(artifacts.mustGetAddress("OptimismMintableERC20FactoryProxy"));
        // Check an unused slot before upgrading.
        bytes32 slot21Before = vm.load(address(l1OptimismMintableERC20Factory), bytes32(uint256(21)));
        assertEq(bytes32(0), slot21Before);

        NextImpl nextImpl = new NextImpl();
        vm.startPrank(EIP1967Helper.getAdmin(address(proxy)));
        // Reviewer note: the NextImpl() still uses reinitializer. If we want to remove that, we'll
        // need to use a two step upgrade with the Storage lib.
        proxy.upgradeToAndCall(address(nextImpl), abi.encodeCall(NextImpl.initialize, (2)));
        assertEq(proxy.implementation(), address(nextImpl));

        // Verify that the NextImpl contract initialized its values as expected.
        bytes32 slot21After = vm.load(address(l1OptimismMintableERC20Factory), bytes32(uint256(21)));
        bytes32 slot21Expected = NextImpl(address(l1OptimismMintableERC20Factory)).slot21Init();
        assertEq(slot21Expected, slot21After);
    }
}
