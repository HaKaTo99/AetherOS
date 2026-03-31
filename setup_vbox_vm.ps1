# AetherOS VirtualBox VM Setup Script
# Creates a new VM and configures it to boot the AetherOS kernel

$ErrorActionPreference = "Stop"

# Configuration
$vmName = "AetherOS"
$kernelPath = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"
$memoryMB = 2048
$cpus = 2

Write-Host "=== AetherOS VirtualBox Setup ===" -ForegroundColor Cyan

# Check if kernel exists
if (-not (Test-Path $kernelPath)) {
    Write-Host "ERROR: Kernel not found at $kernelPath" -ForegroundColor Red
    Write-Host "Please build the kernel first: cd kernel && cargo build --release --target x86_64-unknown-none" -ForegroundColor Yellow
    exit 1
}

Write-Host "[1/5] Checking for existing VM..." -ForegroundColor Yellow
$vboxmanage = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"

# Check if VM exists and remove if present
$vmExists = & $vboxmanage list vms | Select-String -Pattern $vmName -Quiet
if ($vmExists) {
    Write-Host "Removing existing VM..." -ForegroundColor Yellow
    & $vboxmanage controlvm $vmName poweroff 2>$null
    Start-Sleep -Seconds 2
    & $vboxmanage unregistervm $vmName --delete 2>$null
    Start-Sleep -Seconds 1
} else {
    Write-Host "No existing VM found, will create new one." -ForegroundColor Green
}

Write-Host "[2/5] Creating new VM..." -ForegroundColor Yellow
# Use D: drive for VM storage since F: doesn't exist
& $vboxmanage createvm --name $vmName --ostype "Linux26_64" --register --basefolder "D:\VirtualBox VMs"

Write-Host "[3/5] Configuring VM settings..." -ForegroundColor Yellow
& $vboxmanage modifyvm $vmName --memory $memoryMB --cpus $cpus
& $vboxmanage modifyvm $vmName --vram 16
& $vboxmanage modifyvm $vmName --graphicscontroller vmsvga
& $vboxmanage modifyvm $vmName --nic1 nat
& $vboxmanage modifyvm $vmName --boot1 kernel --boot2 none --boot3 none --boot4 none

# Configure kernel boot
& $vboxmanage modifyvm $vmName --kernelpath $kernelPath

Write-Host "[4/5] Creating SATA controller..." -ForegroundColor Yellow
& $vboxmanage storagectl $vmName --name "SATA" --add sata --controller IntelAhci
& $vboxmanage storageattach $vmName --storagectl "SATA" --port 0 --device 0 --type hdd --medium "none"

Write-Host "[5/5] VM Created Successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "VM Name: $vmName" -ForegroundColor White
Write-Host "Memory: $memoryMB MB" -ForegroundColor White
Write-Host "CPUs: $cpus" -ForegroundColor White
Write-Host "Kernel: $kernelPath" -ForegroundColor White
Write-Host ""

# Start the VM
Write-Host "Starting VirtualBox..." -ForegroundColor Cyan
& $vboxmanage startvm $vmName

Write-Host ""
Write-Host "=== DONE ===" -ForegroundColor Green
Write-Host "The VirtualBox window should open now." -ForegroundColor Cyan
