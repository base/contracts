# Multiproof Deployment Guide

This guide covers deploying the multiproof contracts and registering a prover on Sepolia.
Deploy recipes print addresses and write `deployments/<chainId>-*.json`.

## Dev/Test Scripts Only

The scripts in this directory are **development and testing tools only**. They are not suitable for production deployments. They use the simplified `MockAnchorStateRegistry`; the NoNitro path also bypasses AWS Nitro attestation checking through `MockDevTEEProverRegistry.addDevSigner()`.

## Run a Deploy Recipe

See `scripts/multiproof/justfile` for defaults, config paths, and optional anchor-block arguments.

| Path                           | Recipe                                                                                                                                                                       |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| NoNitro dev, no attestation    | `just --justfile scripts/multiproof/justfile deploy-no-nitro-stack`                                                                                                          |
| TDX dev/test verifier policies | `just --justfile scripts/multiproof/justfile deploy-nitro-verifier $NITRO_ROOT_CERT $NITRO_VERIFIER_ID`<br>`just --justfile scripts/multiproof/justfile deploy-tdx-verifier` |
| TDX dev/test stack             | `just --justfile scripts/multiproof/justfile deploy-tdx-stack $NITRO_VERIFIER $TDX_VERIFIER`                                                                                 |

## Register Signers

### NoNitro Dev Signers

Register each signer on the deployed `DevTEEProverRegistry`.

The registry stores signer image hashes; `AggregateVerifier` enforces the expected Nitro and TDX image hashes.

```bash
cast send "$TEE_PROVER_REGISTRY" "addDevSigner(address,bytes32)" "$NITRO_SIGNER" "$NITRO_IMAGE_HASH" \
  --rpc-url "$L1_RPC_URL" --ledger --mnemonic-derivation-path "$LEDGER_PATH"

cast send "$TEE_PROVER_REGISTRY" "addDevTDXSigner(address,bytes32)" "$TDX_SIGNER" "$TDX_IMAGE_HASH" \
  --rpc-url "$L1_RPC_URL" --ledger --mnemonic-derivation-path "$LEDGER_PATH"
```

The deployer address (`finalSystemOwner`) is the owner of `DevTEEProverRegistry` and must sign these calls.

### TDX Dev/Test Signers

Register each signer with its proof output.

```bash
cast send "$TEE_PROVER_REGISTRY" "registerSigner(bytes,bytes)" "$NITRO_OUTPUT" "$NITRO_PROOF_BYTES" \
  --rpc-url "$L1_RPC_URL" --private-key "$PRIVATE_KEY"

cast send "$TEE_PROVER_REGISTRY" "registerTDXSigner(bytes,bytes)" "$TDX_OUTPUT" "$TDX_PROOF_BYTES" \
  --rpc-url "$L1_RPC_URL" --private-key "$PRIVATE_KEY"
```
