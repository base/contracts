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

### Security Considerations and Assumptions

This section describes the security considerations, trust assumptions, and known limitations of the contracts in this repository. It is intended to improve transparency and help integrators and auditors understand the design trade-offs.

#### Known Limitations

1. **Withdrawal finality delay**: Withdrawals from L2 to L1 are subject to a challenge period before they can be finalized. During this window, a withdrawal is not yet final and can be disputed. Users should not consider L2-to-L1 assets fully available until the challenge period has elapsed and the withdrawal has been finalized on L1.

2. **Re-entrancy in bridge finalization**: The `StandardBridge` uses a re-entrancy guard on the `finalizeERC20Withdrawal` and `finalizeETHWithdrawal` functions. However, other entry points that do not use the guard should be reviewed carefully if new functionality is added. Re-entrancy risks should always be considered when modifying bridge logic.

3. **Token bridging assumes 1:1 mapping**: The standard bridge assumes a canonical 1:1 mapping between L1 and L2 tokens. If a token on either chain does not conform to the expected interface (e.g., fee-on-transfer tokens, rebasing tokens, or tokens with blocklists), the bridge may not function as expected and funds could be locked or lost.

4. **ETH bridging relies on `msg.value`**: The `depositETH` and `depositETHTo` functions bridge ETH by sending it as `msg.value`. Contracts that receive ETH on L2 via bridging should be aware that the ETH is delivered through a low-level call and may need to handle it appropriately.

5. **Upgradeability**: Many contracts in this repository are deployed behind upgradeable proxies. The proxy admin owner has the ability to upgrade contract implementations, which could change bridge behavior. See the trust assumptions below for details.

#### Trust Assumptions

1. **System owner / proxy admin**: The `finalSystemOwner` (set during deployment) has the ability to upgrade proxy implementations and perform admin actions on core contracts. This is a centralized trust assumption — users trust that the system owner will act responsibly and will not deploy malicious upgrades.

2. **Cross-domain messenger**: The bridge relies on the `L1CrossDomainMessenger` and `L2CrossDomainMessenger` for relaying messages between layers. The security of cross-domain messages depends on the integrity of these messengers and the underlying fault-proof or validity-proof system.

3. **L1 chain security**: The L1 bridge contracts inherit the security of the underlying L1 (Ethereum). A successful attack or consensus failure on L1 could compromise L1-deposited assets.

4. **TEE prover registry (multiproof)**: The multiproof system uses a `TEEProverRegistry` that relies on AWS Nitro enclave attestation. This introduces a trust assumption in the correctness of the Nitro attestation process and the integrity of the enclave environment. In dev/test configurations, a `MockDevTEEProverRegistry` is used that bypasses attestation entirely — this must **never** be used in production.

5. **Superchain configuration**: The `SuperchainConfig` contract can trigger a global pause via the guardian account. Users trust that the guardian will only pause in genuine emergency situations and will unpause promptly when the issue is resolved.

#### Potential Edge Cases

1. **Zero-value deposits**: Depositing zero ETH or zero ERC20 tokens is technically possible but may result in unexpected behavior in downstream contracts or event indexing. Integrators should validate non-zero amounts before calling deposit functions.

2. **Self-bridging**: A user could bridge tokens to their own address on the other layer. This is valid but may interact unexpectedly with contracts that track balances across both layers.

3. **Duplicate withdrawal finalization**: The bridge does not prevent re-finalization of the same withdrawal message if the cross-domain messenger allows replay. The messenger's own nonce and replay protection should prevent this, but integrators should be aware of the dependency.

4. **Gas limit estimation**: Deposits specify a `_minGasLimit` for the L2 execution. If this value is set too low, the deposit may fail on L2, and the user's funds may be locked in the bridge until manually recovered. Users should ensure adequate gas limits.

5. **Token decimals mismatch**: If an L1 token and its corresponding L2 token have different decimals, the bridge will transfer the raw amount without conversion. This could lead to unexpected value differences across layers.

6. **Pause state during deposits/withdrawals**: If the `SuperchainConfig` guardian pauses the system while a deposit or withdrawal is in flight, the cross-domain message may not be relayed until the pause is lifted. Users should check the pause state before initiating large transfers.

For information on reporting vulnerabilities, see [SECURITY.md](./SECURITY.md).
