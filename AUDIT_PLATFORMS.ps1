# AetherOS Platform Audit & Setup Script
# Tests: PowerShell, CMD, QEMU, VirtualBox

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "AetherOS Platform Audit & Setup v1.0" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Continue"
$results = @{}

# 1. Check Kernel Binary
Write-Host "[1/5] Checking Kernel Binary..." -ForegroundColor Yellow
$KERNEL_PATH = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"
if (Test-Path $KERNEL_PATH) {
    $size = (Get-Item $KERNEL_PATH).Length / 1MB
    Write-Host "   [OK] Kernel found: $size MB" -ForegroundColor Green
    $results["Kernel"] = "OK"
} else {
    Write-Host "   [FAIL] Kernel not found at $KERNEL_PATH" -ForegroundColor Red
    Write-Host "   Building kernel..." -ForegroundColor Yellow
    $results["Kernel"] = "NEED_BUILD"
}

# 2. Check QEMU
Write-Host "[2/5] Checking QEMU..." -ForegroundColor Yellow
$QEMU_PATH = "C:\Program Files\qemu\qemu-system-x86_64.exe"
if (Test-Path $QEMU_PATH) {
    Write-Host "   [OK] QEMU found: $QEMU_PATH" -ForegroundColor Green
    $results["QEMU"] = "OK"
} else {
    Write-Host "   [FAIL] QEMU not found" -ForegroundColor Red
    $results["QEMU"] = "NOT_FOUND"
}

# 3. Check VirtualBox
Write-Host "[3/5] Checking VirtualBox..." -ForegroundColor Yellow
$VBOX = Get-Command VBoxManage -ErrorAction SilentlyContinue
if ($VBOX) {
    Write-Host "   [OK] VirtualBox found: $($VBOX.Source)" -ForegroundColor Green
    $vboxVersion = & VBoxManage --version 2>$null
    Write-Host "   Version: $vboxVersion" -ForegroundColor Gray
    $results["VirtualBox"] = "OK"
} else {
    Write-Host "   [FAIL] VirtualBox not found" -ForegroundColor Red
    $results["VirtualBox"] = "NOT_FOUND"
}

# 4. Check ISO Image
Write-Host "[4/5] Checking ISO Image..." -ForegroundColor Yellow
$ISO_PATH = "D:\GitHub\AetherOS\aetheros.iso"
if (Test-Path $ISO_PATH) {
    $isoSize = (Get-Item $ISO_PATH).Length / 1MB
    Write-Host "   [OK] ISO found: $isoSize MB" -ForegroundColor Green
    $results["ISO"] = "OK"
} else {
    Write-Host "   [WARN] ISO not found" -ForegroundColor Yellow
    $results["ISO"] = "NOT_FOUND"
}

# 5. Test PowerShell & CMD Environment
Write-Host "[5/5] Testing Shell Environments..." -ForegroundColor Yellow
Write-Host "   [OK] PowerShell: $($PSVersionTable.PSVersion)" -ForegroundColor Green
$results["PowerShell"] = "OK"

# CMD check
$cmdTest = cmd /c "echo CMD_TEST_OK"
if ($cmdTest -eq "CMD_TEST_OK") {
    Write-Host "   [OK] CMD: Working" -ForegroundColor Green
    $results["CMD"] = "OK"
} else {
    Write-Host "   [FAIL] CMD: Not working" -ForegroundColor Red
    $results["CMD"] = "FAIL"
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "AUDIT SUMMARY" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
$results.GetEnumerator() | ForEach-Object { 
    $color = if ($_.Value -eq "OK" -or $_.Value -eq "OK") { "Green" } else { "Yellow" }
    Write-Host "  $($_.Key): $($_.Value)" -ForegroundColor $color 
}

Write-Host ""
if ($results["Kernel"] -eq "NEED_BUILD") {
    Write-Host "Action Required: Build kernel first!" -ForegroundColor Red
}

Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Cyan
Write-Host "  1. Run: powershell -File D:\GitHub\AetherOS\TEST_QEMU.ps1" -ForegroundColor Gray
Write-Host "  2. Run: powershell -File D:\GitHub\AetherOS\TEST_VBOX.ps1" -ForegroundColor Gray
Write-Host "  3. Type 'help' in each environment to test input" -ForegroundColor Gray
