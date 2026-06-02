// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IFeeVault } from "interfaces/L2/IFeeVault.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { FeeVault_Uncategorized_Test } from "test/L2/FeeVault.t.sol";
import { Types } from "src/libraries/Types.sol";
import { SemverComp } from "src/libraries/SemverComp.sol";
import { CommonTest } from "test/setup/CommonTest.sol";

contract L1FeeVault_Version_Test is CommonTest {
    function test_version_succeeds() external view {
        SemverComp.parse(l1FeeVault.version());
    }
}

contract L1FeeVault_Uncategorized_Test is FeeVault_Uncategorized_Test {
    function setUp() public virtual override {
        super.setUp();
        recipient = deploy.cfg().l1FeeVaultRecipient();
        minWithdrawalAmount = deploy.cfg().l1FeeVaultMinimumWithdrawalAmount();
        feeVault = IFeeVault(payable(Predeploys.L1_FEE_VAULT));
        withdrawalNetwork = Types.WithdrawalNetwork(deploy.cfg().l1FeeVaultWithdrawalNetwork());
    }
}
