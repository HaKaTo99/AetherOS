#!/usr/bin/env powershell
# AetherOS v10.2 "Supreme Grade" - Simulation Script
# (c) 2026 Architect herman x Antigravity

param(
    [switch]$ShowDisplay = $false
)

Write-Host "[ xAetherOS v10.2 ]: Memulai Simulasi 'Supreme Grade'..." -ForegroundColor Cyan

# Jalur biner relatif terhadap lokasi skrip (target berada di root workspace)
$IMAGE_PATH = "$PSScriptRoot\..\target\x86_64-unknown-none\release\bootimage-aetheros-kernel.bin"

if (-not (Test-Path $IMAGE_PATH)) {
    Write-Host "[ WARNING ]: Citra kernel tidak ditemukan di $IMAGE_PATH" -ForegroundColor Yellow
    Write-Host "[ BUILD ]: Memulai proses build..." -ForegroundColor Cyan
    Push-Location "$PSScriptRoot\..\kernel"
    cargo bootimage --release
    Pop-Location
}

if (Test-Path $IMAGE_PATH) {
    Write-Host "[ SUCCESS ]: Meluncurkan QEMU (Military Grade Harmony Mode)..." -ForegroundColor Green

    # Prepare QEMU arguments: use stdio serial (direct stdin/stdout), safe CPU model and SMP to speed boot
    $accel = "tcg"
    $qemuArgsBase = "-drive format=raw,file=$IMAGE_PATH -m 1024M -smp 2 -cpu qemu64 -accel $accel -serial stdio"
    
    if ($ShowDisplay) { $qemuArgs = $qemuArgsBase } else { $qemuArgs = "$qemuArgsBase -display none" }

    Write-Host "[ CMD ]: qemu-system-x86_64 $qemuArgs" -ForegroundColor DarkGray
    
    # Try with WHPX first, if it fails or hangs, the user can CTRL+C and manually try TCG, 
    # but here we provide a more robust TCG fallback if we detect immediate failure in future versions.
    # For now, let's just make it easier to switch.
    Start-Process -FilePath "qemu-system-x86_64" -ArgumentList $qemuArgs -NoNewWindow -Wait
}
else {
    Write-Host "[ ERROR ]: Gagal mendeteksi citra kernel setelah build. Pastikan toolchain Rust terpasang." -ForegroundColor Red
}
