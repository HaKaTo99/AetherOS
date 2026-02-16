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

$ProjectRoot = Resolve-Path "$PSScriptRoot\.."
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

# 2. Build AetherOS
Write-Host "Building AetherOS..." -ForegroundColor Cyan
$ManifestPath = Join-Path $ProjectRoot "kernel\Cargo.toml"
cargo build --release --target x86_64-unknown-none --manifest-path $ManifestPath
if ($LASTEXITCODE -ne 0) { Write-Error "Build failed"; exit 1 }

# 3. Convert ELF to flat binary (bypasses PVH ELF Note error)
$KernelBin = Join-Path $ProjectRoot "kernel.bin"
Write-Host "Converting to flat binary..." -ForegroundColor Cyan
$ObjCopy = "rust-objcopy"
if (Get-Command $ObjCopy -ErrorAction SilentlyContinue) {
    & $ObjCopy -O binary $KernelPath $KernelBin
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "rust-objcopy failed. Falling back to ELF binary..."
        $KernelBin = $KernelPath
    }
}
else {
    Write-Warning "rust-objcopy not found. Install: rustup component add llvm-tools && cargo install cargo-binutils"
    Write-Warning "Falling back to ELF binary (may fail with PVH error)..."
    $KernelBin = $KernelPath
}

# 4. Run QEMU with -kernel (works with both flat binary and ELF)
Write-Host "Launching AetherOS..." -ForegroundColor Magenta
& $QemuExe -kernel $KernelBin -serial stdio -m 512M -display gtk
