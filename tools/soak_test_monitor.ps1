# Soak Test Monitor Script for AetherOS
# Jalankan di VM host AetherOS untuk monitoring otomatis
# Logging setiap 5 menit selama 24 jam (default)

param(
    [int]$DurationHours = 24,
    [int]$IntervalMinutes = 5,
    [string]$LogFile = "D:/GitHub/AetherOS/out/soak_test_log.txt"
)

$endTime = (Get-Date).AddHours($DurationHours)
"[SOAK TEST] Start: $(Get-Date)" | Out-File -FilePath $LogFile -Encoding utf8

while ((Get-Date) -lt $endTime) {
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $mem = Get-Process | Where-Object {$_.ProcessName -eq "VBoxHeadless" -or $_.ProcessName -eq "qemu-system-x86_64"} | Select-Object ProcessName,CPU,PM,WS,NPM,StartTime
    $log = "[$timestamp] VM Resource: $($mem | Out-String)"
    $log | Out-File -FilePath $LogFile -Append -Encoding utf8
    # Optional: tail log serial/console output
    if (Test-Path "D:/GitHub/AetherOS/out/serial.log") {
        $serial = Get-Content "D:/GitHub/AetherOS/out/serial.log" -Tail 20
        $serial | ForEach-Object { "[$timestamp] SERIAL: $_" | Out-File -FilePath $LogFile -Append -Encoding utf8 }
    }
    Start-Sleep -Seconds ($IntervalMinutes * 60)
}
"[SOAK TEST] End: $(Get-Date)" | Out-File -FilePath $LogFile -Append -Encoding utf8
