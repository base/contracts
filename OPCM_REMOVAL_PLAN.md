# OP Contracts Manager Removal Plan

## Goal

Remove the OP Contracts Manager concept from the codebase, including the deployed
`OPContractsManager` and `OPContractsManagerStandardValidator` contracts, while
keeping a simple script-level API for deploying and upgrading a full system.

The target shape is:

- no deployed OP Contracts Manager contracts or validator contracts;
- a script under `scripts/deploy/` with one public `deploy` function for a full
  system deployment and one public `upgrade` function for a full system upgrade;
- reusable deploy and upgrade data types that are not nested under an OPCM
  interface;
- tests that validate deployment and upgrade results directly, replacing the
  useful checks from `OPContractsManagerStandardValidator`.

## Current Coupling Summary

The OPCM concept is not isolated to one contract. It currently appears in:

- production contracts: `src/L1/OPContractsManager.sol` and
  `src/L1/OPContractsManagerStandardValidator.sol`;
- interfaces: `interfaces/L1/IOPContractsManager.sol` and
  `interfaces/L1/IOPContractsManagerStandardValidator.sol`;
- deploy scripts: `Deploy.s.sol`, `DeployImplementations.s.sol`,
  `DeployOPChain.s.sol`, `ChainAssertions.sol`, and fork setup scripts;
- tests: `test/L1/OPContractsManager.t.sol`,
  `test/L1/OPContractsManagerStandardValidator.t.sol`, and `test/opcm/*`;
- generated metadata: ABI snapshots, storage layout snapshots, semver locks, and
  optimizer overrides in `foundry.toml`;
- CI/check tooling: `scripts/checks/opcm-upgrade-checks`, justfile targets, and
  test validation exclusions.

The deploy behavior to preserve lives mostly in `OPContractsManagerDeployer`.
The upgrade behavior to preserve lives mostly in `OPContractsManagerUpgrader`.
The validation behavior to preserve lives mostly in
`OPContractsManagerStandardValidator`.

## Step 1: Extract OPCM-Owned Types Into Script Types

Move the shared structs out of `IOPContractsManager` into a neutral location,
preferably `scripts/libraries/Types.sol` or a new
`scripts/deploy/DeployTypes.sol`.

Types to move or recreate:

- `Roles`
- `DeployInput`
- `DeployOutput`
- `Implementations`
- `OpChainConfig`
- `UpdatePrestateInput`, `AddGameInput`, and `AddGameOutput` only if the new
  script keeps game-type add or prestate update flows

Also move the helper behavior currently accessed through OPCM, especially
`chainIdToBatchInboxAddress`, into a script library/helper with a neutral name.

Success criteria:

- `scripts/deploy/Deploy.s.sol`, `scripts/deploy/DeployOPChain.s.sol`,
  `scripts/deploy/ChainAssertions.sol`, `test/setup/Setup.sol`, and fork setup
  code can compile against neutral deploy types instead of
  `IOPContractsManager.*`.
- No script or test needs to import `interfaces/L1/IOPContractsManager.sol` only
  for structs.
- Batch inbox derivation is still covered by deploy tests and no longer depends
  on an OPCM instance.

## Step 2: Introduce the New Full-System Deploy/Upgrade Script

Create a new script under `scripts/deploy/`, tentatively
`scripts/deploy/SystemDeploy.s.sol`, with exactly one high-level public
`deploy` function and one high-level public `upgrade` function.

Recommended API shape:

- `deploy(SystemDeploy.DeployInput memory input) returns (SystemDeploy.DeployOutput memory)`
- `upgrade(SystemDeploy.UpgradeInput memory input) returns (SystemDeploy.UpgradeOutput memory)`

The `deploy` function should orchestrate:

- optional Superchain deployment or use of an existing SuperchainConfig;
- implementation/singleton deployment;
- OP Chain proxy deployment;
- proxy initialization;
- artifact saving where the current top-level deploy flow needs it.

The `upgrade` function should orchestrate:

- SuperchainConfig upgrade when required;
- per-chain proxy upgrades;
- dispute game implementation replacement;
- artifact saving for newly introduced or newly discovered contracts.

Keep lower-level helper functions private or internal unless tests need a narrow
hook. Avoid replacing one manager-style abstraction with another public surface.

Success criteria:

- There is a single public full-system deploy entrypoint and a single public
  full-system upgrade entrypoint under `scripts/deploy/`.
- The new script does not deploy, store, or require an OPCM address.
- The script can deploy a fresh local L1 system without calling
  `OPContractsManager.deploy`.
- The script can upgrade a fork/test system without delegatecalling into
  `OPContractsManager.upgrade`.

## Step 3: Inline and Simplify Deployment Logic

Port the behavior from `OPContractsManagerDeployer.deploy` into the new script,
but simplify it for script execution.

Specific simplifications:

- remove blueprint deployment and `OPContractsManagerContractsContainer`;
- deploy `AddressManager`, `ProxyAdmin`, `Proxy`, `L1ChugSplashProxy`, and
  `ResolvedDelegateProxy` directly via existing `DeployUtils` helpers;
- pass implementation addresses through script input/output structs instead of
  storing them in a deployed manager contract;
- keep initializer encoding helpers only if they materially reduce complexity;
- remove the temporary "deploy manager, then call manager" flow from
  `Deploy.s.sol` and `DeployOPChain.s.sol`.

Behavior to preserve:

- deterministic salts and artifact names used by existing tests;
- `ProxyAdmin` ownership transfer;
- `AddressManager` ownership transfer and legacy messenger implementation setup;
- proxy type and implementation name setup for legacy proxies;
- initialization order: SystemConfig before OptimismPortal and ETHLockbox;
- Permissioned Cannon game registration;
- final deploy output fields and artifact saves.

Success criteria:

- Fresh deploy tests assert the same core contract set as before:
  `ProxyAdmin`, `AddressManager`, all L1 bridge/messenger/system proxies,
  `OptimismPortal`, `ETHLockbox`, `DisputeGameFactory`,
  `AnchorStateRegistry`, permissioned game, and DelayedWETH proxies.
- `DeployImplementations` no longer deploys OPCM contracts, blueprints, or
  validator contracts.
- `Deploy.s.sol` no longer saves an `OPContractsManager` artifact.
- `DeployOPChain.s.sol` no longer accepts an `opcm` input.

## Step 4: Inline and Simplify Upgrade Logic

Port the behavior from `OPContractsManagerUpgrader` into the new script's single
`upgrade` function.

Behavior to preserve:

- upgrade SuperchainConfig first when its implementation version is older;
- reject or skip already-up-to-date SuperchainConfig in a documented way;
- upgrade SystemConfig, OptimismPortal, AnchorStateRegistry,
  OptimismMintableERC20Factory, DisputeGameFactory, L1CrossDomainMessenger,
  L1StandardBridge, and L1ERC721Bridge;
- update Permissioned Cannon dispute game implementation and game args;
- update Cannon and Cannon Kona permissionless game implementations when they
  exist;
- preserve existing prestate fallback behavior when upgrade input prestates are
  zero;
- keep the upgrade gas regression test or replace it with an equivalent script
  test.

Decide during implementation whether `addGameType` and `updatePrestate` should
survive as separate script utilities. They are not part of the requested single
deploy/single upgrade API, so the default should be to delete them unless an
active test or production flow still requires them.

Success criteria:

- Fork upgrade setup no longer deploys a new OPCM and no longer delegatecalls
  OPCM functions.
- Upgrade tests exercise the script `upgrade` function directly.
- The upgraded contract implementations and dispute game args match the previous
  OPCM upgrade behavior.
- The upgrade path still handles SuperchainConfig already being up to date.
- If game-type add/prestate update flows are removed, all references and tests
  for those flows are also removed or explicitly replaced.

## Step 5: Replace Standard Validator With Test-Only Assertions

Move the useful validation checks from
`OPContractsManagerStandardValidator` into test-only code.

Recommended target:

- a new test helper such as `test/setup/StandardSystemAssertions.sol`; or
- an expanded `scripts/deploy/ChainAssertions.sol` if the checks remain useful
  for scripts as well as tests.

Checks to preserve:

- SuperchainConfig paused state;
- ProxyAdmin owner and proxy admin wiring;
- SystemConfig implementation/version/config values;
- bridge, messenger, portal, lockbox, factory, and ERC721 bridge wiring;
- DisputeGameFactory ownership and implementation wiring;
- Permissioned Cannon, Cannon, and Cannon Kona game args;
- DelayedWETH, AnchorStateRegistry, MIPS, and PreimageOracle invariants;
- override handling used by fork upgrade tests, but as test helper input rather
  than validator contract state.

Success criteria:

- No deployed validator contract or validator interface remains.
- Deploy and upgrade tests still validate the complete post-state previously
  validated by `OPContractsManagerStandardValidator`.
- Validator tests are either converted into tests for the new assertion helper
  or removed because their coverage is duplicated by deploy/upgrade tests.
- Failure messages remain specific enough to identify the broken subsystem.

## Step 6: Update Test Harnesses and Fork Setup

Update test setup to consume artifacts and script outputs directly rather than
through an OPCM address.

Areas to change:

- `test/setup/Setup.sol`: remove `opcm` state and artifact registration.
- `test/setup/ForkLive.s.sol`: replace live OPCM entries with script-based
  implementation deployment and script-based upgrade execution.
- `test/opcm/DeployOPChain.t.sol`: rename/move to a neutral deploy-script test
  and remove `opcm` input assumptions.
- `test/opcm/DeployImplementations.t.sol`: remove assertions for OPCM,
  container, deployer, upgrader, game type adder, and validator outputs.
- `test/L1/OPContractsManager.t.sol`: convert only the still-relevant deploy and
  upgrade tests to script tests; delete contract-specific tests.
- `test/L1/OPContractsManagerStandardValidator.t.sol`: convert useful cases into
  assertion-helper tests or delete after coverage is represented elsewhere.

Success criteria:

- No test imports `IOPContractsManager` or instantiates
  `OPContractsManager*` contracts.
- The `test/opcm` directory is gone or renamed to a neutral deploy/upgrade
  directory.
- Local deploy setup, L2 setup, and fork setup all pass without an
  `OPContractsManager` artifact.
- Existing fork upgrade test intent remains covered.

## Step 7: Remove Production Contracts, Interfaces, and Generated Metadata

Delete the production OPCM and validator surfaces after all consumers have moved.

Files and metadata to remove or update:

- `src/L1/OPContractsManager.sol`
- `src/L1/OPContractsManagerStandardValidator.sol`
- `interfaces/L1/IOPContractsManager.sol`
- `interfaces/L1/IOPContractsManagerStandardValidator.sol`
- `snapshots/abi/OPContractsManager*.json`
- `snapshots/storageLayout/OPContractsManager*.json`
- semver lock entries for OPCM and validator;
- optimizer overrides in `foundry.toml`;
- `test/vendor/Initializable.t.sol` exclusions;
- `scripts/checks/opcm-upgrade-checks` and justfile targets, unless replaced
  with a script-aware upgrade coverage check.

Success criteria:

- `forge build` succeeds after the files are removed.
- ABI and storage layout snapshots no longer include OPCM or validator artifacts.
- Semver lock and optimizer config no longer mention OPCM or validator contracts.
- Interface checks no longer expect OPCM interfaces.

## Step 8: Replace or Remove OPCM-Specific CI Checks

The existing `scripts/checks/opcm-upgrade-checks` asserts that every L1 contract
with an upgrade method is called from `OPContractsManagerUpgrader`. That check
will become obsolete.

Choose one path:

- delete the check and its justfile targets if script tests fully cover the
  upgrade matrix; or
- replace it with a new check that reads the new deploy/upgrade script artifact
  and verifies that the single `upgrade` function references every required L1
  upgrade function.

Success criteria:

- No justfile target references `opcm-upgrade-checks`.
- CI no longer tries to read `forge-artifacts/OPContractsManager.sol/*`.
- If a replacement check is added, it fails when a required contract upgrade is
  omitted from the new script.

## Step 9: Run Focused Verification

Run the smallest useful checks after each large move, then the broader suite
after cleanup.

Recommended sequence:

1. `forge build`
2. deploy script tests, including former `test/opcm` coverage
3. upgrade tests, including fork-upgrade harnesses where available
4. standard assertion tests replacing validator coverage
5. `just interfaces-check`
6. `just snapshots-check`
7. the repo's normal CI target, if one exists locally

Success criteria:

- Build passes with no OPCM contracts or interfaces.
- Fresh deploy tests pass.
- Upgrade tests pass.
- Snapshot and semver outputs are up to date.
- Interface and test-validation checks pass.

## Step 10: Final Repository-Wide Cleanup

Do a final search and remove naming residue.

Search terms:

- `OPCM`
- `opcm`
- `OPContractsManager`
- `OPContractsManagerStandardValidator`

Allowed matches should be limited to migration notes or this plan file. If the
goal is a complete concept removal, even comments and directory names should be
renamed or removed.

Success criteria:

- `rg -n "OPCM|opcm|OPContractsManager|OPContractsManagerStandardValidator" .`
  returns no code/config/test/check references outside this plan or explicitly
  retained historical documentation.
- No artifact is saved under the name `OPContractsManager`.
- No deployed bytecode artifact for OPCM or the validator is generated.
- The codebase has one clear script-level deploy path and one clear script-level
  upgrade path for the full system.
