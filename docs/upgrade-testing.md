# Upgrade Testing Guide

This document describes how to run and maintain upgrade tests for the Base contracts.

## Overview

Upgrade tests use pinned block numbers to fork the chain at a specific point and run contract tests against that historical state. This ensures upgrades are tested against real onchain data.

## Block Number Configuration

Block numbers are configured in `justfile`:

```justfile
export sepoliaBlockNumber := "9366100"
export mainnetBlockNumber := "23530400"
```

These values are used by the `pinned-block-tests` and `coverage` targets.

## Running Upgrade Tests

```bash
# Run tests against Sepolia fork
just test-fork sepolia

# Run tests against Mainnet fork
just test-fork mainnet

# Run coverage against pinned blocks
just coverage-upgrade
```

## Updating Block Numbers

When to update:
- The pinned block is older than 90 days
- A major upgrade has been deployed
- Tests start failing due to chain state changes

How to update:
1. Find the current block number:
   ```bash
   cast block-number --rpc-url <RPC_URL>
   ```
2. Update the value in `justfile`
3. Run `just test-fork <network>` to verify tests still pass
4. Commit with message: `test: update <network>BlockNumber to <new_number>`

## Staleness Check

A CI check enforces that block numbers are refreshed regularly. If a block is older than 90 days, the CI will fail with a warning to update.

This prevents tests from running against significantly outdated chain state, which could miss issues present in newer contract versions.
