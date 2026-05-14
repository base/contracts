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

### Fixing semver-lock CI failures

If the `semver-lock` CI check fails, install the pinned Foundry version used by CI, then regenerate locally and commit:

foundryup --version <pinned-version>
just semver-lock

Do not run unpinned `foundryup` for `semver-lock` regeneration. Installing the latest Foundry can change compiler metadata, artifact formatting, or lock output and cause CI drift.

The pinned Foundry version must match CI and repository tooling. If CI is updated to a new Foundry version, update the local install command accordingly before regenerating `semver-lock`.

### setup and testing

- If you don't have foundry installed, run `just install-foundry`.
- Install the pinned Foundry version used by CI:
    foundryup --version <pinned-version>
  - `just deps`
- Test contracts: `just test`