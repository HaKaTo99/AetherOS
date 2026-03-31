#!/usr/bin/env pwsh
param(
    [ValidateSet("menu", "dev", "headless", "build", "iso", "info")]
    [string]$Action = "menu",
    [int]$MemoryMb = 1024,
    [switch]$SkipBuild
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = if ($PSScriptRoot) { $PSScriptRoot } else { "D:\GitHub\AetherOS" }

function Get-QemuPath {
    $candidates = @(
        "C:\Program Files\qemu\qemu-system-x86_64.exe",
        "C:\Program Files\QEMU\qemu-system-x86_64.exe"
    )
    foreach ($path in $candidates) {
        if (Test-Path $path) {
            return $path
        }
    }

    $cmd = Get-Command qemu-system-x86_64 -ErrorAction SilentlyContinue
    if ($cmd) {
        return $cmd.Source
    }

    return $null
}

function Get-KernelBinaryPath {
    $prod = Join-Path $Root "target\x86_64-unknown-none\production\aetheros-kernel"
    $release = Join-Path $Root "target\x86_64-unknown-none\release\aetheros-kernel"
    $kernelRelease = Join-Path $Root "kernel\target\x86_64-unknown-none\release\aetheros-kernel"

    if (Test-Path $prod) { return $prod }
    if (Test-Path $release) { return $release }
    if (Test-Path $kernelRelease) { return $kernelRelease }

    return $prod
}

function Build-Kernel {
    Write-Host "[build] Building kernel (profile production)..." -ForegroundColor Cyan
    Push-Location $Root
    try {
        cargo build --profile production --target x86_64-unknown-none -p aetheros-kernel
    } finally {
        Pop-Location
    }
}

function Build-Iso {
    Write-Host "[build] Building ISO..." -ForegroundColor Magenta
    & (Join-Path $Root "build_iso.ps1")
}

function Show-SystemInfo {
    $kernelPath = Get-KernelBinaryPath
    $isoPath = Join-Path $Root "aetheros.iso"
    $qemuPath = Get-QemuPath

    Write-Host ""
    Write-Host "AetherOS Production Status"
    Write-Host "=========================="
    Write-Host "Root   : $Root"
    Write-Host "QEMU   : $(if ($qemuPath) { $qemuPath } else { 'NOT FOUND' })"
    Write-Host "Kernel : $(if (Test-Path $kernelPath) { $kernelPath } else { 'NOT FOUND' })"
    Write-Host "ISO    : $(if (Test-Path $isoPath) { $isoPath } else { 'NOT FOUND' })"
    Write-Host ""
}

function Start-AetherOS {
    param([switch]$Headless)

    $qemuPath = Get-QemuPath
    if (-not $qemuPath) {
        throw "QEMU not found. Install qemu-system-x86_64 and re-run launcher."
    }

    $kernelPath = Get-KernelBinaryPath
    if (-not (Test-Path $kernelPath)) {
        if ($SkipBuild) {
            throw "Kernel binary not found at $kernelPath and -SkipBuild specified."
        }
        Build-Kernel
        $kernelPath = Get-KernelBinaryPath
    }

    if (-not (Test-Path $kernelPath)) {
        throw "Kernel binary still missing after build: $kernelPath"
    }

    if ($Headless) {
        Write-Host "[run] Headless mode. Press Ctrl+A then X to exit QEMU." -ForegroundColor Yellow
        & $qemuPath -kernel $kernelPath -m ("{0}M" -f $MemoryMb) -nographic -serial mon:stdio
    } else {
        Write-Host "[run] Development display mode." -ForegroundColor Green
        & $qemuPath -kernel $kernelPath -m ("{0}M" -f $MemoryMb) -display gtk
    }
}

function Show-Menu {
    Write-Host ""
    Write-Host "AetherOS Production Launcher v10.3"
    Write-Host "====================================="
    Write-Host "1. Development Mode (QEMU with display)"
    Write-Host "2. Headless Mode (QEMU nographic)"
    Write-Host "3. Build Kernel (production profile)"
    Write-Host "4. Build ISO"
    Write-Host "5. System Info"
    Write-Host "Q. Quit"
    Write-Host ""

    $choice = Read-Host "Pilih opsi"
    switch ($choice.ToLowerInvariant()) {
        "1" { Start-AetherOS }
        "2" { Start-AetherOS -Headless }
        "3" { Build-Kernel }
        "4" { Build-Iso }
        "5" { Show-SystemInfo }
        "q" { return }
        default { Write-Host "Unknown option: $choice" -ForegroundColor Red }
    }
}

switch ($Action) {
    "menu" { Show-Menu }
    "dev" { Start-AetherOS }
    "headless" { Start-AetherOS -Headless }
    "build" { Build-Kernel }
    "iso" { Build-Iso }
    "info" { Show-SystemInfo }
}
