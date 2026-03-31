# AetherOS QEMU Interactive Test
# Allows typing 'help' and other commands in QEMU

$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$KERNEL = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "AetherOS QEMU Interactive Test" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Remove old serial log
Remove-Item "D:\GitHub\AetherOS\qemu_serial.log" -ErrorAction SilentlyContinue

Write-Host "Starting QEMU with serial console..." -ForegroundColor Yellow
Write-Host "Kernel: $KERNEL" -ForegroundColor Gray
Write-Host ""
Write-Host "IMPORTANT: Type commands in the QEMU window!" -ForegroundColor Red
Write-Host "After boot, type: help" -ForegroundColor Green
Write-Host ""
Write-Host "To quit QEMU: Press Ctrl+A then X" -ForegroundColor Yellow
Write-Host ""

# Start QEMU with -nographic (serial to stdin/stdout)
# This allows interactive input
& $QEMU -kernel $KERNEL -m 1024M -nographic
