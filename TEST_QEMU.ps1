# AetherOS QEMU Test Script
# Tests kernel boot and input functionality

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "AetherOS QEMU Test v10.2 Supreme Grade" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$KERNEL = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"

# Verify files exist
if (-not (Test-Path $QEMU)) {
    Write-Host "[ERROR] QEMU not found at: $QEMU" -ForegroundColor Red
    exit 1
}
if (-not (Test-Path $KERNEL)) {
    Write-Host "[ERROR] Kernel not found at: $KERNEL" -ForegroundColor Red
    exit 1
}

Write-Host "Starting QEMU with AetherOS Kernel..." -ForegroundColor Yellow
Write-Host "QEMU:   $QEMU" -ForegroundColor Gray
Write-Host "Kernel: $KERNEL" -ForegroundColor Gray
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "INSTRUCTIONS:" -ForegroundColor Yellow
Write-Host "1. QEMU will start in a new window" -ForegroundColor White
Write-Host "2. Wait for AetherShell prompt (AetherShell>)" -ForegroundColor White
Write-Host "3. Type 'help' and press Enter" -ForegroundColor White
Write-Host "4. Press Ctrl+Alt+G to capture mouse (if needed)" -ForegroundColor White
Write-Host "5. Press Ctrl+C in THIS terminal to stop" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Launch QEMU with display window (easier to interact)
# Using -display gtk for GUI, or -nographic for console
Start-Process -FilePath $QEMU -ArgumentList @(
    "-kernel", "`"$KERNEL`"",
    "-m", "1024M",
    "-display", "gtk",
    "-serial", "file:D:\GitHub\AetherOS\qemu_serial.log"
) -NoNewWindow

Write-Host "[INFO] QEMU started with GTK display" -ForegroundColor Green
Write-Host "[INFO] Serial log: D:\GitHub\AetherOS\qemu_serial.log" -ForegroundColor Green
Write-Host ""
Write-Host "Waiting for boot..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# Show serial log
if (Test-Path "D:\GitHub\AetherOS\qemu_serial.log") {
    Write-Host ""
    Write-Host "=== Serial Output ===" -ForegroundColor Cyan
    Get-Content "D:\GitHub\AetherOS\qemu_serial.log" | Select-Object -Last 20
}

Write-Host ""
Write-Host "[TEST] If you see AetherShell> prompt in QEMU window:" -ForegroundColor Green
Write-Host "       Type: help" -ForegroundColor White
Write-Host "       Press Enter" -ForegroundColor White
Write-Host ""
Write-Host "Press Ctrl+C in this terminal to stop testing..." -ForegroundColor Yellow
