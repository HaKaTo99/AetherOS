param(
    [string]$WslDistro = "Ubuntu",
    [string]$KernelSource = "target/x86_64-unknown-none/release/aetheros-kernel"
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path $PSScriptRoot

function Resolve-RepoPath {
    param(
        [string]$Path,
        [string]$RepoRoot
    )

    if ([System.IO.Path]::IsPathRooted($Path)) {
        return $Path
    }

    return Join-Path $RepoRoot $Path
}

$resolvedKernelSource = Resolve-RepoPath -Path $KernelSource -RepoRoot $repoRoot
$singleIsoRelativePath = "out/aetheros.iso"
$resolvedOutputIso = Resolve-RepoPath -Path $singleIsoRelativePath -RepoRoot $repoRoot
$displayOutputIso = $singleIsoRelativePath
$buildId = "INPUT-STABLE-" + (Get-Date -Format "yyyy-MM-dd-HHmmss")

Write-Host "[1/3] Building kernel (release)..."
Push-Location $repoRoot
$oldBuildId = $env:AETHER_BUILD_ID
$env:AETHER_BUILD_ID = $buildId
cargo build -p aetheros-kernel --release
if ($null -ne $oldBuildId) {
    $env:AETHER_BUILD_ID = $oldBuildId
} else {
    Remove-Item Env:\AETHER_BUILD_ID -ErrorAction SilentlyContinue
}
Pop-Location
if ($LASTEXITCODE -ne 0) {
    throw "Kernel build failed."
}

Write-Host "[2/3] Rebuilding ISO: $displayOutputIso"
& "$PSScriptRoot/build_iso.ps1" -KernelSource $resolvedKernelSource -OutputIso $resolvedOutputIso -WslDistro $WslDistro
if ($LASTEXITCODE -ne 0) {
    throw "ISO build failed. If the ISO file is locked, power off VM first."
}

Write-Host "[3/3] Done"
Write-Host "Build marker: $buildId"
Write-Host "ISO ready: $resolvedOutputIso"
Write-Host "Workflow mode: SINGLE ISO (always out/aetheros.iso)"
Write-Host "Tip: Power off VM (no Save State), mount this ISO, then boot."
