# AetherOS VirtualBox Launcher
# Run AetherOS in VirtualBox with full keyboard support

$VMM_NAME = "AetherOS"
$VBOX_MANGE = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
$VDI_PATH = "D:\GitHub\AetherOS\vbox_vms\AetherOS.vdi"

Write-Host "=== AetherOS VirtualBox Launcher ===" -ForegroundColor Cyan

# Check if VM exists
$VMS = & $VBOX_MANGE list vms 2>$null
if ($VMS -match $VMM_NAME) {
    Write-Host "VM '$VMM_NAME' exists. Starting..." -ForegroundColor Yellow
    & $VBOX_MANGE startvm $VMM_NAME --typegui
} else {
    Write-Host "Creating new VM: $VMM_NAME" -ForegroundColor Green
    
    # Create VM
    & $VBOX_MANGE createvm --name $VMM_NAME --ostype "Linux_64" --register
    
    # Set memory
    & $VBOX_MANGE modifyvm $VMM_NAME --memory 2048 --cpus 4
    
    # Use existing VDI if available
    if (Test-Path $VDI_PATH) {
        Write-Host "Using existing VDI: $VDI_PATH" -ForegroundColor Green
        & $VBOX_MANGE storagectl $VMM_NAME --name "SATA" --add sata
        & $VBOX_MANGE storageattach $VMM_NAME --storagectl "SATA" --port 0 --device 0 --type hdd --medium $VDI_PATH
    }
    
    # Enable EFI boot
    & $VBOX_MANGE modifyvm $VMM_NAME --firmware efi64
    
    # Start VM
    Write-Host "Starting AetherOS..." -ForegroundColor Green
    & $VBOX_MANGE startvm $VMM_NAME --typegui
}

Write-Host "Done!" -ForegroundColor Green
