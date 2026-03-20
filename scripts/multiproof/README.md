# Multiproof Deployment Guide

This guide covers deploying the multiproof contracts and registering a prover on Sepolia.

---

## ⚠️ Dev/Test Scripts Only

The scripts in this directory are **development and testing tools only**. They are not suitable for production deployments. Specifically, the NoNitro path (`DeployDevNoNitro.s.sol`):

- Does **no AWS Nitro attestation checking**. Instead it uses a bypass function for quickly registering provers: [`MockDevTEEProverRegistry.addDevSigner()`](https://github.com/base/contracts/blob/main/src/multiproof/mocks/MockDevTEEProverRegistry.sol#L22)
- Uses a simplified mock `AnchorStateRegistry` (with some differences from the real one): [`MockAnchorStateRegistry`](https://github.com/base/contracts/blob/main/scripts/multiproof/mocks/MockAnchorStateRegistry.sol)

---

## Prerequisites

Install dependencies if you haven't already (required after any `lib/` changes):

```bash
make deps
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

| Field | Description |
|---|---|
| `teeProposer` | Address to be registered as the TEE proposer |
| `teeImageHash` | PCR0 hash used when registering the dev signer (use `bytes32(0x01...01)` for dev) |
| `multiproofGameType` | Game type ID for the dispute game |
| `multiproofGenesisOutputRoot` | Initial anchor output root |
| `multiproofGenesisBlockNumber` | Initial anchor L2 block number |

### Step 2: Deploy contracts

```bash
DEPLOY_CONFIG_PATH=deploy-config/sepolia.json forge script scripts/multiproof/DeployDevNoNitro.s.sol --rpc-url https://sepolia.base.org --broadcast --ledger --hd-paths "m/44'/60'/1'/0/0"
```

On success, deployed addresses are printed to the console and saved to `deployments/<chainId>-dev-no-nitro.json`. You will need the `AnchorStateRegistry` and `TEEProverRegistry` addresses for the steps below.

### Step 3: Set the anchor state

The proving system needs a recent anchor state to catch up to chain tip. Set this immediately after deployment using a fresh block.

```bash
# 1. Get the latest L2 block number
BLOCK=$(cast block-number --rpc-url https://base-sepolia-archive-k8s-dev.cbhq.net:8545)

# 2. Get the output root at that block
OUTPUT_ROOT=$(cast rpc optimism_outputAtBlock $(cast 2h $BLOCK) --rpc-url https://base-sepolia-archive-k8s-dev.cbhq.net:7545 | jq -r '.outputRoot')

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

This returns a raw byte array representing an uncompressed secp256k1 public key (65 bytes, starting with `0x04`). To convert it to an Ethereum address, strip the `0x04` prefix byte, keccak256-hash the remaining 64 bytes, and take the last 20 bytes:

```bash
# Example — replace the array with the actual bytes returned by enclave_signerPublicKey
% cast keccak $(python3 -c "data=[4,155,107,175,137,123,186,174,83,167,173,206,55,138,218,209,181,42,87,20,116,162,104,100,19,14,59,133,233,253,147,253,236,102,24,76,164,146,220,67,146,235,73,9,142,114,242,170,122,102,175,104,24,235,26,93,14,6,81,84,116,33,71,62,237]; print('0x' + bytes(data[1:]).hex())")
0xaafcb729589f27eb76b25a90080f42420846c613158d7b4334257c78be5a9b90

% cast to-check-sum-address 0x080f42420846c613158d7b4334257c78be5a9b90
0x080f42420846c613158D7b4334257C78bE5A9B90
```

### Step 5: Register the dev signer

Call `addDevSigner` on the deployed `DevTEEProverRegistry` with:
- The **signer address** derived in Step 4
- The **PCR0 hash** — this must match the `teeImageHash` set in `deploy-config/sepolia.json`, since that value is baked into `AggregateVerifier` as an immutable at deploy time. Changing it requires redeploying `AggregateVerifier`

```bash
# Replace:
#   0x587d... with the TEEProverRegistry address from your deployment output
#   0x080f... with the signer address derived in Step 4
#   0x0000...0001 with the teeImageHash from deploy-config/sepolia.json
cast send 0x587d410B205449fB889EC4a5b351D375C656d084 \
  "addDevSigner(address,bytes32)" \
  0x080f42420846c613158D7b4334257C78bE5A9B90 \
  0x0000000000000000000000000000000000000000000000000000000000000001 \
  --rpc-url https://c3-chainproxy-eth-sepolia-full-dev.cbhq.net \
  --ledger --mnemonic-derivation-path "m/44'/60'/1'/0/0"
```

The deployer address (`finalSystemOwner`) is the owner of `DevTEEProverRegistry` and must sign this call.

---

## Path 2: WithNitro (Dev — Real Attestation)

> **TODO:** Add deployment and registration guide for `DeployDevWithNitro.s.sol`.
