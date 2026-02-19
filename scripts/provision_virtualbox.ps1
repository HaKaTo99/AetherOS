# AetherOS: VirtualBox provisioning (ISO -> VDI)
# Usage examples:
#   .\provision_virtualbox.ps1 -IsoPath ..\aetheros_v79_r1.iso -VdiPath ..\AetherOS.vdi
#   .\provision_virtualbox.ps1 -VmName AetherOS-VM -DiskSizeGB 40 -Headless -ForceRecreateVm -ForceRecreateVdi

param(
    [string]$VmName = "AetherOS-VM",
    [string]$IsoPath = "aetheros_v79_r1.iso",
    [string]$VdiPath = "AetherOS.vdi",
    [string]$BaseFolder = "..\vbox_vms",
    [int]$DiskSizeGB = 32,
    [int]$MemoryMB = 4096,
    [int]$CpuCount = 4,
    [switch]$Headless,
    [switch]$SkipIso,
    [switch]$ForceRecreateVm,
    [switch]$ForceRecreateVdi
)

function Resolve-VBoxManage {
    $cmd = Get-Command VBoxManage -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }
    $default = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
    if (Test-Path $default) { return $default }
    throw "VBoxManage not found. Add VirtualBox to PATH or set VBOX_MANAGE env var."
}

function Assert-FileExists([string]$path, [string]$label) {
    if (-not (Test-Path $path)) { throw "$label not found: $path" }
}

$ErrorActionPreference = "Stop"
$VBoxManage = $env:VBOX_MANAGE
if (-not $VBoxManage) { $VBoxManage = Resolve-VBoxManage }

if (-not $SkipIso) {
    $IsoPath = (Resolve-Path $IsoPath).Path
    Assert-FileExists $IsoPath "ISO"
}

if (-not [System.IO.Path]::IsPathRooted($BaseFolder)) {
    $BaseFolder = Join-Path (Get-Location) $BaseFolder
}
$null = New-Item -ItemType Directory -Path $BaseFolder -Force

$VdiPath = $VdiPath.ToString()
# If VDI path is relative and has no directory, place it under BaseFolder
$vdiHasDir = [System.IO.Path]::GetDirectoryName($VdiPath)
if (-not [System.IO.Path]::IsPathRooted($VdiPath)) {
    if ([string]::IsNullOrEmpty($vdiHasDir)) {
        $VdiPath = Join-Path $BaseFolder $VdiPath
    } else {
        $VdiPath = Join-Path (Get-Location) $VdiPath
    }
}
$vdiDir = Split-Path $VdiPath -Parent
$null = New-Item -ItemType Directory -Path $vdiDir -Force

# Handle existing VM
$vmPattern = '"' + [regex]::Escape($VmName) + '"'
$vmList = (& $VBoxManage list vms) -match $vmPattern
if ($vmList) {
    if ($ForceRecreateVm) {
        try { & $VBoxManage controlvm $VmName poweroff 2>$null } catch {}
        Start-Sleep -Seconds 1
        & $VBoxManage unregistervm $VmName --delete
    } else {
        Write-Host "VM '$VmName' already exists. Use -ForceRecreateVm to replace." -ForegroundColor Yellow
    }
}

# Create VM if missing
$vmList = (& $VBoxManage list vms) -match $vmPattern
if (-not $vmList) {
    & $VBoxManage createvm --name $VmName --ostype "Linux_64" --register --basefolder $BaseFolder
    & $VBoxManage modifyvm $VmName --memory $MemoryMB --cpus $CpuCount --vram 32 --ioapic on --graphicscontroller vmsvga --nic1 nat
    if ($SkipIso) {
        & $VBoxManage modifyvm $VmName --boot1 disk --boot2 none
    } else {
        & $VBoxManage modifyvm $VmName --boot1 dvd --boot2 disk
    }
}

# Prepare VDI
if ((Test-Path $VdiPath -PathType Leaf) -and $ForceRecreateVdi) {
    Remove-Item $VdiPath -Force
}
if (-not (Test-Path $VdiPath)) {
    & $VBoxManage createhd --filename $VdiPath --size ($DiskSizeGB * 1024)
}

# Storage controllers
& $VBoxManage storagectl $VmName --name "SATA" --add sata --controller IntelAhci 2>$null
& $VBoxManage storagectl $VmName --name "IDE" --add ide 2>$null

# Attach disk and ISO
& $VBoxManage storageattach $VmName --storagectl "SATA" --port 0 --device 0 --type hdd --medium $VdiPath
if (-not $SkipIso) {
    & $VBoxManage storageattach $VmName --storagectl "IDE" --port 1 --device 0 --type dvddrive --medium $IsoPath
}

# Start VM
if ($Headless) {
    $startType = "headless"
} else {
    $startType = "gui"
}

& $VBoxManage startvm $VmName --type $startType
if ($SkipIso) {
    Write-Host "VM '$VmName' started from disk only. VDI: $VdiPath" -ForegroundColor Green
} else {
    Write-Host "VM '$VmName' started with ISO attached. Install to $VdiPath then detach ISO." -ForegroundColor Green
}
