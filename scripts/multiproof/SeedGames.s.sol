// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

/**
 * @title SeedGames
 * @notice Seeds the DisputeGameFactory with chained AggregateVerifier games using real L2 output roots.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 *                        POST-DEPLOYMENT GAME SEEDING
 * ══════════════════════════════════════════════════════════════════════════════════
 *
 * Creates a chain of AggregateVerifier (multiproof) games via the DisputeGameFactory
 * deployed by DeployDevWithNitro (or DeployDevNoNitro). Each game's parent is the
 * previous game, forming a linked list suitable for testing forward traversal at
 * proposer restart.
 *
 * Games use ProofType.ZK (value 1) because both deployment scripts wire up a
 * MockVerifier for ZK that auto-accepts any proof — no real ZK proof needed.
 * Output roots, however, are real values fetched from an L2 archive node.
 *
 * PREREQUISITES:
 *   1. Deploy the multiproof stack via DeployDevWithNitro.s.sol (or DeployDevNoNitro.s.sol).
 *   2. Set the anchor state on MockAnchorStateRegistry to a recent block (see README.md).
 *   3. Generate real output roots using generate-roots.sh:
 *
 *        ./scripts/multiproof/generate-roots.sh <anchor_block> <l2_rpc_url> [game_count]
 *
 * USAGE:
 *   FACTORY_ADDRESS=0x... \
 *   ANCHOR_STATE_REGISTRY_ADDRESS=0x... \
 *   forge script scripts/multiproof/SeedGames.s.sol \
 *     --rpc-url $RPC_URL --broadcast --private-key $PRIVATE_KEY
 *
 * OPTIONAL ENV VARS:
 *   GAME_COUNT    Number of games to create (default: 500)
 *   ROOTS_FILE    Path to the roots JSON from generate-roots.sh (default: roots.json)
 *
 * NOTE: All transactions must confirm within the 256-block blockhash window
 * (~51 min on mainnet/Sepolia) of the L1 origin block captured at simulation time.
 * For large game counts, consider using --slow or splitting into batches.
 *
 * ══════════════════════════════════════════════════════════════════════════════════
 */

import { Script } from "forge-std/Script.sol";
import { console2 as console } from "forge-std/console2.sol";

import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";
import { IDisputeGame } from "interfaces/dispute/IDisputeGame.sol";
import { Claim, GameType, Hash } from "src/dispute/lib/Types.sol";

import { MockAnchorStateRegistry } from "./mocks/MockAnchorStateRegistry.sol";

contract SeedGames is Script {
    /// @notice Must match the AggregateVerifier deployment constants from DeployDevWithNitro/NoNitro.
    uint256 public constant BLOCK_INTERVAL = 600;
    uint256 public constant INTERMEDIATE_BLOCK_INTERVAL = 30;
    uint256 public constant INTERMEDIATE_ROOTS_COUNT = BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL;
    uint32 public constant GAME_TYPE_ID = 621;

    /// @dev Stored as state to avoid stack-too-deep in run().
    IDisputeGameFactory internal factory;
    GameType internal gameType;
    uint256 internal initBond;
    uint256 internal anchorBlock;
    bytes32 internal l1OriginHash;
    uint256 internal l1OriginNumber;
    bytes32[] internal allRoots;

    function run() external {
        // ── Configuration ──────────────────────────────────────────────
        address factoryAddr = vm.envAddress("FACTORY_ADDRESS");
        address asrAddr = vm.envAddress("ANCHOR_STATE_REGISTRY_ADDRESS");
        uint256 gameCount = vm.envOr("GAME_COUNT", uint256(500));
        string memory rootsPath = vm.envOr("ROOTS_FILE", string("roots.json"));

        factory = IDisputeGameFactory(factoryAddr);
        gameType = GameType.wrap(GAME_TYPE_ID);
        initBond = factory.initBonds(gameType);

        (, anchorBlock) = MockAnchorStateRegistry(asrAddr).getAnchorRoot();

        // L1 origin — must remain within the blockhash window when txns execute on-chain
        l1OriginHash = blockhash(block.number - 1);
        l1OriginNumber = block.number - 1;

        // ── Load real output roots ─────────────────────────────────────
        string memory rootsJson = vm.readFile(rootsPath);
        allRoots = abi.decode(vm.parseJson(rootsJson, ".roots"), (bytes32[]));

        uint256 expectedRoots = gameCount * INTERMEDIATE_ROOTS_COUNT;
        require(
            allRoots.length == expectedRoots,
            string.concat(
                "Root count mismatch: got ",
                vm.toString(allRoots.length),
                ", expected ",
                vm.toString(expectedRoots),
                ". Re-run generate-roots.sh with matching game count."
            )
        );

        // ── Summary ────────────────────────────────────────────────────
        console.log("=== Seeding Multiproof Games ===");
        console.log("Factory:", factoryAddr);
        console.log("AnchorStateRegistry:", asrAddr);
        console.log("Roots file:", rootsPath);
        console.log("Game count:", gameCount);
        console.log("Game type:", uint256(GAME_TYPE_ID));
        console.log("Init bond per game:", initBond);
        console.log("Anchor block:", anchorBlock);
        console.log("Total ETH required:", initBond * gameCount);

        // ── Create chained games ───────────────────────────────────────
        vm.startBroadcast();

        (address firstGame, address lastGame) = _createGames(asrAddr, gameCount);

        vm.stopBroadcast();

        // ── Output ─────────────────────────────────────────────────────
        uint256 l2Start = anchorBlock + BLOCK_INTERVAL;
        uint256 l2End = anchorBlock + BLOCK_INTERVAL * gameCount;

        console.log("");
        console.log("=== Seeding Complete ===");
        console.log("Games created:", gameCount);
        console.log("First game:", firstGame);
        console.log("Last game:", lastGame);
        console.log("L2 block range start:", l2Start);
        console.log("L2 block range end:", l2End);

        _writeOutput(firstGame, lastGame, gameCount, l2Start, l2End);
    }

    /// @notice Creates `count` chained games, each parented to the previous one.
    /// @param asrAddr The AnchorStateRegistry address (parent of the first game).
    /// @param count The number of games to create.
    /// @return firstGame The address of the first game created.
    /// @return lastGame The address of the last game created.
    function _createGames(address asrAddr, uint256 count) internal returns (address firstGame, address lastGame) {
        address parentAddr = asrAddr;

        for (uint256 i = 0; i < count; i++) {
            address game = _createSingleGame(i, parentAddr);

            if (i == 0) firstGame = game;
            parentAddr = game;

            if ((i + 1) % 100 == 0) {
                console.log("  Created games:", i + 1);
            }
        }

        lastGame = parentAddr;
    }

    /// @notice Creates a single game at the given index in the chain.
    /// @param index The zero-based index of this game in the chain.
    /// @param parentAddr The parent game address (or ASR address for the first game).
    /// @return game The address of the newly created game.
    function _createSingleGame(uint256 index, address parentAddr) internal returns (address game) {
        uint256 l2Block = anchorBlock + BLOCK_INTERVAL * (index + 1);

        // Slice this game's intermediate roots from the flat array
        uint256 rootsOffset = index * INTERMEDIATE_ROOTS_COUNT;
        bytes32 rootClaimHash = allRoots[rootsOffset + INTERMEDIATE_ROOTS_COUNT - 1];

        bytes memory intermediateRoots = _sliceRoots(rootsOffset);
        bytes memory extraData = abi.encodePacked(l2Block, parentAddr, intermediateRoots);

        // ZK proof — MockVerifier auto-accepts any input
        bytes memory proof = abi.encodePacked(
            uint8(1), // ProofType.ZK
            l1OriginHash,
            l1OriginNumber,
            bytes32(0) // dummy proof payload
        );

        IDisputeGame created =
            factory.createWithInitData{ value: initBond }(gameType, Claim.wrap(rootClaimHash), extraData, proof);

        return address(created);
    }

    /// @notice Packs INTERMEDIATE_ROOTS_COUNT roots from allRoots starting at offset.
    /// @param offset The starting index in allRoots.
    /// @return roots The abi.encodePacked intermediate roots.
    function _sliceRoots(uint256 offset) internal view returns (bytes memory roots) {
        for (uint256 j = 0; j < INTERMEDIATE_ROOTS_COUNT; j++) {
            roots = abi.encodePacked(roots, allRoots[offset + j]);
        }
    }

    /// @notice Writes seeding metadata to a JSON file.
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
