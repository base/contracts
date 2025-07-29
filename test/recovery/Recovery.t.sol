// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {Test} from "forge-std/Test.sol";
import {Recovery} from "../../src/recovery/Recovery.sol";

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract RecoveryTest is Test {
    Recovery public recovery;
    Recovery public implementation;

    address public owner = makeAddr("owner");
    address public user = makeAddr("user");

    address public recipient1 = makeAddr("recipient1");
    address public recipient2 = makeAddr("recipient2");

    function setUp() public {
        // Deploy implementation
        implementation = new Recovery(owner);

        // Deploy proxy with implementation
        bytes memory initData = "";
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        recovery = Recovery(address(proxy));

        // Fund the recovery contract with some ETH
        vm.deal(address(recovery), 10 ether);

        // Fund test accounts
        vm.deal(owner, 1 ether);
        vm.deal(user, 1 ether);
    }

    function test_Owner() public view {
        assertEq(recovery.OWNER(), owner);
    }

    function test_WithdrawETH_Success() public {
        // Arrange
        address[] memory targets = new address[](2);
        targets[0] = recipient1;
        targets[1] = recipient2;

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 1 ether;
        amounts[1] = 2 ether;

        uint256 initialBalance1 = recipient1.balance;
        uint256 initialBalance2 = recipient2.balance;

        // Act
        vm.prank(owner);
        recovery.withdrawETH(targets, amounts);

        // Assert
        assertEq(recipient1.balance, initialBalance1 + 1 ether);
        assertEq(recipient2.balance, initialBalance2 + 2 ether);
    }

    function test_WithdrawETH_Unauthorized() public {
        // Arrange
        address[] memory targets = new address[](1);
        targets[0] = recipient1;

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1 ether;

        // Act & Assert
        vm.expectRevert(Recovery.Unauthorized.selector);
        vm.prank(user);
        recovery.withdrawETH(targets, amounts);
    }

    function test_WithdrawETH_SkipsFailedTransfers() public {
        // Create a contract that rejects ETH
        RejectETH rejecter = new RejectETH();

        // Arrange - set up 2 transfers where the 2nd one will fail
        address[] memory targets = new address[](2);
        targets[0] = recipient1; // This transfer will succeed
        targets[1] = address(rejecter); // This transfer will fail

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 1 ether;
        amounts[1] = 2 ether;

        // Record initial balances
        uint256 initialBalance1 = recipient1.balance;
        uint256 initialContractBalance = address(recovery).balance;

        // Act
        vm.prank(owner);
        recovery.withdrawETH(targets, amounts);

        // Assert - only the successful transfer should have gone through
        assertEq(recipient1.balance, initialBalance1 + 1 ether, "Recipient1 should receive ETH");
        assertEq(address(rejecter).balance, 0, "Rejecter should not receive ETH");
        assertEq(address(recovery).balance, initialContractBalance - 1 ether, "Contract should have sent only 1 ETH");
    }

    function test_AuthorizeUpgrade_OnlyOwner() public {
        // Deploy new implementation
        Recovery newImplementation = new Recovery(owner);

        // Non-owner cannot upgrade
        vm.expectRevert(Recovery.Unauthorized.selector);
        vm.prank(user);
        recovery.upgradeTo(address(newImplementation));

        // Owner can upgrade
        vm.prank(owner);
        recovery.upgradeTo(address(newImplementation));
    }
}

// Helper contract that rejects ETH
contract RejectETH {
    receive() external payable {
        revert("ETH rejected");
    }
}
