// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IFeeVault } from "interfaces/L2/IFeeVault.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { FeeVault_Uncategorized_Test } from "test/L2/FeeVault.t.sol";
import { Types } from "src/libraries/Types.sol";
import { SemverComp } from "src/libraries/SemverComp.sol";
import { CommonTest } from "test/setup/CommonTest.sol";

contract OperatorFeeVault_Uncategorized_Test is FeeVault_Uncategorized_Test {
    function setUp() public override {
        super.setUp();
        recipient = deploy.cfg().operatorFeeVaultRecipient();
        minWithdrawalAmount = deploy.cfg().operatorFeeVaultMinimumWithdrawalAmount();
        feeVault = IFeeVault(payable(Predeploys.OPERATOR_FEE_VAULT));
        withdrawalNetwork = Types.WithdrawalNetwork(uint8(deploy.cfg().operatorFeeVaultWithdrawalNetwork()));
    }
}

contract OperatorFeeVault_Version_Test is CommonTest {
    function test_version_validFormat_succeeds() external view {
        SemverComp.parse(operatorFeeVault.version());
    }
}
