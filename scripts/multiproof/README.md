# Multiproof Deployment Guide

This guide covers deploying the multiproof contracts and registering a prover on Sepolia.

## Dev/Test Scripts Only

The scripts in this directory are **development and testing tools only**. They are not suitable for production deployments. Specifically, the NoNitro path (`DeployDevNoNitro.s.sol`):

- Does **no AWS Nitro attestation checking**. Instead it uses `MockDevTEEProverRegistry.addDevSigner()` in `test/mocks/MockDevTEEProverRegistry.sol`.
- Uses the simplified `MockAnchorStateRegistry` in `scripts/multiproof/mocks/MockAnchorStateRegistry.sol`.

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

The recipe resolves a recent L2 output root and deploys `DeployDevNoNitro`. Pass an anchor block as the first argument if you need a specific one.

```bash
just --justfile scripts/multiproof/justfile deploy-no-nitro-stack
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
METHOD=addDevSigner SIGNER="$NITRO_SIGNER_ADDRESS" IMAGE_HASH="$TEE_NITRO_IMAGE_HASH"
# METHOD=addDevTDXSigner SIGNER="$TDX_SIGNER_ADDRESS" IMAGE_HASH="$TEE_TDX_IMAGE_HASH"

cast send "$TEE_PROVER_REGISTRY" \
  "${METHOD}(address,bytes32)" \
  "$SIGNER" \
  "$IMAGE_HASH" \
  --rpc-url "$L1_RPC_URL" \
  --ledger \
  --mnemonic-derivation-path "$LEDGER_PATH"
```

The deployer address (`finalSystemOwner`) is the owner of `DevTEEProverRegistry` and must sign this call.

## Path 2: TDX (Production-Path PoC)

TDX registration requires both Nitro and TDX signers. `TDXVerifier` verifies `TDXVerifierJournal`; `TEEProverRegistry` registers Nitro signers through `registerSigner(bytes,bytes)` and TDX signers through `registerTDXSigner(bytes,bytes)`.

### Step 1: Deploy verifier policy contracts

Deploy the verifier policy contracts separately. Sepolia defaults live in `scripts/multiproof/justfile`.

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

Register each signer with its proof output:

| Signer | Method | Output | Proof |
| --- | --- | --- | --- |
| Nitro | `registerSigner` | `$NITRO_OUTPUT` | `$NITRO_PROOF_BYTES` |
| TDX | `registerTDXSigner` | `$TDX_OUTPUT` | `$TDX_PROOF_BYTES` |

```bash
METHOD=registerSigner OUTPUT="$NITRO_OUTPUT" PROOF="$NITRO_PROOF_BYTES"
# METHOD=registerTDXSigner OUTPUT="$TDX_OUTPUT" PROOF="$TDX_PROOF_BYTES"

cast send "$TEE_PROVER_REGISTRY" \
  "${METHOD}(bytes,bytes)" \
  "$OUTPUT" \
  "$PROOF" \
  --rpc-url "$L1_RPC_URL" \
  --private-key "$PRIVATE_KEY"
```

## Pre-Seeding Games (Post-Deployment)

For forward-traversal testing, use `scripts/multiproof/generate-roots.sh` to write roots and `scripts/multiproof/SeedGames.s.sol` to create games onchain. Their usage and env docs live in those files.
