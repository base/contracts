#!/usr/bin/env bash
set -euo pipefail

# Extract the contract version, declared either as
# `string constant version = "..."` or as a `version()` function returning a string.
extract_version() {
  local file=$1
  local version

  version=$(grep -o 'string.*constant.*version.*=.*"[^"]*"' "$file" | sed 's/.*"\([^"]*\)".*/\1/' || true)
  if [ -z "$version" ]; then
    version=$(sed -n '/function.*version()/,/return/p' "$file" | grep -o '"[^"]*"' | sed 's/"//g' || true)
  fi
  echo "$version"
}
