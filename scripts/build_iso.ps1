#!/usr/bin/env pwsh
<#!
Build GRUB-based bootable ISO for AetherOS.
- Builds kernel (release, bindeps) for x86_64-unknown-none
- Prepares iso_root with /boot/aetheros-kernel and grub.cfg
- Runs grub-mkrescue to produce aetheros.iso (or custom output)
#>
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

param(
    [string]$OutIso = "aetheros.iso",
    [string]$KernelPath = "target/x86_64-unknown-none/release/aetheros-kernel",
    [string]$IsoRoot = "iso_root",
    [switch]$Toram,
    [switch]$Debug
)

function Assert-Tool([string]$name) {
    if (-not (Get-Command $name -ErrorAction SilentlyContinue)) {
        throw "Executable not found: $name"
    }
}

# Resolve repo root (scripts directory is under repo)
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

Assert-Tool "grub-mkrescue"

# Build kernel (release) with required bindeps
Write-Host "[build] cargo +nightly -Z bindeps build -p aetheros-kernel --release --target x86_64-unknown-none" -ForegroundColor Cyan
$env:CARGO_UNSTABLE_BETA_BINDEPS = 1
& cargo +nightly -Z bindeps build -p aetheros-kernel --release --target x86_64-unknown-none
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Remove-Item Env:CARGO_UNSTABLE_BETA_BINDEPS -ErrorAction SilentlyContinue

$kernelFull = Resolve-Path $KernelPath
if (-not (Test-Path $kernelFull)) { throw "Kernel tidak ditemukan: $KernelPath" }

# Prepare ISO root
Remove-Item -Recurse -Force $IsoRoot -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path (Join-Path $IsoRoot "boot/grub") | Out-Null
Copy-Item $kernelFull (Join-Path $IsoRoot "boot/aetheros-kernel")

# Build grub.cfg
$cmdline = @()
if ($Toram) { $cmdline += "toram" }
if ($Debug) { $cmdline += "debug" }
$cmdlineStr = $cmdline -join " "
@"
set timeout=3
set default=0

menuentry "AetherOS Supreme Grade" {
    multiboot2 /boot/aetheros-kernel $cmdlineStr
    boot
}
"@ | Set-Content (Join-Path $IsoRoot "boot/grub/grub.cfg") -Encoding ASCII

# Create ISO
Write-Host "[iso ] grub-mkrescue -o $OutIso $IsoRoot" -ForegroundColor Cyan
& grub-mkrescue -o $OutIso $IsoRoot
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "[done] ISO created: $(Resolve-Path $OutIso)" -ForegroundColor Green
