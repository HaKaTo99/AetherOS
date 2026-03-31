# Multi-Node/Cluster Test Guide for AetherOS

## Tujuan
Panduan automasi pengujian distributed mesh, migration, dan failover pada 3 VM/instance AetherOS.

## Langkah Automasi

1. **Siapkan 3 VM/instance AetherOS**
   - Pastikan ISO hasil build terbaru sudah tersedia.
   - Konfigurasi network (NAT/Bridged/Host-only) agar VM saling terhubung.

2. **Boot Semua VM**
   - Jalankan VM secara bersamaan.
   - Pastikan setiap VM mendapatkan IP dan dapat saling ping.

3. **Validasi Mesh & Discovery**
   - Cek log boot: pastikan marker mesh/discovery muncul di semua VM.
   - Pastikan node saling mengenali di log (peer discovery OK).

4. **Automasi Distributed Migration**
   - Trigger migrasi task dari node 1 ke node 2, dan node 2 ke node 3.
   - Cek log: pastikan migrate_task sukses dan status mesh update.

5. **Simulasi Failover**
   - Matikan salah satu VM, pastikan node lain mendeteksi dan recovery berjalan.
   - Nyalakan kembali VM, cek rejoin mesh.

6. **Kumpulkan Log & Export**
   - Salin semua log boot, serial, dan event dari setiap VM ke folder out/cluster_logs.
   - Export summary hasil pengujian ke file JSON/CSV.

## Tips
- Gunakan snapshot VM untuk mempercepat recovery.
- Untuk pengujian lanjutan, tambahkan node hingga 5+ untuk stress test mesh.
- Dokumentasikan semua marker, error, dan event penting.

---

**Setelah pengujian, update hasil dan milestone di MASTER_TODO.md.**
