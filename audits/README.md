# Audits

Audits of the formerly shared OP Stack components can be found in [Optimism’s repository](https://github.com/ethereum-optimism/optimism/tree/develop/docs/security-reviews). The audits of the OP Stack components through Upgrade 18 are relevant for Base. Going forward, audits of the Base Stack smart contracts and protocol can be found here.

| Date | Reviewer | Focus and Scope | Report Link | Commit Hash |
| :---- | :---- | :---- | :---- | :---- |
| 03/17/26 | [Cantina](https://cantina.xyz/portfolio/25ba64ea-d6f3-411e-8338-419ffc385ba6) | AggregateVerifier Multiproof Contracts | [Report](./cantina_coinbase_multiproof_mar2026.pdf) | [b6c4689b8f814bf23fb915ac1d68a537d707ae4e](https://github.com/base/contracts/tree/b6c4689b8f814bf23fb915ac1d68a537d707ae4e/)  |
| 03/19/26 | [Cantina](https://cantina.xyz/portfolio/423a9f33-b710-4445-a125-950b0a7771d7) | TEE Multiproof Contracts | [Report](./cantina_coinbase_nitro_enclave_mar2026.pdf) | [2421afdd332a98e9b45c6caf0a2e26b896b17e0d](https://github.com/base/contracts/tree/2421afdd332a98e9b45c6caf0a2e26b896b17e0d)  |
| 04/10/26 | [Cantina](https://cantina.xyz/portfolio/a4f952cf-1c5b-4e3c-8153-c3adff899613) | TEE Multiproof Contracts 2 | [Report](./cantina_coinbase_nitro_enclave_apr2026.pdf) | [fe2af8cbffefa44bbb1a3917507f8bd8ebec7a2](https://github.com/base/contracts/tree/ffe2af8cbffefa44bbb1a3917507f8bd8ebec7a2)  |
| 04/10/26 | [Cantina](https://cantina.xyz/portfolio/b72c7078-f6da-4074-a3bd-4f938f469fb7) | AggregateVerifier Multiproof Contracts 2 | [Report](./cantina_coinbase_aggregateverifier_apr2026.pdf) | [ffe2af8cbffefa44bbb1a3917507f8bd8ebec7a2](https://github.com/base/contracts/tree/ffe2af8cbffefa44bbb1a3917507f8bd8ebec7a2)  |
| 06/04/26 | [Cantina](https://cantina.xyz/portfolio/6ce647dc-3b2c-448c-9421-426087341ce8) | Proof Contracts Update | [Report](./cantina_coinbase_proof_contracts_update_jun2026.pdf) | [e225648a7ed538e7e28c041d44f3b7a606ba7743](https://github.com/base/contracts/tree/e225648a7ed538e7e28c041d44f3b7a606ba7743)  |
| 08/03/26 | [Cantina](https://cantina.xyz/) | Hinted P-384 Nitro Attestation Validator | [Report](https://github.com/base/nitro-validator/blob/0ea0d12366b4fa44f9e07e4755f2ad36561cb674/audits/cantina-nitro-validator-05294ec0-2026-08-03.pdf) | [05294ec098f7f38ef33b2d2470cfbbd08186b943](https://github.com/base/nitro-validator/tree/05294ec098f7f38ef33b2d2470cfbbd08186b943) |

The report's initial review baseline is `05294ec0`. The `nitro-validator` dependency is pinned at
[`0ea0d123`](https://github.com/base/nitro-validator/tree/0ea0d12366b4fa44f9e07e4755f2ad36561cb674),
which includes the Cantina-verified remediation PRs documented in the final report and the report itself.
