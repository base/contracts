# Multiproof Deployment Guide

This guide covers deploying the multiproof contracts and registering a prover on Sepolia.

---

## ⚠️ Dev/Test Scripts Only

The scripts in this directory are **development and testing tools only**. They are not suitable for production deployments. Specifically, the NoNitro path (`DeployDevNoNitro.s.sol`):

- Does **no AWS Nitro attestation checking**. Instead it uses a bypass function for quickly registering provers: [`MockDevTEEProverRegistry.addDevSigner()`](https://github.com/base/contracts/blob/main/test/mocks/MockDevTEEProverRegistry.sol#L22)
- Uses a simplified mock `AnchorStateRegistry` (with some differences from the real one): [`MockAnchorStateRegistry`](https://github.com/base/contracts/blob/main/scripts/multiproof/mocks/MockAnchorStateRegistry.sol)

---

## Prerequisites

Install dependencies if you haven't already (required after any `lib/` changes):

```bash
just deps
```

---

## Path 1: NoNitro (Dev — No Attestation)

Use this when you don't have access to an AWS Nitro enclave and want to quickly test the prover without attestation overhead.

### Step 1: Configure `deploy-config/sepolia.json`

Ensure `finalSystemOwner` is set to the address you will deploy from (i.e. the address on your Ledger at the HD path you intend to use). This address becomes the owner of all deployed contracts and must sign all subsequent admin calls.

```json
{
  "finalSystemOwner": "0xYOUR_DEPLOYER_ADDRESS",
  ...
}
```

Other relevant fields:

| Field                          | Description                                             |
| ------------------------------ | ------------------------------------------------------- |
| `teeProposer`                  | Address to be registered as the TEE proposer            |
| `teeNitroImageHash`            | PCR0 hash used when registering the Nitro dev signer    |
| `teeTdxImageHash`              | TDX image hash used when registering the TDX dev signer |
| `multiproofGameType`           | Game type ID for the dispute game                       |
| `multiproofGenesisOutputRoot`  | Initial anchor output root                              |
| `multiproofGenesisBlockNumber` | Initial anchor L2 block number                          |

### Step 2: Deploy contracts

```bash
DEPLOY_CONFIG_PATH=deploy-config/sepolia.json forge script scripts/multiproof/DeployDevNoNitro.s.sol --rpc-url https://sepolia.base.org --broadcast --ledger --hd-paths "m/44'/60'/1'/0/0"
```

On success, deployed addresses are printed to the console and saved to `deployments/<chainId>-dev-no-nitro.json`. You will need the `AnchorStateRegistry` and `TEEProverRegistry` addresses for the steps below.

### Step 3: Set the anchor state

The proving system needs a recent anchor state to catch up to chain tip. Set this immediately after deployment using a fresh block.

```bash
# 1. Get the latest L2 block number
BLOCK=$(cast block-number --rpc-url https://base-sepolia-reth-proofs-k8s-donotuse.cbhq.net:8545)

# 2. Get the output root at that block
OUTPUT_ROOT=$(cast rpc optimism_outputAtBlock $(cast 2h $BLOCK) --rpc-url https://base-sepolia-reth-internal-rpc-donotuse.cbhq.net:7545 | jq -r '.outputRoot')

# 3. Set the anchor state on the deployed MockAnchorStateRegistry
#    Replace 0x983b... with the AnchorStateRegistry address from your deployment output
cast send 0x983bD53AE522C74F1d505fb3A55d5d5B774573A7 \
  "setAnchorState(bytes32,uint256)" $OUTPUT_ROOT $BLOCK \
  --rpc-url https://c3-chainproxy-eth-sepolia-full-dev.cbhq.net \
  --ledger --mnemonic-derivation-path "m/44'/60'/1'/0/0"
```

> **Note:** `MockAnchorStateRegistry.setAnchorState()` has no access control — any address can call it.

### Step 4: Get the enclave signer public key

Query the enclave for its signer public key:

```bash
cast rpc enclave_signerPublicKey -r https://base-proofs-prover-nitro-dev.cbhq.net
```

This returns a raw byte array representing an uncompressed secp256k1 public key (65 bytes, starting with `0x04`). To convert it to an Ethereum address, strip the `0x04` prefix byte, keccak256-hash the remaining 64 bytes, and take the last 20 bytes.

### Step 5: Register the dev signers

Call `addDevSigner` for the Nitro signer and `addDevTDXSigner` for the TDX signer on the deployed `DevTEEProverRegistry`.

> **Note:** PCR0 / TDX image enforcement is handled by `AggregateVerifier` (which bakes
> `teeNitroImageHash` and `teeTdxImageHash` into the journal the enclaves sign). The registry
> only tracks which signer addresses are valid.

```bash
# Replace:
#   0x587d... with the TEEProverRegistry address from your deployment output
#   0x080f... with the Nitro signer address derived in Step 4
cast send 0x587d410B205449fB889EC4a5b351D375C656d084 \
  "addDevSigner(address,bytes32)" \
  0x080f42420846c613158D7b4334257C78bE5A9B90 \
  $TEE_NITRO_IMAGE_HASH \
  --rpc-url https://c3-chainproxy-eth-sepolia-full-dev.cbhq.net \
  --ledger --mnemonic-derivation-path "m/44'/60'/1'/0/0"

# Register a TDX dev signer for the TDX image hash.
cast send 0x587d410B205449fB889EC4a5b351D375C656d084 \
  "addDevTDXSigner(address,bytes32)" \
  $TDX_SIGNER_ADDRESS \
  $TEE_TDX_IMAGE_HASH \
  --rpc-url https://c3-chainproxy-eth-sepolia-full-dev.cbhq.net \
  --ledger --mnemonic-derivation-path "m/44'/60'/1'/0/0"
```

The deployer address (`finalSystemOwner`) is the owner of `DevTEEProverRegistry` and must sign this call.

---

## Path 2: TDX (Production-Path PoC)

TDX registration requires both Nitro and TDX signers. `TDXVerifier` verifies `TDXVerifierJournal`; `TEEProverRegistry` registers Nitro signers through `registerSigner(bytes,bytes)` and TDX signers through `registerTDXSigner(bytes,bytes)`.

### Step 1: Deploy verifier policy contracts

`NitroEnclaveVerifier` and `TDXVerifier` are deployed separately from the main multiproof stack because they depend on verifier interfaces that require Solidity `^0.8.20`, while the rest of the multiproof deployment stack is pinned to Solidity `0.8.15`.

Sepolia defaults live in `scripts/multiproof/justfile`. Config-driven deployments get `tdxVerifier` and image hashes from the deploy config.

Deploy Nitro:

```bash
just --justfile scripts/multiproof/justfile deploy-nitro-verifier $NITRO_ROOT_CERT $NITRO_VERIFIER_ID
```

This saves output to `deployments/<chainId>-nitro-verifier.json`.

Deploy TDX:

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-verifier
```

The script saves output to `deployments/<chainId>-tdx-verifier.json`.

### Step 2: Deploy the TDX multiproof test stack

The recipe defaults `DEPLOY_CONFIG_PATH` to `deploy-config/zeronet-tdx.json` and requires the `NitroEnclaveVerifier` and `TDXVerifier` addresses from Step 1. `finalSystemOwner` in the deploy config must be the account broadcasting this transaction because the script updates `NitroEnclaveVerifier.proofSubmitter` to the deployed `TEEProverRegistry`.

The TDX registry manager is set to `TDX_REGISTRATION_MANAGER`, allowing that address to call `registerTDXSigner(bytes,bytes)`. Register a Nitro signer through `registerSigner(bytes,bytes)` as well before submitting TEE proposal proofs.

The `deploy-tdx-stack` recipe resolves a recent L2 output root before invoking `DeployDevWithTDX`, then injects the verifier addresses, resolved output root, and L2 block through `run(address,address,address,bytes32,uint256)`. Use `L2_OUTPUT_ROOT_RPC_URL` if the `optimism_outputAtBlock` endpoint differs from the L2 execution RPC, or pass an explicit third argument to anchor a fixed L2 block.

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-stack $NITRO_VERIFIER $TDX_VERIFIER
```

The script saves output to `deployments/<chainId>-dev-with-tdx.json`.

### Step 3: Register Nitro and TDX signers

Register a Nitro signer with a ZK-proven Nitro attestation:

```bash
cast send $TEE_PROVER_REGISTRY \
  "registerSigner(bytes,bytes)" \
  $NITRO_OUTPUT \
  $NITRO_PROOF_BYTES \
  --rpc-url $L1_RPC_URL \
  --private-key $PRIVATE_KEY
```

Once you have the ABI-encoded `TDXVerifierJournal` output and matching RISC Zero proof bytes from the TDX DCAP guest, register the signer through the TDX-aware registry:

```bash
cast send $TEE_PROVER_REGISTRY \
  "registerTDXSigner(bytes,bytes)" \
  $TDX_OUTPUT \
  $PROOF_BYTES \
  --rpc-url $L1_RPC_URL \
  --private-key $PRIVATE_KEY
```

---

## Pre-Seeding Games (Post-Deployment)

For forward-traversal testing, use `scripts/multiproof/generate-roots.sh` to write roots and `scripts/multiproof/SeedGames.s.sol` to create games onchain. Their usage and env docs live in those files.
