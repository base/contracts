![Base](logo.png)

# contracts

This repo contains the contracts and scripts for Base.
Base no longer relies on the OP Stack contracts published in Optimism's repo; all of the contracts that power Base are now maintained directly in this repository.
For contract deployment artifacts, see [base-org/contract-deployments](https://github.com/base-org/contract-deployments).

The contracts are organized under `src/`:

- `src/L1` — L1 contracts, including the portal, bridges, messengers, `SystemConfig`, `SuperchainConfig`, and the fault `proofs/` system (dispute games, anchor state registry, and TEE/ZK verifiers).
- `src/L2` — L2 predeploys, including bridges, messengers, fee vaults, the fee disburser, `L1Block`, and the gas price oracle.
- `src/universal` — Shared contracts used across L1 and L2, such as the standard bridge, cross-domain messenger, mintable ERC20/ERC721 factories, and proxy contracts.
- `src/legacy` — Legacy contracts retained for compatibility (e.g. `AddressManager`, proxy variants).
- `src/libraries` — Shared Solidity libraries (encoding, hashing, predeploys, tries, etc.).
- `src/vendor` — Third-party contracts vendored into the repo.

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

### Fixing semver-lock CI failures

If the `semver-lock` CI check fails, regenerate locally and commit:

```bash
just semver-lock
```

If CI still rejects it (Foundry version mismatch), update your local Foundry first:

```bash
foundryup
just semver-lock
```

### setup and testing

- If you don't have foundry installed, run `just install-foundry`.
- `just deps`
- Test contracts: `just test`
