# AetherOS Production Launcher
# Jalankan ini untuk mulai menggunakan AetherOS

param(
    [string]$Mode = "menu"
)

function Show-Menu {
    Clear-Host
    Write-Host ""
    Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║         AetherOS Production Launcher v10.2             ║" -ForegroundColor Cyan
    Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  [1] Development Mode    - QEMU dengan display" -ForegroundColor Green
    Write-Host "  [2] Headless Mode       - QEMU tanpa display (nographic)" -ForegroundColor Yellow
    Write-Host "  [3] Build Kernel        - Compile kernel dari source" -ForegroundColor Blue
    Write-Host "  [4] Build ISO           - Buat bootable ISO" -ForegroundColor Magenta
    Write-Host "  [5] VirtualBox          - Setup dan jalankan di VirtualBox" -ForegroundColor Cyan
    Write-Host "  [6] System Info         - Info sistem dan status" -ForegroundColor Gray
    Write-Host "  [Q] Quit                - Keluar" -ForegroundColor Red
    Write-Host ""
}

function Start-DevelopmentMode {
    Write-Host "Starting AetherOS in Development Mode..." -ForegroundColor Green
    & "C:\Program Files\qemu\qemu-system-x86_64.exe" -kernel "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel" -m 1024M -display gtk
}

function Start-HeadlessMode {
    Write-Host "Starting AetherOS in Headless Mode..." -ForegroundColor Yellow
    Write-Host "Tekan Ctrl+A lalu X untuk keluar" -ForegroundColor Yellow
    & "C:\Program Files\qemu\qemu-system-x86_64.exe" -kernel "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel" -m 1024M -nographic -serial mon:stdio
}

function Build-Kernel {
    Write-Host "Building AetherOS Kernel..." -ForegroundColor Blue
    Set-Location "D:\GitHub\AetherOS\kernel"
    cargo build --release --target x86_64-unknown-none
    Set-Location "D:\GitHub\AetherOS"
    Write-Host "Build complete! Kernel: target/x86_64-unknown-none/release/aetheros-kernel" -ForegroundColor Green
    Read-Host "Tekan Enter untuk lanjut"
}

function Build-ISO {
    Write-Host "Building ISO Image..." -ForegroundColor Magenta
    Set-Location "D:\GitHub\AetherOS"
    & cmd /c "make iso-image"
    Write-Host "ISO created: aetheros.iso" -ForegroundColor Green
    Read-Host "Tekan Enter untuk lanjut"
}

function Start-VirtualBox {
    Write-Host "Opening VirtualBox..." -ForegroundColor Cyan
    & "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe"
}

function Show-SystemInfo {
    Write-Host ""
    Write-Host "=== System Status ===" -ForegroundColor Cyan
    
    $kernelPath = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"
    if (Test-Path $kernelPath) {
        $size = [math]::Round((Get-Item $kernelPath).Length / 1MB, 2)
        $sizeStr = "$size MB"
        Write-Host "  Kernel: Found ($sizeStr)" -ForegroundColor Green
    } else {
        Write-Host "  Kernel: NOT FOUND" -ForegroundColor Red
    }
    
    $isoPath = "D:\GitHub\AetherOS\aetheros.iso"
    if (Test-Path $isoPath) {
        $size = [math]::Round((Get-Item $isoPath).Length / 1MB, 2)
        $sizeStr = "$size MB"
        Write-Host "  ISO: Found ($sizeStr)" -ForegroundColor Green
    } else {
        Write-Host "  ISO: NOT FOUND" -ForegroundColor Red
    }
    
    $qemuPath = "C:\Program Files\qemu\qemu-system-x86_64.exe"
    if (Test-Path $qemuPath) {
        Write-Host "  QEMU: Found" -ForegroundColor Green
    } else {
        Write-Host "  QEMU: NOT FOUND" -ForegroundColor Red
    }
    
    $vboxPath = "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe"
    if (Test-Path $vboxPath) {
        Write-Host "  VirtualBox: Found" -ForegroundColor Green
    } else {
        Write-Host "  VirtualBox: NOT FOUND" -ForegroundColor Red
    }
    
    Write-Host ""
    Read-Host "Tekan Enter untuk lanjut"
}

# Main Loop
do {
    Show-Menu
    $choice = Read-Host "Pilih opsi"
    
    switch ($choice) {
        "1" { Start-DevelopmentMode }
        "2" { Start-HeadlessMode }
        "3" { Build-Kernel }
        "4" { Build-ISO }
        "5" { Start-VirtualBox }
        "6" { Show-SystemInfo }
        "Q" { exit }
        "q" { exit }
    }
} while ($true)
