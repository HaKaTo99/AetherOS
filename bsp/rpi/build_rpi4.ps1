# Build Script for AetherOS on Raspberry Pi 4
$ErrorActionPreference = "Stop"

Write-Host "🔨 Building AetherOS for Raspberry Pi 4..."

# 1. Build Kernel
Set-Location "$PSScriptRoot\..\..\kernel"
cargo build --release --target aarch64-unknown-none --lib
if ($LASTEXITCODE -ne 0) { exit 1 }

# 2. Check for rust-objcopy
$KernelElf = "..\target\aarch64-unknown-none\release\aetheros_kernel"
$OutputDir = "$PSScriptRoot\..\..\build\rpi4"
$KernelImg = "$OutputDir\kernel8.img"

if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
}

if (Get-Command "rust-objcopy" -ErrorAction SilentlyContinue) {
    Write-Host "📦 Converting ELF to raw binary (kernel8.img)..."
    rust-objcopy --strip-all -O binary $KernelElf $KernelImg
} else {
    Write-Warning "start-objcopy not found! install: cargo install cargo-binutils; rustup component add llvm-tools-preview"
    Write-Warning "Skipping kernel8.img generation."
}

# 3. Copy Config Files
Write-Host "COPY Config files..."
Copy-Item "$PSScriptRoot\config.txt" -Destination $OutputDir
Copy-Item "$PSScriptRoot\cmdline.txt" -Destination $OutputDir

Write-Host "✅ Build Complete! Artifacts in $OutputDir"
