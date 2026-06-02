// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test, stdError } from "lib/forge-std/src/Test.sol";
import { ResourceMetering } from "src/L1/ResourceMetering.sol";
import { Constants } from "src/libraries/Constants.sol";
import { IResourceMetering } from "interfaces/L1/IResourceMetering.sol";

uint256 constant MAX_EMPTY_BLOCK_GAP = 433_576_281_058_164_217_753_225_238_677_900_874_458_690;

function defaultResourceConfig() pure returns (ResourceMetering.ResourceConfig memory) {
    IResourceMetering.ResourceConfig memory rcfg = Constants.DEFAULT_RESOURCE_CONFIG();
    ResourceMetering.ResourceConfig memory config;
    assembly ("memory-safe") {
        config := rcfg
    }
    return config;
}

contract MeterUser is ResourceMetering {
    ResourceMetering.ResourceConfig public innerConfig;

    constructor() {
        initialize();
        innerConfig = defaultResourceConfig();
    }

    function initialize() public initializer {
        __ResourceMetering_init();
    }

    function resourceConfig() public view returns (ResourceMetering.ResourceConfig memory) {
        return _resourceConfig();
    }

    function _resourceConfig() internal view override returns (ResourceMetering.ResourceConfig memory) {
        return innerConfig;
    }

    function use(uint64 _amount) public metered(_amount) { }

    function measuredUse(uint64 _amount) public returns (uint256) {
        uint256 initialGas = gasleft();
        _metered(_amount, initialGas);
        return initialGas - gasleft();
    }

    function set(uint128 _prevBaseFee, uint64 _prevBoughtGas, uint64 _prevBlockNum) public {
        params = ResourceMetering.ResourceParams({
            prevBaseFee: _prevBaseFee, prevBoughtGas: _prevBoughtGas, prevBlockNum: _prevBlockNum
        });
    }

    function setParams(ResourceMetering.ResourceConfig memory newConfig) public {
        innerConfig = newConfig;
    }
}

/// @title ResourceMetering_TestInit
/// @notice Reusable test initialization for `ResourceMetering` tests.
abstract contract ResourceMetering_TestInit is Test {
    MeterUser internal meter;

    /// @notice Sets up the test contract.
    function setUp() public {
        meter = new MeterUser();
    }
}

/// @title ResourceMetering_Metered_Test
/// @notice Tests the `metered` modifier on the `ResourceMetering` contract.
/// @dev Tests are based on the default config values. It is expected that these config values are
///      used in production.
contract ResourceMetering_Metered_Test is ResourceMetering_TestInit {
    /// @notice Tests that updating the resource params to the same values works correctly.
    function test_metered_updateParamsNoChange_succeeds() external {
        meter.use(0); // equivalent to just updating the base fee and block number
        (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum) = meter.params();
        meter.use(0);
        (uint128 postBaseFee, uint64 postBoughtGas, uint64 postBlockNum) = meter.params();

        assertEq(postBaseFee, prevBaseFee);
        assertEq(postBoughtGas, prevBoughtGas);
        assertEq(postBlockNum, prevBlockNum);
    }

    /// @notice Tests that updating after multiple empty blocks maintains correct base fee.
    function testFuzz_metered_emptyBlocks_succeeds(uint256 _blockDiff) external {
        _blockDiff = bound(_blockDiff, 1, 100);
        uint256 startBlock = block.number;
        vm.roll(startBlock + _blockDiff);
        meter.use(0);
        (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum) = meter.params();

        assertEq(prevBaseFee, 1 gwei);
        assertEq(prevBoughtGas, 0);
        assertEq(prevBlockNum, startBlock + _blockDiff);
    }

    /// @notice Tests that updating the gas delta sets the meter params correctly.
    function test_metered_updateNoGasDelta_succeeds() external {
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();
        uint256 target = uint256(rcfg.maxResourceLimit) / uint256(rcfg.elasticityMultiplier);
        meter.use(uint64(target));
        (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum) = meter.params();

        assertEq(prevBaseFee, rcfg.minimumBaseFee);
        assertEq(prevBoughtGas, target);
        assertEq(prevBlockNum, block.number);
    }

    /// @notice Tests that the meter params are set correctly for the maximum gas delta.
    function test_metered_useMax_succeeds() external {
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();
        uint64 maxResourceLimit = uint64(rcfg.maxResourceLimit);

        meter.use(maxResourceLimit);

        (, uint64 prevBoughtGas,) = meter.params();
        assertEq(prevBoughtGas, maxResourceLimit);

        vm.roll(block.number + 1);
        meter.use(0);
        (uint128 postBaseFee,,) = meter.params();
        assertEq(postBaseFee, 2125000000);
    }

    /// @notice Tests that the metered modifier reverts if the baseFeeMaxChangeDenominator is set
    ///         to 1. Since the metered modifier internally calls solmate's powWad function, it
    ///         will revert with the error string "UNDEFINED" since the first parameter will be
    ///         computed as 0.
    function test_metered_denominatorEq1_reverts() external {
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();
        uint64 maxResourceLimit = uint64(rcfg.maxResourceLimit);
        rcfg.baseFeeMaxChangeDenominator = 1;
        meter.setParams(rcfg);
        meter.use(maxResourceLimit);

        (, uint64 prevBoughtGas,) = meter.params();
        assertEq(prevBoughtGas, maxResourceLimit);

        vm.roll(block.number + 2);

        vm.expectRevert("UNDEFINED");
        meter.use(0);
    }

    /// @notice Tests that the metered modifier reverts if the value is greater than allowed.
    function test_metered_useMoreThanMax_reverts() external {
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();

        vm.expectRevert(ResourceMetering.OutOfGas.selector);
        meter.use(uint64(rcfg.maxResourceLimit) + 1);
    }

    /// @notice Tests that resource metering can handle large gaps between deposits.
    function testFuzz_metered_largeBlockDiff_succeeds(uint64 _amount, uint256 _blockDiff) external {
        // Bounds the empty block gap to values that keep the base fee decay math in range.
        _blockDiff = uint256(bound(_blockDiff, 0, MAX_EMPTY_BLOCK_GAP));

        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();

        _amount = uint64(bound(_amount, 0, rcfg.maxResourceLimit));

        vm.roll(block.number + _blockDiff);
        meter.use(_amount);
    }

    function testFuzz_metered_useGas_succeeds(uint64 _amount) external {
        (, uint64 prevBoughtGas,) = meter.params();
        _amount = uint64(bound(_amount, 0, meter.resourceConfig().maxResourceLimit - prevBoughtGas));

        meter.use(_amount);

        (, uint64 postPrevBoughtGas,) = meter.params();
        assertEq(postPrevBoughtGas, prevBoughtGas + _amount);
    }

    /// @notice Tests that metering works correctly when block.basefee is 0.
    function test_metered_zeroBlockBaseFee_succeeds() external {
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();
        uint64 target = uint64(rcfg.maxResourceLimit) / uint64(rcfg.elasticityMultiplier);

        vm.fee(0);

        meter.use(target / 2);

        (uint128 prevBaseFee, uint64 prevBoughtGas,) = meter.params();
        assertEq(prevBoughtGas, target / 2);
        assertGt(prevBaseFee, 0);
    }

    /// @notice Tests that base fee decreases when gas usage is below target.
    function test_metered_belowTargetUsage_succeeds() external {
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();
        uint64 target = uint64(rcfg.maxResourceLimit) / uint64(rcfg.elasticityMultiplier);

        meter.use(target * 2);
        vm.roll(block.number + 1);
        meter.use(target * 2);
        vm.roll(block.number + 1);
        meter.use(0);

        (uint128 baselineBaseFee,,) = meter.params();

        vm.roll(block.number + 1);
        meter.use(target / 2);
        vm.roll(block.number + 1);
        meter.use(0);

        (uint128 newBaseFee,,) = meter.params();

        assertLt(newBaseFee, baselineBaseFee);
    }

    /// @notice Tests metering with minimum base fee configuration.
    function test_metered_minimumBaseFee_succeeds() external {
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();

        meter.set(uint128(rcfg.minimumBaseFee), 0, uint64(block.number));

        meter.use(100);

        (, uint64 prevBoughtGas,) = meter.params();
        assertEq(prevBoughtGas, 100);
    }
}

/// @title ResourceMetering_ResourceMeteringInit_Test
/// @notice Tests the `__ResourceMeteringInit` contract's initialization.
contract ResourceMetering_ResourceMeteringInit_Test is ResourceMetering_TestInit {
    /// @notice Tests that the initial resource params are set correctly.
    function test_resourceMeteringInit_initialResourceParams_succeeds() external view {
        (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum) = meter.params();
        ResourceMetering.ResourceConfig memory rcfg = meter.resourceConfig();

        assertEq(prevBaseFee, rcfg.minimumBaseFee);
        assertEq(prevBoughtGas, 0);
        assertEq(prevBlockNum, block.number);
    }

    /// @notice Tests that reinitializing the resource params are set correctly.
    function test_resourceMeteringInit_reinitializedResourceParams_succeeds() external {
        (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum) = meter.params();

        vm.store(address(meter), bytes32(uint256(0)), bytes32(uint256(0)));
        meter.initialize();

        (uint128 postBaseFee, uint64 postBoughtGas, uint64 postBlockNum) = meter.params();
        assertEq(prevBaseFee, postBaseFee);
        assertEq(prevBoughtGas, postBoughtGas);
        assertEq(prevBlockNum, postBlockNum);
    }
}

/// @title ArtifactResourceMetering_Metered_Test
/// @notice A table test that sets the state of the ResourceParams and then requests various
///         amounts of gas. This test ensures that a wide range of values can safely be used with
///         the `ResourceMetering` contract. It also writes a CSV file to disk that includes useful
///         information about how much gas is used and how expensive it is in USD terms to purchase
///         the deposit gas.
contract ArtifactResourceMetering_Metered_Test is Test {
    string internal outfile;

    /// @notice Generates a CSV file. No more than the L1 block gas limit should be supplied to the
    ///         `meter` function to avoid long execution time. This test is skipped because there
    ///         is no need to run it every time. It generates a CSV file on disk that can be used
    ///         to analyze the gas usage and cost of the `ResourceMetering` contract. The next time
    ///         that the gas usage needs to be analyzed, the skip may be removed.
    function test_meter_generateArtifact_succeeds() external {
        vm.skip({ skipTest: true });

        vm.roll(1_000_000);

        ResourceMetering.ResourceConfig memory rcfg = defaultResourceConfig();
        uint128 minimumBaseFee = uint128(rcfg.minimumBaseFee);
        uint128 maximumBaseFee = rcfg.maximumBaseFee;
        uint64 maxResourceLimit = uint64(rcfg.maxResourceLimit);
        uint64 targetResourceLimit = uint64(rcfg.maxResourceLimit) / uint64(rcfg.elasticityMultiplier);
        outfile = string.concat(vm.projectRoot(), "/.resource-metering.csv");
        try vm.removeFile(outfile) { } catch { }

        vm.writeLine(
            outfile,
            "prevBaseFee,prevBoughtGas,prevBlockNumDiff,l1BaseFee,requestedGas,gasConsumed,ethPrice,usdCost,success"
        );

        // prevBaseFee value in ResourceParams
        uint128[] memory prevBaseFees = new uint128[](5);
        prevBaseFees[0] = minimumBaseFee;
        prevBaseFees[1] = maximumBaseFee;
        prevBaseFees[2] = uint128(50 gwei);
        prevBaseFees[3] = uint128(100 gwei);
        prevBaseFees[4] = uint128(200 gwei);

        // prevBlockNum diff, simulates blocks with no deposits when non zero
        uint64[] memory prevBlockNumDiffs = new uint64[](2);
        prevBlockNumDiffs[0] = 0;
        prevBlockNumDiffs[1] = 1;

        // The amount of L2 gas that a user requests
        uint64[] memory requestedGases = new uint64[](3);
        requestedGases[0] = maxResourceLimit;
        requestedGases[1] = targetResourceLimit;
        requestedGases[2] = uint64(100_000);

        // The L1 base fee
        uint256[] memory l1BaseFees = new uint256[](4);
        l1BaseFees[0] = 1 gwei;
        l1BaseFees[1] = 50 gwei;
        l1BaseFees[2] = 75 gwei;
        l1BaseFees[3] = 100 gwei;

        // USD price of 1 ether
        uint256[] memory ethPrices = new uint256[](2);
        ethPrices[0] = 1600;
        ethPrices[1] = 3200;

        // Iterate over all of the test values and run a test
        for (uint256 i; i < prevBaseFees.length; i++) {
            for (uint256 k; k < prevBlockNumDiffs.length; k++) {
                for (uint256 l; l < requestedGases.length; l++) {
                    for (uint256 m; m < l1BaseFees.length; m++) {
                        for (uint256 n; n < ethPrices.length; n++) {
                            uint256 snapshotId = vm.snapshot();

                            uint128 prevBaseFee = prevBaseFees[i];
                            uint64 prevBoughtGas = 0;
                            uint64 prevBlockNumDiff = prevBlockNumDiffs[k];
                            uint64 requestedGas = requestedGases[l];
                            uint256 l1BaseFee = l1BaseFees[m];
                            uint256 ethPrice = ethPrices[n];
                            string memory result = "success";

                            vm.fee(l1BaseFee);

                            MeterUser meter = new MeterUser();
                            meter.set(prevBaseFee, prevBoughtGas, uint64(block.number));

                            vm.roll(block.number + prevBlockNumDiff);

                            uint256 gasConsumed = 0;
                            try meter.measuredUse{ gas: 30_000_000 }(requestedGas) returns (uint256 gasConsumed_) {
                                gasConsumed = gasConsumed_;
                            } catch (bytes memory err) {
                                bytes4 selector;
                                if (err.length >= 4) {
                                    assembly {
                                        selector := mload(add(err, 0x20))
                                    }
                                }

                                if (selector == ResourceMetering.OutOfGas.selector) {
                                    result = "ResourceMetering: cannot buy more gas than available gas limit";
                                } else if (keccak256(err) == keccak256(stdError.arithmeticError)) {
                                    result = "arithmetic overflow/underflow";
                                } else if (err.length == 0) {
                                    result = "out of gas";
                                } else {
                                    result = "UNKNOWN ERROR";
                                }
                            }

                            uint256 usdCost = (gasConsumed * l1BaseFee * ethPrice) / 1 ether;

                            vm.writeLine(
                                outfile,
                                string.concat(
                                    vm.toString(prevBaseFee),
                                    ",",
                                    vm.toString(prevBoughtGas),
                                    ",",
                                    vm.toString(prevBlockNumDiff),
                                    ",",
                                    vm.toString(l1BaseFee),
                                    ",",
                                    vm.toString(requestedGas),
                                    ",",
                                    vm.toString(gasConsumed),
                                    ",",
                                    "$",
                                    vm.toString(ethPrice),
                                    ",",
                                    "$",
                                    vm.toString(usdCost),
                                    ",",
                                    result
                                )
                            );

                            assertTrue(vm.revertTo(snapshotId));
                        }
                    }
                }
            }
        }
    }
}
