package main

import (
	"bytes"
	"fmt"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"

	"github.com/ethereum-optimism/optimism/cannon/mipsevm/arch"
	"github.com/ethereum-optimism/optimism/cannon/mipsevm/memory"
	"github.com/ethereum-optimism/optimism/op-chain-ops/crossdomain"
	"github.com/ethereum-optimism/optimism/op-core/predeploys"
	"github.com/ethereum-optimism/optimism/op-service/eth"
)

// ABI types
var (
	dynBytes, _ = abi.NewType("bytes", "", nil)
	bytesArgs   = abi.Arguments{
		{Type: dynBytes},
	}

	fixedBytes, _  = abi.NewType("bytes32", "", nil)
	fixedBytesArgs = abi.Arguments{
		{Type: fixedBytes},
	}

	uint32Type, _ = abi.NewType("uint32", "", nil)

	uint256Type, _ = abi.NewType("uint256", "", nil)

	// Decoded nonce tuple (nonce, version)
	decodedNonce, _ = abi.NewType("tuple", "DecodedNonce", []abi.ArgumentMarshaling{
		{Name: "nonce", Type: "uint256"},
		{Name: "version", Type: "uint256"},
	})
	decodedNonceArgs = abi.Arguments{
		{Name: "encodedNonce", Type: decodedNonce},
	}

	// Decoded ecotone scalars (uint32, uint32)
	decodedScalars = abi.Arguments{
		{Name: "basefeeScalar", Type: uint32Type},
		{Name: "blobbasefeeScalar", Type: uint32Type},
	}

	// Prove withdrawal inputs tuple (bytes32, bytes32, bytes32, bytes32, bytes[])
	proveWithdrawalInputs, _ = abi.NewType("tuple", "ProveWithdrawalInputs", []abi.ArgumentMarshaling{
		{Name: "worldRoot", Type: "bytes32"},
		{Name: "stateRoot", Type: "bytes32"},
		{Name: "outputRoot", Type: "bytes32"},
		{Name: "withdrawalHash", Type: "bytes32"},
		{Name: "proof", Type: "bytes[]"},
	})
	proveWithdrawalInputsArgs = abi.Arguments{
		{Name: "inputs", Type: proveWithdrawalInputs},
	}

	// cannonMemoryProof inputs tuple (bytes32, bytes)
	cannonMemoryProof, _ = abi.NewType("tuple", "CannonMemoryProof", []abi.ArgumentMarshaling{
		{Name: "memRoot", Type: "bytes32"},
		{Name: "proof", Type: "bytes"},
	})
	cannonMemoryProofArgs = abi.Arguments{
		{Name: "encodedCannonMemoryProof", Type: cannonMemoryProof},
	}

	// Super root proof tuple (uint8, uint64, OutputRootWithChainId[])
	superRootProof, _ = abi.NewType("tuple", "SuperRootProof", []abi.ArgumentMarshaling{
		{Name: "version", Type: "bytes1"},
		{Name: "timestamp", Type: "uint64"},
		{Name: "outputRoots", Type: "tuple[]", Components: []abi.ArgumentMarshaling{
			{Name: "chainId", Type: "uint256"},
			{Name: "root", Type: "bytes32"},
		}},
	})
	superRootProofArgs = abi.Arguments{
		{Type: superRootProof},
	}

	// Dependency tuple (uint256)
	dependencyArgs = abi.Arguments{{Name: "chainId", Type: uint256Type}}
)

func DiffTestUtils() {
	args := os.Args[2:]
	if len(args) == 0 {
		panic("Error: No arguments provided")
	}
	variant := args[0]

	switch variant {
	case "decodeVersionedNonce":
		nonce, version := crossdomain.DecodeVersionedNonce(parseBigInt(args[1]))

		packAndPrint(decodedNonceArgs, &struct {
			Nonce   *big.Int
			Version *big.Int
		}{nonce, version})
	case "encodeCrossDomainMessage":
		nonce, sender, target, value, gasLimit, data := parseCrossDomainArgs(args)

		encoded, err := encodeCrossDomainMessage(nonce, sender, target, value, gasLimit, data)
		checkErr(err, "Error encoding cross domain message")

		packAndPrint(bytesArgs, &encoded)
	case "hashCrossDomainMessage":
		nonce, sender, target, value, gasLimit, data := parseCrossDomainArgs(args)

		encoded, err := encodeCrossDomainMessage(nonce, sender, target, value, gasLimit, data)
		checkErr(err, "Error encoding cross domain message")

		hash := crypto.Keccak256Hash(encoded)
		packAndPrint(fixedBytesArgs, &hash)
	case "hashDepositTransaction":
		l1BlockHash := common.HexToHash(args[1])
		logIndex := parseBigInt(args[2])
		from := common.HexToAddress(args[3])
		to := common.HexToAddress(args[4])
		mint := parseBigInt(args[5])
		value := parseBigInt(args[6])
		gasLimit := parseBigInt(args[7])
		data := common.FromHex(args[8])

		depositTx := makeDepositTx(from, to, value, mint, gasLimit, false, data, l1BlockHash, logIndex)

		encoded, err := types.NewTx(&depositTx).MarshalBinary()
		checkErr(err, "Error encoding deposit transaction")

		hash := crypto.Keccak256Hash(encoded)
		packAndPrint(fixedBytesArgs, &hash)
	case "encodeDepositTransaction":
		from := common.HexToAddress(args[1])
		to := common.HexToAddress(args[2])
		value := parseBigInt(args[3])
		mint := parseBigInt(args[4])
		gasLimit := parseBigInt(args[5])
		isCreate := args[6] == "true"
		data := common.FromHex(args[7])
		l1BlockHash := common.HexToHash(args[8])
		logIndex := parseBigInt(args[9])

		depositTx := makeDepositTx(from, to, value, mint, gasLimit, isCreate, data, l1BlockHash, logIndex)

		encoded, err := types.NewTx(&depositTx).MarshalBinary()
		checkErr(err, "Failed to RLP encode deposit transaction")
		packAndPrint(bytesArgs, &encoded)
	case "hashWithdrawal":
		nonce, sender, target, value, gasLimit, data := parseCrossDomainArgs(args)

		hash, err := hashWithdrawal(nonce, sender, target, value, gasLimit, data)
		checkErr(err, "Error hashing withdrawal")

		packAndPrint(fixedBytesArgs, &hash)
	case "hashOutputRootProof":
		version := common.HexToHash(args[1])
		stateRoot := common.HexToHash(args[2])
		messagePasserStorageRoot := common.HexToHash(args[3])
		latestBlockHash := common.HexToHash(args[4])

		hash, err := hashOutputRootProof(version, stateRoot, messagePasserStorageRoot, latestBlockHash)
		checkErr(err, "Error hashing output root proof")

		packAndPrint(fixedBytesArgs, &hash)
	case "getProveWithdrawalTransactionInputs":
		nonce, sender, target, value, gasLimit, data := parseCrossDomainArgs(args)

		wdHash, err := hashWithdrawal(nonce, sender, target, value, gasLimit, data)
		checkErr(err, "Error hashing withdrawal")

		zero := common.Hash{}
		slotKey := crypto.Keccak256Hash(wdHash.Bytes(), zero.Bytes())

		state := newEmptyStateTrie()
		checkErr(state.UpdateStorage(common.Address{}, slotKey.Bytes(), []byte{0x01}), "Error updating storage")

		world := newEmptyStateTrie()
		stateRoot := state.Hash()
		account := types.StateAccount{
			Nonce:   0,
			Balance: common.U2560,
			Root:    stateRoot,
		}
		writer := new(bytes.Buffer)
		checkErr(account.EncodeRLP(writer), "Error encoding account")
		err = world.UpdateStorage(common.Address{}, predeploys.L2ToL1MessagePasserAddr.Bytes(), writer.Bytes())
		checkErr(err, "Error updating storage")

		var proof proofList
		checkErr(state.Prove(predeploys.L2ToL1MessagePasserAddr.Bytes(), &proof), "Error getting proof")

		worldRoot := world.Hash()
		outputRoot, err := hashOutputRootProof(common.Hash{}, worldRoot, stateRoot, common.Hash{})
		checkErr(err, "Error hashing output root proof")

		packTupleAndPrint(proveWithdrawalInputsArgs, &struct {
			WorldRoot      common.Hash
			StateRoot      common.Hash
			OutputRoot     common.Hash
			WithdrawalHash common.Hash
			Proof          proofList
		}{
			WorldRoot:      worldRoot,
			StateRoot:      stateRoot,
			OutputRoot:     outputRoot,
			WithdrawalHash: wdHash,
			Proof:          proof,
		})
	case "cannonMemoryProof":
		// <memAddr0, memValue0, [memAddr1, memValue1], [memAddr2, memValue2]>
		// Equivalent to the cannon STF prestate proofs of the program counter and memory access for instruction execution.
		if len(args) != 3 && len(args) != 5 && len(args) != 7 {
			panic("Error: cannonMemoryProof requires 2, 4, or 6 arguments")
		}
		mem := memory.NewMemory()
		memAddr0 := parseUintN(args[1], arch.WordSize)
		mem.SetWord(arch.Word(memAddr0), arch.Word(parseUintN(args[2], arch.WordSize)))

		var proof1 []byte
		if len(args) >= 5 {
			memAddr1 := parseUintN(args[3], arch.WordSize)
			mem.SetWord(arch.Word(memAddr1), arch.Word(parseUintN(args[4], arch.WordSize)))
			proofAddr := memAddr1
			if len(args) == 7 {
				memAddr2 := parseUintN(args[5], arch.WordSize)
				mem.SetWord(arch.Word(memAddr2), arch.Word(parseUintN(args[6], arch.WordSize)))
				proofAddr = memAddr2
			}
			proof := mem.MerkleProof(arch.Word(proofAddr))
			proof1 = proof[:]
		}
		proof0 := mem.MerkleProof(arch.Word(memAddr0))

		packTupleAndPrint(cannonMemoryProofArgs, &cannonMemoryProofOutput{
			MemRoot: mem.MerkleRoot(),
			Proof:   append(proof0[:], proof1...),
		})
	case "cannonMemoryProof2":
		// <memAddr0, memValue0, [memAddr1, memValue1], memAddr2>
		// Generates memory proof of `memAddr2` for a trie containing `memValue0` and `memValue1`
		if len(args) != 6 {
			panic("Error: cannonMemoryProof2 requires 5 arguments")
		}
		mem := memory.NewMemory()
		mem.SetWord(arch.Word(parseUintN(args[1], arch.WordSize)), arch.Word(parseUintN(args[2], arch.WordSize)))
		mem.SetWord(arch.Word(parseUintN(args[3], arch.WordSize)), arch.Word(parseUintN(args[4], arch.WordSize)))
		memProof := mem.MerkleProof(arch.Word(parseUintN(args[5], arch.WordSize)))

		packTupleAndPrint(cannonMemoryProofArgs, &cannonMemoryProofOutput{
			MemRoot: mem.MerkleRoot(),
			Proof:   memProof[:],
		})
	case "cannonMemoryProofWrongLeaf":
		// <memAddr0, memValue0, memAddr1, memValue1>
		if len(args) != 5 {
			panic("Error: cannonMemoryProofWrongLeaf requires 4 arguments")
		}
		mem := memory.NewMemory()
		memAddr0 := parseUintN(args[1], arch.WordSize)
		mem.SetWord(arch.Word(memAddr0), arch.Word(parseUintN(args[2], arch.WordSize)))
		memAddr1 := parseUintN(args[3], arch.WordSize)
		mem.SetWord(arch.Word(memAddr1), arch.Word(parseUintN(args[4], arch.WordSize)))

		memProof := mem.MerkleProof(arch.Word(memAddr1 + arch.WordSize))
		insnProof := mem.MerkleProof(arch.Word(memAddr0 + arch.WordSize))

		packTupleAndPrint(cannonMemoryProofArgs, &cannonMemoryProofOutput{
			MemRoot: mem.MerkleRoot(),
			Proof:   append(insnProof[:], memProof[:]...),
		})
	case "encodeScalarEcotone":
		encoded := eth.EncodeScalar(eth.EcotoneScalars{
			BaseFeeScalar:     uint32(parseUintN(args[1], 32)),
			BlobBaseFeeScalar: uint32(parseUintN(args[2], 32)),
		})
		fmt.Print(hexutil.Encode(encoded[:]))
	case "decodeScalarEcotone":
		scalar := common.HexToHash(args[1])
		scalars, err := eth.DecodeScalar(scalar)
		checkErr(err, "Error decoding scalar")

		packAndPrint(decodedScalars, scalars.BaseFeeScalar, scalars.BlobBaseFeeScalar)
	case "encodeDependency":
		encoded, err := dependencyArgs.Pack(parseBigInt(args[1]))
		checkErr(err, "Error encoding dependency")

		packAndPrint(bytesArgs, &encoded)
	case "encodeSuperRootProof":
		if len(args) < 2 {
			panic("Error: encodeSuperRootProof requires at least 1 argument")
		}
		encoded := parseAndEncodeSuperRoot(args[1])
		packAndPrint(bytesArgs, &encoded)
	case "hashSuperRootProof":
		if len(args) < 2 {
			panic("Error: hashSuperRootProof requires at least 1 argument")
		}
		hash := crypto.Keccak256Hash(parseAndEncodeSuperRoot(args[1]))
		packAndPrint(fixedBytesArgs, &hash)
	default:
		panic(fmt.Sprintf("Unknown command: %s", variant))
	}
}
