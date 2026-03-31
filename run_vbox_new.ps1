# AetherOS VirtualBox - Create New VM with Latest Kernel
$VBOX = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
$VM_NAME = "AetherOS-Kernel"
$KERNEL = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel"

Write-Host "=== Creating AetherOS VM with Latest Kernel ===" -ForegroundColor Cyan

# Check if VM exists
$VMS = & $VBOX list vms
if ($VM_NAME -in $VMS) {
    Write-Host "VM '$VM_NAME' exists. Removing..." -ForegroundColor Yellow
    & $VBOX unregistervm $VM_NAME --delete
}

Write-Host "Creating new VM..." -ForegroundColor Green
& $VBOX createvm --name $VM_NAME --ostype "Linux_64" --register
& $VBOX modifyvm $VM_NAME --memory 2048 --cpus 2
& $VBOX modifyvm $VM_NAME --firmware efi64
& $VBOX storagectl $VM_NAME --name "SATA" --add sata

# Create new VDI disk
Write-Host "Creating virtual disk..." -ForegroundColor Green
& $VBOX createmedium disk --filename "D:\GitHub\AetherOS\vbox_vms\aetheros-new.vdi" --size 4096 --format VDI
& $VBOX storageattach $VM_NAME --storagectl "SATA" --port 0 --device 0 --type hdd --medium "D:\GitHub\AetherOS\vbox_vms\aetheros-new.vdi"

Write-Host "Starting VM..." -ForegroundColor Green
& $VBOX startvm $VM_NAME --type=gui
Write-Host "Done! VM should be starting..." -ForegroundColor Green
