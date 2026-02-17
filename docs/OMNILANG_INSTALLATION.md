# Panduan Instalasi OmniLang

**Status**: Terintegrasi Native di xAetherOS v7.0
**Tipe**: Panduan Pengguna & Developer

---

## 1. Runtime Environment (Untuk Pengguna Biasa)
**Anda TIDAK PERLU menginstall OmniLang.**

Berbeda dengan Java atau Python yang harus diinstall terpisah di Windows, OmniLang adalah **bagian dari Kernel xAetherOS**.
- **Status**: *Pre-installed* & *Always-on*.
- **Versi**: Terikat dengan versi Kernel (saat ini v7.0).
- **Cara Cek**: Buka terminal dan ketik `omni --version`.

## 2. Aplikasi OmniLang (Untuk Pengguna Akhir)
Untuk menginstall aplikasi yang dibuat dengan OmniLang (file `.apkg`):

Via Terminal AetherOS:
```bash
# Install dari repositori resmi
apm install com.aetheros.status-monitor

# Install dari file lokal
apm install ./my-app.apkg
```

Via **Ability Market** (GUI):
Cukup, "niatkan" atau klik pada aplikasi di market holografik, dan sistem akan mengunduh paket `.apkg` yang sudah ditandatangani secara aman.

## 3. OmniLang SDK (Untuk Developer)
Jika Anda ingin **membuat** aplikasi, Anda perlu menginstall SDK di komputer pengembangan Anda (bisa Windows, Mac, atau Linux).

### Perintah Instalasi (Universal)
```bash
curl -sSL https://get.aetheros.dev/sdk | bash
```

### Verifikasi
```bash
omnic --version
# Output: OmniLang Compiler v1.0 (AetherScript Backend)
```

---

## 📋 Rangkuman Arsitektur Instalasi

| Komponen | Lokasi | Cara Install | Keterangan |
|----------|--------|--------------|------------|
| **Runtime** | Kernel | **Bawaan** | Mesin eksekusi (JIT/AOT) |
| **Aplikasi** | User Space | `apm install` | Paket `.apkg` (Sandboxed) |
| **Compiler** | Dev Machine | `curl script` | Tools untuk coding |
