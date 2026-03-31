#!/usr/bin/env pwsh
# build_iso.ps1 - Build AetherOS ISO with proper GRUB setup

$ErrorActionPreference = "Stop"

$KernelPath = "D:\GitHub\AetherOS\kernel\target\x86_64-unknown-none\release\aetheros-kernel"
$IsoPath = "D:\GitHub\AetherOS\aetheros.iso"
$IsoDir = "D:\GitHub\AetherOS\iso"

Write-Host "=== Building AetherOS ISO ===" -ForegroundColor Cyan

# Check kernel exists
if (!(Test-Path $KernelPath)) {
    Write-Host "ERROR: Kernel not found at $KernelPath" -ForegroundColor Red
    Write-Host "Building kernel first..." -ForegroundColor Yellow
    Set-Location D:\GitHub\AetherOS\kernel
    cargo build --release --target x86_64-unknown-none
    Set-Location D:\GitHub\AetherOS
}

# Create directories
$BootGrubDir = "$IsoDir\boot\grub"
$i386PcDir = "$BootGrubDir\i386-pc"

New-Item -ItemType Directory -Force -Path $i386PcDir | Out-Null

# Copy kernel
Write-Host "Copying kernel..." -ForegroundColor Yellow
Copy-Item $KernelPath "$IsoDir\boot\aetheros_kernel" -Force

# Copy GRUB modules from WSL
Write-Host "Copying GRUB modules from WSL..." -ForegroundColor Yellow

# Try using grub-mkrescue directly (it should work without explicit modules if structure is correct)
Write-Host "Creating ISO with GRUB..." -ForegroundColor Yellow

# Use WSL to create ISO
$createIsoCmd = "cd /mnt/d/GitHub/AetherOS && grub-mkrescue -o aetheros.iso iso 2>&1"
$wslResult = wsl bash -c $createIsoCmd 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "ISO created successfully!" -ForegroundColor Green
} else {
    Write-Host "WSL grub-mkrescue failed. Trying alternative..." -ForegroundColor Yellow
    Write-Host $wslResult
    
    # Alternative: Use xorriso directly
    $altCmd = "cd /mnt/d/GitHub/AetherOS/iso && xorriso -as mkisofs -iso-level 3 -o ../aetheros.iso -volid AETHEROS -boot-info-table -pad ."
    wsl bash -c $altCmd
}

# Verify ISO
if (Test-Path $IsoPath) {
    $isoSize = (Get-Item $IsoPath).Length / 1MB
    Write-Host "ISO created: $IsoPath ($([math]::Round($isoSize, 2)) MB)" -ForegroundColor Green
} else {
    Write-Host "ERROR: ISO creation failed!" -ForegroundColor Red
    exit 1
}
