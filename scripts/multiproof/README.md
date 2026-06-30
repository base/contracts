# Multiproof Deployment Guide

This guide covers deploying the multiproof contracts and registering a prover on Sepolia.

## Dev/Test Scripts Only

The scripts in this directory are **development and testing tools only**. They are not suitable for production deployments. Specifically, the NoNitro path (`DeployDevNoNitro.s.sol`):

- Does **no AWS Nitro attestation checking**. Instead it uses a bypass function for quickly registering provers: [`MockDevTEEProverRegistry.addDevSigner()`](https://github.com/base/contracts/blob/main/test/mocks/MockDevTEEProverRegistry.sol#L22)
- Uses a simplified mock `AnchorStateRegistry` (with some differences from the real one): [`MockAnchorStateRegistry`](https://github.com/base/contracts/blob/main/scripts/multiproof/mocks/MockAnchorStateRegistry.sol)

## Prerequisites

Install dependencies if you haven't already (required after any `lib/` changes):

```bash
just deps
```

## Path 1: NoNitro (Dev - No Attestation)

Use this when you don't have access to an AWS Nitro enclave and want to quickly test the prover without attestation overhead.

### Step 1: Configure `deploy-config/sepolia.json`

Set `finalSystemOwner` to the deployer address, then set the multiproof and TEE fields for this environment. `finalSystemOwner` becomes the owner of all deployed contracts and must sign later admin calls.

```json
{
  "finalSystemOwner": "0xYOUR_DEPLOYER_ADDRESS",
  ...
}
```

### Step 2: Deploy contracts with a fresh anchor

The proving system needs a recent anchor state to catch up to chain tip. Pass it to `DeployDevNoNitro.run(bytes32,uint256)` during deployment.

```bash
BLOCK=$(cast block-number --rpc-url "$L2_RPC_URL")
OUTPUT_ROOT=$(cast rpc optimism_outputAtBlock "$(cast to-hex "$BLOCK")" --rpc-url "$L2_OUTPUT_ROOT_RPC_URL" | jq -er '.outputRoot')

DEPLOY_CONFIG_PATH=deploy-config/sepolia.json \
forge script scripts/multiproof/DeployDevNoNitro.s.sol:DeployDevNoNitro \
  --sig "run(bytes32,uint256)" \
  "$OUTPUT_ROOT" \
  "$BLOCK" \
  --rpc-url "$L1_RPC_URL" \
  --broadcast \
  --ledger \
  --hd-paths "$LEDGER_PATH"
```

On success, deployed addresses are printed to the console and saved to `deployments/<chainId>-dev-no-nitro.json`.

### Step 3: Get the enclave signer public key

Query the enclave for its signer public key:

```bash
cast rpc enclave_signerPublicKey --rpc-url "$ENCLAVE_RPC_URL"
```

This returns a raw byte array representing an uncompressed secp256k1 public key (65 bytes, starting with `0x04`). To convert it to an Ethereum address, strip the `0x04` prefix byte, keccak256-hash the remaining 64 bytes, and take the last 20 bytes.

### Step 4: Register the dev signers

Call `addDevSigner` for the Nitro signer and `addDevTDXSigner` for the TDX signer on the deployed `DevTEEProverRegistry`.

> **Note:** PCR0 / TDX image enforcement is handled by `AggregateVerifier` (which bakes
> `teeNitroImageHash` and `teeTdxImageHash` into the journal the enclaves sign). The registry
> only tracks which signer addresses are valid.

```bash
cast send "$TEE_PROVER_REGISTRY" \
  "addDevSigner(address,bytes32)" \
  "$NITRO_SIGNER_ADDRESS" \
  "$TEE_NITRO_IMAGE_HASH" \
  --rpc-url "$L1_RPC_URL" \
  --ledger \
  --mnemonic-derivation-path "$LEDGER_PATH"

cast send "$TEE_PROVER_REGISTRY" \
  "addDevTDXSigner(address,bytes32)" \
  "$TDX_SIGNER_ADDRESS" \
  "$TEE_TDX_IMAGE_HASH" \
  --rpc-url "$L1_RPC_URL" \
  --ledger \
  --mnemonic-derivation-path "$LEDGER_PATH"
```

The deployer address (`finalSystemOwner`) is the owner of `DevTEEProverRegistry` and must sign this call.

## Path 2: TDX (Production-Path PoC)

TDX registration requires both Nitro and TDX signers. `TDXVerifier` verifies `TDXVerifierJournal`; `TEEProverRegistry` registers Nitro signers through `registerSigner(bytes,bytes)` and TDX signers through `registerTDXSigner(bytes,bytes)`.

### Step 1: Deploy verifier policy contracts

`NitroEnclaveVerifier` and `TDXVerifier` are deployed separately from the main multiproof stack because they depend on verifier interfaces that require Solidity `^0.8.20`, while the rest of the multiproof deployment stack is pinned to Solidity `0.8.15`.

Sepolia defaults live in `scripts/multiproof/justfile`.

```bash
just --justfile scripts/multiproof/justfile deploy-nitro-verifier $NITRO_ROOT_CERT $NITRO_VERIFIER_ID
just --justfile scripts/multiproof/justfile deploy-tdx-verifier
```

These save output to `deployments/<chainId>-nitro-verifier.json` and `deployments/<chainId>-tdx-verifier.json`.

### Step 2: Deploy the TDX multiproof test stack

The recipe defaults to `deploy-config/zeronet-tdx.json`, resolves a recent L2 output root, and deploys `DeployDevWithTDX`. Set `L2_OUTPUT_ROOT_RPC_URL` if the output-root RPC differs from `L2_RPC_URL`, or pass an anchor block as the third argument.

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-stack $NITRO_VERIFIER $TDX_VERIFIER
```

The script saves output to `deployments/<chainId>-dev-with-tdx.json`.

### Step 3: Register Nitro and TDX signers

Register a Nitro signer with a ZK-proven Nitro attestation:

```bash
cast send "$TEE_PROVER_REGISTRY" \
  "registerSigner(bytes,bytes)" \
  "$NITRO_OUTPUT" \
  "$NITRO_PROOF_BYTES" \
  --rpc-url "$L1_RPC_URL" \
  --private-key "$PRIVATE_KEY"
```

Once you have the ABI-encoded `TDXVerifierJournal` output and matching RISC Zero proof bytes from the TDX DCAP guest, register the signer through the TDX-aware registry:

```bash
cast send "$TEE_PROVER_REGISTRY" \
  "registerTDXSigner(bytes,bytes)" \
  "$TDX_OUTPUT" \
  "$PROOF_BYTES" \
  --rpc-url "$L1_RPC_URL" \
  --private-key "$PRIVATE_KEY"
```

## Pre-Seeding Games (Post-Deployment)

For forward-traversal testing, use `scripts/multiproof/generate-roots.sh` to write roots and `scripts/multiproof/SeedGames.s.sol` to create games onchain. Their usage and env docs live in those files.
