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

| Field                          | Description                                                                       |
| ------------------------------ | --------------------------------------------------------------------------------- |
| `teeProposer`                  | Address to be registered as the TEE proposer                                      |
| `teeNitroImageHash`            | PCR0 hash used when registering the Nitro dev signer                              |
| `teeTdxImageHash`              | TDX image hash used when registering the TDX dev signer                           |
| `multiproofGameType`           | Game type ID for the dispute game                                                 |
| `multiproofGenesisOutputRoot`  | Initial anchor output root                                                        |
| `multiproofGenesisBlockNumber` | Initial anchor L2 block number                                                    |

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

## Path 2: WithNitro (Dev — Real Attestation)

> **TODO:** Add deployment and registration guide for `DeployDevWithNitro.s.sol`.

---

## Path 3: TDX (Production-Path PoC)

The TDX path follows the same split as Nitro: expensive attestation verification happens off-chain in a ZK guest,
and Solidity verifies the proof plus the on-chain acceptance policy before registering the signer.

| Contract            | Purpose                                                                                                                                                                                                                                       |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `TDXVerifier`       | Verifies a RISC Zero or SP1 proof whose public values are an ABI-encoded `TDXVerifierJournal`, then checks trusted Intel root, TCB status policy, collateral expiry, quote freshness, signer derivation, and `REPORTDATA` public-key binding. |
| `TEEProverRegistry` | Registers Nitro signers through `registerSigner(bytes,bytes)` and TDX signers through `registerTDXSigner(bytes,bytes)`, tracking which TEE type each signer came from for `TEEVerifier`.                                                      |

The ZK verifier guest is expected to perform the full Intel DCAP verification path:

```text
TD Quote signature
PCK certificate chain
TCB info signing chain and TCB status
QE identity signing chain
CRLs/revocation state
TDREPORT field extraction
```

The Solidity verifier then enforces local policy over the proven journal. The PoC maps TDX measurements into `TEE_TDX_IMAGE_HASH` as:

```text
keccak256(MRTD || RTMR0 || RTMR1 || RTMR2 || RTMR3)
```

The attested public key must be supplied as an uncompressed 65-byte secp256k1 public key:

```text
0x04 || x || y
```

The quote's TDREPORT `REPORTDATA` must put `keccak256(x || y)` in the first 32 bytes. The last 32 bytes are returned by the verifier as app-specific binding data and emitted by the registry.

`TEEVerifier` is still the proposal-proof verifier, but a TEE proposal proof now requires two signatures over the same journal: one from a Nitro-registered signer and one from a TDX-registered signer. The proof bytes are `proposer || signatureA || signatureB`; either signature order is accepted as long as both registered TEE types are present and both signers match their expected type-specific image hash.

> **PoC boundary:** this repo now contains the production-shaped Solidity path and policy checks. The remaining off-chain piece is the actual RISC Zero/SP1 TDX DCAP guest that emits `TDXVerifierJournal` after verifying Intel collateral.

### Step 1: Deploy verifier policy contracts

`NitroEnclaveVerifier` and `TDXVerifier` are deployed separately from the main multiproof stack because they depend on verifier interfaces that require Solidity `^0.8.20`, while the rest of the multiproof deployment stack is pinned to Solidity `0.8.15`.

For Sepolia TDX testing, `scripts/multiproof/justfile` defaults to:

```bash
NITRO_RISC0_VERIFIER_ROUTER=0xB121B667dd2cf27F95f9F5107137696F56f188f6
TDX_RISC0_VERIFIER_ROUTER=0x925d8331ddc0a1F0d96E68CF073DFE1d92b69187
RISC0_SET_BUILDER_IMAGE_ID=0x70909b25db0db00f1d4b4016aeb876f53568a3e5a8e6397cb562d79947a02cc9
TDX_VERIFIER_ID=0xb9681d1f76f5dbf70da84ad06b5b20befa392638060e947965269b6f63ebbf3b
INTEL_ROOT_CA_HASH=0xa1acc73eb45794fa1734f14d882e91925b6006f79d3bb2460df9d01b333d7009
TDX_IMAGE_HASH=0x4cb35ee476a8098c4e567098714c65f5afe25236fc460b38487a356e14e7db66
TDX_REGISTRATION_MANAGER=0x44E999A5859c2D12378a349882fAe5805DCE71b9
```

`deploy-config/sepolia.json` uses the same TDX RISC Zero verifier router and `TDX_IMAGE_HASH`. The `TEEProverRegistry`
constructor also requires a non-zero `tdxVerifier`; for config-driven deployments, set `tdxVerifier` in the deploy
config to the deployed `TDXVerifier` address.

Deploy both TEE verifier policy contracts with one command:

```bash
just --justfile scripts/multiproof/justfile deploy-tee-verifiers $NITRO_ROOT_CERT $NITRO_VERIFIER_ID
```

This saves output to `deployments/<chainId>-tee-verifiers.json`.

To deploy a fresh Nitro verifier, a fresh TDX verifier, and then the TDX
multiproof stack against those exact verifier addresses without editing
`deploy-config/sepolia.json`, use:

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-system $NITRO_ROOT_CERT $NITRO_VERIFIER_ID
```

To deploy Nitro separately:

```bash
just --justfile scripts/multiproof/justfile deploy-nitro-verifier $NITRO_ROOT_CERT $NITRO_VERIFIER_ID
```

This saves output to `deployments/<chainId>-nitro-verifier.json`.

To deploy TDX separately:

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-verifier
```

To override any TDX verifier input manually, pass all three verifier args:

```bash
forge script scripts/multiproof/DeployTDXVerifier.s.sol:DeployTDXVerifier \
  --sig "run(address,address,bytes32,bytes32)" \
  $OWNER \
  $TDX_RISC0_VERIFIER_ROUTER \
  $TDX_VERIFIER_ID \
  $INTEL_ROOT_CA_HASH \
  --rpc-url $L1_RPC_URL \
  --broadcast \
  --private-key $PRIVATE_KEY
```

The script saves output to `deployments/<chainId>-tdx-verifier.json`.

To override Nitro inputs manually:

```bash
forge script scripts/multiproof/DeployNitroVerifier.s.sol:DeployNitroVerifier \
  --sig "run(address,address,bytes32,bytes32,bytes32)" \
  $OWNER \
  $NITRO_RISC0_VERIFIER_ROUTER \
  $RISC0_SET_BUILDER_IMAGE_ID \
  $NITRO_ROOT_CERT \
  $NITRO_VERIFIER_ID \
  --rpc-url $L1_RPC_URL \
  --broadcast \
  --private-key $PRIVATE_KEY
```

Use `DeployTEEVerifiers.s.sol` with `run(address,address,address,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)` to override all Nitro and TDX inputs while still deploying both in one broadcast.

### Step 2: Deploy the TDX multiproof test stack

Set `DEPLOY_CONFIG_PATH` to the Sepolia deploy config and pass the `TDXVerifier` address from Step 1. The deploy config must also contain the `NitroEnclaveVerifier` address, because TEE proposal proofs now require both Nitro and TDX signatures. `finalSystemOwner` in the deploy config must be the account broadcasting this transaction because the script updates both `TDXVerifier.proofSubmitter` and `NitroEnclaveVerifier.proofSubmitter` to the deployed `TEEProverRegistry`.

The TDX registry manager is set to `TDX_REGISTRATION_MANAGER`, allowing that address to call `registerTDXSigner(bytes,bytes)`. Register a Nitro signer through `registerSigner(bytes,bytes)` as well before submitting TEE proposal proofs.

The `deploy-tdx-stack` recipe resolves a recent L2 output root before invoking `DeployDevWithTDX`, then injects the resolved output root and L2 block through `run(address,address,bytes32,uint256)`. Use `L2_OUTPUT_ROOT_RPC_URL` if the `optimism_outputAtBlock` endpoint differs from the L2 execution RPC, and `ASR_ANCHOR_BLOCK_LOOKBACK` to anchor a fixed number of L2 blocks behind head. For a fixed anchor, set both `ASR_ANCHOR_OUTPUT_ROOT` and `ASR_ANCHOR_BLOCK_NUMBER`.

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-stack $TDX_VERIFIER
```

If the Nitro verifier in `deploy-config/sepolia.json` is owned by another account, do not edit the shared Sepolia deploy
config. Pass both verifier addresses explicitly instead:

```bash
just --justfile scripts/multiproof/justfile deploy-tdx-stack-with-verifiers $NITRO_VERIFIER $TDX_VERIFIER
```

To override the manager manually, use:

```bash
export L2_RPC_URL=<l2-execution-rpc>
export L2_OUTPUT_ROOT_RPC_URL=<l2-op-node-or-archive-rpc>
ASR_ANCHOR_BLOCK_NUMBER=$(cast block-number --rpc-url $L2_RPC_URL)
ASR_ANCHOR_OUTPUT_ROOT=$(cast rpc optimism_outputAtBlock $(cast to-hex $ASR_ANCHOR_BLOCK_NUMBER) \
  --rpc-url $L2_OUTPUT_ROOT_RPC_URL | jq -r '.outputRoot')

forge script scripts/multiproof/DeployDevWithTDX.s.sol:DeployDevWithTDX \
  --sig "run(address,address,bytes32,uint256)" \
  $TDX_VERIFIER \
  $TDX_REGISTRATION_MANAGER \
  $ASR_ANCHOR_OUTPUT_ROOT \
  $ASR_ANCHOR_BLOCK_NUMBER \
  --rpc-url $L1_RPC_URL \
  --broadcast \
  --private-key $PRIVATE_KEY
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

After deploying via either path, you can pre-seed the `DisputeGameFactory` with a chain of `AggregateVerifier` games. This is useful for testing forward traversal at proposer restart — the proposer can walk the linked list of games to find where to resume.

Games are created using `ProofType.ZK` with the `MockVerifier` (deployed by both WithNitro and NoNitro), which auto-accepts any proof. The output roots themselves are real values fetched from an L2 archive node.

### Step 1: Set the anchor state

Pick an anchor block far enough behind the L2 tip to cover all the games you want to create. Each game covers `BLOCK_INTERVAL` (600) L2 blocks, so for 500 games you need 300,000 blocks of headroom.

```bash
# Calculate an anchor block 300,000 blocks behind the L2 tip
ANCHOR_BLOCK=$(( $(cast block-number --rpc-url $L2_RPC_URL) - 300000 ))

# Get the real output root at that block
OUTPUT_ROOT=$(cast rpc optimism_outputAtBlock $(printf "0x%x" $ANCHOR_BLOCK) \
  --rpc-url $L2_RPC_URL | jq -r '.outputRoot')

# Set it on the MockAnchorStateRegistry (no access control — any caller works)
cast send $ANCHOR_STATE_REGISTRY_ADDRESS \
  "setAnchorState(bytes32,uint256)" $OUTPUT_ROOT $ANCHOR_BLOCK \
  --rpc-url $L1_RPC_URL --private-key $PRIVATE_KEY
```

### Step 2: Generate real output roots

Fetch the real L2 output roots for every intermediate block across all games. This queries `optimism_outputAtBlock` on the L2 archive node (10,000 queries for 500 games, parallelized).

```bash
./scripts/multiproof/generate-roots.sh $ANCHOR_BLOCK $L2_RPC_URL 500
```

Arguments: `<anchor_block> <l2_rpc_url> [game_count] [parallelism] [output_file]`

Defaults: `game_count=500`, `parallelism=20`, `output_file=roots.json`.

### Step 3: Move roots file for Foundry access

Foundry's filesystem sandbox only allows reads from paths listed in `foundry.toml` `fs_permissions`. The `deployments/` directory already has read-write access, so move the file there:

```bash
mv roots.json deployments/roots.json
```

### Step 4: Run the seeding script

Create all games on-chain. Each game is chained to the previous one (game 0's parent is the `AnchorStateRegistry`, game N's parent is game N-1). The account running this needs enough ETH for bonds and gas (500 games at 0.00001 ETH bond = 0.005 ETH + gas).

```bash
ROOTS_FILE=./deployments/roots.json \
FACTORY_ADDRESS=$FACTORY_ADDRESS \
ANCHOR_STATE_REGISTRY_ADDRESS=$ANCHOR_STATE_REGISTRY_ADDRESS \
forge script scripts/multiproof/SeedGames.s.sol \
  --rpc-url $L1_RPC_URL --broadcast --private-key $PRIVATE_KEY
```

> **Note:** Use `--private-key` instead of `--ledger` to avoid manually confirming 500 transactions on a hardware wallet.

Optional env vars:

| Variable     | Default      | Description                   |
| ------------ | ------------ | ----------------------------- |
| `GAME_COUNT` | 500          | Number of games to create     |
| `ROOTS_FILE` | `roots.json` | Path to the output roots JSON |

### Step 5: Verify on-chain

```bash
# Check total game count
cast call $FACTORY_ADDRESS "gameCount()(uint256)" --rpc-url $L1_RPC_URL

# Check first game's parent is the AnchorStateRegistry
FIRST_GAME=$(cast call $FACTORY_ADDRESS \
  "gameAtIndex(uint256)(uint32,uint64,address)" 0 --rpc-url $L1_RPC_URL | tail -1)
cast call $FIRST_GAME "parentAddress()(address)" --rpc-url $L1_RPC_URL
```

Output metadata is saved to `deployments/<chainId>-seeded-games.json`.
