#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

param(
    [switch]$ShowDisplay = $false,
    [int]$TimeoutSeconds = 120
)

Write-Host "[ xAetherOS v10.2 ]: Memulai Simulasi 'Supreme Grade'..." -ForegroundColor Cyan

function Assert-Tool($name) {
    if (-not (Get-Command $name -ErrorAction SilentlyContinue)) {
        throw "Executable not found: $name"
    }
}

$IMAGE_PATH = Join-Path $PSScriptRoot "..\target\x86_64-unknown-none\release\bootimage-aetheros-kernel.bin"
Assert-Tool "qemu-system-x86_64"

if (-not (Test-Path $IMAGE_PATH)) {
    Write-Host "[ WARNING ]: Citra kernel tidak ditemukan di $IMAGE_PATH" -ForegroundColor Yellow
    Write-Host "[ BUILD ]: Memulai proses build..." -ForegroundColor Cyan
    Push-Location (Join-Path $PSScriptRoot "..\kernel")
    cargo +nightly -Z bindeps bootimage --release
    Pop-Location
}

if (-not (Test-Path $IMAGE_PATH)) {
    throw "Citra kernel tetap tidak ditemukan di $IMAGE_PATH setelah build"
}

$accel = "whpx"
try {
    # Quick probe: qemu will fail fast if WHPX unavailable
    $null = & qemu-system-x86_64 -accel $accel -machine accel=help 2>$null
} catch {
    $accel = "tcg"
}

$qemuArgsBase = "-drive format=raw,file=$IMAGE_PATH -m 1024M -smp 2 -cpu qemu64 -accel $accel -serial stdio -no-reboot -no-shutdown -nographic"
if ($ShowDisplay) { $qemuArgs = $qemuArgsBase -replace "-nographic", "" } else { $qemuArgs = $qemuArgsBase + " -display none" }

Write-Host "[ CMD ]: qemu-system-x86_64 $qemuArgs" -ForegroundColor DarkGray

$proc = Start-Process -FilePath "qemu-system-x86_64" -ArgumentList $qemuArgs -PassThru -NoNewWindow
if (-not $proc.WaitForExit($TimeoutSeconds * 1000)) {
    Write-Warning "QEMU melebihi timeout ${TimeoutSeconds}s, menghentikan proses"
    try { $proc.Kill() } catch {}
}
if (-not $proc.HasExited -or $proc.ExitCode -ne 0) {
    throw "QEMU gagal atau dihentikan (exit code $($proc.ExitCode))"
}
Write-Host "[ SUCCESS ]: QEMU selesai" -ForegroundColor Green
