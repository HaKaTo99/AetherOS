# AetherOS Unified Rebuild Script (Build Kernel + Stamp + Build ISO)

param(
    [string]$WslDistro = "Ubuntu",
    [string]$KernelSource = "target/x86_64-unknown-none/release/aetheros-kernel"
)

$ErrorActionPreference = "Stop"

# Derive repo root from script location
$ToolsRoot = $PSScriptRoot
$RepoRoot = Split-Path $ToolsRoot -Parent

function Resolve-RepoPath {
    param(
        [string]$Path,
        [string]$Root
    )
    if ([System.IO.Path]::IsPathRooted($Path)) { return $Path }
    return Join-Path $Root $Path
}

# Standardize Output ISO path to out/aetheros.iso
$outIsoRelative = "out/aetheros.iso"
$resolvedOutputIso = Resolve-RepoPath -Path $outIsoRelative -Root $RepoRoot
$resolvedKernelSource = Resolve-RepoPath -Path $KernelSource -Root $RepoRoot

# Generate Build Identifier
$buildId = "INPUT-STABLE-" + (Get-Date -Format "yyyy-MM-dd-HHmmss")

Write-Host "[1/3] Building kernel (release profile)..." -ForegroundColor Magenta
Push-Location $RepoRoot
try {
    $oldBuildId = $env:AETHER_BUILD_ID
    $env:AETHER_BUILD_ID = $buildId
    
    # Nightly cargo build for x86_64-unknown-none
    cargo build -p aetheros-kernel --release --target x86_64-unknown-none
    
    # Restore build ID
    if ($null -ne $oldBuildId) {
        $env:AETHER_BUILD_ID = $oldBuildId
    } else {
        Remove-Item Env:\AETHER_BUILD_ID -ErrorAction SilentlyContinue
    }
} finally {
    Pop-Location
}

if ($LASTEXITCODE -ne 0) { throw "Kernel build failed. Check cargo output for errors." }

Write-Host "[2/3] Rebuilding ISO: $outIsoRelative" -ForegroundColorCyan
# Call consolidated build_iso.ps1 in the same directory
& (Join-Path $ToolsRoot "build_iso.ps1") -KernelSource $resolvedKernelSource -OutputIso $resolvedOutputIso -WslDistro $WslDistro

if ($LASTEXITCODE -ne 0) { 
    throw "ISO build failed. If the file is locked, ensure VM/QEMU is powered off first." 
}

Write-Host "[3/3] Done" -ForegroundColor Green
Write-Host "Build marker: $buildId" -ForegroundColor Cyan
Write-Host "ISO ready   : $resolvedOutputIso" -ForegroundColor Gray
Write-Host "Workflow    : SINGLE ISO (Canonical out/aetheros.iso)" -ForegroundColor Gray
Write-Host "Note        : Please mount $resolvedOutputIso in your VM to boot." -ForegroundColor Yellow
