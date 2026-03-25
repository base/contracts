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
% PUB_KEY_HEX=$(python3 -c 'data=[4,100,32,206,76,214,221,167,247,152,244,81,135,139,245,114,92,16,194,181,5,126,180,170,159,214,176,6,51,103,228,117,224,176,243,160,107,112,6,214,20,46,169,42,75,45,190,178,224,54,111,208,42,6,11,198,138,118,144,226,1,147,38,86,196]; print("0x" + bytes(data[1:]).hex())')
% HASH=$(cast keccak $PUB_KEY_HEX)
% RAW_ADDRESS="0x${HASH: -40}"
% cast to-check-sum-address $RAW_ADDRESS
0x0cbe4A965B41DA6B2D5AF4d53c0C16a37d6f9F7D
```

### Step 5: Register the dev signer

Call `addDevSigner` on the deployed `DevTEEProverRegistry` with the **signer address** derived in Step 4.

> **Note:** PCR0 enforcement is handled by `AggregateVerifier` (which bakes `teeImageHash` into the
> journal the enclave signs). The registry only tracks which signer addresses are valid.

```bash
# Replace:
#   0x587d... with the TEEProverRegistry address from your deployment output
#   0x080f... with the signer address derived in Step 4
cast send 0x587d410B205449fB889EC4a5b351D375C656d084 \
  "addDevSigner(address)" \
  0x080f42420846c613158D7b4334257C78bE5A9B90 \
  --rpc-url https://c3-chainproxy-eth-sepolia-full-dev.cbhq.net \
  --ledger --mnemonic-derivation-path "m/44'/60'/1'/0/0"
```

The deployer address (`finalSystemOwner`) is the owner of `DevTEEProverRegistry` and must sign this call.

---

## Path 2: WithNitro (Dev — Real Attestation)

> **TODO:** Add deployment and registration guide for `DeployDevWithNitro.s.sol`.
