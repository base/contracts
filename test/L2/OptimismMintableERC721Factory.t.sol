// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CommonTest } from "test/setup/CommonTest.sol";
import { OptimismMintableERC721 } from "src/L2/OptimismMintableERC721.sol";

/// @title OptimismMintableERC721Factory_TestInit
/// @notice Reusable test initialization for `OptimismMintableERC721Factory` tests.
abstract contract OptimismMintableERC721Factory_TestInit is CommonTest {
    address internal constant REMOTE_TOKEN = address(1234);
    string internal constant TOKEN_NAME = "L2Token";
    string internal constant TOKEN_SYMBOL = "L2T";

    event OptimismMintableERC721Created(address indexed localToken, address indexed remoteToken, address deployer);

    /// @notice Precalculates the address of the token contract.
    function _calculateTokenAddress(
        address _remote,
        string memory _name,
        string memory _symbol
    )
        internal
        view
        returns (address)
    {
        bytes memory constructorArgs =
            abi.encode(address(l2ERC721Bridge), deploy.cfg().l1ChainId(), _remote, _name, _symbol);
        bytes memory bytecode = abi.encodePacked(type(OptimismMintableERC721).creationCode, constructorArgs);
        bytes32 salt = keccak256(abi.encode(_remote, _name, _symbol));
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0xff), address(l2OptimismMintableERC721Factory), salt, keccak256(bytecode))
        );
        return address(uint160(uint256(hash)));
    }
}

/// @title OptimismMintableERC721Factory_Constructor_Test
/// @notice Tests the `constructor` of the `OptimismMintableERC721Factory` contract.
contract OptimismMintableERC721Factory_Constructor_Test is OptimismMintableERC721Factory_TestInit {
    /// @notice Tests that the constructor sets the correct values.
    function test_constructor_succeeds() external view {
        assertEq(l2OptimismMintableERC721Factory.BRIDGE(), address(l2ERC721Bridge));
        assertEq(l2OptimismMintableERC721Factory.bridge(), address(l2ERC721Bridge));
        assertEq(l2OptimismMintableERC721Factory.REMOTE_CHAIN_ID(), deploy.cfg().l1ChainId());
        assertEq(l2OptimismMintableERC721Factory.remoteChainID(), deploy.cfg().l1ChainId());
    }
}

/// @title OptimismMintableERC721Factory_CreateOptimismMintableERC721_Test
/// @notice Tests the `createOptimismMintableERC721` function of the
///         `OptimismMintableERC721Factory` contract.
contract OptimismMintableERC721Factory_CreateOptimismMintableERC721_Test is OptimismMintableERC721Factory_TestInit {
    /// @notice Tests that the `createOptimismMintableERC721` function succeeds.
    function test_createOptimismMintableERC721_succeeds() external {
        address local = _calculateTokenAddress(REMOTE_TOKEN, TOKEN_NAME, TOKEN_SYMBOL);

        vm.expectEmit(address(l2OptimismMintableERC721Factory));
        emit OptimismMintableERC721Created(local, REMOTE_TOKEN, alice);

        vm.prank(alice);
        OptimismMintableERC721 created = OptimismMintableERC721(
            l2OptimismMintableERC721Factory.createOptimismMintableERC721(REMOTE_TOKEN, TOKEN_NAME, TOKEN_SYMBOL)
        );

        assertEq(address(created), local);
        assertTrue(l2OptimismMintableERC721Factory.isOptimismMintableERC721(address(created)));

        assertEq(created.name(), TOKEN_NAME);
        assertEq(created.symbol(), TOKEN_SYMBOL);
        assertEq(created.REMOTE_TOKEN(), REMOTE_TOKEN);
        assertEq(created.BRIDGE(), address(l2ERC721Bridge));
        assertEq(created.REMOTE_CHAIN_ID(), deploy.cfg().l1ChainId());
    }

    /// @notice Tests that the `createOptimismMintableERC721` function reverts if the same token is
    ///         created twice.
    function test_createOptimismMintableERC721_sameTwice_reverts() external {
        vm.prank(alice);
        l2OptimismMintableERC721Factory.createOptimismMintableERC721(REMOTE_TOKEN, TOKEN_NAME, TOKEN_SYMBOL);

        vm.expectRevert(); // nosemgrep: sol-safety-expectrevert-no-args

        vm.prank(alice);
        l2OptimismMintableERC721Factory.createOptimismMintableERC721(REMOTE_TOKEN, TOKEN_NAME, TOKEN_SYMBOL);
    }

    /// @notice Tests that the `createOptimismMintableERC721` function reverts if the remote token
    ///         address is zero.
    function test_createOptimismMintableERC721_zeroRemoteToken_reverts() external {
        vm.expectRevert("OptimismMintableERC721Factory: L1 token address cannot be address(0)");
        l2OptimismMintableERC721Factory.createOptimismMintableERC721(address(0), TOKEN_NAME, TOKEN_SYMBOL);
    }
}
