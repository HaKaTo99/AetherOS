# TODO - Perbaikan Shell Command Resolution

## Tujuan
Perbaiki bug di shell.rs dimana command help, calc, clear, exit jatuh ke "Unknown" karena resolusi yang salah di `resolve_primary_command`.

## Langkah-langkah:

- [x] 1. Perbaiki fungsi `resolve_primary_command` - cek command lengkap bukan hanya 2 karakter
- [x] 2. Build kernel - BERHASIL
- [ ] 3. Build ISO baru

## Detail Perbaikan:

### 1. Fix resolve_primary_command (baris ~660)
- Current: Hanya cek 2 karakter pertama
- Fix: Validasi dengan `starts_with` untuk command lengkap
- Status: SELESAI - Build berhasil

### 2. Build ISO
- Jalankan rebuild_vm_iso.ps1
