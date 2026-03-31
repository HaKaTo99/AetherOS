#!/usr/bin/env pwsh
# build_iso.ps1 - Build AetherOS ISO with proper GRUB setup

$ErrorActionPreference = "Stop"

$KernelPath = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"
$KernelPathFallback = "D:\GitHub\AetherOS\kernel\target\x86_64-unknown-none\release\aetheros-kernel"
$IsoPath = "D:\GitHub\AetherOS\aetheros.iso"
$IsoDir = "D:\GitHub\AetherOS\iso"

Write-Host "=== Building AetherOS ISO ===" -ForegroundColor Cyan

# Check kernel exists
if (!(Test-Path $KernelPath) -and (Test-Path $KernelPathFallback)) {
    $KernelPath = $KernelPathFallback
}

if (!(Test-Path $KernelPath)) {
    Write-Host "ERROR: Kernel not found at $KernelPath" -ForegroundColor Red
    Write-Host "Building kernel first..." -ForegroundColor Yellow
    Set-Location D:\GitHub\AetherOS\kernel
    cargo build --release --target x86_64-unknown-none
    Set-Location D:\GitHub\AetherOS

    if (!(Test-Path $KernelPath) -and (Test-Path $KernelPathFallback)) {
        $KernelPath = $KernelPathFallback
    }
}

# Create directories
$BootGrubDir = "$IsoDir\boot\grub"
$i386PcDir = "$BootGrubDir\i386-pc"

New-Item -ItemType Directory -Force -Path $i386PcDir | Out-Null

function Invoke-PosixCommand {
    param(
        [Parameter(Mandatory = $true)][string]$Command
    )

    $oldPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        try {
            $bashOutput = (& wsl -e bash -lc $Command 2>&1)
            $bashExit = $LASTEXITCODE
        } catch {
            $bashOutput = $_
            $bashExit = 127
        }

        if ($bashExit -eq 0) {
            return @{ ExitCode = 0; Output = $bashOutput }
        }

        try {
            $shOutput = (& wsl -e sh -lc $Command 2>&1)
            $shExit = $LASTEXITCODE
        } catch {
            $shOutput = $_
            $shExit = 127
        }

        return @{ ExitCode = $shExit; Output = @($bashOutput, $shOutput) }
    } finally {
        $ErrorActionPreference = $oldPreference
    }
}

# Copy kernel
Write-Host "Copying kernel..." -ForegroundColor Yellow
Copy-Item $KernelPath "$IsoDir\boot\aetheros_kernel" -Force

# Copy GRUB modules from WSL
Write-Host "Copying GRUB modules from WSL..." -ForegroundColor Yellow

# Try using grub-mkrescue directly (it should work without explicit modules if structure is correct)
Write-Host "Creating ISO with GRUB..." -ForegroundColor Yellow

# Use WSL to create ISO
$createIsoCmd = "cd /mnt/d/GitHub/AetherOS && grub-mkrescue -o aetheros.iso iso 2>&1"
$wslResult = Invoke-PosixCommand -Command $createIsoCmd

if ($wslResult.ExitCode -eq 0) {
    Write-Host "ISO created successfully!" -ForegroundColor Green
} else {
    Write-Host "WSL grub-mkrescue failed. Trying alternative..." -ForegroundColor Yellow
    if ($wslResult.Output) { Write-Host $wslResult.Output }
    
    # Alternative: Use xorriso directly
    $altCmd = "cd /mnt/d/GitHub/AetherOS/iso && xorriso -as mkisofs -iso-level 3 -o ../aetheros.iso -volid AETHEROS -boot-info-table -pad ."
    $altResult = Invoke-PosixCommand -Command $altCmd
    if ($altResult.ExitCode -ne 0) {
        if ($altResult.Output) { Write-Host $altResult.Output }

        if (Get-Command grub-mkrescue -ErrorAction SilentlyContinue) {
            Write-Host "Trying host grub-mkrescue..." -ForegroundColor Yellow
            grub-mkrescue -o $IsoPath $IsoDir
        } elseif (Get-Command xorriso -ErrorAction SilentlyContinue) {
            Write-Host "Trying host xorriso..." -ForegroundColor Yellow
            xorriso -as mkisofs -iso-level 3 -o $IsoPath -volid AETHEROS -boot-info-table -pad $IsoDir
        }
    }
}

# Verify ISO
if (Test-Path $IsoPath) {
    $isoSize = (Get-Item $IsoPath).Length / 1MB
    Write-Host "ISO created: $IsoPath ($([math]::Round($isoSize, 2)) MB)" -ForegroundColor Green
} else {
    Write-Host "ERROR: ISO creation failed!" -ForegroundColor Red
    exit 1
}
