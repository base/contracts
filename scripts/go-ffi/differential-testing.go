package main

import (
	"fmt"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"

	"github.com/ethereum-optimism/optimism/op-chain-ops/crossdomain"
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
		checkErr(err, "Error encoding deposit transaction")
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
		packTupleAndPrint(proveWithdrawalInputsArgs, buildProveWithdrawalInputs(nonce, sender, target, value, gasLimit, data))
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
	case "encodeSuperRootProof":
		if len(args) != 2 {
			panic("Error: encodeSuperRootProof requires 1 argument")
		}
		encoded := parseAndEncodeSuperRoot(args[1])
		packAndPrint(bytesArgs, &encoded)
	case "hashSuperRootProof":
		if len(args) != 2 {
			panic("Error: hashSuperRootProof requires 1 argument")
		}
		hash := crypto.Keccak256Hash(parseAndEncodeSuperRoot(args[1]))
		packAndPrint(fixedBytesArgs, &hash)
	default:
		panic(fmt.Sprintf("Unknown command: %s", variant))
	}
}
