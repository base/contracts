# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Base contracts repository — an Optimism Bedrock-based L2 blockchain. Contains Solidity smart contracts for Base's L1 and L2 infrastructure, built on top of Optimism's contracts-bedrock (op-contracts/v6.0.0-rc.2). Uses Foundry (Forge) as the build/test framework.

## Build & Development Commands

All commands use `just` (justfile) unless noted. The Makefile handles initial setup only.

### Setup
```bash
make install-foundry    # Install Foundry toolchain
make deps               # Install all Solidity dependencies (forge install + manual clones)
just build-go-ffi       # Build the Go FFI tool (required before running tests)
```

### Building
```bash
just build              # Production build (runs lint-fix first, optimizer_runs=999999)
just build-dev          # Fast dev build (FOUNDRY_PROFILE=lite, no optimization)
just build-source       # Build src/ only (skip tests and scripts)
```

### Testing
```bash
just test               # Run all tests (builds go-ffi first)
just test-dev           # Fast dev tests (lite profile, 8 fuzz runs)
just test -- --match-test "test_myFunction"        # Run a single test by name
just test -- --match-contract "MyContract"          # Run tests in a specific contract
just test -- --match-path "test/L1/*"               # Run tests matching a path
just test-rerun         # Re-run only failed tests with verbose output
just test-upgrade       # Fork tests against mainnet/sepolia (requires ETH_RPC_URL)
just coverage           # Coverage report
```

### Linting & Formatting
```bash
just lint               # Format + check (forge fmt)
just lint-check         # Check only (no auto-fix)
forge fmt               # Direct format command
```
Line length: 120 chars. Multiline function headers: all params on separate lines.

### Pre-PR Workflow
```bash
just pre-pr             # Full workflow: build-dev, lint, build-source, all checks
just pre-pr --clean     # Same but cleans build artifacts first
just check              # Run all validation checks without building
```

### Snapshots & Semver
```bash
just snapshots          # Regenerate ABI/storage snapshots and semver-lock
just snapshots-check    # Verify snapshots are up to date
```

## Architecture

### Contract Structure (src/)
- **L1/**: Ethereum mainnet contracts — `OPContractsManager.sol` (central upgrade manager), `OptimismPortal2.sol` (proof finalization), `SystemConfig.sol`, `SuperchainConfig.sol`
- **L2/**: L2 contracts — `L1Block.sol`, `L2StandardBridge.sol`, `CrossL2Inbox.sol`, `SuperchainERC20.sol`
- **dispute/**: Fault proof game contracts with v1, v2, and zk variants
- **cannon/**: Cannon VM contracts (MIPS-based fault proof execution)
- **universal/**: Cross-chain utilities (bridges, messengers)
- **libraries/**: Shared libraries — `Constants.sol`, `Predeploys.sol` (canonical L2 addresses), `Preinstalls.sol`
- **periphery/**: Non-core contracts (drippie, faucet, monitoring)
- **safe/**: Safe multisig extensions
- **governance/**, **recovery/**, **revenue-share/**, **smart-escrow/**: Domain-specific contracts

### Interfaces (interfaces/)
Separate directory for contract interfaces, mirroring the src/ structure. Go checks (`scripts/checks/interfaces`) validate that interfaces match their implementations.

### Test Structure (test/)
- Mirrors `src/` directory layout
- `CommonTest.t.sol`: Base test class with standard accounts (alice, bob, admin, deployer)
- `test/setup/Setup.sol`: Full system deployment setup using `Deploy.s.sol`
- `test/setup/FeatureFlags.sol`: Development feature gating for tests
- `test/mocks/`: 20+ mock contracts
- `test/invariants/`: Property-based invariant tests
- `test/kontrol/`: K framework formal verification proofs
- FFI enabled: tests can call external programs via `scripts/go-ffi/`

### Deployment (scripts/)
- `scripts/deploy/Deploy.s.sol`: Main deployment script
- Input/Output pattern (`BaseDeployIO.sol`): Modular deployment composition
- `deploy-config/`: JSON configs per network (mainnet.json, sepolia.json, hardhat.json)
- `scripts/L2Genesis.s.sol`: L2 genesis state generation

## Key Patterns

- **Proxy pattern**: EIP-1967 transparent proxies for upgradeability
- **Compilation restrictions**: Dispute game contracts and OPContractsManager use 5000 optimizer runs (not 999999) due to size constraints. The `lite` profile mirrors these restrictions with 0 runs.
- **Foundry profiles**: `default` (production), `lite` (dev), `ci` (128 fuzz runs), `ciheavy` (20000 fuzz runs)
- **Semver tracking**: Contracts use semantic versioning tracked in `snapshots/semver-lock.json`
- **Storage spacers**: Validated by `scripts/checks/spacers` to prevent storage layout conflicts in upgradeable contracts
- **Solidity version**: 0.8.15 for main contracts, some use ^0.8.0

## CI

GitHub Actions runs on PR: `forge fmt --check` → `make deps` → `forge build --sizes` → `forge test -vvv` with `FOUNDRY_PROFILE=ci`.
