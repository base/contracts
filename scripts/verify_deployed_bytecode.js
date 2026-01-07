```js
/**
 * Compares deployed contract bytecode with local artifact bytecode.
 *
 * Usage:
 *   RPC_URL=https://... ADDRESS=0x... ARTIFACT=./artifacts/MyContract.json node scripts/verify_deployed_bytecode.js
 */

const fs = require("fs");

async function main() {
  const rpcUrl = process.env.RPC_URL;
  const address = process.env.ADDRESS;
  const artifactPath = process.env.ARTIFACT;

  if (!rpcUrl || !address || !artifactPath) {
    console.error("Missing env vars. Set RPC_URL, ADDRESS, ARTIFACT.");
    process.exit(1);
  }

  const artifact = JSON.parse(fs.readFileSync(artifactPath, "utf8"));
  const localBytecode = (artifact.deployedBytecode || artifact.bytecode || "").toLowerCase();

  if (!localBytecode || localBytecode === "0x") {
    console.error("Artifact does not include bytecode fields (deployedBytecode/bytecode).");
    process.exit(1);
  }

  const payload = {
    jsonrpc: "2.0",
    id: 1,
    method: "eth_getCode",
    params: [address, "latest"],
  };

  const res = await fetch(rpcUrl, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

  const json = await res.json();
  const chainBytecode = (json.result || "").toLowerCase();

  if (!chainBytecode || chainBytecode === "0x") {
    console.error("No bytecode found at address (is it a contract?).");
    process.exit(1);
  }

  const same = chainBytecode === localBytecode;
  console.log(`Local bytecode: ${localBytecode.slice(0, 18)}... (${localBytecode.length} chars)`);
  console.log(`Chain bytecode: ${chainBytecode.slice(0, 18)}... (${chainBytecode.length} chars)`);
  console.log(same ? "Match: ✅ bytecode identical" : "Mismatch: ❌ bytecode differs");
  process.exit(same ? 0 : 2);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
