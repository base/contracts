-include .env

##
# Solidity Setup / Testing
##
.PHONY: install-foundry
install-foundry:
	curl -L https://foundry.paradigm.xyz | bash
	~/.foundry/bin/foundryup

.PHONY: deps
deps: clean-lib
	forge install --no-git \
		github.com/foundry-rs/forge-std@6853b9ec7df5dc0c213b05ae67785ad4f4baa0ea \
		github.com/runtimeverification/kontrol-cheatcodes@2c48ae1ab44228c199dca29414c0b4b18a3434e6 \
		github.com/ethereum-optimism/lib-keccak@3b1e7bbb4cc23e9228097cfebe42aedaf3b8f2b9 \
		github.com/OpenZeppelin/openzeppelin-contracts@ecd2ca2cd7cac116f7a37d0e474bbb3d7d5e1c4d \
		github.com/OpenZeppelin/openzeppelin-contracts-upgradeable@0a2cb9a445c365870ed7a8ab461b12acf3e27d63 \
		github.com/transmissions11/solmate@8f9b23f8838670afda0fd8983f2c41e8037ae6bc \
		github.com/safe-global/safe-contracts@bf943f80fec5ac647159d26161446ac5d716a294 \
		github.com/Vectorized/solady@502cc1ea718e6fa73b380635ee0868b0740595f0 \
		github.com/base/nitro-validator@0f006d2075637dd9640e530c4a7065f5c8bb2132 \
		github.com/base/op-enclave@a2d5398f04c3a8e4df929d58ee638ba4a037bfec
	forge install --no-git \
		github.com/ethereum-optimism/superchain-registry@84bce73573f130008d84bae6e924163bab589a11
	@# openzeppelin-contracts-v5 and solady-v0.0.245 use the same orgs as their
	@# counterparts but are pinned to different commits, so we clone them manually.
	git clone --no-checkout https://github.com/OpenZeppelin/openzeppelin-contracts.git lib/openzeppelin-contracts-v5 && \
		cd lib/openzeppelin-contracts-v5 && git checkout dbb6104ce834628e473d2173bbc9d47f81a9eec3
	git clone --no-checkout https://github.com/Vectorized/solady.git lib/solady-v0.0.245 && \
		cd lib/solady-v0.0.245 && git checkout e0ef35adb0ccd1032794731a995cb599bba7b537

.PHONY: test
test:
	forge test --ffi -vvv

.PHONY: clean-lib
clean-lib:
	rm -rf lib

.PHONY: bindings
bindings:
	go install github.com/ethereum/go-ethereum/cmd/abigen@v1.15.8
	forge build
	mkdir -p bindings
	abigen --abi out/BalanceTracker.sol/BalanceTracker.abi.json --pkg bindings --type BalanceTracker --out bindings/balance_tracker.go
	abigen --abi out/FeeDisburser.sol/FeeDisburser.abi.json --pkg bindings --type FeeDisburser --out bindings/fee_disburser.go
	cd bindings && go mod tidy
