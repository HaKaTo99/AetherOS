# AetherOS Sovereign Visual Verification Script
# This script launches QEMU, waits for boot, and takes a framebuffer screendump.

$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$ISO = "d:\GitHub\AetherOS\out\aetheros.iso"
$SCREENSHOT = "d:\GitHub\AetherOS\out\final_verify.ppm"
$MONITOR_PORT = 4444

Write-Host "[Aether] Launching QEMU for Visual Verification..." -ForegroundColor Cyan

# 1. Start QEMU with Monitor port
$qemuArgs = @(
    "-cdrom", $ISO,
    "-m", "1024M",
    "-monitor", "telnet:127.0.0.1:$MONITOR_PORT,server,nowait",
    "-serial", "file:d:\GitHub\AetherOS\qemu_verify.log",
    "-nographic"
)

$proc = Start-Process -FilePath $QEMU -ArgumentList $qemuArgs -PassThru -WindowStyle Hidden
Write-Host "[run] QEMU started (PID: $($proc.Id)). Waiting for Sovereign Desktop (20s)..." -ForegroundColor Yellow

# 2. Wait for boot and desktop render
Start-Sleep -Seconds 20

# 3. Connect to Monitor and take screendump
Write-Host "[capture] Connecting to QEMU Monitor via TCP:$MONITOR_PORT..." -ForegroundColor Yellow
try {
    $client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", $MONITOR_PORT)
    $stream = $client.GetStream()
    $writer = New-Object System.IO.StreamWriter($stream)
    
    # Send screendump command
    $writer.WriteLine("screendump $SCREENSHOT")
    $writer.Flush()
    
    Start-Sleep -Seconds 2
    Write-Host "[ok] Screendump command sent: $SCREENSHOT" -ForegroundColor Green
    
    $client.Close()
} catch {
    Write-Host "[error] Failed to connect to QEMU monitor: $($_.Exception.Message)" -ForegroundColor Red
}

# 4. Stop QEMU
Write-Host "[cleanup] Terminating QEMU..." -ForegroundColor Yellow
Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue

Write-Host "[done] Visual Verification Sequence Complete." -ForegroundColor Green
if (Test-Path $SCREENSHOT) {
    $size = (Get-Item $SCREENSHOT).Length / 1KB
    Write-Host "[!] Screenshot generated: $SCREENSHOT ($([math]::Round($size, 1)) KB)" -ForegroundColor Cyan
} else {
    Write-Host "[!] Screenshot FAILED to generate." -ForegroundColor Red
}
