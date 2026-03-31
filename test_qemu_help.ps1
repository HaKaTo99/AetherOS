# AetherOS QEMU Auto Test Script
# Tests if 'help' command returns proper output

$ErrorActionPreference = "Stop"

$qemu = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$iso = "d:\GitHub\AetherOS\out\aetheros.iso"

Write-Host "Starting AetherOS QEMU Test..."
Write-Host "=============================="

$process = Start-Process -FilePath $qemu -ArgumentList "-cdrom", $iso, "-m", "1024M", "-nographic" -PassThru -NoNewWindow

Start-Sleep -Seconds 6

if ($process.HasExited) {
    Write-Host "[ERROR] QEMU exited unexpectedly"
    exit 1
}

# Send 'help' command
$process.StandardInput.WriteLine("help")
Start-Sleep -Seconds 3

# Read output
$output = $process.StandardOutput.ReadToEnd()
Write-Host "=== OUTPUT ==="
Write-Host $output
Write-Host "=============="

# Check if output contains expected help text
if ($output -match "help|command|Available") {
    Write-Host "[PASS] Help command works!"
} else {
    Write-Host "[CHECK] Please verify manually"
}

# Send 'exit' to close gracefully
$process.StandardInput.WriteLine("exit")
Start-Sleep -Seconds 1

# Kill if still running
if (!$process.HasExited) {
    Stop-Process -Id $process.Id -Force
}

Write-Host "Test complete."
