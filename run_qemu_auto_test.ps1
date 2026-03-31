# AetherOS Auto Test Script - Kirim command via serial
# Command dijalankan otomatis tanpa perlu interaktif

$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$KERNEL = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"

Write-Host "=== AetherOS Auto Test ===" -ForegroundColor Cyan
Write-Host ""

# Start QEMU dengan serial redirect ke file
$serialFile = "d:\GitHub\AetherOS\serial_io.txt"

# Hapus file serial lama
if (Test-Path $serialFile) { Remove-Item $serialFile }

Write-Host "Starting QEMU with serial output to $serialFile..." -ForegroundColor Yellow

# Start QEMU di background dengan serial redirect
$proc = Start-Process -FilePath $QEMU -ArgumentList "-kernel", $KERNEL, "-m", "1024M", "-serial", "file:$serialFile", "-nographic" -PassThru -WindowStyle Hidden

Write-Host "QEMU started with PID: $($proc.Id)" -ForegroundColor Green
Write-Host "Waiting for boot..." -ForegroundColor Yellow

# Wait untuk boot
Start-Sleep -Seconds 8

# Baca output serial
Write-Host ""
Write-Host "=== Serial Output ===" -ForegroundColor Cyan
if (Test-Path $serialFile) {
    Get-Content $serialFile | Select-Object -Last 30
} else {
    Write-Host "Serial file not found!" -ForegroundColor Red
}

# Kill QEMU
Write-Host ""
Write-Host "Stopping QEMU..." -ForegroundColor Yellow
Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "=== Test Complete ===" -ForegroundColor Green
