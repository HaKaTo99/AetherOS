# Panduan Awam: Update Satu ISO di VirtualBox (AetherOS)

Panduan ini menjelaskan cara update **satu file ISO yang sama** (`out/aetheros.iso`) dengan aman, supaya tidak error lock dan tidak tertukar versi.

## Kenapa kadang gagal overwrite ISO?

Karena file `out/aetheros.iso` masih dipakai VM (masih dianggap seperti CD terpasang), sehingga proses build tidak bisa menulis ulang file.

Gejala umum:
- `Permission denied`
- `grub-mkrescue/xorriso failed`

## Langkah cepat (untuk pemula)

1. **Matikan VM benar-benar**
   - Di jendela VM: `Machine` → `Close` → pilih `Power Off`.
   - Jangan pilih `Save the machine state`.

2. **Lepas ISO dari Optical Drive VM**
   - `Devices` → `Optical Drives` → `Remove disk from virtual drive`.

3. **Tutup jendela VM**
   - Pastikan status VM di VirtualBox Manager = `Powered Off`.

4. **Build ulang ISO dari PowerShell**
   - Dari root repo (`D:\GitHub\AetherOS`):
   - `.\tools\rebuild_vm_iso.ps1 -WslDistro Ubuntu`

5. **Pasang lagi ISO yang sama**
   - Di VirtualBox: `Settings` → `Storage` → pilih Optical Drive → pilih `out/aetheros.iso`.

6. **Boot VM**
   - Start seperti biasa.

7. **Verifikasi revisi kernel**
   - Cek baris `[BUILD]` di layar boot.
   - Pastikan sesuai revisi terbaru.

## Command yang benar vs salah

Benar:
- `.\tools\rebuild_vm_iso.ps1 -WslDistro Ubuntu`

Salah (ini format link, bukan command PowerShell):
- `.[rebuild_vm_iso.ps1](http://_vscodecontentref_/...)`

## Troubleshooting singkat

- Jika masih `Permission denied`:
  - pastikan VM benar-benar `Powered Off` (bukan `Saved`).
  - pastikan ISO sudah di-remove dari Optical Drive.

- Jika command tidak dikenali:
   - jalankan dengan prefix path, mis. `.\tools\rebuild_vm_iso.ps1`.
