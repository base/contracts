package main

import (
	"os"

	"github.com/ethereum-optimism/optimism/op-challenger/game/keccak/merkle"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
)

const (
	genProof = "gen_proof"
)

var (
	rootAndProof, _ = abi.NewType("tuple", "", []abi.ArgumentMarshaling{
		{Name: "root", Type: "bytes32"},
		{Name: "proof", Type: "bytes32[]"},
	})

	merkleEncoder = abi.Arguments{
		{Type: rootAndProof},
	}
)

// DiffMerkle generates an abi-encoded `merkleTestCase` of a specified variant.
func DiffMerkle() {
	args := os.Args[2:]
	if len(args) == 0 {
		panic("Must pass a variant to the merkle diff tester!")
	}
	variant := args[0]

	switch variant {
	case genProof:
		if len(args) < 3 {
			panic("Invalid arguments to `gen_proof` variant.")
		}

		rawLeaves, err := hexutil.Decode(args[1])
		checkErr(err, "Failed to decode leaves")
		index := parseUintN(args[2], 64)

		merkleTree := merkle.NewBinaryMerkleTree()
		for i := 0; i < len(rawLeaves)/32; i++ {
			leaf := common.BytesToHash(rawLeaves[i<<5 : (i+1)<<5])
			merkleTree.AddLeaf(leaf)
		}

		packTupleAndPrint(merkleEncoder, struct {
			Root  common.Hash
			Proof [merkle.BinaryMerkleTreeDepth]common.Hash
		}{
			Root:  merkleTree.RootHash(),
			Proof: merkleTree.ProofAtIndex(index),
		})
	default:
		panic("Invalid variant passed to merkle diff tester!")
	}
}
