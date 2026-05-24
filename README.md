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
[![Discord](https://img.shields.io/discord/1067165013397286?label=discord)](https://base.org/discord)
[![Twitter Base](https://img.shields.io/twitter/follow/Base?style=social)](https://twitter.com/Base)

<!-- Badge row 3 - detailed status -->

[![GitHub pull requests by-label](https://img.shields.io/github/issues-pr-raw/base-org/contracts)](https://github.com/base/contracts/pulls)
[![GitHub Issues](https://img.shields.io/github/issues-raw/base-org/contracts.svg)](https://github.com/base/contracts/issues)

---

## Security Considerations

This section describes the security considerations, trust assumptions, and known limitations of the contracts in this repository.

### Trust Assumptions

- **Proxy Admin Owner**: The bridge contracts are upgradeable via a `ProxyAdmin` owned by the `finalSystemOwner` address. This owner has the ability to upgrade contract implementations, which can change the behavior of bridging, withdrawals, and other critical functions. Users trust that the proxy admin owner acts responsibly and does not deploy malicious implementations.
- **Cross-Domain Messenger**: The `L1StandardBridge` and `L2StandardBridge` rely on the `CrossDomainMessenger` for authenticating cross-chain messages. Withdrawal finalization on L1 trusts that the messenger correctly verifies the origin of L2 messages. A compromise of the messenger would allow fraudulent withdrawal finalization.
- **System Config Owner**: The `ISystemConfig` contract is referenced by the L1 bridge and is controlled by a privileged owner who can modify gas limits and other system parameters that affect bridge operations.
- **TEE Prover Registry**: The multiproof system relies on a `TEEProverRegistry` that validates AWS Nitro enclave attestations. In production, trust is placed in the correctness of the Nitro attestation verification and the integrity of the enclave's signing key. Dev/test deployments bypass this via `addDevSigner()`, which must **never** be used in production.
- **Superchain Config**: The `ISuperchainConfig` contract governs pause and escalation settings. The config owner can pause bridge deposits and withdrawals system-wide.

### Known Limitations

- **Upgradeability Risk**: All core bridge contracts are behind upgradeable proxies. While this allows for bug fixes and feature additions, it also means that the current contract logic is not immutable. Users should monitor governance actions for any upgrades.
- **Reinitializable Pattern**: Contracts use a reinitializable pattern with an `initVersion` counter. Re-initialization is possible but gated by version checks. Incorrect initialization sequencing could leave contracts in an unexpected state.
- **ETH Bridging**: The `depositETH` and `depositETHTo` functions accept ETH via `msg.value`. If a user sends ETH directly to the bridge contract without calling a deposit function, the funds may be irrecoverable.
- **Withdrawal Proving**: L2-to-L1 withdrawals require proving against an output root published by the L2 output oracle. If the oracle is delayed or censored, withdrawals cannot be finalized until a valid output root is published.
- **Dev/Test Scripts Not for Production**: The multiproof deployment scripts in `scripts/multiproof/` explicitly bypass attestation checks (`MockDevTEEProverRegistry.addDevSigner()`) and use mock anchor state registries with no access control. These must never be used in production environments.

### Potential Edge Cases

- **Duplicate Deposits**: If a deposit transaction is submitted with the same nonce to both `depositERC20` and `depositERC20To`, they are treated as separate deposits. Users should ensure unique parameters if replay protection is desired.
- **Finalization Gas Limits**: Withdrawal finalization on L1 requires sufficient gas. If `_minGasLimit` is set too low, the finalization transaction may revert, requiring the user to resubmit with a higher gas limit.
- **Token Pair Mismatches**: The bridge does not natively enforce that `_l1Token` and `_l2Token` are valid pairs. Depositing with an incorrect or unregistered L2 token address could result in locked or irrecoverable tokens.
- **Reentrancy in Finalization**: The `finalizeERC20Withdrawal` and `finalizeETHWithdrawal` functions make external calls to token contracts and the cross-domain messenger. While the messenger uses a reentrancy guard, users should be aware of potential reentrancy vectors through malicious token implementations.
- **Cross-Domain Message Ordering**: Messages sent across domains are ordered by nonce within the messenger. If a lower-nonce message reverts, subsequent messages in the queue are blocked until the failing message is successfully executed or dropped.

For information on reporting vulnerabilities, see [SECURITY.md](./SECURITY.md).

---

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
