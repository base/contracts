#!/usr/bin/env bash
# Production wrapper for SystemDeploy.s.sol — deploys (or upgrades)
# Required env:
#   DEPLOY_ETH_RPC_URL   L1 RPC endpoint (e.g. Hoodi)
#   DEPLOY_CONFIG_PATH   path to deploy-config/<network>.json (read by Config.deployConfigPath())
# Signing (choose one):
#   DEPLOY_PRIVATE_KEY                 raw private key, OR
#   DEPLOY_LEDGER=1 + DEPLOY_HD_PATH   sign with a Ledger
# Optional:
#   SALT_MIXER           CREATE2 salt-mixer override (default "salt mixer").
#   DEPLOY_VERIFY=1      run contract verification
#   VERIFIER_URL / ETHERSCAN_API_KEY   verification params
#

set -euo pipefail

: "${DEPLOY_ETH_RPC_URL:?set DEPLOY_ETH_RPC_URL}"
: "${DEPLOY_CONFIG_PATH:?set DEPLOY_CONFIG_PATH (e.g. deploy-config/zeronet.json)}"
export DEPLOY_CONFIG_PATH
export SALT_MIXER="${SALT_MIXER:-salt mixer}"

SCRIPT="scripts/deploy/SystemDeploy.s.sol:SystemDeploy"

signer_args=()
if [ -n "${DEPLOY_LEDGER:-}" ]; then
  signer_args=(--ledger --hd-paths "${DEPLOY_HD_PATH:?set DEPLOY_HD_PATH when DEPLOY_LEDGER=1}")
elif [ -n "${DEPLOY_PRIVATE_KEY:-}" ]; then
  signer_args=(--private-key "$DEPLOY_PRIVATE_KEY")
else
  echo "error: set DEPLOY_PRIVATE_KEY, or DEPLOY_LEDGER=1 with DEPLOY_HD_PATH" >&2
  exit 1
fi

verify_args=()
if [ "${DEPLOY_VERIFY:-}" = "1" ]; then
  verify_args=(--verify)
  [ -n "${VERIFIER_URL:-}" ] && verify_args+=(--verifier custom --verifier-url "$VERIFIER_URL")
  [ -n "${ETHERSCAN_API_KEY:-}" ] && verify_args+=(--verifier-api-key "$ETHERSCAN_API_KEY")
fi

echo "> SystemDeploy  (config=$DEPLOY_CONFIG_PATH  saltMixer=$SALT_MIXER  rpc=$DEPLOY_ETH_RPC_URL)"
forge script -vvvv "$SCRIPT" \
  --rpc-url "$DEPLOY_ETH_RPC_URL" \
  --broadcast \
  ${signer_args[@]+"${signer_args[@]}"} \
  ${verify_args[@]+"${verify_args[@]}"}
