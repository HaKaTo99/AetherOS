# AetherOS Interactive Test Script
# Cara pakai: ketik command di terminal setelah QEMU mulai

$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$KERNEL = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"

Write-Host "=== AetherOS Interactive Test ===" -ForegroundColor Cyan
Write-Host "Kernel: $KERNEL" -ForegroundColor Yellow
Write-Host ""
Write-Host "QEMU akan start dengan mode -nographic" -ForegroundColor Green
Write-Host "Ketik command di window QEMU setelah boot selesai:" -ForegroundColor Green
Write-Host "  - help" -ForegroundColor Yellow
Write-Host "  - calc" -ForegroundColor Yellow
Write-Host "  - clear" -ForegroundColor Yellow
Write-Host "  - exit" -ForegroundColor Yellow
Write-Host ""
Write-Host "Tekan Ctrl+C untuk keluar" -ForegroundColor Red
Write-Host ""

# Jalankan QEMU dengan -nographic - output ke terminal saat ini
& $QEMU -kernel $KERNEL -m 1024M -nographic
