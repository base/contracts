########################################################
#                        INSTALL                       #
########################################################

# Installs Foundry.
install-foundry:
  curl -L https://foundry.paradigm.xyz | bash
  ~/.foundry/bin/foundryup

# Installs pinned dependencies.
deps: clean-lib
  forge install --no-git \
    github.com/foundry-rs/forge-std@6853b9ec7df5dc0c213b05ae67785ad4f4baa0ea \
    github.com/runtimeverification/kontrol-cheatcodes@2c48ae1ab44228c199dca29414c0b4b18a3434e6 \
    github.com/ethereum-optimism/lib-keccak@3b1e7bbb4cc23e9228097cfebe42aedaf3b8f2b9 \
    github.com/OpenZeppelin/openzeppelin-contracts@ecd2ca2cd7cac116f7a37d0e474bbb3d7d5e1c4d \
    github.com/OpenZeppelin/openzeppelin-contracts-upgradeable@0a2cb9a445c365870ed7a8ab461b12acf3e27d63 \
    github.com/transmissions11/solmate@8f9b23f8838670afda0fd8983f2c41e8037ae6bc \
    github.com/Vectorized/solady@502cc1ea718e6fa73b380635ee0868b0740595f0 \
    github.com/risc0/risc0-ethereum@a78ac4a52fe9cfa14120c3b496430f0d42e1d8d3
  @# openzeppelin-contracts-v5 and solady-v0.0.245 use the same orgs as their
  @# counterparts but are pinned to different commits, so we clone them manually.
  git clone --no-checkout https://github.com/OpenZeppelin/openzeppelin-contracts.git lib/openzeppelin-contracts-v5 && \
    cd lib/openzeppelin-contracts-v5 && git checkout dbb6104ce834628e473d2173bbc9d47f81a9eec3
  git clone --no-checkout https://github.com/Vectorized/solady.git lib/solady-v0.0.245 && \
    cd lib/solady-v0.0.245 && git checkout 8583a6e386b897f3db142a541f86d5953eccd835

# Cleans dependency installs.
clean-lib:
  rm -rf lib

########################################################
#                         BUILD                        #
########################################################

# Core forge build command.

forge-build *ARGS:
  forge build {{ARGS}}

  @# Forge build compiles only the src/ graph; the scripts/ graph is compiled by `forge script`.
  @# On the first invocation, `forge script` may compile a small set of dependencies.
  @# To avoid paying this cost in every CI test, we pre‑warm the script cache once here.
  @#
  @# Notes:
  @# - A single `forge script <any script> --skip-simulation` is sufficient to compile the script
  @#   dependency graph into the cache. Subsequent `forge script` runs (including other scripts)
  @#   will typically print "No files changed, compilation skipped".
  @# - We pass `--skip "/**/test/**"` to keep tests out of the graph and suppress warnings.
  @# - Providing a signature/args is not required for compilation; compilation happens before
  @#   argument validation and before execution. We still use `--skip-simulation` to guarantee
  @#   nothing runs in any case.
  @forge script "scripts/deploy/SystemDeploy.s.sol:SystemDeploy" \
    --skip "/**/test/**" \
    --sig "idonotexist()" \
    --skip-simulation \
    2>/dev/null || true

# Developer build command (faster).
forge-build-dev *ARGS:
  FOUNDRY_PROFILE=lite forge build {{ARGS}}

# Builds source contracts only.
build-source:
  forge build --skip "/**/test/**" --skip "/**/scripts/**"

# Builds source contracts and scripts, skipping tests.
build-no-tests:
  forge build --skip "/**/test/**"

# Builds the contracts.
build *ARGS: lint-fix-no-fail
  just forge-build {{ARGS}}

# Builds the contracts (developer mode).
build-dev *ARGS: lint-fix-no-fail
  just forge-build-dev {{ARGS}}

# Builds the go-ffi tool for contract tests.
build-go-ffi:
  cd ./scripts/go-ffi && go build -buildvcs=false

# Cleans build artifacts and deployments.
clean:
  rm -rf ./artifacts ./forge-artifacts ./cache ./scripts/go-ffi/go-ffi ./deployments/hardhat/*


########################################################
#                         TEST                         #
########################################################

# Runs standard contract tests.
test *ARGS: build-go-ffi
  forge test {{ARGS}}

# Runs standard contract tests (developer mode).
test-dev *ARGS: build-go-ffi
  FOUNDRY_PROFILE=lite forge test {{ARGS}}

# Default block number for the forked upgrade path.

export sepoliaBlockNumber := "9366100"
export mainnetBlockNumber := "23530400"

export pinnedBlockNumber := if env_var_or_default("FORK_BASE_CHAIN", "") == "mainnet" {
    mainnetBlockNumber
} else if env_var_or_default("FORK_BASE_CHAIN", "") == "sepolia" {
    sepoliaBlockNumber
} else {
    mainnetBlockNumber
}

print-pinned-block-number:
  echo $pinnedBlockNumber

# Prepares the environment for upgrade path variant of contract tests and coverage.
# Env Vars:
# - ETH_RPC_URL must be set to a production (Sepolia or Mainnet) RPC URL.
# - FORK_BLOCK_NUMBER can be set in the env, or else will fallback to the default block number.
#   Reusing the default block number greatly speeds up the test execution time by caching the
#   rpc call responses in ~/.foundry/cache/rpc. The default block will need to be updated
#   when the L1 chain is upgraded.
prepare-upgrade-env *ARGS : build-go-ffi
  #!/bin/bash
  export FORK_BLOCK_NUMBER=$pinnedBlockNumber
  echo "Running upgrade tests at block $FORK_BLOCK_NUMBER"
  export FORK_RPC_URL=$ETH_RPC_URL
  export FORK_RETRIES=10
  export FORK_BACKOFF=1000
  export FORK_TEST=true
  {{ARGS}} \
  --match-path "test/{L1,dispute,cannon}/**"

# Runs upgrade path variant of contract tests.
test-upgrade *ARGS:
  just prepare-upgrade-env "forge test {{ARGS}}"

test-upgrade-rerun *ARGS: build-go-ffi
  just test-upgrade {{ARGS}} --rerun -vvvv

# Starts a local anvil node with a mainnet fork and sends it to the background
# Requires ETH_RPC_URL to be set to a production (Sepolia or Mainnet) RPC URL.
anvil-fork:
  anvil --fork-url $ETH_RPC_URL

# Use anvil-fork in a separate terminal before running this command.
# Helpful for debugging.
test-upgrade-against-anvil *ARGS: build-go-ffi
  #!/bin/bash
  echo "Running upgrade tests at block $pinnedBlockNumber"
  export FORK_BLOCK_NUMBER=$pinnedBlockNumber
  export FORK_RPC_URL=http://127.0.0.1:8545
  export FORK_TEST=true
  forge test {{ARGS}} \
  --match-path "test/{L1,dispute,cannon}/**"

# Runs standard contract tests with rerun flag.
test-rerun: build-go-ffi
  forge test --rerun -vvv

# Runs standard contract tests with rerun flag (developer mode).
test-dev-rerun: build-go-ffi
  FOUNDRY_PROFILE=lite forge test --rerun -vvv

# Run Kontrol tests and build all dependencies.
test-kontrol: build-go-ffi build kontrol-summary-full test-kontrol-no-build

# Run Kontrol tests without dependencies.
test-kontrol-no-build:
  ./test/kontrol/scripts/run-kontrol.sh script

# Runs contract coverage.
coverage: build-go-ffi
  forge coverage

# Runs contract coverage with lcov.
coverage-lcov *ARGS: build-go-ffi
  forge coverage {{ARGS}} --report lcov --report-file lcov.info

# Runs upgrade path variant of contract coverage tests.
coverage-upgrade *ARGS:
  just prepare-upgrade-env "forge coverage {{ARGS}}"

# Runs contract coverage with lcov.
coverage-lcov-upgrade *ARGS: build-go-ffi
  just coverage-upgrade {{ARGS}} --report lcov --report-file lcov-upgrade.info

# Runs coverage-lcov and coverage-lcov-upgrade and merges their output files info one file
coverage-lcov-all *ARGS:
  just coverage-lcov {{ARGS}} && \
  just coverage-lcov-upgrade {{ARGS}} && \
  lcov -a lcov.info -a lcov-upgrade.info -o lcov-all.info

########################################################
#                        DEPLOY                        #
########################################################

# Generates the L2 genesis state.
genesis:
  forge script scripts/L2Genesis.s.sol:L2Genesis --sig 'runWithStateDump()'

# Deploys the contracts.
deploy:
  ./scripts/deploy/deploy.sh


########################################################
#                       SNAPSHOTS                      #
########################################################

# Generates default Kontrol summary.
kontrol-summary:
  ./test/kontrol/scripts/make-summary-deployment.sh

# Generates fault proofs Kontrol summary.
kontrol-summary-fp:
  KONTROL_FP_DEPLOYMENT=true ./test/kontrol/scripts/make-summary-deployment.sh

# Generates all Kontrol summaries (default and FP).
kontrol-summary-full: kontrol-summary kontrol-summary-fp

# Generates ABI snapshots for contracts.
snapshots-abi-storage-no-build:
  go run ./scripts/autogen/generate-snapshots .

# Generates ABI snapshots for contracts.
snapshots-abi-storage: build-source snapshots-abi-storage-no-build

# Updates the snapshots/semver-lock.json file without building contracts.
semver-lock-no-build:
  go run scripts/autogen/generate-semver-lock/main.go

# Updates the snapshots/semver-lock.json file.
# Uses --force to ensure artifacts reflect the current profile regardless of cache state.
semver-lock:
  forge build --force --skip "/**/test/**" --skip "/**/scripts/**"
  just semver-lock-no-build

# Generates core snapshots without building contracts.
snapshots-no-build: snapshots-abi-storage-no-build semver-lock-no-build

# Builds contracts and then generates core snapshots.
# Uses `semver-lock` (which includes --force) instead of `semver-lock-no-build`
# to ensure artifacts reflect the current profile regardless of cache state.
snapshots: snapshots-abi-storage semver-lock


########################################################
#                        CHECKS                        #
########################################################

# Checks if the snapshots are up to date without building.
snapshots-check-no-build: snapshots-no-build

# Checks if the snapshots are up to date.
snapshots-check: build snapshots-check-no-build

# Checks interface correctness without building.
interfaces-check-no-build:
  go run ./scripts/checks/interfaces

# Checks that all interfaces are appropriately named and accurately reflect the corresponding
# contract that they're meant to represent. We run "clean" before building because leftover
# artifacts can cause the script to detect issues incorrectly.
interfaces-check: clean build interfaces-check-no-build

# Checks that all upgrade/initialize functions have proper reinitializer modifiers.
reinitializer-check: build-source reinitializer-check-no-build

# Checks that all upgrade/initialize functions have proper reinitializer modifiers.
# Does not build contracts.
reinitializer-check-no-build:
  go run ./scripts/checks/reinitializer

# Checks that the size of the contracts is within the limit.
size-check:
  forge build --sizes --skip "/**/test/**" --skip "/**/scripts/**"

# Checks that any contracts with a modified semver lock also have a modified semver version.
# Does not build contracts.
semver-diff-check-no-build:
  ./scripts/checks/check-semver-diff.sh

# Checks that any contracts with a modified semver lock also have a modified semver version.
semver-diff-check: build semver-diff-check-no-build

# Checks that the semgrep tests are valid.
semgrep-test-validity-check:
  forge fmt ../../.semgrep/tests/sol-rules.t.sol --check

# Validates forge test conventions and structure. Does not build contracts.
lint-forge-tests-check-no-build:
  go run ./scripts/checks/test-validation

# Validates forge test conventions and structure.
lint-forge-tests-check: build lint-forge-tests-check-no-build

# Checks that contracts are properly linted.
lint-check:
  forge fmt --check

# Checks for unused imports in Solidity contracts. Does not build contracts.
unused-imports-check-no-build:
  go run ./scripts/checks/unused-imports

# Checks for unused imports in Solidity contracts.
unused-imports-check: build unused-imports-check-no-build

# Checks that the semver of contracts are valid. Does not build contracts.
valid-semver-check-no-build:
  go run ./scripts/checks/valid-semver-check/main.go

# Checks that the semver of contracts are valid.
valid-semver-check: build valid-semver-check-no-build

# Checks that the deploy configs are valid.
validate-deploy-configs:
  ./scripts/checks/check-deploy-configs.sh

# Checks that spacer variables are correctly inserted without building.
validate-spacers-no-build:
  go run ./scripts/checks/spacers

# Checks that spacer variables are correctly inserted.
validate-spacers: build validate-spacers-no-build

# Checks that the Kontrol summary dummy files have not been modified.
# If you have changed the summary files deliberately, update the hashes in the script.
# Use `openssl dgst -sha256` to generate the hash for a file.
check-kontrol-summaries-unchanged:
  ./scripts/checks/check-kontrol-summaries-unchanged.sh

# Runs semgrep on the contracts.
semgrep:
  cd ../../ && semgrep scan --config .semgrep/rules/ ./packages/contracts-bedrock

# Runs semgrep tests.
semgrep-test:
  cd ../../ && semgrep scan --test --config .semgrep/rules/ .semgrep/tests/

# Runs all checks.
check:
  @just semgrep-test-validity-check \
  semgrep \
  lint-check \
  snapshots-check-no-build \
  unused-imports-check-no-build \
  valid-semver-check-no-build \
  semver-diff-check-no-build \
  validate-deploy-configs \
  validate-spacers-no-build \
  reinitializer-check-no-build \
  interfaces-check-no-build \
  lint-forge-tests-check-no-build

########################################################
#                      DEV TOOLS                       #
########################################################

# Cleans, builds, lints, and runs all checks.
# Alias for pre-pr.
pre-commit *ARGS:
  just pre-pr {{ARGS}}

# Cleans, builds, lints, and runs all checks.
pre-pr *ARGS:
  #!/bin/bash
  set -e
  # Optionally clean the previous build.
  # --clean is typically not needed but can force a clean build if you suspect cache issues.
  if [[ "{{ARGS}}" == *"--clean"* ]]; then
      just clean
  fi

  # Create temp directory for build cache if it doesn't exist.
  TEMP_BUILD_DIR=$(mktemp -d)
  trap '[ -d "$TEMP_BUILD_DIR" ] && rm -rf "$TEMP_BUILD_DIR"' EXIT INT

  just build-dev

  # Cache build artifacts for the dev build. We expect that developers will
  # generally be using the dev build, but a production build of the src
  # contracts is required for generating the correct snapshots. Production
  # build will overwrite the cache for the dev build, which ultimately means
  # that developers would have to wait a full minute for the dev build each
  # time they run this command. By caching the artifacts for the dev build, we
  # can save a lot of marginal time.
  cp -r artifacts "$TEMP_BUILD_DIR/"
  cp -r forge-artifacts "$TEMP_BUILD_DIR/"
  cp -r cache "$TEMP_BUILD_DIR/"

  # Clean forge-artifacts and cache so build-source starts fresh with the
  # default profile. Without this, stale lite-profile bytecode can survive
  # incremental compilation and produce incorrect semver-lock hashes.
  rm -rf forge-artifacts cache

  just lint
  just build-source
  just check

  # Restore build artifacts after running checks.
  if [ -d "$TEMP_BUILD_DIR" ]; then
    cp -r "$TEMP_BUILD_DIR/artifacts" ./
    cp -r "$TEMP_BUILD_DIR/forge-artifacts" ./
    cp -r "$TEMP_BUILD_DIR/cache" ./
  fi

twrap:
    #!/usr/bin/env sh
    if [ -z "$WRAP_DISABLED" ]; then
        tput rmam
        export WRAP_DISABLED=1
        echo "Terminal line wrapping disabled"
    else
        tput smam
        unset WRAP_DISABLED
        echo "Terminal line wrapping enabled"
    fi

# Fixes linting errors.
lint-fix:
  forge fmt

# Fixes linting errors but doesn't fail if there are syntax errors. Useful for build command
# because the output of forge fmt can sometimes be difficult to understand but if there's a syntax
# error the build will fail anyway and provide more context about what's wrong.
lint-fix-no-fail:
  forge fmt || true

# Fixes linting errors and checks that the code is correctly formatted.
lint: lint-fix lint-check


########################################################
#                         DOCS                         #
########################################################

# Generates a table of contents for the POLICY.md file.
toc:
  md_toc -p github meta/POLICY.md
