# 🌌 AetherOS

**Version 10.2.0** — Supreme Grade: TERKONFIRMASI (Sovereign Framework Hardened)

![AetherOS](https://img.shields.io/badge/AetherOS-v10.2_Supreme_Grade-blueviolet)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen)
![Certification](https://img.shields.io/badge/Certification-Universal_Sovereignty-gold)

---

### 📖 **[BACA DULU: PROSPEKTUS UNIVERSAL AETHEROS](docs/PROSPECTUS.md)**
**Manifesto Kedaulatan Digital untuk Seluruh Umat Manusia.**

---

## 🚀 How to Run AetherOS (v10.2 Supreme)

AetherOS v10.2 menggunakan **Cognitive Boot Sequence** yang dioptimalkan untuk QEMU, Raspberry Pi 4, dan hardware native.

### Fast Launch (Canonical Entry Point)
AetherOS kini menggunakan satu pintu utama untuk mempermudah operasional:

```powershell
# Gunakan script Aether untuk Build, Run, atau Test
.\Aether.ps1
```

### Manual Build & Launch
1. **Build Kernel**:
   ```powershell
   cd kernel
   cargo run --release
   ```
2. **Access Shell**: Gunakan terminal serial untuk berinteraksi dengan **AetherAI** di Ring 0.

### VirtualBox Workflow (Single ISO)
Gunakan satu file ISO tetap agar tidak tertukar versi saat debugging:

- Panduan awam lengkap: [VM Single ISO Beginner Guide](docs/guides/VM_SINGLE_ISO_BEGINNER_GUIDE.md)

```powershell
.\tools\rebuild_vm_iso.ps1 -WslDistro Ubuntu
```

- Script ini akan:
   1. build kernel release,
   2. copy kernel terbaru ke staging `iso/boot/aetheros_kernel`,
   3. rebuild `out/aetheros.iso`.
- Di VirtualBox, selalu **Power Off** VM (bukan Save State) sebelum overwrite ISO.
- Setelah boot, cek marker `[BUILD]` di layar untuk memastikan versi terbaru benar-benar ter-load.

#### 15-Detik Checklist (Sebelum Boot)
1. Jalankan: `.\tools\rebuild_vm_iso.ps1 -WslDistro Ubuntu`
2. Pastikan output berakhir dengan `ISO ready:` tanpa error.
3. VM status: **Powered Off** (bukan Saved).
4. Optical media menunjuk ke `out/aetheros.iso`.
5. Setelah boot, verifikasi baris `[BUILD]` sesuai revisi terbaru sebelum mengetes command.

### Smoke Test (CI parity, Linux)
```bash
chmod +x scripts/qemu-smoke.sh
scripts/qemu-smoke.sh
```
- Build dengan Rust nightly + target `x86_64-unknown-none`, lalu boot via QEMU headless.
- Log tersimpan di `target/qemu-smoke.log`; tes lulus jika marker `AetherShell>` muncul.

### Smoke Test (Windows/PowerShell)
```powershell
powershell -ExecutionPolicy Bypass -File scripts/qemu-smoke.ps1
```
- Menggunakan toolchain Rust nightly, QEMU headless, log di `target/qemu-smoke.log`. Marker bisa dioverride via env `REQUIRED_MARKERS` (baris-per-baris).

### Debug Preset (GDB attach)
```bash
chmod +x scripts/qemu-debug.sh
scripts/qemu-debug.sh
# lalu di terminal lain: gdb -ex "target remote :1234" target/x86_64-unknown-none/release/aetheros-kernel
```
- QEMU berjalan dengan `-s -S` sehingga kernel berhenti di awal; lanjutkan via GDB.

---

## 💎 Key Features: The Sovereign Fabric (v10.2)

### 🧠 Intelligence Fabric
- **Cognitive Intent Parser**: Memprediksi niat syscall untuk optimasi preemptif (v10.0).
- **AetherAI Local Edge**: Integrasi Llama-7B native pada akselerator NPU (v9.0).
- **Proactive SMME Scaling**: Manajemen memori yang menyesuaikan kapasitas secara otomatis berdasarkan prediksi beban kerja.

### 🛡️ Post-Quantum Security
- **Quantum Fortress (v6.0)**: Algoritma Kyber/Dilithium default untuk seluruh jalur IPC.
- **Micro-Kernel Isolation**: Segmentasi memori militer dengan `W^X` dan `KASLR` tingkat lanjut.
- **Sovereign RBAC**: Kontrol akses berbasis peran yang terintegrasi secara kriptografis (v8.0).

### 🌐 Global Self-Healing Mesh
- **Quantum Bus (v5.0)**: Protokol komunikasi terdistribusi lintas perangkat tanpa pusat kendali (Decentralized).
- **Ability Economy**: Ekosistem di mana node dapat berbagi GPU/NPU secara otonom (v7.0).
- **Failover <500ms**: Pemulihan mandiri mesh jika node utama mengalami gangguan.

---

## 📂 Project Structure

```text
├── Aether.ps1       # Entry Point Utama (Build, Run, Test)
├── tools/           # Skrip Build Kanonikal (rebuild_vm_iso.ps1)
├── scripts/         # Utilitas Testing (qemu-smoke.ps1)
├── boot/           # Konfigurasi bootloader (GRUB/UEFI)
├── kernel/         # Source code kernel Rust (SMME, Scheduler, AI)
├── out/            # Output binari standar (aetheros.iso)
└── docs/           # Basis Pengetahuan Terfaktorisasi (v10.0)
```

---

## 📖 Documentation Hub
Untuk pengalaman eksplorasi yang sinkron dan tanpa redundansi:

- **[Master Index](docs/MASTER_INDEX.md)** — Satu-satunya gerbang menuju seluruh pengetahuan AetherOS.
- **[Strategic Vision](docs/STRATEGIC_VISION.md)** — Memahami peta jalan menuju Singularity v30.0.
- **[Developer Guide](docs/DEVELOPER_GUIDE.md)** — Instruksi build mendalam dan kontribusi.
- **[Master Roadmap](docs/MASTER_TODO.md)** — Status 30/30 Fase: Singularity Era Seeded.

---

## 📊 Project Maturity Matrix

| Milestone | Status | Era |
|-----------|--------|-----|
| Core Foundation | ✅ Complete | v1.0 |
| Distributed Mesh | ✅ Complete | v2.0 |
| Cross-Platform Bridge | ✅ Complete | v3.0 |
| Enterprise & Cloud | ✅ Complete | v4.0 |
| Post-Quantum Security| ✅ Complete | v6.0 |
| Self-Healing Mesh | ✅ Complete | v7.0 |
| Universal Intelligence| ✅ Complete | v9.0 |
| **The Fabric (Sovereign)** | ✅ **PREMIUM** | **v10.2** |

---

## 🏛️ License & Acknowledgments
AetherOS dilisensikan di bawah **MIT License**. Dibangun dengan penuh gairah menggunakan **Rust Nightly**, terinspirasi oleh Symbian OS dan Zircon, dan didedikasikan untuk kedaulatan digital masa depan.

**"One Mind. One Mesh. Zero Compromise."** 🔥
