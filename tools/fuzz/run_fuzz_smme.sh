#!/usr/bin/env bash
# Helper to run a SMME fuzz harness (requires honggfuzz or cargo-fuzz)

set -euo pipefail

if ! command -v honggfuzz >/dev/null 2>&1; then
  echo "honggfuzz not found. Install honggfuzz or use cargo-fuzz."
  exit 1
fi

echo "Starting SMME fuzz (ensure you built a harness crate)
Run honggfuzz with your harness binary path as -- ./harness"

HFUZZ_OPTS="--input inputs --output findings --no-ui"

# Example (user must provide harness):
# honggfuzz -s -o findings -i inputs -- ./target/release/smme_harness

echo "Edit this script to point at your harness binary. See docs/FUZZING.md for guidance."
