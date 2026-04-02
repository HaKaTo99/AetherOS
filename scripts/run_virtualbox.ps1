# xAetherOS v10.2: Oracle VirtualBox Automation Script
# Author: Antigravity (Sovereign AI Architect)
# License: Military Grade Harmony Certification

$VBOX_MANAGE = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
$VM_NAME = "AetherOS_TheFabric"
$RAW_IMAGE = "target\x86_64-unknown-none\release\bootimage-aetheros-kernel.bin"
$VDI_IMAGE = "target\x86_64-unknown-none\release\aetheros_fabric.vdi"

Write-Host "`n🌌 [xAetherOS v10.2]: Memulai Integrasi VirtualBox..." -ForegroundColor Cyan

# 1. Check Source Binary
if (-not (Test-Path $RAW_IMAGE)) {
    Write-Host "❌ [ERROR]: Kernel binary tidak ditemukan! Silakan build dulu menggunakan cargo bootimage." -ForegroundColor Red
    exit 1
}

# 2. Convert Raw to VDI
Write-Host "🛡️ [1/5]: Mengonversi Binary ke format VDI..." -ForegroundColor Yellow
if (Test-Path $VDI_IMAGE) { Remove-Item $VDI_IMAGE -Force }

& $VBOX_MANAGE convertfromraw $RAW_IMAGE $VDI_IMAGE --format VDI
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ [ERROR]: Konversi VDI gagal!" -ForegroundColor Red
    exit 1
}

# 3. Handle Existing VM
Write-Host "🛡️ [2/5]: Menyiapkan Virtual Machine..." -ForegroundColor Yellow
$vm_exists = & $VBOX_MANAGE list vms | Select-String $VM_NAME
if ($vm_exists) {
    Write-Host "🔄 [INFO]: VM '$VM_NAME' terdeteksi. Mereset konfigurasi..." -ForegroundColor Cyan
    & $VBOX_MANAGE controlvm $VM_NAME poweroff 2>$null
    Start-Sleep -Seconds 1
    & $VBOX_MANAGE unregistervm $VM_NAME --delete
}

# 4. Create and Configure VM
Write-Host "🛡️ [3/5]: Melakukan Provisioning VM..." -ForegroundColor Yellow
& $VBOX_MANAGE createvm --name $VM_NAME --ostype "Other_64" --register
& $VBOX_MANAGE modifyvm $VM_NAME --memory 512 --vram 16 --boot1 disk --nic1 nat --nictype1 e1000 --uart1 0x3F8 4 --uartmode1 "file" "$(Get-Location)\vbox_serial.log"

# 5. Add Storage Controller and Attach VDI
Write-Host "🛡️ [4/5]: Menghubungkan Media Sovereignty (VDI)..." -ForegroundColor Yellow
& $VBOX_MANAGE storagectl $VM_NAME --name "SATA Controller" --add sata --controller IntelAhci
& $VBOX_MANAGE storageattach $VM_NAME --storagectl "SATA Controller" --port 0 --device 0 --type hdd --medium $VDI_IMAGE

# 6. Start VM
Write-Host "🛡️ [5/5]: Meluncurkan AetherOS v10.2 di Oracle VirtualBox! 🚀" -ForegroundColor Green
& $VBOX_MANAGE startvm $VM_NAME --type separate

Write-Host "`n✅ [SUCCESS]: VM Berhasil Diluncurkan." -ForegroundColor Green
Write-Host "Log serial tersedia di: vbox_serial.log" -ForegroundColor Cyan
