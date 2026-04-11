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

ANCHOR_BLOCK="${1:?Usage: $0 <anchor_block> <l2_rpc_url> [game_count] [parallelism] [output_file]}"
L2_RPC_URL="${2:?Usage: $0 <anchor_block> <l2_rpc_url> [game_count] [parallelism] [output_file]}"
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

# Verify tools are available
command -v cast >/dev/null 2>&1 || { echo "ERROR: cast not found. Install Foundry first." >&2; exit 1; }
command -v jq >/dev/null 2>&1 || { echo "ERROR: jq not found." >&2; exit 1; }

# Quick sanity check: verify the RPC is reachable and the anchor block exists
echo "Verifying RPC connectivity..."
TEST_HEX=$(printf "0x%x" "$ANCHOR_BLOCK")
TEST_RESULT=$(cast rpc optimism_outputAtBlock "$TEST_HEX" --rpc-url "$L2_RPC_URL" 2>&1) || {
    echo "ERROR: Failed to query optimism_outputAtBlock for anchor block $ANCHOR_BLOCK" >&2
    echo "       RPC: $L2_RPC_URL" >&2
    echo "       Response: $TEST_RESULT" >&2
    exit 1
}
echo "RPC OK."
echo ""

# Create temp directory for individual root files
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Fetch roots in batches of PARALLELISM
echo "Fetching roots..."
for ((batch_start=0; batch_start<TOTAL_ROOTS; batch_start+=PARALLELISM)); do
    batch_end=$((batch_start + PARALLELISM))
    if [ "$batch_end" -gt "$TOTAL_ROOTS" ]; then
        batch_end=$TOTAL_ROOTS
    fi

    for ((i=batch_start; i<batch_end; i++)); do
        game=$((i / ROOTS_PER_GAME))
        j=$(( (i % ROOTS_PER_GAME) + 1 ))
        block=$((ANCHOR_BLOCK + game * BLOCK_INTERVAL + j * INTERMEDIATE_BLOCK_INTERVAL))

        (
            hex_block=$(printf "0x%x" "$block")
            root=$(cast rpc optimism_outputAtBlock "$hex_block" --rpc-url "$L2_RPC_URL" | jq -r '.outputRoot')

            if [ -z "$root" ] || [ "$root" = "null" ]; then
                echo "ERROR: Failed to fetch root for block $block (index $i)" >&2
                exit 1
            fi

            echo "$root" > "$TMPDIR/$i"
        ) &
    done

    wait

    # Progress reporting every 200 roots
    if (( batch_end % 200 == 0 )) || [ "$batch_end" -eq "$TOTAL_ROOTS" ]; then
        echo "  Fetched $batch_end / $TOTAL_ROOTS roots"
    fi
done

# Verify all roots were fetched
echo ""
echo "Verifying completeness..."
MISSING=0
for ((i=0; i<TOTAL_ROOTS; i++)); do
    if [ ! -f "$TMPDIR/$i" ]; then
        echo "  MISSING root at index $i" >&2
        MISSING=$((MISSING + 1))
    fi
done

if [ "$MISSING" -gt 0 ]; then
    echo "ERROR: $MISSING roots missing. Aborting." >&2
    exit 1
fi

# Assemble JSON
echo "Assembling JSON..."
{
    echo -n '{"roots":['
    for ((i=0; i<TOTAL_ROOTS; i++)); do
        if [ "$i" -gt 0 ]; then echo -n ','; fi
        echo -n "\"$(cat "$TMPDIR/$i")\""
    done
    echo ']}'
} > "$OUTPUT_FILE"

echo ""
echo "=== Done ==="
echo "Saved $TOTAL_ROOTS roots to $OUTPUT_FILE"
echo ""
echo "Next step: run SeedGames.s.sol with ROOTS_FILE=$OUTPUT_FILE"
