package snapshots

import (
	"bytes"
	_ "embed"

	"github.com/ethereum/go-ethereum/accounts/abi"
)

//go:embed abi/DisputeGameFactory.json
var disputeGameFactory []byte

//go:embed abi/DelayedWETH.json
var delayedWETH []byte

//go:embed abi/SystemConfig.json
var systemConfig []byte

func LoadDisputeGameFactoryABI() *abi.ABI {
	return loadABI(disputeGameFactory)
}
func LoadDelayedWETHABI() *abi.ABI {
	return loadABI(delayedWETH)
}

func LoadSystemConfigABI() *abi.ABI {
	return loadABI(systemConfig)
}

func loadABI(json []byte) *abi.ABI {
	if parsed, err := abi.JSON(bytes.NewReader(json)); err != nil {
		panic(err)
	} else {
		return &parsed
	}
}
