# Contributing to AetherOS v10.0 "The Fabric"

Terima kasih atas dedikasi Anda untuk mengembangkan **AetherOS**, kain intelijen terdistribusi pertama di dunia. Dokumen ini memberikan panduan kolaborasi untuk menjaga standar **Diamond Grade**.

---

## 🤝 Code of Conduct
AetherOS adalah proyek visioner. Kami menghargai profesionalisme, pemikiran strategis, dan rasa hormat antar kontributor. Fokus utama kita adalah kedaulatan digital dan inovasi tanpa kompromi.

---

## 🐞 Pelaporan Bug (v10.0)
Gunakan versi terbaru (`v10.0.0-gold`) sebelum melaporkan isu. Pastikan Anda telah menjalankan audit mandiri.

**Bug Report Template**:
```markdown
**Environment**:
- AetherOS Version: v10.0.0 (The Fabric)
- Platform: x86_64 (UEFI) / aarch64 (RPi4)
- Intent Mode: (Development / Production / Minimal)

**Description**: Deskripsi jelas mengenai anomali.

**Steps to Reproduce**:
1. Langkah inisialisasi
2. Trigger anomali
3. Hasil aktual vs hasil proaktif yang diharapkan

**Logs**: Lampirkan log dari serial console AetherOS atau terminal QEMU.
```

---

## 🚀 Alur Kerja Pull Request (PR)

### 1. Persiapan
Pastikan kode Anda selaras dengan prinsip **Cognitive-First**:
```bash
# Format kode
cargo fmt

# Lakukan audit mandiri
cargo clippy --all-targets -- -D warnings

# Verifikasi build universal
./run_fabric.ps1 --build-only
```

### 2. Standar Komit
Gunakan pesan komit yang deskriptif dan visioner:
- `feat: Implement cognitive intent parser for filesystem`
- `fix: Resolve race condition in Quantum Bus peer discovery`
- `docs: Update OmniLang handbook for Organic UI 2.0`

---

## 🏗️ Standar Arsitektur (Diamond Grade)
Setiap kontribusi harus mematuhi pilar berikut:
1. **Memory Sovereignty**: Gunakan SMME secara efisien; hindari fragmentasi.
2. **OmniLang Integration**: Logika antarmuka harus menggunakan OmniLang jika memungkinkan.
3. **PQC First**: Seluruh komunikasi baru harus terenkripsi secara pasca-kuantum.
4. **Active Objects**: Prioritaskan pola Active Object (Symbian-Style) daripada threading mentah.

---

## 🏛️ Lisensi
Dengan berkontribusi, Anda setuju bahwa seluruh kontribusi Anda dilisensikan di bawah **MIT License**.

**"One Mind. One Mesh. Zero Compromise."** 🔥
