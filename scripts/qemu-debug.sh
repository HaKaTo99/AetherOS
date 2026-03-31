#!/usr/bin/env bash
# QEMU debug preset (-s -S) for attaching gdb
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
BOOT_BIN="$ROOT_DIR/target/x86_64-unknown-none/release/aetheros-kernel"

if [[ ! -f "$BOOT_BIN" ]]; then
  echo "[warn] Kernel belum dibangun. Membangun release (bindeps)..."
  (cd "$ROOT_DIR/kernel" && CARGO_UNSTABLE_BETA_BINDEPS=1 cargo +nightly -Z bindeps build --release)
fi

if [[ ! -f "$BOOT_BIN" ]]; then
  echo "[error] Kernel masih belum ada di $BOOT_BIN"
  exit 1
fi

QEMU_CMD=(
  qemu-system-x86_64
  -kernel "$BOOT_BIN"
  -m "${MEM_MB:-1024}M"
  -smp "${SMP_CORES:-2}"
  -cpu "${CPU_MODEL:-qemu64}"
  -serial stdio
  -display none
  -s        # expose gdb stub on tcp::1234
  -S        # halt at startup until gdb continues
)

echo "[run] ${QEMU_CMD[*]}"
exec "${QEMU_CMD[@]}"
