#!/usr/bin/env bash
# QEMU smoke test for AetherOS kernel
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
LOG_PATH="$ROOT_DIR/target/qemu-smoke.log"
BOOT_BIN="$ROOT_DIR/target/x86_64-unknown-none/release/aetheros-kernel"
TIMEOUT_SECONDS=${TIMEOUT_SECONDS:-60}
BOOT_MARKER=${BOOT_MARKER:-"AetherShell>"}
# Markers penting yang harus muncul untuk lulus (bisa override via env REQUIRED_MARKERS)
REQUIRED_MARKERS_DEFAULT=(
  "X86_64 HAL Initialized (v10.2 Supreme Grade)"
  "GDT/IDT Initialized"
  "[Intent] Cognitive Listener Active"
  "Global Mesh: Harmony Baseline Stable"
  "[Security] Sovereign Identity Mesh: ACTIVE"
  "[Security] Hardware Entropy Chain: SEALED"
  "[Security] Military Grade Deployment Readiness: 100%"
)
IFS=$'\n' read -r -d '' -a REQUIRED_MARKERS <<< "${REQUIRED_MARKERS:-$(printf '%s\n' "${REQUIRED_MARKERS_DEFAULT[@]}")}" || true

cd "$ROOT_DIR"

echo "[setup] Ensuring Rust target x86_64-unknown-none is installed"
rustup target add x86_64-unknown-none >/dev/null

echo "[build] Building kernel (release) with nightly toolchain"
cargo +nightly build --package aetheros-kernel --release

echo "[check] Verifying QEMU availability"
if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
  echo "[error] qemu-system-x86_64 not found"
  exit 1
fi

rm -f "$LOG_PATH"

QEMU_CMD=(
  qemu-system-x86_64
  -kernel "$BOOT_BIN"
  -m "${MEM_MB:-1024}M"
  -serial stdio
  -display none
  -no-reboot
  -no-shutdown
  -smp "${SMP_CORES:-2}"
  -cpu "${CPU_MODEL:-qemu64}"
)

echo "[run] ${QEMU_CMD[*]} (timeout: ${TIMEOUT_SECONDS}s)"

set +e
PATH_LOGGED_OUTPUT=$(timeout --preserve-status "${TIMEOUT_SECONDS}s" "${QEMU_CMD[@]}" 2>&1 | tee "$LOG_PATH")
QEMU_STATUS=$?
set -e

if [[ $QEMU_STATUS -eq 124 ]]; then
  echo "[warn] QEMU timed out after ${TIMEOUT_SECONDS}s; proceeding to parse log"
elif [[ $QEMU_STATUS -ne 0 ]]; then
  echo "[error] QEMU exited with status $QEMU_STATUS"
  exit $QEMU_STATUS
fi

fail=0

if grep -q "$BOOT_MARKER" "$LOG_PATH"; then
  echo "[ok] Boot marker '$BOOT_MARKER' ditemukan"
else
  echo "[fail] Boot marker '$BOOT_MARKER' tidak ditemukan"
  fail=1
fi

for marker in "${REQUIRED_MARKERS[@]}"; do
  if grep -q "$marker" "$LOG_PATH"; then
    echo "[ok] Marker '$marker' ditemukan"
  else
    echo "[fail] Marker '$marker' tidak ditemukan"
    fail=1
  fi
done

if [[ $fail -ne 0 ]]; then
  echo "------ log tail ------"
  tail -n 120 "$LOG_PATH" || true
  echo "----------------------"
  exit 1
fi

echo "[ok] Semua marker ditemukan"
