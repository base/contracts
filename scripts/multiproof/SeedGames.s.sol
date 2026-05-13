// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Script } from "lib/forge-std/src/Script.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { AggregateVerifier } from "src/L1/proofs/AggregateVerifier.sol";
import { Claim, GameType } from "src/libraries/bridge/Types.sol";

import { MockAnchorStateRegistry } from "./mocks/MockAnchorStateRegistry.sol";

/// @title SeedGames
/// @notice Seeds the DisputeGameFactory with a chain of AggregateVerifier games using
///         real L2 output roots. Each game is parented to the previous one, producing
///         a linked list suitable for testing forward traversal at proposer restart.
/// @dev    Games use ProofType.ZK because the dev deploy scripts wire up a MockVerifier
///         that auto-accepts any proof. Output roots are real values fetched via
///         generate-roots.sh.
///
///         Required env: FACTORY_ADDRESS, ANCHOR_STATE_REGISTRY_ADDRESS.
///         Optional env: GAME_COUNT (default 500), ROOTS_FILE (default roots.json).
///
///         All transactions must confirm within the 256-block blockhash window of the
///         L1 origin captured at simulation time. For large counts, use --slow.
contract SeedGames is Script {
    /// @notice Must match the AggregateVerifier deployment constants from DeployDevWithNitro/NoNitro.
    uint256 public constant BLOCK_INTERVAL = 600;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 30;
    uint256 public constant INTERMEDIATE_ROOTS_COUNT = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL;
    uint32 public constant GAME_TYPE_ID = 621;
    uint256 public constant PROGRESS_LOG_INTERVAL = 100;

    struct SeedCtx {
        IDisputeGameFactory factory;
        GameType gameType;
        uint256 initBond;
        uint256 anchorBlock;
        bytes32[] roots;
        bytes proof;
    }

    function run() external {
        address factoryAddr = vm.envAddress("FACTORY_ADDRESS");
        address asrAddr = vm.envAddress("ANCHOR_STATE_REGISTRY_ADDRESS");
        uint256 gameCount = vm.envOr("GAME_COUNT", uint256(500));
        string memory rootsPath = vm.envOr("ROOTS_FILE", string("roots.json"));

        SeedCtx memory ctx = _loadCtx(factoryAddr, asrAddr, rootsPath, gameCount);

        console.log("=== Seeding Multiproof Games ===");
        console.log("Factory:", factoryAddr);
        console.log("AnchorStateRegistry:", asrAddr);
        console.log("Roots file:", rootsPath);
        console.log("Game count:", gameCount);
        console.log("Game type:", uint256(GAME_TYPE_ID));
        console.log("Init bond per game:", ctx.initBond);
        console.log("Anchor block:", ctx.anchorBlock);
        console.log("Total ETH required:", ctx.initBond * gameCount);

        vm.startBroadcast();
        (address firstGame, address lastGame) = _createGames(ctx, asrAddr);
        vm.stopBroadcast();

        uint256 l2Start = ctx.anchorBlock + BLOCK_INTERVAL;
        uint256 l2End = ctx.anchorBlock + BLOCK_INTERVAL * gameCount;

        console.log("");
        console.log("=== Seeding Complete ===");
        console.log("Games created:", gameCount);
        console.log("First game:", firstGame);
        console.log("Last game:", lastGame);
        console.log("L2 block range start:", l2Start);
        console.log("L2 block range end:", l2End);

        _writeOutput(firstGame, lastGame, gameCount, l2Start, l2End);
    }

    function _loadCtx(
        address factoryAddr,
        address asrAddr,
        string memory rootsPath,
        uint256 gameCount
    )
        internal
        view
        returns (SeedCtx memory ctx)
    {
        ctx.factory = IDisputeGameFactory(factoryAddr);
        ctx.gameType = GameType.wrap(GAME_TYPE_ID);
        ctx.initBond = ctx.factory.initBonds(ctx.gameType);
        (, ctx.anchorBlock) = MockAnchorStateRegistry(asrAddr).getAnchorRoot();

        string memory rootsJson = vm.readFile(rootsPath);
        ctx.roots = abi.decode(vm.parseJson(rootsJson, ".roots"), (bytes32[]));

        uint256 expectedRoots = gameCount * INTERMEDIATE_ROOTS_COUNT;
        require(
            ctx.roots.length == expectedRoots,
            string.concat(
                "Root count mismatch: got ",
                vm.toString(ctx.roots.length),
                ", expected ",
                vm.toString(expectedRoots),
                ". Re-run generate-roots.sh with matching game count."
            )
        );

        // MockVerifier auto-accepts any payload; build the constant proof bytes once.
        ctx.proof = abi.encodePacked(
            uint8(AggregateVerifier.ProofType.ZK), blockhash(block.number - 1), block.number - 1, bytes32(0)
        );
    }

    function _createGames(
        SeedCtx memory ctx,
        address asrAddr
    )
        internal
        returns (address firstGame, address lastGame)
    {
        uint256 count = ctx.roots.length / INTERMEDIATE_ROOTS_COUNT;
        address parentAddr = asrAddr;

        for (uint256 i = 0; i < count; i++) {
            address game = _createSingleGame(ctx, i, parentAddr);

            if (i == 0) firstGame = game;
            parentAddr = game;

            if ((i + 1) % PROGRESS_LOG_INTERVAL == 0) {
                console.log("  Created games:", i + 1);
            }
        }

        lastGame = parentAddr;
    }

    function _createSingleGame(SeedCtx memory ctx, uint256 index, address parentAddr) internal returns (address) {
        uint256 l2Block = ctx.anchorBlock + BLOCK_INTERVAL * (index + 1);
        uint256 rootsOffset = index * INTERMEDIATE_ROOTS_COUNT;
        bytes32 rootClaimHash = ctx.roots[rootsOffset + INTERMEDIATE_ROOTS_COUNT - 1];

        bytes memory extraData = abi.encodePacked(l2Block, parentAddr, _sliceRoots(ctx.roots, rootsOffset));

        IDisputeGame created = ctx.factory.createWithInitData{ value: ctx.initBond }(
            ctx.gameType, Claim.wrap(rootClaimHash), extraData, ctx.proof
        );
        return address(created);
    }

    function _sliceRoots(bytes32[] memory all, uint256 offset) internal pure returns (bytes memory) {
        bytes32[] memory slice = new bytes32[](INTERMEDIATE_ROOTS_COUNT);
        for (uint256 j = 0; j < INTERMEDIATE_ROOTS_COUNT; j++) {
            slice[j] = all[offset + j];
        }
        return abi.encodePacked(slice);
    }

    function _writeOutput(
        address firstGame,
        address lastGame,
        uint256 gameCount,
        uint256 l2Start,
        uint256 l2End
    )
        internal
    {
        string memory key = "seeding";
        vm.serializeAddress(key, "firstGame", firstGame);
        vm.serializeAddress(key, "lastGame", lastGame);
        vm.serializeUint(key, "gameCount", gameCount);
        vm.serializeUint(key, "l2BlockStart", l2Start);
        string memory json = vm.serializeUint(key, "l2BlockEnd", l2End);

        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-seeded-games.json");
        vm.writeJson(json, outPath);
        console.log("Output saved to:", outPath);
    }
}
