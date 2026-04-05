$content = Get-Content -Raw 'target\qemu-smoke.log'
$markers = @('HAL Initialized','GDT/IDT Initialized','[SMOKE] AetherShell-PRE')
foreach ($m in $markers) {
    if ($content -match [regex]::Escape($m)) {
        Write-Host "FOUND: $m"
    } else {
        Write-Host "MISSING: $m"
    }
}
