# 🛡️ AetherOS v10.3.0: Sovereign SUPREME Production Guide

Selamat Datang di tingkat **SUPREME v10.3**. Panduan ini merinci langkah-langkah kritis untuk mengoperasikan AetherOS dalam lingkungan produksi militer, edge, dan infrastruktur berdaulat dengan stabilitas **100% Visual & Kernel Handshake**.

## 🏗️ 1. Unified Build Sequence

Gunakan skrip kontrol pusat `Aether.ps1` untuk seluruh siklus hidup sistem. Status Supreme menjamin **Zero-Warning** pada setiap kompilasi.

### Membangun ISO Produksi (v10.3 Supreme)
```powershell
.\Aether.ps1 -Action build
```
Output akan tersedia di `.\out\aetheros.iso`.

## 🖥️ 2. LFB & Video Hardware Hardening

AetherOS v10.3 memperkenalkan **Hardware Mirroring Sync** untuk menjamin visualisasi pada driver VirtIO-GPU dan VESA standar.

### Visual Handshake Requirements
Jika layar muncul sebagai "Black Screen" di QEMU atau Hardware fisik:
1. Pastikan LFB dipetakan secara identitas di `0xFD000000` (atau alamat yang dilaporkan Multiboot2).
2. **Kewajiban Cache Bypass**: Setiap entri tabel halaman untuk video MMIO **WAJIB** memiliki bit `PWT` (Page-level Write-Through) dan `PCD` (Page-level Cache Disable) yang aktif.
3. Gunakan `fb.flush()` setelah setiap siklus render untuk memicu pembersihan buffer hardware.

## 🧪 3. Military Grade Validation

Sebelum deployment, sistem **WAJIB** lulus uji beban (Stress Test) dan verifikasi fungsional.
1. Jalankan `.\Aether.ps1 -Action smoke`.
2. Kriteria Lulus: Zero Panic, LFB Mapping SUCCESS (Phase 0xFD Protected), dan Heartbeat "." terdeteksi di serial log.

## 🔒 4. Sovereign Trust & Security

AetherOS menggunakan model **Zero-Trust** terdesentralisasi dengan enkripsi pasca-kuantum.
- **Root Authority**: KYBER-1024 Master Root.
- **Verification**: Tanda tangan kernel Dilithium-5 diverifikasi secara otomatis di `kernel_init`.

---

**"The Sovereign Desktop is active. Your visualization is secured."**
*AetherOS Engineering Team - xAetherOS Distribution Branch.*
