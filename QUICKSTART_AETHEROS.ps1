# AetherOS Quick Start Script
# One-click to boot AetherOS in QEMU

$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$KERNEL = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"

Write-Host ""
Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║          AetherOS Quick Start - Press Enter             ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "Press ENTER to start QEMU with AetherOS kernel..." -ForegroundColor Yellow
Read-Host ""

Write-Host "Starting AetherOS..." -ForegroundColor Green
& $QEMU -kernel $KERNEL -m 1024M -display gtk
