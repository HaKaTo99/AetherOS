# Build BIOS-bootable ISO via GRUB (uses WSL tools for grub-mkrescue/xorriso)

param(
    [string]$KernelSource = "",
    [string]$OutputIso = "out/aetheros.iso",
    [string]$WslDistro = "Ubuntu"
)

$ErrorActionPreference = "Stop"

# Derive repo root from script location
$ToolsRoot = $PSScriptRoot
$RepoRoot = Split-Path $ToolsRoot -Parent

function Convert-ToWslPath {
    param([string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $drive = $full.Substring(0,1).ToLowerInvariant()
    $rest = $full.Substring(2) -replace "\\", "/"
    return "/mnt/$drive/$rest"
}

function Invoke-WslCommand {
    param([string]$Command)
    $distro = $WslDistro
    if (-not $distro) { $distro = "Ubuntu" }
    $baseArgs = @("-d", $distro, "--", "bash", "-c", $Command)
    wsl @baseArgs
    if ($LASTEXITCODE -ne 0) { throw "WSL command failed on ${distro}: $Command" }
}

function Resolve-RepoPath {
    param(
        [string]$Path,
        [string]$Root
    )
    if ([string]::IsNullOrWhiteSpace($Path)) { return $Path }
    if ([System.IO.Path]::IsPathRooted($Path)) { return [System.IO.Path]::GetFullPath($Path) }
    return [System.IO.Path]::GetFullPath((Join-Path $Root $Path))
}

# Standardize Paths
$resolvedKernelSource = Resolve-RepoPath -Path $KernelSource -Root $RepoRoot
$resolvedOutputIso = Resolve-RepoPath -Path $OutputIso -Root $RepoRoot

# ISO Staging directory (canonical: iso/)
$isoDir = Join-Path $RepoRoot "iso"
$kernelTarget = Join-Path $isoDir "boot/aetheros_kernel"

if (-not (Test-Path $isoDir)) { 
    throw "ISO staging directory not found at $isoDir. Please ensure 'iso/' exists with 'boot/grub/grub.cfg'." 
}

# Ensure Output Directory exists
$outDir = Split-Path $resolvedOutputIso
if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir -Force | Out-Null
}

# Copy Kernel to Staging
if ($resolvedKernelSource -and (Test-Path $resolvedKernelSource)) {
    Write-Host "[iso ] Copying kernel: $resolvedKernelSource -> $kernelTarget" -ForegroundColor Gray
    Copy-Item -Path $resolvedKernelSource -Destination $kernelTarget -Force
} elseif (-not (Test-Path $kernelTarget)) {
    throw "Kernel payload missing at $kernelTarget. Provide -KernelSource to copy it in."
}

# WSL Dependency Check
if (-not (Get-Command wsl -ErrorAction SilentlyContinue)) {
    throw "WSL is required for grub-mkrescue/xorriso on Windows. Please install WSL and a Linux distro (Ubuntu recommended)."
}

$wslRepoRoot = Convert-ToWslPath $RepoRoot
$wslIsoOut = Convert-ToWslPath $resolvedOutputIso
$wslIsoDir = Convert-ToWslPath $isoDir

# Check deps inside WSL
$depCheck = "command -v grub-mkrescue >/dev/null && command -v xorriso >/dev/null && command -v mformat >/dev/null"
$depArgs = @("-d", $WslDistro, "--", "bash", "-c", $depCheck)
wsl @depArgs 2>$null
if ($LASTEXITCODE -ne 0) {
    throw "Missing deps in WSL ($WslDistro). Run: sudo apt update && sudo apt install -y grub-pc-bin xorriso mtools"
}

# Execute ISO creation
Write-Host "[iso ] Building ISO via WSL ($WslDistro): $OutputIso" -ForegroundColor Cyan
Invoke-WslCommand "cd $wslRepoRoot && grub-mkrescue -o $wslIsoOut $wslIsoDir"

Write-Host "[done] ISO created at $resolvedOutputIso" -ForegroundColor Green
