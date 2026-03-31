#!/usr/bin/env pwsh
# build_iso.ps1 - Build AetherOS ISO with proper GRUB setup

$ErrorActionPreference = "Stop"

$KernelPath = "D:\GitHub\AetherOS\target\x86_64-unknown-none\production\aetheros-kernel"
$KernelPathFallback = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"
$KernelPathFallback2 = "D:\GitHub\AetherOS\kernel\target\x86_64-unknown-none\release\aetheros-kernel"
$IsoPath = "D:\GitHub\AetherOS\aetheros.iso"
$IsoDir = "D:\GitHub\AetherOS\iso"

Write-Host "=== Building AetherOS ISO ===" -ForegroundColor Cyan

# Check kernel exists
if (!(Test-Path $KernelPath) -and (Test-Path $KernelPathFallback)) {
    $KernelPath = $KernelPathFallback
} elseif (!(Test-Path $KernelPath) -and (Test-Path $KernelPathFallback2)) {
    $KernelPath = $KernelPathFallback2
}

if (!(Test-Path $KernelPath)) {
    Write-Host "ERROR: Kernel not found at $KernelPath" -ForegroundColor Red
    Write-Host "Building kernel first..." -ForegroundColor Yellow
    Set-Location D:\GitHub\AetherOS
    cargo build --profile production --target x86_64-unknown-none -p aetheros-kernel

    if (!(Test-Path $KernelPath) -and (Test-Path $KernelPathFallback)) {
        $KernelPath = $KernelPathFallback
    } elseif (!(Test-Path $KernelPath) -and (Test-Path $KernelPathFallback2)) {
        $KernelPath = $KernelPathFallback2
    }
}

# Create directories
$BootGrubDir = "$IsoDir\boot\grub"
$i386PcDir = "$BootGrubDir\i386-pc"

New-Item -ItemType Directory -Force -Path $i386PcDir | Out-Null

function Invoke-PosixCommand {
    param(
        [Parameter(Mandatory = $true)][string]$Command
    )

    $oldPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        try {
            $bashOutput = (& wsl -e bash -lc $Command 2>&1)
            $bashExit = $LASTEXITCODE
        } catch {
            $bashOutput = $_
            $bashExit = 127
        }

        if ($bashExit -eq 0) {
            return @{ ExitCode = 0; Output = $bashOutput }
        }

        try {
            $shOutput = (& wsl -e sh -lc $Command 2>&1)
            $shExit = $LASTEXITCODE
        } catch {
            $shOutput = $_
            $shExit = 127
        }

        return @{ ExitCode = $shExit; Output = @($bashOutput, $shOutput) }
    } finally {
        $ErrorActionPreference = $oldPreference
    }
}

function Get-WslWorkspacePath {
    $candidates = @(
        "/mnt/d/GitHub/AetherOS",
        "/mnt/host/d/GitHub/AetherOS"
    )
    foreach ($path in $candidates) {
        $probe = Invoke-PosixCommand -Command ("test -d {0}" -f $path)
        if ($probe.ExitCode -eq 0) {
            return $path
        }
    }
    return $null
}

function Get-WslTool {
    param([Parameter(Mandatory = $true)][string]$Tool)
    $probe = Invoke-PosixCommand -Command ("command -v {0}" -f $Tool)
    if ($probe.ExitCode -eq 0 -and $probe.Output) {
        return ($probe.Output | Select-Object -First 1).ToString().Trim()
    }
    return $null
}

# Copy kernel
Write-Host "Copying kernel..." -ForegroundColor Yellow
Copy-Item $KernelPath "$IsoDir\boot\aetheros_kernel" -Force

# Copy GRUB modules from WSL
Write-Host "Copying GRUB modules from WSL..." -ForegroundColor Yellow

# Try using grub-mkrescue directly (it should work without explicit modules if structure is correct)
Write-Host "Creating ISO with GRUB..." -ForegroundColor Yellow

$workspaceWsl = Get-WslWorkspacePath
$wslGrub = Get-WslTool -Tool "grub-mkrescue"

$isoBuilt = $false

if ($workspaceWsl -and $wslGrub) {
    $createIsoCmd = "cd $workspaceWsl && grub-mkrescue -o aetheros.iso iso 2>&1"
    $wslResult = Invoke-PosixCommand -Command $createIsoCmd
    if ($wslResult.ExitCode -eq 0) {
        Write-Host "ISO created successfully via WSL grub-mkrescue." -ForegroundColor Green
        $isoBuilt = $true
    } else {
        Write-Host "WSL grub-mkrescue failed." -ForegroundColor Yellow
        if ($wslResult.Output) { Write-Host $wslResult.Output }
    }
}

if (-not $isoBuilt -and (Get-Command grub-mkrescue -ErrorAction SilentlyContinue)) {
    Write-Host "Trying host grub-mkrescue..." -ForegroundColor Yellow
    grub-mkrescue -o $IsoPath $IsoDir
    if ($LASTEXITCODE -eq 0) {
        $isoBuilt = $true
    }
}

if (-not $isoBuilt) {
    Write-Host "ERROR: Bootable ISO build requires grub-mkrescue (WSL or host)." -ForegroundColor Red
    Write-Host "Install requirement options:" -ForegroundColor Yellow
    Write-Host "  1) WSL distro with grub-mkrescue + xorriso + mtools" -ForegroundColor Yellow
    Write-Host "  2) Host grub-mkrescue toolchain in PATH" -ForegroundColor Yellow
    exit 1
}

# Verify ISO
if (Test-Path $IsoPath) {
    $isoSize = (Get-Item $IsoPath).Length / 1MB
    Write-Host "ISO created: $IsoPath ($([math]::Round($isoSize, 2)) MB)" -ForegroundColor Green
} else {
    Write-Host "ERROR: ISO creation failed!" -ForegroundColor Red
    exit 1
}
