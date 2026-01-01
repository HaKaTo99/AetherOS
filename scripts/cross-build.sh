#!/usr/bin/env bash
# Helper script for local cross-builds for CI parity
# Usage: ./scripts/cross-build.sh aarch64-unknown-linux-gnu
set -euo pipefail

TARGET=${1:-x86_64-unknown-linux-gnu}

echo "Adding Rust target: ${TARGET}"
rustup target add ${TARGET}

case "${TARGET}" in
  aarch64-unknown-linux-gnu)
    echo "(Optional) install cross C toolchain on Debian/Ubuntu: sudo apt install gcc-aarch64-linux-gnu"
    ;;
  x86_64-unknown-linux-gnu)
    # host target
    ;;
  *)
    echo "Unknown target: ${TARGET}"
    exit 1
    ;;
esac

echo "Building workspace for ${TARGET}"
cargo build --workspace --release --target ${TARGET}

echo "Build finished: target/${TARGET}/release/"