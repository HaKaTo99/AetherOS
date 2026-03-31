#!/bin/bash
# AetherOS QEMU Launcher via WSL
# This provides proper stdio support for the shell

KERNEL_PATH="/mnt/d/GitHub/AetherOS/target/x86_64-unknown-none/release/aetheros-kernel"

echo "=== AetherOS x86_64 QEMU (WSL) ==="
echo "Kernel: $KERNEL_PATH"

# Run QEMU with nographic for proper stdio in WSL
qemu-system-x86_64 \
    -kernel "$KERNEL_PATH" \
    -m 1024M \
    -nographic \
    -serial mon:stdio
