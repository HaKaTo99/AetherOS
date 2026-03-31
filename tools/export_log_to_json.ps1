# Export Log & Audit Hasil Distributed ke JSON
# Jalankan di folder out/cluster_logs atau out/

param(
    [string]$InputLog = "D:/GitHub/AetherOS/out/soak_test_log.txt",
    [string]$OutputJson = "D:/GitHub/AetherOS/out/soak_test_summary.json"
)

$lines = Get-Content $InputLog
$events = @()
foreach ($line in $lines) {
    if ($line -match '\[(.*?)\] (.*)') {
        $events += [PSCustomObject]@{
            Timestamp = $matches[1]
            Message = $matches[2]
        }
    }
}
$events | ConvertTo-Json -Depth 4 | Out-File -FilePath $OutputJson -Encoding utf8
Write-Host "Exported $($events.Count) events to $OutputJson"
