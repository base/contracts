# Security

## Security Considerations and Assumptions

This section outlines known limitations, trust assumptions, and potential risks for developers integrating with or extending these contracts.

### Trust Assumptions

- **Cross-domain messengers**: The bridge contracts trust the L1 and L2 `CrossDomainMessenger` contracts for authentic message relay. If a messenger is compromised, bridged assets could be at risk.
- **System configuration**: `SystemConfig` and `SuperchainConfig` are trusted sources for chain parameters (e.g., gas limits, pause status). These are controlled by a privileged `finalSystemOwner` / proxy admin.
- **Address aliasing**: L1↔L2 message passing relies on `AddressAliasHelper` to apply the correct address transformation. Contracts that do not account for aliasing when checking `msg.sender` may be vulnerable to spoofing.
- **Token pair validity**: The `StandardBridge` does not verify that an L1 token and its L2 counterpart are legitimately paired. Users and integrators are responsible for confirming token pair correctness before bridging.

### Known Limitations

- **Asynchronous finality**: Deposits (L1→L2) and withdrawals (L2→L1) are asynchronous. A deposit is not finalized on L2 until the cross-domain message is relayed, and a withdrawal requires a challenge period on L1 before it can be finalized.
- **Non-standard ERC-20 tokens**: The bridge uses `unsafeERC20TransferFrom` and `safeERC20TransferFrom` to handle tokens that do not fully comply with the ERC-20 standard (e.g., missing return values). While this improves compatibility, it also means the bridge may interact with tokens that have unusual behavior.
- **Cross-domain reentrancy**: Although the bridge employs reentrancy guards, cross-domain reentrancy (where a callback on the other chain triggers a second message back) is an inherent property of the two-way bridge design. Integrators must account for this in their own contracts.
- **Gas limit risks**: Cross-domain messages specify a `_minGasLimit`. If the gas limit is set too low, the message may fail on the receiving chain, requiring manual replay. If set too high, it may waste gas.
- **Upgradeability**: Core bridge and messenger contracts are deployed behind proxies. The proxy admin can upgrade implementations, which changes contract behavior. Integrators should monitor upgrade governance processes.

### Potential Risks

- **Message censorship**: A sequencer or validator could delay or reorder cross-domain messages, affecting bridge latency.
- **Incorrect token pairing**: Supplying an incorrect L2 token address when depositing (or an incorrect L1 token address when finalizing a withdrawal) can result in irrecoverable loss of funds.
- **Pause mechanism**: `SuperchainConfig` can pause deposits and withdrawals. During a pause, bridged funds cannot be moved until the pause is lifted.
- **Withdrawal proving**: Finalizing a withdrawal requires submitting a Merkle proof against a published output root. If the output oracle is delayed or publishes incorrect roots, withdrawals cannot be finalized.
- **Replay across chains**: Developers must ensure that signed messages or approvals intended for one chain cannot be replayed on another.

## Bug bounty program

In line with our strategy of being the safest way for users to access crypto:

- Coinbase extended our [best-in-industry](https://www.coinbase.com/blog/celebrating-10-years-of-our-bug-bounty-program) million-dollar [HackerOne bug bounty program](https://hackerone.com/coinbase?type=team) to cover the Base network and Base infrastructure.

- Coinbase has launched a 5 million-dollar [Cantina bug bounty program](https://cantina.xyz/code/55316f42-3c5e-4746-9bd0-0f18dcbc344b) to cover all deployed smart contracts for Base, and those used as part of Coinbase products and services.

## Reporting vulnerabilities

All potential vulnerability reports can be submitted via the following platforms:

1. [**HackerOne**](https://hackerone.com/coinbase): For offchain components and services.
   For more information on reporting vulnerabilities and our HackerOne bug bounty program, view our [security program policies](https://hackerone.com/coinbase?view_policy=true).

2. [**Cantina**](https://cantina.xyz/bounties/55316f42-3c5e-4746-9bd0-0f18dcbc344b): For deployed smart contracts.
   For more information on what smart contracts are considered within the scope of the Cantina bug bounty program, view our [Tier 0](https://cantina.xyz/code/55316f42-3c5e-4746-9bd0-0f18dcbc344b/overview?overviewTab=1&assetGroup=0) and [Tier 1](https://cantina.xyz/code/55316f42-3c5e-4746-9bd0-0f18dcbc344b/overview?overviewTab=1&assetGroup=1) scope guides.

For all other security related inquiries, please reach out to [security@coinbase.com](mailto:security@coinbase.com).
