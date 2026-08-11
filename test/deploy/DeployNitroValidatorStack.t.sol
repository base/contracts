// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import { Test } from "lib/forge-std/src/Test.sol";

import { DeployNitroValidatorStack } from "scripts/multiproof/DeployNitroValidatorStack.s.sol";

contract DeployNitroValidatorStackTest is Test {
    DeployNitroValidatorStack internal deployer;

    function setUp() public {
        deployer = new DeployNitroValidatorStack();
    }

    function test_run_succeeds() public {
        address owner = makeAddr("owner");
        address revoker = makeAddr("revoker");

        DeployNitroValidatorStack.Deployment memory deployment = deployer.run(owner, revoker);

        assertNotEq(address(deployment.p384Verifier), address(0));
        assertNotEq(address(deployment.certManager), address(0));
        assertNotEq(address(deployment.nitroValidator), address(0));
        assertLe(address(deployment.p384Verifier).code.length, 24_576);
        assertLe(address(deployment.certManager).code.length, 24_576);
        assertLe(address(deployment.nitroValidator).code.length, 24_576);

        assertEq(address(deployment.certManager.p384Verifier()), address(deployment.p384Verifier));
        assertEq(deployment.certManager.owner(), owner);
        assertEq(deployment.certManager.revoker(), revoker);
        assertGt(deployment.certManager.verified(deployment.certManager.ROOT_CA_CERT_HASH()).length, 0);
        assertEq(address(deployment.nitroValidator.certManager()), address(deployment.certManager));
        assertEq(address(deployment.nitroValidator.p384Verifier()), address(deployment.p384Verifier));
    }

    function test_run_zeroOwner_reverts() public {
        vm.expectRevert(DeployNitroValidatorStack.InvalidOwner.selector);
        deployer.run(address(0), makeAddr("revoker"));
    }

    function test_run_zeroRevoker_reverts() public {
        vm.expectRevert(DeployNitroValidatorStack.InvalidRevoker.selector);
        deployer.run(makeAddr("owner"), address(0));
    }
}
