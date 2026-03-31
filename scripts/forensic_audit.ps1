Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# AetherOS v10.0 "The Fabric" - Forensic Runtime Audit v6
# (c) 2026 Architect herman x Antigravity

$IMAGE_PATH = "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\bootimage-aetheros-kernel.bin"
$LOG_FILE = "D:\GitHub\AetherOS\runtime_audit.log"
$TIMEOUT_SECONDS = 30

Write-Host "[ AUDIT ]: Menyiapkan lingkungan audit forensik v6 (Native QEMU File)..." -ForegroundColor Cyan

function Assert-Tool($name) {
    if (-not (Get-Command $name -ErrorAction SilentlyContinue)) {
        throw "Executable not found: $name"
    }
}

Assert-Tool "qemu-system-x86_64"

# 1. Kill any existing QEMU processes
Get-Process qemu-system-x86_64 -ErrorAction SilentlyContinue | Stop-Process -Force

# 2. Rebuild kernel
Write-Host "[ AUDIT ]: Melakukan kedaulatan build..." -ForegroundColor Cyan
Push-Location "$PSScriptRoot\..\kernel"
& cargo +nightly -Z bindeps bootimage --release
Pop-Location

# 3. Launch QEMU with native file redirection
Write-Host "[ AUDIT ]: Meluncurkan QEMU (Native Serial File, $TIMEOUT_SECONDS detik)..." -ForegroundColor Cyan
if (Test-Path $LOG_FILE) { Remove-Item $LOG_FILE }

$qemuArgs = "-drive format=raw,file=`"$IMAGE_PATH`" -m 512M -serial file:`"$LOG_FILE`" -display none -no-reboot -no-shutdown -nographic"
Write-Host "[ EXEC ]: qemu-system-x86_64 $qemuArgs" -ForegroundColor Gray

$process = Start-Process qemu-system-x86_64 -ArgumentList $qemuArgs -PassThru -WindowStyle Hidden

Write-Host "[ AUDIT ]: Menunggu eksekusi harmonisasi..." -ForegroundColor Yellow
if (-not $process.WaitForExit($TIMEOUT_SECONDS * 1000)) {
    Write-Warning "QEMU melebihi timeout ${TIMEOUT_SECONDS}s; menghentikan proses"
    try { $process | Stop-Process -Force } catch {}
}

# 4. Cleanup state
if (-not $process.HasExited) { $process | Stop-Process -Force }

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
        Start-Sleep -Seconds 2
        Get-Content $LOG_FILE
    }
}
else {
    Write-Host "[ ERROR ]: File log tidak tercipta." -ForegroundColor Red
}
