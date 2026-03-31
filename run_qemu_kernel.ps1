# AetherOS QEMU Launcher - Fixed with QEMU Monitor
$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$KERNEL = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"

Write-Host "=== Starting AetherOS Kernel in QEMU ===" -ForegroundColor Cyan
Write-Host "Kernel: $KERNEL" -ForegroundColor Yellow

# Run with -nographic to combine serial and monitor
# Serial goes to stdout, QEMU monitor available via Ctrl+A+C
& $QEMU -kernel $KERNEL -m 1024M -nographic
