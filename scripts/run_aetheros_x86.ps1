<#
.SYNOPSIS
    AetherOS x86_64 Launcher Script
    Bypasses ISO creation by directly loading the kernel into QEMU.
    
.DESCRIPTION
    This script attempts to run the compiled AetherOS kernel using QEMU.
    It checks if QEMU is installed and in PATH.
    If found, it runs: qemu-system-x86_64 -kernel <kernel_elf> -serial stdio
    
.NOTES
    Requires: qemu-system-x86_64 installed and in PATH.
#>

$KernelPath = "$PSScriptRoot\..\target\x86_64-unknown-none\release\aetheros-kernel"
if (-not (Test-Path $KernelPath)) {
    $KernelPath = "$PSScriptRoot\..\kernel\target\x86_64-unknown-none\release\aetheros-kernel"
}
$QemuExe = "qemu-system-x86_64.exe"

Write-Host "AetherOS x86_64 Launcher" -ForegroundColor Cyan
Write-Host "========================" -ForegroundColor Cyan

# 1. Check if Kernel exists
if (-not (Test-Path $KernelPath)) {
    Write-Error "Kernel binary not found at: $KernelPath"
    Write-Host "Please run: cd kernel; cargo build --release --target x86_64-unknown-none" -ForegroundColor Yellow
    exit 1
}
Write-Host "[OK] Kernel binary found." -ForegroundColor Green

# 2. Check for QEMU
if (-not (Get-Command $QemuExe -ErrorAction SilentlyContinue)) {
    Write-Warning "QEMU not found in PATH."
    Write-Host "To verify AetherOS, please install QEMU:"
    Write-Host "  winget install -e --id SoftwareFreedomConservancy.QEMU"
    Write-Host "  OR download from https://www.qemu.org/download/#windows"
    Write-Host ""
    Write-Host "After installation, restart your terminal and run this script again."
    exit 1
}
Write-Host "[OK] QEMU found." -ForegroundColor Green

# 3. Run QEMU
Write-Host "Launching AetherOS..." -ForegroundColor Magenta
Write-Host "Command: $QemuExe -kernel aetheros-kernel -serial stdio"

& $QemuExe -kernel $KernelPath -serial stdio -m 512M -display gtk
