include .env

##
# Solidity Setup / Testing
##
.PHONY: install-foundry
install-foundry:
	curl -L https://foundry.paradigm.xyz | bash
	~/.foundry/bin/foundryup

.PHONY: deps
deps: clean-lib checkout-op-commit
	forge install --no-git github.com/foundry-rs/forge-std \
		github.com/OpenZeppelin/openzeppelin-contracts@v4.9.3 \
		github.com/OpenZeppelin/openzeppelin-contracts-upgradeable@v4.7.3 \
		github.com/rari-capital/solmate@8f9b23f8838670afda0fd8983f2c41e8037ae6bc \
		github.com/Vectorized/solady@862a0afd3e66917f50e987e91886b9b90c4018a1 \
		github.com/safe-global/safe-smart-account@v1.4.1-3

.PHONY: test
test:
	forge test --ffi -vvv

.PHONY: clean-lib
clean-lib:
	rm -rf lib

.PHONY: checkout-op-commit
checkout-op-commit:
	[ -n "$(OP_COMMIT)" ] || (echo "OP_COMMIT must be set in .env" && exit 1)
	rm -rf lib/optimism
	mkdir -p lib/optimism
	cd lib/optimism; \
	git init; \
	git remote add origin https://github.com/ethereum-optimism/optimism.git; \
	git fetch --depth=1 origin $(OP_COMMIT); \
	git reset --hard FETCH_HEAD

.PHONY: bindings
bindings:
	go install github.com/ethereum/go-ethereum/cmd/abigen@v1.15.8
	forge build
	mkdir -p bindings
	abigen --abi out/BalanceTracker.sol/BalanceTracker.abi.json --pkg bindings --type BalanceTracker --out bindings/balance_tracker.go
	abigen --abi out/FeeDisburser.sol/FeeDisburser.abi.json --pkg bindings --type FeeDisburser --out bindings/fee_disburser.go
	cd bindings && go mod tidy
