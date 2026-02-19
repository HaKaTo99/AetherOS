# AetherOS v10.0 "The Fabric" - Forensic Runtime Audit v6
# (c) 2026 Architect herman x Antigravity

$IMAGE_PATH = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\bootimage-aetheros-kernel.bin"
$LOG_FILE = "D:\GitHub\AetherOS\runtime_audit.log"

Write-Host "[ AUDIT ]: Menyiapkan lingkungan audit forensik v6 (Native QEMU File)..." -ForegroundColor Cyan

# 1. Kill any existing QEMU processes
Get-Process qemu-system-x86_64 -ErrorAction SilentlyContinue | Stop-Process -Force

# 2. Rebuild kernel
Write-Host "[ AUDIT ]: Melakukan kedaulatan build..." -ForegroundColor Cyan
Push-Location "$PSScriptRoot\..\kernel"
cargo bootimage --release
Pop-Location

# 3. Launch QEMU with native file redirection
Write-Host "[ AUDIT ]: Meluncurkan QEMU (Native Serial File, 30 detik)..." -ForegroundColor Cyan
if (Test-Path $LOG_FILE) { Remove-Item $LOG_FILE }

# Jalankan QEMU secara langsung (tidak dalam job agar interaksi COM lebih lancar di Windows)
# Gunakan -serial file: agar QEMU sendiri yang menulis file
$qemuArgs = "-drive format=raw,file=`"$IMAGE_PATH`" -m 512M -serial file:`"$LOG_FILE`" -display none"
Write-Host "[ EXEC ]: qemu-system-x86_64 $qemuArgs" -ForegroundColor Gray

# Start QEMU as a background process
$process = Start-Process qemu-system-x86_64 -ArgumentList $qemuArgs -PassThru -WindowStyle Hidden

Write-Host "[ AUDIT ]: Menunggu eksekusi harmonisasi..." -ForegroundColor Yellow
for ($i = 0; $i -lt 30; $i++) {
    Write-Progress -Activity "Melakukan Audit Forensik" -Status "Detik ke-$i dari 30" -PercentComplete (($i / 30) * 100)
    Start-Sleep -Seconds 1
}

# 4. Cleanup
Write-Host "[ AUDIT ]: Menghentikan proses audit..." -ForegroundColor Cyan
if (!$process.HasExited) { $process | Stop-Process -Force }

# 5. Read log
if (Test-Path $LOG_FILE) {
    $content = Get-Content $LOG_FILE
    if ($content) {
        Write-Host "`n--- ISI LOG AUDIT FORENSIK (Military Grade) ---`n" -ForegroundColor White
        $content
        Write-Host "`n---------------------------------------------`n" -ForegroundColor White
    }
    else {
        Write-Host "[ WARNING ]: Log masih kosong. Mencoba mode DEBUG..." -ForegroundColor Red
        # Terkadang QEMU butuh waktu untuk flush ke file
        Start-Sleep -Seconds 2
        Get-Content $LOG_FILE
    }
}
else {
    Write-Host "[ ERROR ]: File log tidak tercipta." -ForegroundColor Red
}
