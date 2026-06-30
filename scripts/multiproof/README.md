# Multiproof Deployment Guide

This guide covers deploying the multiproof contracts and registering a prover on Sepolia.
Deploy recipes print addresses and write `deployments/<chainId>-*.json`.

## Dev/Test Scripts Only

The scripts in this directory are **development and testing tools only**. They are not suitable for production deployments. Specifically, the NoNitro path (`DeployDevNoNitro.s.sol`):

- Does **no AWS Nitro attestation checking**. Instead it uses `MockDevTEEProverRegistry.addDevSigner()` in `test/mocks/MockDevTEEProverRegistry.sol`.
- Uses the simplified `MockAnchorStateRegistry` in `scripts/multiproof/mocks/MockAnchorStateRegistry.sol`.

## Path 1: NoNitro (Dev - No Attestation)

Use this when you don't have access to an AWS Nitro enclave and want to quickly test the prover without attestation overhead.

### Configure `deploy-config/sepolia.json`

Set `finalSystemOwner` in `deploy-config/sepolia.json` to the deployer address, then set the multiproof and TEE fields for this environment. `finalSystemOwner` becomes the owner of all deployed contracts and must sign later admin calls.

### Deploy contracts with a fresh anchor

The recipe resolves a recent L2 output root and deploys `DeployDevNoNitro`. Pass an anchor block as the first argument if you need a specific one.

```bash
just --justfile scripts/multiproof/justfile deploy-no-nitro-stack
```

### Get the enclave signer public key

Query the enclave for its signer public key:

```bash
cast rpc enclave_signerPublicKey --rpc-url "$ENCLAVE_RPC_URL"
```

This returns a raw byte array representing an uncompressed secp256k1 public key (65 bytes, starting with `0x04`). To convert it to an Ethereum address, strip the `0x04` prefix byte, keccak256-hash the remaining 64 bytes, and take the last 20 bytes.

### Register the dev signers

Register each signer on the deployed `DevTEEProverRegistry`.

> **Note:** PCR0 / TDX image enforcement is handled by `AggregateVerifier` (which bakes
> `teeNitroImageHash` and `teeTdxImageHash` into the journal the enclaves sign). The registry
> only tracks which signer addresses are valid.

```bash
cast send "$TEE_PROVER_REGISTRY" "addDevSigner(address,bytes32)" "$NITRO_SIGNER" "$NITRO_IMAGE_HASH" \
  --rpc-url "$L1_RPC_URL" --ledger --mnemonic-derivation-path "$LEDGER_PATH"

cast send "$TEE_PROVER_REGISTRY" "addDevTDXSigner(address,bytes32)" "$TDX_SIGNER" "$TDX_IMAGE_HASH" \
  --rpc-url "$L1_RPC_URL" --ledger --mnemonic-derivation-path "$LEDGER_PATH"
```

The deployer address (`finalSystemOwner`) is the owner of `DevTEEProverRegistry` and must sign these calls.

## Path 2: TDX (Production-Path PoC)

TDX registration requires both Nitro and TDX signers. `TDXVerifier` verifies `TDXVerifierJournal`; `TEEProverRegistry` registers Nitro signers through `registerSigner(bytes,bytes)` and TDX signers through `registerTDXSigner(bytes,bytes)`.

### Deploy verifier policy contracts

Deploy the verifier policy contracts separately. Sepolia defaults live in `scripts/multiproof/justfile`.

```bash
just --justfile scripts/multiproof/justfile deploy-nitro-verifier $NITRO_ROOT_CERT $NITRO_VERIFIER_ID
just --justfile scripts/multiproof/justfile deploy-tdx-verifier
```

### Deploy the TDX multiproof test stack

The recipe defaults to `deploy-config/zeronet-tdx.json`, resolves a recent L2 output root, and deploys `DeployDevWithTDX`. Set `L2_OUTPUT_ROOT_RPC_URL` if the output-root RPC differs from `L2_RPC_URL`, or pass an anchor block as the third argument.

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-stack $NITRO_VERIFIER $TDX_VERIFIER
```

### Register Nitro and TDX signers

Register each signer with its proof output.

```bash
cast send "$TEE_PROVER_REGISTRY" "registerSigner(bytes,bytes)" "$NITRO_OUTPUT" "$NITRO_PROOF_BYTES" \
  --rpc-url "$L1_RPC_URL" --private-key "$PRIVATE_KEY"

cast send "$TEE_PROVER_REGISTRY" "registerTDXSigner(bytes,bytes)" "$TDX_OUTPUT" "$TDX_PROOF_BYTES" \
  --rpc-url "$L1_RPC_URL" --private-key "$PRIVATE_KEY"
```
