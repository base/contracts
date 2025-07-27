# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains Solidity smart contracts and deployment scripts for Base, an Ethereum L2 using Optimism's bedrock architecture. The codebase includes revenue sharing contracts, smart escrow functionality, fee vault fixes, multisig deployment utilities, and challenger contracts.

## Environment Setup

Before working with this codebase:
1. Ensure you have an `.env` file with `OP_COMMIT` variable set (required for dependency management)
2. Run `make deps` to install all dependencies including Optimism contracts at the specified commit

## Common Commands

### Development Commands
- `make deps` - Install all dependencies (foundry, OpenZeppelin, Optimism contracts)
- `make test` - Run all tests with verbose output and FFI enabled
- `forge build` - Build all contracts
- `make bindings` - Generate Go bindings for BalanceTracker and FeeDisburser contracts

### Setup Commands
- `make install-foundry` - Install Foundry toolkit (if not already installed)
- `make clean-lib` - Clean lib directory

## Architecture Overview

### Core Contract Categories

1. **Revenue Share System** (`src/revenue-share/`)
   - `BalanceTracker.sol` - Manages funding of system addresses and profit distribution
   - `FeeDisburser.sol` - Handles fee disbursement logic

2. **Smart Escrow** (`src/smart-escrow/`)
   - `SmartEscrow.sol` - Vesting contract for OP token payments with termination capabilities

3. **Fee Vault Fixes** (`src/fee-vault-fixes/`)
   - `FeeVault.sol` - Enhanced fee vault implementation

4. **Challenger System** (`src/`)
   - `Challenger1of2.sol` - Deprecated 1-of-2 challenger contract (pre-fault proofs)

5. **Multisig Utilities** (`script/universal/`)
   - Complete multisig deployment and management system with nested and double-nested capabilities
   - Includes signature handling, simulation, and deployment scripts

### Test Structure
- Tests mirror the `src/` directory structure
- Uses Foundry's testing framework with FFI enabled
- Common test utilities in `test/CommonTest.t.sol`
- Mock contracts available in `test/*/mocks/` directories

### Dependencies
- Built on Foundry with Solidity 0.8.15
- Uses OpenZeppelin contracts v4.9.3 for core functionality
- Integrates with Optimism bedrock contracts (version specified by OP_COMMIT)
- Includes Solmate and Solady for additional utilities

### Contract Deployment
- L1 and L2 deployment scripts in `script/deploy/`
- Test deployment scripts for both layers
- Universal deployment utilities for cross-chain operations

## Important Notes

- This repository primarily extends Optimism's bedrock contracts rather than replacing them
- The OP_COMMIT environment variable controls which version of Optimism contracts to use
- Contracts use 999999 optimizer runs for maximum gas efficiency
- All contracts target Solidity 0.8.15 for consistency