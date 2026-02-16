# Arsip Diskusi & Evolusi Teknis AetherOS v6.0

Dokumen ini merangkum seluruh perjalanan pengembangan, tantangan teknis, dan solusi yang diimplementasikan selama sesi stabilisasi **AetherOS v6.0 "Quantum Fortress"**.

## 1. Objektif Utama
Memastikan AetherOS v6.0 dapat di-boot dengan stabil di QEMU x86_64 dan memiliki fondasi memori yang cukup kuat untuk mendukung fitur-fitur masa depan (AI, Universal Runtime).

## 2. Jurnal Perjalanan Teknis

### Fase 1: Krisis Booting QEMU
*   **Masalah**: QEMU 10.2 (rilis terbaru) menolak memuat ELF 64-bit secara langsung via `-kernel` jika tidak memiliki "PVH ELF Note" yang valid.
*   **Eksperimen**: Mencoba memindahkan Multiboot2 header ke berbagai offset (byte 0, 1MB, dll). QEMU tetap menolak karena ambiguitas antara Multiboot dan PVH pada image 64-bit.
*   **Solusi Final**: Mengalihkan metode booting dari *direct ELF loading* menjadi **GRUB-based ISO**. Metode ini paling andal karena GRUB menangani protokol Multiboot2 secara efisien sebelum masuk ke kernel 64-bit.

### Fase 2: Krisis Alokasi Memori (Kernel Panic)
*   **Masalah**: Setelah boot berhasil, sistem mengalami *panic* saat mencoba mengalokasi 32KB atau 512KB.
*   **Penyebab**: Pool memori L0 (64KB) terlalu kecil untuk kebutuhan v6.0 yang kompleks (QuickJS, AI Agent stub, UI logging). Selain itu, terdapat dua instance alokator yang berjalan secara paralel, menyebabkan inefisiensi dan risiko tabrakan memori.
*   **Solusi Final**: 
    1.  **Unifikasi Alokator**: Menggunakan `GlobalAllocatorProxy` untuk memastikan seluruh sistem (kernel & runtime) menggunakan satu instance SMME yang sama.
    2.  **Maksimalisasi Kapasitas**: Memperbesar pool heap secara signifikan:
        *   **L0 (Fast)**: 16MB
        *   **L1 (General)**: 128MB
        *   **L2 (Large)**: 256MB

### Fase 3: Integrasi Visi Antarmuka Masa Depan
*   **Kontribusi Pengguna**: Diskusi mengenai OUI, PUI, BUI/BCI, dan MMUI.
*   **Implementasi**: Menambahkan log inisialisasi "Quantum Interface Engine" pada urutan boot kernel sebagai representasi kesiapan arsitektur v6.0 untuk mendukung antarmuka biologis dan multimodal tersebut.

## 3. Keputusan Arsitektur Kunci
| Komponen | Status v6.0 | Rationale |
|----------|-------------|-----------|
| **Boot Format** | ISO (via GRUB) | Kompatibilitas universal pada QEMU modern. |
| **Heap Manager** | Singleton SMME | Menghindari `OutOfMemory` pada beban kerja AI intensif. |
| **Paging** | 1GB Identity Map | Simplisitas selama fase transisi ke long mode. |
| **UI Paradigm** | VectorDelta | Memungkinkan layar fleksibel (OUI) tanpa kehilangan ketajaman. |

---
*Dokumentasi ini dibuat untuk memastikan integritas pengetahuan dari pengembangan AetherOS v6.0.*
