package main

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"math/big"
	"strconv"

	"github.com/ethereum-optimism/optimism/cannon/mipsevm/arch"
	"github.com/ethereum-optimism/optimism/op-chain-ops/crossdomain"
	"github.com/ethereum-optimism/optimism/op-core/predeploys"
	"github.com/ethereum-optimism/optimism/op-node/bindings"
	"github.com/ethereum-optimism/optimism/op-node/rollup"
	"github.com/ethereum-optimism/optimism/op-node/rollup/derive"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/trie"
	"github.com/ethereum/go-ethereum/triedb"
	"github.com/ethereum/go-ethereum/triedb/hashdb"
)

type OutputRootWithChainId struct {
	ChainId *big.Int
	Root    common.Hash
}

type SuperRootProof struct {
	Version     uint8
	Timestamp   uint64
	OutputRoots []OutputRootWithChainId
}

var UnknownNonceVersion = errors.New("Unknown nonce version")

func checkErr(err error, failReason string) {
	if err != nil {
		panic(fmt.Errorf("%s: %w", failReason, err))
	}
}

// parseBigInt parses a base-10 *big.Int and panics if invalid.
func parseBigInt(s string) *big.Int {
	v, ok := new(big.Int).SetString(s, 10)
	if !ok {
		panic(fmt.Errorf("parseBigInt: invalid base-10 integer %q", s))
	}
	return v
}

// parseUintN parses a base-10 unsigned integer with the given bit size and panics if invalid.
func parseUintN(s string, bits int) uint64 {
	v, err := strconv.ParseUint(s, 10, bits)
	checkErr(err, "Error decoding uint")
	return v
}

// wordArg parses a base-10 cannon memory word from a CLI argument.
func wordArg(s string) arch.Word {
	return arch.Word(parseUintN(s, arch.WordSize))
}

// parseCrossDomainArgs reads the (nonce, sender, target, value, gasLimit, data)
// tuple shared by the cross-domain-message and withdrawal CLI variants from positional
// args starting at args[1].
func parseCrossDomainArgs(args []string) (nonce *big.Int, sender common.Address, target common.Address, value *big.Int, gasLimit *big.Int, data []byte) {
	return parseBigInt(args[1]),
		common.HexToAddress(args[2]),
		common.HexToAddress(args[3]),
		parseBigInt(args[4]),
		parseBigInt(args[5]),
		common.FromHex(args[6])
}

// packAndPrint ABI-encodes vals using args and writes the hex result to stdout.
func packAndPrint(args abi.Arguments, vals ...any) {
	packed, err := args.Pack(vals...)
	checkErr(err, "Error encoding output")
	fmt.Print(hexutil.Encode(packed))
}

// packTupleAndPrint is like packAndPrint but strips the leading 32-byte head word
// that go-ethereum prepends when the top-level Arguments wraps a single dynamic
// tuple, so Solidity callers decode the inner tuple directly.
func packTupleAndPrint(args abi.Arguments, v any) {
	packed, err := args.Pack(v)
	checkErr(err, "Error encoding output")
	fmt.Print(hexutil.Encode(packed[32:]))
}

// cannonMemoryProofOutput is the ABI shape returned by all cannonMemoryProof variants.
type cannonMemoryProofOutput struct {
	MemRoot common.Hash
	Proof   []byte
}

func encodeCrossDomainMessage(nonce *big.Int, sender common.Address, target common.Address, value *big.Int, gasLimit *big.Int, data []byte) ([]byte, error) {
	_, version := crossdomain.DecodeVersionedNonce(nonce)
	switch version.Uint64() {
	case 0:
		return crossdomain.EncodeCrossDomainMessageV0(target, sender, data, nonce)
	case 1:
		return crossdomain.EncodeCrossDomainMessageV1(nonce, sender, target, value, gasLimit, data)
	default:
		return nil, UnknownNonceVersion
	}
}

func parseSuperRootProof(abiEncodedProof []byte) (*SuperRootProof, error) {
	unpacked, err := superRootProofArgs.Unpack(abiEncodedProof)
	if err != nil {
		return nil, err
	}
	if len(unpacked) != 1 {
		return nil, errors.New("unexpected number of values after unpacking super root proof")
	}

	// abi.Unpack maps Solidity bytes1 to a [1]uint8 array, not a plain uint8.
	tmp := unpacked[0].(struct {
		Version     [1]uint8 `json:"version"`
		Timestamp   uint64   `json:"timestamp"`
		OutputRoots []struct {
			ChainId *big.Int `json:"chainId"`
			Root    [32]byte `json:"root"`
		} `json:"outputRoots"`
	})

	proof := SuperRootProof{
		Version:     tmp.Version[0],
		Timestamp:   tmp.Timestamp,
		OutputRoots: make([]OutputRootWithChainId, len(tmp.OutputRoots)),
	}
	for i, o := range tmp.OutputRoots {
		proof.OutputRoots[i] = OutputRootWithChainId{
			ChainId: o.ChainId,
			Root:    common.BytesToHash(o.Root[:]),
		}
	}

	return &proof, nil
}

func encodeSuperRootProof(superRootProof *SuperRootProof) ([]byte, error) {
	if superRootProof.Version != 0x01 {
		return nil, errors.New("invalid super root version")
	}
	if len(superRootProof.OutputRoots) == 0 {
		return nil, errors.New("empty super root")
	}

	encoded := make([]byte, 9, 9+64*len(superRootProof.OutputRoots))
	encoded[0] = superRootProof.Version
	binary.BigEndian.PutUint64(encoded[1:9], superRootProof.Timestamp)

	for _, outputRoot := range superRootProof.OutputRoots {
		var chainIdBytes [32]byte
		outputRoot.ChainId.FillBytes(chainIdBytes[:])
		encoded = append(encoded, chainIdBytes[:]...)
		encoded = append(encoded, outputRoot.Root.Bytes()...)
	}

	return encoded, nil
}

func newEmptyStateTrie() *trie.StateTrie {
	t, err := trie.NewStateTrie(
		trie.TrieID(types.EmptyRootHash),
		triedb.NewDatabase(rawdb.NewMemoryDatabase(), &triedb.Config{HashDB: hashdb.Defaults}),
	)
	checkErr(err, "Error creating secure trie")
	return t
}

// parseAndEncodeSuperRoot parses an abi-encoded super root proof hex string
// and returns its packed binary encoding.
func parseAndEncodeSuperRoot(hexStr string) []byte {
	proof, err := parseSuperRootProof(common.FromHex(hexStr))
	checkErr(err, "Error parsing super root proof")
	encoded, err := encodeSuperRootProof(proof)
	checkErr(err, "Error encoding super root proof")
	return encoded
}

func hashWithdrawal(nonce *big.Int, sender common.Address, target common.Address, value *big.Int, gasLimit *big.Int, data []byte) (common.Hash, error) {
	wd := crossdomain.Withdrawal{
		Nonce:    nonce,
		Sender:   &sender,
		Target:   &target,
		Value:    value,
		GasLimit: gasLimit,
		Data:     data,
	}
	return wd.Hash()
}

func hashOutputRootProof(version common.Hash, stateRoot common.Hash, messagePasserStorageRoot common.Hash, latestBlockHash common.Hash) (common.Hash, error) {
	hash, err := rollup.ComputeL2OutputRoot(&bindings.TypesOutputRootProof{
		Version:                  version,
		StateRoot:                stateRoot,
		MessagePasserStorageRoot: messagePasserStorageRoot,
		LatestBlockhash:          latestBlockHash,
	})
	if err != nil {
		return common.Hash{}, err
	}
	return common.Hash(hash), nil
}

func makeDepositTx(
	from common.Address,
	to common.Address,
	value *big.Int,
	mint *big.Int,
	gasLimit *big.Int,
	isCreate bool,
	data []byte,
	l1BlockHash common.Hash,
	logIndex *big.Int,
) types.DepositTx {
	udp := derive.UserDepositSource{
		L1BlockHash: l1BlockHash,
		LogIndex:    logIndex.Uint64(),
	}

	depositTx := types.DepositTx{
		SourceHash:          udp.SourceHash(),
		From:                from,
		Value:               value,
		Gas:                 gasLimit.Uint64(),
		IsSystemTransaction: false,
		Data:                data,
	}

	if mint.Sign() > 0 {
		depositTx.Mint = mint
	}
	if !isCreate {
		depositTx.To = &to
	}

	return depositTx
}

type proveWithdrawalResult struct {
	WorldRoot      common.Hash
	StateRoot      common.Hash
	OutputRoot     common.Hash
	WithdrawalHash common.Hash
	Proof          proofList
}

// buildProveWithdrawalInputs constructs the world/state tries, output root, and
// inclusion proof needed to call OptimismPortal.proveWithdrawalTransaction.
func buildProveWithdrawalInputs(nonce *big.Int, sender, target common.Address, value, gasLimit *big.Int, data []byte) *proveWithdrawalResult {
	wdHash, err := hashWithdrawal(nonce, sender, target, value, gasLimit, data)
	checkErr(err, "Error hashing withdrawal")

	zero := common.Hash{}
	slotKey := crypto.Keccak256Hash(wdHash.Bytes(), zero.Bytes())

	state := newEmptyStateTrie()
	checkErr(state.UpdateStorage(common.Address{}, slotKey.Bytes(), []byte{0x01}), "Error updating storage")

	stateRoot := state.Hash()
	account := types.StateAccount{
		Nonce:   0,
		Balance: common.U2560,
		Root:    stateRoot,
	}
	writer := new(bytes.Buffer)
	checkErr(account.EncodeRLP(writer), "Error encoding account")

	world := newEmptyStateTrie()
	checkErr(world.UpdateStorage(common.Address{}, predeploys.L2ToL1MessagePasserAddr.Bytes(), writer.Bytes()), "Error updating storage")

	var proof proofList
	checkErr(state.Prove(predeploys.L2ToL1MessagePasserAddr.Bytes(), &proof), "Error getting proof")

	worldRoot := world.Hash()
	outputRoot, err := hashOutputRootProof(common.Hash{}, worldRoot, stateRoot, common.Hash{})
	checkErr(err, "Error hashing output root proof")

	return &proveWithdrawalResult{
		WorldRoot:      worldRoot,
		StateRoot:      stateRoot,
		OutputRoot:     outputRoot,
		WithdrawalHash: wdHash,
		Proof:          proof,
	}
}

type proofList [][]byte

func (n *proofList) Put(key []byte, value []byte) error {
	*n = append(*n, value)
	return nil
}

func (n *proofList) Delete(key []byte) error {
	panic("not supported")
}
