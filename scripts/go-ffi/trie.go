package main

import (
	"crypto/rand"
	"fmt"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/ethereum/go-ethereum/trie"
	"github.com/ethereum/go-ethereum/triedb"
)

const (
	// Generate a test case with a valid proof of inclusion for the k/v pair in the trie.
	valid string = "valid"
	// Generate an invalid test case with an extra proof element attached to an otherwise
	// valid proof of inclusion for the passed k/v.
	extraProofElems = "extra_proof_elems"
	// Generate an invalid test case where the proof is malformed.
	corruptedProof = "corrupted_proof"
	// Generate an invalid test case where a random element of the proof has more bytes than the
	// length designates within the RLP list encoding.
	invalidDataRemainder = "invalid_data_remainder"
	// Generate an invalid test case where a long proof element is incorrect for the root.
	invalidLargeInternalHash = "invalid_large_internal_hash"
	// Generate an invalid test case where a small proof element is incorrect for the root.
	invalidInternalNodeHash = "invalid_internal_node_hash"
	// Generate a valid test case with a key that has been given a random prefix
	prefixedValidKey = "prefixed_valid_key"
	// Generate a valid test case with a proof of inclusion for an empty key.
	emptyKey = "empty_key"
	// Generate an invalid test case with a partially correct proof
	partialProof = "partial_proof"
)

// FuzzTrie generates an abi-encoded `trieTestCase` of a specified variant.
func FuzzTrie() {
	variant := os.Args[2]
	testCase := genTrieTestCase(variant == emptyKey)

	switch variant {
	case valid, emptyKey:
		// Valid as generated; no mutation.
	case extraProofElems:
		testCase.Proof = append(testCase.Proof, testCase.Proof[len(testCase.Proof)-1])
	case corruptedProof:
		idx := randRange(0, int64(len(testCase.Proof)))
		encoded, _ := rlp.EncodeToBytes(testCase.Proof[idx])
		testCase.Proof[idx] = encoded
	case invalidDataRemainder:
		// Extend a proof element without updating its RLP-encoded length so the
		// decoder sees trailing bytes past the declared end.
		idx := randRange(0, int64(len(testCase.Proof)))
		testCase.Proof[idx] = append(testCase.Proof[idx], randBytes(randRange(1, 512))...)
	case invalidLargeInternalHash:
		// Clobber 4 bytes within a random non-root proof element.
		// TODO: Improve by decoding the proof elem and choosing random bytes to overwrite.
		idx := randRange(1, int64(len(testCase.Proof)))
		copy(testCase.Proof[idx][20:24], randBytes(4))
	case invalidInternalNodeHash:
		// Replace the last proof element with an RLP list containing a 29-byte value.
		e, _ := rlp.EncodeToBytes(randBytes(29))
		testCase.Proof[len(testCase.Proof)-1] = append([]byte{0xc0 + 30}, e...)
	case prefixedValidKey:
		testCase.Key = append(randBytes(randRange(1, 16)), testCase.Key...)
	case partialProof:
		testCase.Proof = testCase.Proof[:len(testCase.Proof)/2]
	default:
		panic(fmt.Errorf("Invalid variant passed to trie fuzzer: %q", variant))
	}

	packTupleAndPrint(abi.Arguments{{Type: trieTestCaseTuple}}, &testCase)
}

// genTrieTestCase builds a random Merkle trie and produces a proof of inclusion
// for one of its k/v pairs.
func genTrieTestCase(selectEmptyKey bool) trieTestCase {
	randTrie := trie.NewEmpty(triedb.NewDatabase(rawdb.NewMemoryDatabase(), nil))

	randN := randRange(2, 1024)
	randSelect := randRange(0, randN)

	var key, value []byte
	for i := range randN {
		k := randBytes(32)
		v := randBytes(randRange(2, 1024))
		if i == randSelect && selectEmptyKey {
			k = nil
		}
		checkErr(randTrie.Update(k, v), "Error adding key-value pair to trie")
		if i == randSelect {
			key, value = k, v
		}
	}

	var proof proofList
	checkErr(randTrie.Prove(key, &proof), "Error creating proof for key inclusion")

	return trieTestCase{
		Root:  randTrie.Hash(),
		Key:   key,
		Value: value,
		Proof: proof,
	}
}

// trieTestCase represents a test case for bedrock's `MerkleTrie.sol`.
type trieTestCase struct {
	Root  common.Hash
	Key   []byte
	Value []byte
	Proof [][]byte
}

var trieTestCaseTuple, _ = abi.NewType("tuple", "TrieTestCase", []abi.ArgumentMarshaling{
	{Name: "root", Type: "bytes32"},
	{Name: "key", Type: "bytes"},
	{Name: "value", Type: "bytes"},
	{Name: "proof", Type: "bytes[]"},
})

// randBytes returns n cryptographically-random bytes.
func randBytes(n int64) []byte {
	b := make([]byte, n)
	_, err := rand.Read(b)
	checkErr(err, "Error generating random bytes")
	return b
}

// randRange returns a cryptographically secure random 64-bit integer in [min, max).
func randRange(min int64, max int64) int64 {
	r, err := rand.Int(rand.Reader, big.NewInt(max-min))
	checkErr(err, "Failed to generate random number within bounds")
	return r.Int64() + min
}
