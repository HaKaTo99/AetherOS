# AetherOS Legacy Launcher (PRODUCTION_LAUNCHER.ps1)
# NOTE: This script is deprecated. Please use .\Aether.ps1 instead.

Write-Host "`n[!] WARNING: You are using a legacy launcher script." -ForegroundColor Yellow
Write-Host "[!] DIVERGING TO CANONICAL ENTRY POINT: .\Aether.ps1" -ForegroundColor Yellow
Write-Host "----------------------------------------------------`n" -ForegroundColor Gray

# Forward to the new canonical entry point
$RepoRoot = $PSScriptRoot
& (Join-Path $RepoRoot "Aether.ps1")
