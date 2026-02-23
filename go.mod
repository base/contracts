module github.com/base/contracts

go 1.24.0

toolchain go1.25.1

require (
	github.com/BurntSushi/toml v1.5.0
	github.com/bmatcuk/doublestar/v4 v4.8.1
	github.com/ethereum-optimism/optimism v1.16.3-0.20260114213306-018f5ae926ec
	github.com/ethereum/go-ethereum v1.16.3
	github.com/stretchr/testify v1.10.0
	golang.org/x/sync v0.14.0
)

require (
	github.com/davecgh/go-spew v1.1.2-0.20180830191138-d8f796af33cc // indirect
	github.com/decred/dcrd/dcrec/secp256k1/v4 v4.3.0 // indirect
	github.com/holiman/uint256 v1.3.2 // indirect
	github.com/pmezard/go-difflib v1.0.1-0.20181226105442-5d4384ee4fb2 // indirect
	golang.org/x/crypto v0.36.0 // indirect
	golang.org/x/sys v0.36.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)

replace github.com/ethereum/go-ethereum => github.com/ethereum-optimism/op-geth v1.101604.0-synctest.0
