use alloy_sol_types::sol;

sol!(
    #[sol(rpc, abi)]
    FlashblockIndex,
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/artifacts/FlashblockIndex.json"
    )
);
