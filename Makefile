# Load environment variables from .env file if it exists
ifneq (,$(wildcard .env))
    include .env
    export
endif

# Default target: show help
.DEFAULT_GOAL := help

##
# -----------------------------------------------------------------------------
# Solidity Setup / Testing / DevOps
# -----------------------------------------------------------------------------
##

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: install-foundry
install-foundry: ## Install Foundry toolchain (forge, cast, anvil)
	@echo "[+] Installing Foundry..."
	curl -L https://foundry.paradigm.xyz | bash
	@echo "[!] Please verify your PATH or restart your shell to use 'forge'."

.PHONY: deps
deps: clean-lib checkout-op-commit ## Clean and re-install all Solidity dependencies
	@echo "[+] Installing standard dependencies..."
	forge install --no-git github.com/foundry-rs/forge-std \
		github.com/OpenZeppelin/openzeppelin-contracts@v4.9.3 \
		github.com/OpenZeppelin/openzeppelin-contracts-upgradeable@v4.7.3 \
		github.com/rari-capital/solmate@8f9b23f8838670afda0fd8983f2c41e8037ae6bc \
		github.com/Vectorized/solady@862a0afd3e66917f50e987e91886b9b90c4018a1 \
		github.com/safe-global/safe-smart-account@v1.4.1-3

.PHONY: test
test: ## Run tests (Warning: --ffi enabled, check dependencies!)
	@echo "[!] Running tests with FFI enabled. Ensure you trust all dependencies."
	forge test --ffi -vvv

.PHONY: snapshot
snapshot: ## Generate a gas snapshot for contract methods
	forge snapshot --ffi

.PHONY: fmt
fmt: ## Format Solidity code
	forge fmt

.PHONY: clean-lib
clean-lib: ## Remove the lib directory
	@echo "[+] Cleaning lib directory..."
	rm -rf lib

.PHONY: checkout-op-commit
checkout-op-commit: ## Sparse checkout specific Optimism commit defined in .env
	@echo "[+] Checking out Optimism commit: $(OP_COMMIT)"
	@[ -n "$(OP_COMMIT)" ] || (echo "Error: OP_COMMIT must be set in .env" && exit 1)
	@rm -rf lib/optimism
	@mkdir -p lib/optimism
	@cd lib/optimism; \
	git init; \
	git remote add origin https://github.com/ethereum-optimism/optimism.git; \
	git fetch --depth=1 origin $(OP_COMMIT); \
	git reset --hard FETCH_HEAD

.PHONY: bindings
bindings: ## Generate Go bindings from Solidity ABIs
	@echo "[+] Installing abigen (v1.15.8)..."
	go install github.com/ethereum/go-ethereum/cmd/abigen@v1.15.8
	@echo "[+] Building contracts..."
	forge build
	@echo "[+] Generating Go bindings..."
	mkdir -p bindings
	abigen --abi out/BalanceTracker.sol/BalanceTracker.abi.json --pkg bindings --type BalanceTracker --out bindings/balance_tracker.go
	abigen --abi out/FeeDisburser.sol/FeeDisburser.abi.json --pkg bindings --type FeeDisburser --out bindings/fee_disburser.go
	@echo "[+] Tidying Go modules..."
	cd bindings && go mod tidy
