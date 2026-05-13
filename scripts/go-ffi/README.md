# `go-ffi`

A lightweight binary for utilities accessed via `forge`'s `ffi` cheatcode in the `contracts` test suite.

<pre>
├── go-ffi
│   ├── <a href="./bin.go">bin</a>: `go-ffi`'s entry point and subcommand dispatcher
│   ├── <a href="./differential-testing.go">diff-testing</a>: Utility for differential testing Solidity implementations against their respective Go implementations
│   ├── <a href="./trie.go">trie</a>: Utility for generating random merkle trie roots / inclusion proofs
│   ├── <a href="./merkle.go">merkle</a>: Utility for generating binary merkle tree roots / inclusion proofs
│   └── <a href="./utils.go">utils</a>: Shared encoding and parsing helpers
</pre>

## Usage

To build, run `just build-go-ffi` from the `contracts` package.

### In a Forge Test

To use `go-ffi` in a forge test, invoke the binary with Foundry's `vm.ffi` cheatcode.

```solidity
function myFFITest() public {
    string[] memory commands = new string[](3);
    commands[0] = "./scripts/go-ffi/go-ffi";
    commands[1] = "trie";
    commands[2] = "valid";
    bytes memory result = vm.ffi(commands);

    // Do something with the result of the command
}
```

### Available Modes

There are three modes available in `go-ffi`: `diff`, `trie`, and `merkle`. Each is a subcommand to the `go-ffi` binary, with its own set of variants.

#### `diff`

> **Note**
> Variant required for diff mode.

| Variant                               | Description                                                                                                        |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `decodeVersionedNonce`                | Decodes a versioned nonce and prints the decoded arguments                                                         |
| `encodeCrossDomainMessage`            | Encodes a cross domain message and prints the encoded message                                                      |
| `hashCrossDomainMessage`              | Encodes and hashes a cross domain message and prints the digest                                                    |
| `hashDepositTransaction`              | Encodes and hashes a deposit transaction and prints the digest                                                     |
| `encodeDepositTransaction`            | RLP encodes a deposit transaction                                                                                  |
| `hashWithdrawal`                      | Hashes a withdrawal message and prints the digest                                                                  |
| `hashOutputRootProof`                 | Hashes an output root proof and prints the digest                                                                  |
| `getProveWithdrawalTransactionInputs` | Generates the inputs for a `getProveWithdrawalTransaction` call to the `OptimismPortal` given a withdrawal message |
| `cannonMemoryProof`                   | Computes a merkle proof of Cannon's memory                                                                         |
| `cannonMemoryProof2`                  | Generates a memory proof for a third address against a trie containing two preset values                           |
| `encodeScalarEcotone`                 | Encodes Ecotone basefee and blob basefee scalars into a single packed scalar                                       |
| `decodeScalarEcotone`                 | Decodes a packed Ecotone scalar into its basefee and blob basefee scalar components                                |
| `encodeSuperRootProof`                | ABI-encodes a super root proof                                                                                     |
| `hashSuperRootProof`                  | Encodes and hashes a super root proof and prints the digest                                                        |

#### `trie`

> **Note**
> Variant required for `trie` mode.

| Variant                       | Description                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `valid`                       | Generate a test case with a valid proof of inclusion for the k/v pair in the trie.                                                        |
| `extra_proof_elems`           | Generate an invalid test case with an extra proof element attached to an otherwise valid proof of inclusion for the passed k/v.           |
| `corrupted_proof`             | Generate an invalid test case where the proof is malformed.                                                                               |
| `invalid_data_remainder`      | Generate an invalid test case where a random element of the proof has more bytes than the length designates within the RLP list encoding. |
| `invalid_large_internal_hash` | Generate an invalid test case where a long proof element is incorrect for the root.                                                       |
| `invalid_internal_node_hash`  | Generate an invalid test case where a small proof element is incorrect for the root.                                                      |
| `prefixed_valid_key`          | Generate a valid test case with a key that has been given a random prefix                                                                 |
| `empty_key`                   | Generate a valid test case with a proof of inclusion for an empty key.                                                                    |
| `partial_proof`               | Generate an invalid test case with a partially correct proof                                                                              |

#### `merkle`

> **Note**
> Variant required for `merkle` mode.

| Variant     | Description                                                                                                 |
| ----------- | ----------------------------------------------------------------------------------------------------------- |
| `gen_proof` | Builds a binary merkle tree from the given hex-encoded leaves and emits the root and proof at a given index |
