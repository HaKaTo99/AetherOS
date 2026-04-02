# AetherOS Legacy ISO Builder (build_iso.ps1)
# NOTE: This script is deprecated. Please use .\tools\build_iso.ps1 instead.

Write-Host "`n[!] WARNING: You are using a legacy build script (root build_iso.ps1)." -ForegroundColor Yellow
Write-Host "[!] DIVERGING TO CANONICAL TOOLS PATH: tools\build_iso.ps1" -ForegroundColor Yellow
Write-Host "----------------------------------------------------`n" -ForegroundColor Gray

# Forward to the new canonical tools path
$RepoRoot = $PSScriptRoot
$ToolsDir = Join-Path $RepoRoot "tools"
& (Join-Path $ToolsDir "build_iso.ps1") @args
