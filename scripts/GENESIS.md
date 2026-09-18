# Devnet genesis exports

`Genesis.s.sol` exports L1 deployments and L2 predeploy allocations offline using
`SystemDeploy` and `L2Genesis`. It is intended for development networks with known
funded accounts. The caller assembles execution genesis and consensus configs.

Build with the default Foundry profile, then run from the repository root:

```sh
forge build --skip '/**/test/**'
mkdir -p deployments/genesis/l1-preview deployments/genesis/l2 deployments/genesis/l1-final
forge script scripts/Genesis.s.sol:BaseL1Genesis \
  --sig 'generate(string,string)' deployments/genesis/input.json deployments/genesis/l1-preview \
  --offline --disable-code-size-limit --chain 1337 --block-timestamp 0 \
  --sender 0x0000000000000000000000000000000000000001
forge script scripts/Genesis.s.sol:BaseL2Genesis \
  --sig 'generate(string,string)' deployments/genesis/input.json deployments/genesis/l2 \
  --offline --disable-code-size-limit --chain 8453 --block-timestamp 0 \
  --sender 0x0000000000000000000000000000000000000001
```

`input.json` uses the `DeployConfig` schema plus `salt` (bytes32), `addressesPath`
(the L1 preview's `addresses.json`), and `fork` (the L2 genesis fork enum).
Paths must lie within Foundry's permitted directories; use absolute paths for
`addressesPath`. Set chain IDs and role addresses in the input to match the devnet.
Initialization runs at timestamp zero so the imported protocol schedule can meet
its notice period at the actual genesis timestamp.

Each call writes `alloc.json` (Foundry's account-state dump) and `cleanup.json`
(`{"helpers": [address, ...]}`). Remove these helper accounts and zero storage
slots before assembling genesis. L1 also writes `addresses.json`, keyed by
contract name, for wiring the L2 bridges and rollup config.

After computing the final L2 output root, replace `multiproofGenesisOutputRoot` in
`input.json` and repeat the L1 invocation into `deployments/genesis/l1-final`.
Keep all other inputs fixed and require the preview and final address maps to
match. Use the final L1 allocations for genesis. The scripts never broadcast
transactions or require an RPC endpoint.
