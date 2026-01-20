# Universal Scripts

This directory contains a collection of utility scripts and libraries used in Base smart contract operations tasks. These utilities are designed to simplify the process of creating, simulating, signing, and executing transactions, particularly those involving Gnosis Safe multisigs.

For the actual operations tasks and usage examples, please refer to the [contract-deployments repository](https://github.com/base/contract-deployments).

## Key Files

### `MultisigScript.sol`

This is the core script for building Forge scripts that interact with Gnosis Safes. It supports:

- **Nested Multisigs**: Handles arbitrary depths of nested Safes (Safes owned by other Safes).
- **Workflows**: Provides standard functions for `sign` (generating signatures), `approve` (onchain approval for nested Safes), `simulate` (dry-run with state overrides), and `run` (execution).
- **Simulation**: Integrates with `Simulation.sol` to provide detailed simulation links and state diffs.

### `MultisigScriptDeposit.sol`

An extension of `MultisigScript` for L1 → L2 deposit transactions. Task writers define L2 calls via `_buildL2Calls()`, and the framework automatically wraps them in an `OptimismPortal.depositTransaction` call. Features:

- **ETH Bridging**: Supports bridging ETH along with the message.
- **Chain-Aware Defaults**: Provides default `OptimismPortal` addresses for Mainnet and Sepolia.
- **Gas Limit**: Set via `L2_GAS_LIMIT` env var (all signers must use the same value).

### `Simulation.sol`

A library for simulating multisig transactions with state overrides. It is particularly useful for:

- Generating Tenderly simulation links.
- Overriding contract storage (e.g., modifying Safe thresholds or nonces) to test execution paths that would otherwise be blocked.

### `MultisigBuilder.sol`

_Deprecated_. Use `MultisigScript.sol` instead. This was an earlier version of the multisig tooling.

### Other Utilities

- `IGnosisSafe.sol`: Interface definitions for interacting with Gnosis Safe contracts.
- `Signatures.sol`: Helper functions for handling and formatting signatures for Gnosis Safe transactions.
- `StateDiff.sol`: Utilities for capturing and reporting state changes during simulations.

## Usage

These scripts are typically imported by specific task scripts in the `contract-deployments` repository. A typical task script inherits from `MultisigScript` and implements the `_buildCalls` method to define the actions to be taken.
