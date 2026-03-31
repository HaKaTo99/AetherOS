# AetherOS VirtualBox Test Script

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "AetherOS VirtualBox Test v1.0" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check for VBoxManage
$VBOX = Get-Command VBoxManage -ErrorAction SilentlyContinue
if (-not $VBOX) {
    Write-Host "[ERROR] VirtualBox not found!" -ForegroundColor Red
    Write-Host "Please install VirtualBox from: https://www.virtualbox.org/" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Alternative: Use QEMU instead:" -ForegroundColor Cyan
    Write-Host "  powershell -File D:\GitHub\AetherOS\TEST_QEMU.ps1" -ForegroundColor Gray
    exit 1
}

Write-Host "VirtualBox found: $($VBOX.Source)" -ForegroundColor Green
Write-Host ""

# List existing VMs
Write-Host "=== Existing VMs ===" -ForegroundColor Yellow
$vms = & VBoxManage list vms
if ($vms) {
    $vms | ForEach-Object { Write-Host "  $_" -ForegroundColor White }
} else {
    Write-Host "  No VMs found" -ForegroundColor Gray
}

Write-Host ""
Write-Host "=== Running VMs ===" -ForegroundColor Yellow
$running = & VBoxManage list runningvms
if ($running) {
    $running | ForEach-Object { Write-Host "  $_" -ForegroundColor Green }
} else {
    Write-Host "  No running VMs" -ForegroundColor Gray
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "TO BOOT AETHEROS IN VIRTUALBOX:" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Option 1: Use ISO Image" -ForegroundColor White
Write-Host "  1. Open VirtualBox Manager" -ForegroundColor Gray
Write-Host "  2. Create New VM (Type: Other, 64-bit)" -ForegroundColor Gray
Write-Host "  3. Attach ISO: D:\GitHub\AetherOS\aetheros.iso" -ForegroundColor Gray
Write-Host "  4. Start VM" -ForegroundColor Gray
Write-Host ""
Write-Host "Option 2: Use Kernel directly" -ForegroundColor White
Write-Host "  1. Create VM with EFI enabled" -ForegroundColor Gray
Write-Host "  2. Use: D:\GitHub\AetherOS\aetheros.iso (contains GRUB)" -ForegroundColor Gray
Write-Host ""
Write-Host "Current ISO: D:\GitHub\AetherOS\aetheros.iso" -ForegroundColor Cyan

# Check if aetheros.iso has GRUB
Write-Host ""
Write-Host "=== ISO Contents Check ===" -ForegroundColor Yellow
if (Test-Path "D:\GitHub\AetherOS\aetheros.iso") {
    Write-Host "[OK] ISO exists" -ForegroundColor Green
    
    # Try to mount and check contents
    Write-Host "[INFO] ISO size: $((Get-Item 'D:\GitHub\AetherOS\aetheros.iso').Length / 1MB) MB" -ForegroundColor Gray
} else {
    Write-Host "[ERROR] ISO not found" -ForegroundColor Red
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "VirtualBox Setup Required!" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Since VirtualBox is not currently running/configured," -ForegroundColor White
Write-Host "please manually set up the VM using the ISO." -ForegroundColor White
