// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { GameTypes } from "src/libraries/bridge/Types.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";
import { CommonTest } from "test/setup/CommonTest.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

contract ForkLive_Run_Test is CommonTest {
    address internal constant ENTRYPOINT = 0x73a79Fab69143498Ed3712e519A88a918e1f4072;
    string internal constant ALLOCS_PATH = "deployments/fork-live-test.json";

    function setUp() public override {
        super.setUp();
        AggregateVerifier gameImpl = new AggregateVerifier(
            GameTypes.AGGREGATE_VERIFIER,
            anchorStateRegistry,
            delayedWeth,
            IVerifier(address(0)),
            IVerifier(address(0)),
            bytes32(0),
            AggregateVerifier.ZkHashes(bytes32(0), bytes32(0)),
            bytes32(0),
            systemConfig.l2ChainId(),
            AggregateVerifier.IntervalConfig({
                slowBlockInterval: 100,
                slowIntermediateBlockInterval: 10,
                fastBlockInterval: 1000,
                fastIntermediateBlockInterval: 100
            }),
            AggregateVerifier.ScheduleConfig({
                protocolVersions: protocolVersions, genesisBlockNumber: 0, genesisTimestamp: 1, blockTime: 2
            })
        );
        vm.prank(disputeGameFactory.owner());
        disputeGameFactory.setImplementation(GameTypes.AGGREGATE_VERIFIER, IDisputeGame(address(gameImpl)));
        vm.chainId(1);
        vm.etch(ENTRYPOINT, address(systemConfig).code);
        vm.copyStorage(address(systemConfig), ENTRYPOINT);
        // dumpState needs the initialized storage still cached in the setup transaction.
        vm.dumpState(ALLOCS_PATH);
    }

    // vm.setEnv is process-wide, so these paths must not run in parallel tests.
    function test_run_requiresAndLoadsOpsAllocs_succeeds() public {
        string memory previousPath = vm.envOr("SUPERCHAIN_OPS_ALLOCS_PATH", string(""));
        vm.setEnv("SUPERCHAIN_OPS_ALLOCS_PATH", "");
        vm.expectRevert("ForkLive: SUPERCHAIN_OPS_ALLOCS_PATH must be set");
        forkLive.run();

        address systemConfigImpl = EIP1967Helper.getImplementation(address(systemConfig));
        address delayedWethImpl = EIP1967Helper.getImplementation(address(delayedWeth));
        vm.etch(ENTRYPOINT, hex"");
        EIP1967Helper.setImplementation(address(delayedWeth), address(0));
        vm.setEnv("SUPERCHAIN_OPS_ALLOCS_PATH", ALLOCS_PATH);

        forkLive.run();

        assertEq(artifacts.mustGetAddress("SystemConfigProxy"), ENTRYPOINT);
        assertEq(artifacts.mustGetAddress("SystemConfigImpl"), systemConfigImpl);
        assertEq(artifacts.mustGetAddress("DelayedWETHImpl"), delayedWethImpl);
        assertTrue(forkLive.useOpsRepo());
        vm.setEnv("SUPERCHAIN_OPS_ALLOCS_PATH", previousPath);
        vm.removeFile(ALLOCS_PATH);
    }
}
