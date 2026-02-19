# Panduan Simulasi xAetherOS v10.0 "The Fabric" 🌌

Dokumen ini berisi langkah-langkah untuk menjalankan simulasi **xAetherOS v10.0 "The Fabric"** dalam lingkungan emulator QEMU. Versi ini telah disertifikasi untuk stabilitas tingkat militer (**Golden Release**).

---

## 🧰 **Prasyarat**

Pastikan sistem Anda memiliki komponen berikut:

- **Rust Nightly**: Gunakan versi `nightly-2026-01-20` untuk menjamin kompatibilitas penuh dengan dependensi kernel.
  ```bash
  rustup toolchain install nightly-2026-01-20
  rustup default nightly-2026-01-20
  rustup component add rust-src llvm-tools-preview
  ```
- **QEMU**: Emulator x86_64.
  - **Windows**: `winget install -e --id SoftwareFreedomConservancy.QEMU`
  - **Linux**: `sudo apt install qemu-system-x86`
- **Bootimage Tool**: Untuk pembuatan citra bootable otomatis.
  ```bash
  cargo install bootimage
  ```

---

## 🏗️ **Proses Kompilasi (Build)**

xAetherOS menggunakan sistem build terintegrasi. Anda hanya perlu menjalankan perintah berikut di direktori kernel:

1. **Masuk ke direktori kernel**:
   ```bash
   cd kernel
   ```

2. **Membangun Citra Bootable**:
   ```bash
   cargo bootimage --release
   ```
   Citra biner yang dihasilkan akan berada di:  
   `target/x86_64-unknown-none/release/bootimage-aetheros-kernel.bin`

---

## ▶️ **Menjalankan Simulasi**

### **0. Melalui Skrip Gerbang (Rekomendasi Utama)**
Anda dapat menjalankan simulasi secara otomatis dari root direktori dengan skrip PowerShell:
```powershell
.\run_fabric.ps1
```
Skrip ini akan secara otomatis memeriksa keberadaan citra kernel, melakukan build jika perlu, dan meluncurkan QEMU.

### **1. Melalui Cargo (Otomatis)**
Karena runner telah dikonfigurasi, Anda dapat langsung menjalankan kernel di QEMU dengan:
```bash
cargo run --release
```

### **2. Menjalankan QEMU Secara Manual**
Jika Anda ingin kontrol manual (misalnya untuk debugging atau integrasi mesh), gunakan perintah:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-aetheros-kernel.bin -m 512M -serial stdio -display none
```

**Penjelasan Parameter:**
- `-drive format=raw,file=...`: Memuat citra xAetherOS.
- `-m 512M`: Alokasi memori RAM.
- `-serial stdio`: Output log kernel ke terminal Anda.
- `-display none`: Menjalankan secara headless (opsional).

---

## 🧪 **Verifikasi Status "Gold"**

Setelah booting selesai, pastikan Anda melihat log verifikasi berikut di terminal:

```text
[ v10.0] The Fabric: Military Grade Harmony [ CERTIFIED ]
```

Log ini menandakan:
- **SMME**: Memori telah tervalidasi bebas fragmentasi.
- **PQC Security**: Fortress Fortress Enforced aktif.
- **Harmony Audit**: Keselarasan seluruh subsistem tercapai.

---

## 🛠️ **Pemecahan Masalah**

| Gejala | Solusi |
|--------|--------|
| **Error: unknown unstable option: `json-target-spec`** | Hapus berkas `.cargo/config.toml` global atau di direktori induk. Gunakan yang ada di folder kernel. |
| **Linker error: cannot find linker script** | Pastikan Anda menjalankan perintah dari folder `kernel` atau gunakan path absolut. |
| **Panic pada build script bootloader** | Pastikan toolchain menggunakan `nightly-2026-01-20` dan `llvm-tools-preview` sudah terpasang. |

---

**xAetherOS v10.0 "The Fabric": One Mind. One Mesh. Zero Compromise.** 🔥
