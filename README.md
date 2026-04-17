# 🌌 AetherOS

**Sovereign Intelligence OS — Developer Preview v0.9 "Trinity Core"**

![Status](https://img.shields.io/badge/Status-Developer%20Preview%20v0.9-orange)
![Build](https://img.shields.io/badge/Build-Rust%20Nightly%20no__std-blue)
![Platform](https://img.shields.io/badge/Platform-x86__64%20QEMU-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

> ⚠️ **Status: Alpha / Developer Preview** — Proyek ini dalam pengembangan aktif.
> Fondasi kernel, desktop Trinity, dan shell sudah berfungsi. Banyak fitur canggih
> (mesh, post-quantum crypto, universal runtime) masih dalam tahap kerangka arsitektur.
> **Tidak untuk penggunaan produksi.**

---

## ✅ Yang Sudah Berfungsi (Terverifikasi)

| Komponen | Status | Keterangan |
|----------|:------:|------------|
| Boot GRUB/Multiboot2 (QEMU x86_64) | ✅ | Stabil |
| HAL x86_64 — Serial, VGA, Framebuffer (LFB), PS/2 | ✅ | Stabil |
| SMME — 3-tier heap, canaries, memory poisoning | ✅ | Stabil |
| Active Object Scheduler + Message Passing | ✅ | Berfungsi |
| Watchdog & Self-Healing | ✅ | Berfungsi |
| Desktop Trinity — Glassmorphism, Blur, 60 FPS | ✅ | Di QEMU |
| AetherShell — `help`, `status`, `calc`, `clear`, `exit`, `meshstatus` | ✅ | Responsif |
| Event Queue System (thread-safe, generic) | ✅ | Selesai |
| VirtIO Network (struktur ring descriptor) | ✅ | Sebagian |
| Build pipeline satu perintah → ISO bootable | ✅ | `Aether.ps1` |

---

## 🟡 Dalam Pengembangan (Kerangka Ada, Belum Fungsional)

| Komponen | Keterangan |
|----------|------------|
| POSIX Compatibility Layer | Struktur syscall & VFS ada; eksekusi nyata belum |
| VirtIO Network — TX/RX DMA | Ring descriptor ada; DMA transfer belum dihubungkan |
| Post-Quantum Crypto (Kyber/Dilithium) | Dependensi ada; belum digunakan di jalur kritis |
| Runtime WASM / QuickJS | Modul ada; eksekusi nyata belum |
| Enterprise (RBAC, OTA, Audit, Policy) | Kerangka ada; sebagian besar masih stub |
| Performance Metrics | Modul ada; beberapa nilai saat ini hardcoded |

---

## ❌ Belum Diimplementasikan

| Komponen | Keterangan |
|----------|------------|
| Mesh Networking & BFT Consensus | Placeholder; belum ada implementasi nyata |
| Universal Runtime (Windows/Android/Mac) | Belum ada |
| Ability Economy (`captrade`) | Belum ada transaksi nyata |
| Driver Hardware Fisik (E1000, NVMe) | Hanya VirtIO untuk QEMU |
| Sertifikasi Common Criteria / FIPS | Direncanakan jauh ke depan |

---

## 🚀 Cara Mencoba

### Prasyarat
- Windows 10/11
- [Rust Nightly](https://rustup.rs/) + target `x86_64-unknown-none`
- [QEMU for Windows](https://www.qemu.org/download/)
- PowerShell 7+

### Build & Jalankan
```powershell
git clone https://github.com/HaKaTo99/AetherOS.git
cd AetherOS

# Build kernel dan buat ISO
.\Aether.ps1 -Action build

# Jalankan di QEMU
.\Aether.ps1 -Action run
```

### Smoke Test
```powershell
# Linux/WSL
chmod +x scripts/qemu-smoke.sh && scripts/qemu-smoke.sh

# Windows PowerShell
powershell -ExecutionPolicy Bypass -File scripts/qemu-smoke.ps1
```

Tes lulus jika marker `AetherShell>` muncul di log serial.

### Debug dengan GDB
```bash
scripts/qemu-debug.sh
# Terminal lain:
gdb -ex "target remote :1234" target/x86_64-unknown-none/release/aetheros-kernel
```

---

## 📂 Struktur Proyek

```
AetherOS/
├── Aether.ps1          # Entry point utama (build, run, test)
├── kernel/src/
│   ├── hal/            # Hardware Abstraction Layer (x86_64, RPi)
│   ├── memory/         # SMME — Sovereign Memory Management Engine
│   ├── scheduler/      # Active Object Scheduler
│   ├── ui/             # Trinity Desktop (framebuffer renderer)
│   ├── events/         # Event queue system ✅
│   ├── net/            # Network drivers (VirtIO) 🟡
│   ├── runtime/        # POSIX, WASM, QuickJS 🟡
│   ├── enterprise/     # RBAC, OTA, Audit 🟡
│   └── mesh/ bus/      # Distributed mesh ❌
├── boot/               # GRUB/Multiboot2 config
├── docs/               # Dokumentasi teknis
└── scripts/            # CI & testing utilities
```

---

## 📖 Dokumentasi

| Dokumen | Tujuan |
|---------|--------|
| [STATUS.md](STATUS.md) | Status implementasi lengkap per modul |
| [ROADMAP.md](ROADMAP.md) | Rencana pengembangan Q2–Q4 2026 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Panduan kontribusi |
| [RELEASE_NOTES.md](RELEASE_NOTES.md) | Catatan rilis Developer Preview v0.9 |

---

## 🤝 Kontribusi

AetherOS menyambut baik kontribusi dari komunitas! Sebelum membuat kontribusi, silakan pelajari [CONTRIBUTING.md](CONTRIBUTING.md) untuk memahami pedoman arsitektur dan standar kode AetherOS.

**PENTING:** Dengan mengirimkan kontribusi (Pull Request) ke AetherOS, Anda secara otomatis menyetujui seluruh ketentuan dalam [Contributor License Agreement (CLA)](CLA.md) kami.

---

## 🏛️ Lisensi

Dilisensikan di bawah **MIT License**.
Dibangun dengan Rust Nightly. Terinspirasi oleh Symbian OS dan Zircon.

**"One Mind. One Mesh. Zero Compromise."** 🔥
