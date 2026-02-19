# AetherOS v10.0 "The Fabric" - Simulation Gateway
# (c) 2026 Architect herman x Antigravity

Write-Host "🌌 xAetherOS v10.0: Mengaktifkan Gerbang Simulasi 'The Fabric'..." -ForegroundColor Cyan

# Panggil skrip otomasi utama dengan paksa build jika perlu
if (Test-Path "scripts\run_aetheros_x86.ps1") {
    powershell.exe -ExecutionPolicy Bypass -File ".\scripts\run_aetheros_x86.ps1" -ShowDisplay
}
else {
    Write-Host "❌ Skrip otomasi tidak ditemukan di folder scripts." -ForegroundColor Red
}
