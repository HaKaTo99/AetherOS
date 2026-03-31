# AetherOS x86_64 QEMU Launcher
# Run this script from PowerShell

$kernel = "target\x86_64-unknown-none\release\aetheros-kernel"

Write-Host "=== AetherOS x86_64 QEMU Launcher ===" -ForegroundColor Cyan
Write-Host "Starting QEMU with kernel: $kernel" -ForegroundColor Yellow

# Run QEMU with USB keyboard support for better input handling on Windows
qemu-system-x86_64 -kernel $kernel -m 1024M -display none -serial stdio -device qemu-xhci -device usb-kbd
