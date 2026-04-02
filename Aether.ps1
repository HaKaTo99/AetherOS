#!/usr/bin/env pwsh
# xAetherOS Canonical Entry Point (Aether.ps1)
# Arsitek: Herman Krisnanto

param(
    [ValidateSet("menu", "build", "run", "smoke", "info", "stress", "verifikasi")]
    [string]$Action = "menu",
    [string]$WslDistro = "Ubuntu"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Derive repo root from script location
$RepoRoot = $PSScriptRoot
$ToolsDir = Join-Path $RepoRoot "tools"
$ScriptsDir = Join-Path $RepoRoot "scripts"
$OutDir = Join-Path $RepoRoot "out"
$IsoPath = Join-Path $OutDir "aetheros.iso"

function Get-QemuPath {
    $candidates = @(
        "C:\Program Files\qemu\qemu-system-x86_64.exe",
        "C:\Program Files\QEMU\qemu-system-x86_64.exe"
    )
    foreach ($path in $candidates) {
        if (Test-Path $path) { return $path }
    }
    $cmd = Get-Command qemu-system-x86_64 -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }
    return $null
}

function Start-AetherBuild {
    Write-Host "[Aether] Initiating Unified Build Cycle..." -ForegroundColor Cyan
    Write-Host "[1/3] Building kernel (release profile)..." -ForegroundColor Yellow
    cargo build --package aetheros-kernel --release
    if ($LASTEXITCODE -ne 0) { Write-Error "Kernel build failed."; exit 1 }
    
    # [NEW] Phase 2.2.1: Sovereign Signature Anchor
    $kernelPath = Join-Path $RepoRoot "target/x86_64-unknown-none/release/aetheros-kernel"
    & (Join-Path $RepoRoot "tools/sign_kernel.ps1") -KernelPath $kernelPath

    Write-Host "[2/3] Preparing bootloader & ISO structure..." -ForegroundColor Yellow
    & (Join-Path $ToolsDir "rebuild_vm_iso.ps1") -WslDistro $WslDistro
}

function Start-AetherOS {
    param([switch]$Headless)
    
    $qemuPath = Get-QemuPath
    if (-not $qemuPath) {
        Write-Host "[error] QEMU not found. Please install qemu-system-x86_64." -ForegroundColor Red
        return
    }

    if (-not (Test-Path $IsoPath)) {
        Write-Host "[warn] $IsoPath not found. Attempting build first..." -ForegroundColor Yellow
        Start-AetherBuild
    }

    Write-Host "[run] Launching AetherOS Sovereign Grade..." -ForegroundColor Green
    if ($Headless) {
        & $qemuPath -cdrom $IsoPath -m 1024M -nographic -serial mon:stdio
    } else {
        & $qemuPath -cdrom $IsoPath -m 1024M -display gtk
    }
}

function Start-SmokeTest {
    Write-Host "`n[Aether] Launching QEMU Smoke Test Pipeline..." -ForegroundColor Cyan
    & (Join-Path $ScriptsDir "qemu-smoke.ps1")
}

function Start-StressTest {
    Write-Host "`n[Aether] Initiating Military Grade Stress Test (Soak Test)..." -ForegroundColor Red
    Write-Host "[Aether] Duration: 300s (5 minutes) | Target: 0 Panics" -ForegroundColor Gray
    
    $env:TIMEOUT_SECONDS = 300
    & (Join-Path $ScriptsDir "qemu-smoke.ps1")
    Remove-Item Env:TIMEOUT_SECONDS -ErrorAction SilentlyContinue
}

function Start-Verifikasi {
    Write-Host "[Aether] Initiating Final Sovereign Verification Sequence..." -ForegroundColor Cyan
    & (Join-Path $RepoRoot "tests\verifikasi_final.ps1")
}

function Show-SystemInfo {
    $qemuPath = Get-QemuPath
    $qemuDisplay = if ($null -ne $qemuPath) { $qemuPath } else { 'NOT FOUND' }
    
    Write-Host "`nxAetherOS - Secure Distributed Intelligence Fabric" -ForegroundColor Cyan
    Write-Host "================================================" -ForegroundColor White
    Write-Host "Repo Root : $RepoRoot"
    Write-Host "Out ISO   : $IsoPath ( $(if (Test-Path $IsoPath) { 'EXISTS' } else { 'MISSING' }) )"
    Write-Host "QEMU Path : $qemuDisplay"
    Write-Host "WSL Distro: $WslDistro"
    Write-Host ""
}

function Show-Menu {
    Clear-Host
    $banner = @'
   ▄████████    ▄████████  ▄████████  ▄█    █▄     ▄████████    ▄████████ 
  ███    ███   ███    ███ ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    ███   ███    █▀  ███    █▀  ███    ███   ███    █▀    ███    ███ 
  ███    ███  ▄███▄▄▄     ███        ▄███▄▄███▄▄ ▄███▄▄▄      ▄███▄▄▄▄██▀ 
 ▀███████████ ▀▀███▀▀▀     ███▀▀▀▀▀  ▀▀███▀▀███▀ ▀▀███▀▀▀     ▀▀███▀▀▀▀▀   
  ███    ███   ███    █▄  ███    █▄  ███    ███   ███    █▄  ▀███████████ 
  ███    ███   ███    ███ ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    █▀    ██████████ ██████████  █▀    ▀█    ██████████   ███    ███ 
                                                                ███    ███ 
'@
    Write-Host $banner -ForegroundColor Cyan

    Write-Host "xAetherOS Master Control v10.2 (Supreme Grade)" -ForegroundColor White
    Write-Host "----------------------------------------------------" -ForegroundColor Gray
    Write-Host "1. Build AetherOS (Kernel + ISO)" -ForegroundColor Magenta
    Write-Host "2. Run AetherOS (QEMU Display)" -ForegroundColor Green
    Write-Host "3. Run AetherOS (Headless Mode)" -ForegroundColor Cyan
    Write-Host "4. Execute QEMU Smoke Test" -ForegroundColor Yellow
    Write-Host "5. Execute Heavy Stress Test (5m)" -ForegroundColor Red
    Write-Host "6. Execute Sovereign Verification" -ForegroundColor Blue
    Write-Host "7. Show System Diagnostics" -ForegroundColor White
    Write-Host "Q. Exit" -ForegroundColor Red
    Write-Host ""

    $choice = Read-Host "Select Action"
    switch ($choice.Trim().ToLower()) {
        "1" { Start-AetherBuild }
        "2" { Start-AetherOS }
        "3" { Start-AetherOS -Headless }
        "4" { Start-SmokeTest }
        "5" { Start-StressTest }
        "6" { Start-Verifikasi }
        "7" { Show-SystemInfo; Read-Host "Press Enter to return" }
        "q" { return }
        default { Write-Host "Invalid choice." -ForegroundColor Red; Start-Sleep -s 1 }
    }
    Show-Menu
}

switch ($Action) {
    "menu" { Show-Menu }
    "build" { Start-AetherBuild }
    "run" { Start-AetherOS }
    "smoke" { Start-SmokeTest }
    "stress" { Start-StressTest }
    "verifikasi" { Start-Verifikasi }
    "info" { Show-SystemInfo }
}
