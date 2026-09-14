#!/usr/bin/env bash
set -euo pipefail

# ══════════════════════════════════════════════════════════════════════════════
# generate-roots.sh
#
# Fetches real L2 output roots from an archive node for use with SeedGames.s.sol.
# Queries optimism_outputAtBlock for every intermediate block across all games.
#
# USAGE:
#   ./scripts/multiproof/generate-roots.sh <anchor_block> <l2_rpc_url> [game_count] [parallelism] [output_file]
#
# EXAMPLE:
#   ./scripts/multiproof/generate-roots.sh 37223829 https://my-l2-archive:7545 500 20 roots.json
#
# PREREQUISITES:
#   - cast (from Foundry toolchain)
#   - jq
#   - L2 archive node with optimism_outputAtBlock RPC method
#
# OUTPUT:
#   JSON file: {"roots": ["0x...", "0x...", ...]}
#   Flat array of bytes32 output roots, ordered by game index then intermediate root index.
#   For game i, roots are at indices [i*ROOTS_PER_GAME, (i+1)*ROOTS_PER_GAME).
#   The last root for each game is the root claim.
# ══════════════════════════════════════════════════════════════════════════════

USAGE="Usage: $0 <anchor_block> <l2_rpc_url> [game_count] [parallelism] [output_file]"
ANCHOR_BLOCK="${1:?$USAGE}"
L2_RPC_URL="${2:?$USAGE}"
GAME_COUNT="${3:-500}"
PARALLELISM="${4:-20}"
OUTPUT_FILE="${5:-roots.json}"

# Must match AggregateVerifier / SeedGames constants
BLOCK_INTERVAL=600
INTERMEDIATE_BLOCK_INTERVAL=30
ROOTS_PER_GAME=$((BLOCK_INTERVAL / INTERMEDIATE_BLOCK_INTERVAL))
TOTAL_ROOTS=$((GAME_COUNT * ROOTS_PER_GAME))

LAST_BLOCK=$((ANCHOR_BLOCK + GAME_COUNT * BLOCK_INTERVAL))

echo "=== Generating Output Roots ==="
echo "Anchor block:     $ANCHOR_BLOCK"
echo "Game count:       $GAME_COUNT"
echo "Roots per game:   $ROOTS_PER_GAME"
echo "Total roots:      $TOTAL_ROOTS"
echo "L2 block range:   [$((ANCHOR_BLOCK + INTERMEDIATE_BLOCK_INTERVAL)), $LAST_BLOCK]"
echo "Parallelism:      $PARALLELISM"
echo "Output file:      $OUTPUT_FILE"
echo ""

command -v cast >/dev/null 2>&1 || { echo "ERROR: cast not found. Install Foundry first." >&2; exit 1; }
command -v jq >/dev/null 2>&1 || { echo "ERROR: jq not found." >&2; exit 1; }

echo "Verifying RPC connectivity..."
TEST_HEX=$(printf "0x%x" "$ANCHOR_BLOCK")
if ! TEST_RESULT=$(cast rpc optimism_outputAtBlock "$TEST_HEX" --rpc-url "$L2_RPC_URL" 2>&1); then
    echo "ERROR: Failed to query optimism_outputAtBlock for anchor block $ANCHOR_BLOCK" >&2
    echo "       RPC: $L2_RPC_URL" >&2
    echo "       Response: $TEST_RESULT" >&2
    exit 1
fi
echo "RPC OK."
echo ""

fetch_root() {
    local i=$1
    local game=$((i / ROOTS_PER_GAME))
    local j=$(( (i % ROOTS_PER_GAME) + 1 ))
    local block=$((ANCHOR_BLOCK + game * BLOCK_INTERVAL + j * INTERMEDIATE_BLOCK_INTERVAL))
    local hex_block root
    hex_block=$(printf "0x%x" "$block")
    root=$(cast rpc optimism_outputAtBlock "$hex_block" --rpc-url "$L2_RPC_URL" | jq -r '.outputRoot')
    if [ -z "$root" ] || [ "$root" = "null" ]; then
        echo "ERROR: Failed to fetch root for block $block (index $i)" >&2
        return 1
    fi
    printf '%s\t%s\n' "$i" "$root"
}
export -f fetch_root
export ANCHOR_BLOCK ROOTS_PER_GAME BLOCK_INTERVAL INTERMEDIATE_BLOCK_INTERVAL L2_RPC_URL

RESULTS_FILE=$(mktemp)
trap 'rm -f "$RESULTS_FILE"' EXIT

echo "Fetching $TOTAL_ROOTS roots..."
seq 0 $((TOTAL_ROOTS - 1)) \
    | xargs -n 1 -P "$PARALLELISM" -I {} bash -c 'fetch_root "$@"' _ {} \
    | tee "$RESULTS_FILE" \
    | awk -v total="$TOTAL_ROOTS" '
        NR % 200 == 0 || NR == total { printf "  Fetched %d / %d roots\n", NR, total > "/dev/stderr" }
      '

FETCHED=$(wc -l < "$RESULTS_FILE")
if [ "$FETCHED" -ne "$TOTAL_ROOTS" ]; then
    echo "ERROR: fetched $FETCHED of $TOTAL_ROOTS roots. Aborting." >&2
    exit 1
fi

echo "Assembling JSON..."
sort -n "$RESULTS_FILE" | cut -f2- | jq -Rn '{roots: [inputs]}' > "$OUTPUT_FILE"

echo ""
echo "=== Done ==="
echo "Saved $TOTAL_ROOTS roots to $OUTPUT_FILE"
echo ""
echo "Next step: run SeedGames.s.sol with ROOTS_FILE=$OUTPUT_FILE"
