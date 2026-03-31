# AetherOS Production Launcher v10.2

Write-Host ""
Write-Host "AetherOS Production Launcher v10.2"
Write-Host "====================================="
Write-Host ""
Write-Host "1. Development Mode (QEMU with display)"
Write-Host "2. Headless Mode (QEMU nographic)"
Write-Host "3. Build Kernel"
Write-Host "4. Build ISO"
Write-Host "5. System Info"
Write-Host "Q. Quit"
Write-Host ""

$choice = Read-Host "Pilih opsi"

if ($choice -eq "1") {
    Write-Host "Starting AetherOS..." -ForegroundColor Green
    & "C:\Program Files\qemu\qemu-system-x86_64.exe" -kernel "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel" -m 1024M -display gtk
}
elseif ($choice -eq "2") {
    Write-Host "Starting AetherOS (Headless)..." -ForegroundColor Yellow
    Write-Host "Press Ctrl+A then X to exit"
    & "C:\Program Files\qemu\qemu-system-x86_64.exe" -kernel "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel" -m 1024M -nographic -serial mon:stdio
}
elseif ($choice -eq "3") {
    Write-Host "Building kernel..." -ForegroundColor Blue
    Set-Location "D:\GitHub\AetherOS\kernel"
    cargo build --release --target x86_64-unknown-none
    Write-Host "Done!" -ForegroundColor Green
}
elseif ($choice -eq "4") {
    Write-Host "Building ISO..." -ForegroundColor Magenta
    Set-Location "D:\GitHub\AetherOS"
    make iso-image
    Write-Host "Done!" -ForegroundColor Green
}
elseif ($choice -eq "5") {
    Write-Host ""
    Write-Host "System Status:"
    if (Test-Path "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel") {
        Write-Host "  Kernel: OK" -ForegroundColor Green
    } else {
        Write-Host "  Kernel: NOT FOUND" -ForegroundColor Red
    }
    if (Test-Path "D:\GitHub\AetherOS\aetheros.iso") {
        Write-Host "  ISO: OK" -ForegroundColor Green
    } else {
        Write-Host "  ISO: NOT FOUND" -ForegroundColor Red
    }
}
