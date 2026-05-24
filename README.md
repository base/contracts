![Base](logo.png)

# contracts

This repo contains contracts and scripts for Base.
Note that Base primarily utilizes Optimism's bedrock contracts located in Optimism's repo [here](https://github.com/ethereum-optimism/optimism/tree/develop/packages/contracts-bedrock).
For contract deployment artifacts, see [base-org/contract-deployments](https://github.com/base-org/contract-deployments).

<!-- Badge row 1 - status -->

[![GitHub contributors](https://img.shields.io/github/contributors/base-org/contracts)](https://github.com/base/contracts/graphs/contributors)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/w/base-org/contracts)](https://github.com/base/contracts/graphs/contributors)
[![GitHub Stars](https://img.shields.io/github/stars/base-org/contracts.svg)](https://github.com/base/contracts/stargazers)
![GitHub repo size](https://img.shields.io/github/repo-size/base-org/contracts)
[![GitHub](https://img.shields.io/github/license/base-org/contracts?color=blue)](https://github.com/base/contracts/blob/main/LICENSE)

<!-- Badge row 2 - links and profiles -->

[![Website base.org](https://img.shields.io/website-up-down-green-red/https/base.org.svg)](https://base.org)
[![Blog](https://img.shields.io/badge/blog-up-green)](https://base.mirror.xyz/)
[![Docs](https://img.shields.io/badge/docs-up-green)](https://docs.base.org/)
[![Discord](https://img.shields.io/discord/1067165013397213286?label=discord)](https://base.org/discord)
[![Twitter Base](https://img.shields.io/twitter/follow/Base?style=social)](https://twitter.com/Base)

<!-- Badge row 3 - detailed status -->

[![GitHub pull requests by-label](https://img.shields.io/github/issues-pr-raw/base-org/contracts)](https://github.com/base/contracts/pulls)
[![GitHub Issues](https://img.shields.io/github/issues-raw/base-org/contracts.svg)](https://github.com/base/contracts/issues)

## Contract Architecture

### Bridge Contracts

The bridge contracts enable token transfers between L1 (Ethereum) and L2 (Base). They follow a hierarchical pattern:

- **`StandardBridge`** (`src/universal/StandardBridge.sol`) — Abstract base contract containing shared bridging logic for both L1↔L2 directions. Handles ETH and ERC-20 deposits/withdrawals, including fee calculations and cross-chain message passing.
- **`L1StandardBridge`** (`src/L1/L1StandardBridge.sol`) — L1-specific bridge that extends `StandardBridge`. Handles deposits from Ethereum to L2 and processes withdrawals from L2 back to Ethereum.
- **`L2StandardBridge`** (`src/L2/L2StandardBridge.sol`) — L2-specific bridge that extends `StandardBridge`. Handles withdrawals from L2 to Ethereum and processes deposits from L1 on L2.

The corresponding interfaces mirror this hierarchy:
- `IStandardBridge` → `IL1StandardBridge` and `IL2StandardBridge`

### Key Concepts

- **Proxied contracts**: Many contracts are deployed behind upgradeable proxies (EIP-1967). These are tagged with `@custom:proxied` in their NatSpec.
- **Predeployed contracts**: Some L2 contracts are pre-installed at fixed addresses (e.g., `0x42000000000000000000000000000000000000XX`). These are tagged with `@custom:predeploy`.
- **Initialization**: Contracts use OpenZeppelin's `Initializable` pattern instead of constructors, since proxy delegates cannot use constructors. See `ForgeArtifacts.isInitialized` (OZ v4) and `ForgeArtifacts.isInitializedV5` for testing utilities.

### Directory Structure

```
├── interfaces/          # Solidity interfaces organized by layer
│   ├── L1/             # L1-specific interfaces
│   ├── L2/             # L2-specific interfaces
│   └── universal/       # Shared interfaces used across layers
├── src/                # Contract implementations
│   ├── L1/             # L1-specific contracts
│   ├── L2/             # L2-specific contracts
│   ├── universal/       # Shared contract logic
│   └── vendor/          # Third-party contracts (e.g., EAS schema registry)
├── scripts/            # Deployment and utility scripts
│   ├── deploy/          # Deployment scripts and config
│   ├── libraries/       # Shared script libraries (e.g., ForgeArtifacts)
│   └── multiproof/      # Multiproof deployment scripts
├── test/               # Foundry test suite
│   ├── L1/             # L1 contract tests
│   ├── L2/             # L2 contract tests
│   ├── universal/       # Shared contract tests
│   └── setup/           # Test infrastructure (ForkLive, FeatureFlags)
└── snapshots/          # ABI and storage layout snapshots for semver-lock checks
    ├── abi/
    └── storageLayout/
```

### Testing with Forks

The test suite supports forking live networks via `ForkLive.s.sol`. When `FORK_TEST=true` is set:

1. The test harness loads production addresses instead of deploying from source.
2. Supported chains: Ethereum mainnet (chainId 1), Sepolia (chainId 11155111), and Base testnet (chainId 560048).
3. Optionally, state can be loaded from the superchain-ops repo via `SUPERCHAIN_OPS_ALLOCS_PATH`.

## Fixing semver-lock CI failures

If the `semver-lock` CI check fails, regenerate locally and commit:

```bash
just semver-lock
```

If CI still rejects it (Foundry version mismatch), update your local Foundry first:

```bash
foundryup
just semver-lock
```

## Setup and Testing

- If you don't have foundry installed, run `just install-foundry`.
- `just deps`
- Test contracts: `just test`

## Contributing

When adding or modifying contracts, please:

- Add comprehensive NatSpec comments (`@notice`, `@param`, `@return`) to all public/external functions.
- Use `@custom:proxied` and `@custom:predeploy` tags where applicable.
- Include inline comments explaining non-obvious logic, assumptions, and design decisions.
- Update the relevant ABI and storage layout snapshots by running `just semver-lock`.
