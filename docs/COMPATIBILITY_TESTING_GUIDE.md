# Panduan Pengujian Kompatibilitas Universal xAetherOS v10.0 🌐

xAetherOS dirancang sebagai "Universal Intelligence Fabric" yang mampu menjalankan aplikasi dari berbagai ekosistem melalui lapisan abstraksi cerdas. Berikut adalah cara menguji masing-masing lapisan:

## 1. Linux / Unix (POSIX)
AetherOS mendukung syscall standard POSIX melalui modul `posix_extra`.
- **Cara Uji**: Jalankan binary ELF statis melalui shell.
- **Internal**: Kernel memetakan syscall Linux (seperti `sys_write`, `sys_open`) ke primitive AetherOS secara native.

## 2. Windows (Win32)
Menggunakan `Win32Loader` untuk memetakan file PE (Portable Executable).
- **Cara Uji**: Gunakan perintah shell: `exec_win32 "C:\app.exe"`.
- **Log**: Anda akan melihat log `[Win32] Resolving KERNEL32.DLL imports`.
- **Lokasi**: [kernel/src/compat/win32/mod.rs](file:///d:/GitHub/AetherOS/kernel/src/compat/win32/mod.rs)

## 3. Android (ART)
Menggunakan `ArtRuntime` untuk interpretasi bytecode Dalvik (.dex).
- **Cara Uji**: Masukkan file `.apk` dan panggil `ArtRuntime::load_dex()`.
- **Lokasi**: [kernel/src/runtime/art.rs](file:///d:/GitHub/AetherOS/kernel/src/runtime/art.rs)

## 4. Mac / iOS (Darwin)
Menggunakan `MachOLoader` untuk segmen file Mach-O.
- **Cara Uji**: Panggil fungsi kernel `darwin::MachOLoader::load_macho()`.
- **Lokasi**: [kernel/src/compat/darwin/mod.rs](file:///d:/GitHub/AetherOS/kernel/src/compat/darwin/mod.rs)

## 5. Symbian & BlackBerry (Legacy DNA)
xAetherOS mewarisi arsitektur *Active Objects* dari Symbian (EPOC).
- **Symbian (.app/.exe)**: Gunakan `EpocLoader` untuk memetakan binary E32.
- **BlackBerry**: Karena BlackBerry 10 berbasis QNX (Microkernel), AetherOS secara native kompatibel melalui arsitektur microkernel dan pesan (messaging) yang serupa.
- **Lokasi EPOC**: [kernel/src/compat/epoc/mod.rs](file:///d:/GitHub/AetherOS/kernel/src/compat/epoc/mod.rs)

## 6. HarmonyOS (OpenHarmony)
AetherOS mendukung distribusi "Ability" melalui jembatan "Soft Bus" native.
- **Cara Uji**: Panggil `harmony::HarmonyLoader::load_hap()`.
- **Fitur**: Sinkronisasi state antar device menggunakan Mesh AetherOS sebagai substrate.
- **Lokasi**: [kernel/src/compat/harmony/mod.rs](file:///d:/GitHub/AetherOS/kernel/src/compat/harmony/mod.rs)

## 7. WebOS (Hybrid Runtime)
Mendukung aplikasi berbasis web (HTML5/Enact) melalui kontainerisasi native.
- **Cara Uji**: Panggil `webos::WebOSRuntime::launch_app("com.webos.app")`.
- **Fitur**: Jembatan Luna Bus ke syscall AetherOS secara transparan.
- **Lokasi**: [kernel/src/compat/webos/mod.rs](file:///d:/GitHub/AetherOS/kernel/src/compat/webos/mod.rs)

## 8. OmniLang (Universal Target)
Bahasa native AetherOS yang bisa dikompilasi ke WASM (Web) atau JVM (Java/Android).
- **Cara Uji**: Jalankan script `.omni` langsung di boot.
- **Lokasi**: [kernel/src/runtime/omnilang.rs](file:///d:/GitHub/AetherOS/kernel/src/runtime/omnilang.rs)

---

### Strategi Pengujian Otomatis
Gunakan **Cognitive Intent Parser** untuk mendeteksi jenis aplikasi secara otomatis:
1. Masukkan payload binary.
2. Kernel akan memindai *magic header* (`MZ` untuk Win, `ELF` untuk Linux, `0xCAFEBABE` untuk Mach-O).
3. `IntentParser` akan memilih *runtime* yang sesuai secara otonom.

**"One Mesh. All Apps. Zero Friction."** 🌌
