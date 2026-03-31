# AetherOS v10.2 "Supreme Grade" - Simulation Gateway
# (c) 2026 Architect herman x Antigravity

Write-Host "xAetherOS v10.2: Mengaktifkan Gerbang Simulasi 'Supreme Grade'..." -ForegroundColor Cyan

# Panggil booter untuk build & run
if (Test-Path "$PSScriptRoot\booter") {
    Push-Location "$PSScriptRoot\booter"
    Write-Host "Mencoba kompilasi sistem..." -ForegroundColor Gray
    
    # Menjalankan build via booter
    cargo -Z bindeps run --release
    $cargoResult = $LASTEXITCODE
    
    if ($cargoResult -ne 0) {
        Write-Host "Kompilasi gagal (kemungkinan masalah toolchain/network)." -ForegroundColor Yellow
        Write-Host "Menjalankan ISO terakhir yang berhasil dibangun..." -ForegroundColor Cyan
        Pop-Location
        if (Test-Path "$PSScriptRoot\aetheros.iso") {
            qemu-system-x86_64 -drive "format=raw,file=aetheros.iso" -m 512M -serial stdio
        }
        else {
            Write-Host "ISO tidak ditemukan. Tidak ada yang bisa dijalankan." -ForegroundColor Red
        }
    }
    else {
        Pop-Location
    }
}
else {
    Write-Host "Folder booter tidak ditemukan." -ForegroundColor Red
    if (Test-Path "$PSScriptRoot\aetheros.iso") {
        Write-Host "Menjalankan ISO yang sudah ada: aetheros.iso..." -ForegroundColor Yellow
        qemu-system-x86_64 -drive "format=raw,file=aetheros.iso" -m 512M -serial stdio
    }
}
