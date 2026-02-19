# 💎 OmniLang Handbook: The Universal Intelligence Language

**Status**: Official Specification (v10.0 Harmony)  
**Target**: `kernel/src/runtime/omnilang.rs` & `kernel/src/ui/organic_ui.rs`

---

## 🚀 1. Filosofi & Visi
OmniLang (sebelumnya OmniLang) bukan sekadar bahasa pemrograman; ia adalah **mantra operasional** untuk Fabric. Dalam AetherOS, kode **adalah** antarmuka. Tidak ada batasan antara logika bisnis dan representasi visual.

### Pilar Utama:
- **Cognitive Awareness**: Kode yang sadar akan niat (*intent*) pengguna.
- **Surface Awareness**: UI yang secara organik beradaptasi dengan geometri fisik (OLED, Proyektor, Glass).
- **Native Distribution**: Anotasi `@distributed` untuk orkestrasi tugas otomatis di seluruh mesh.

---

## 🏗️ 2. Model Eksekusi (The Pipeline)
Bagaimana ide berubah menjadi realitas organik:

1.  **Lexer/Parser**: Membangun AST yang memahami deklarasi UI dan logika.
2.  **CodeGen**: Menghasilkan WASM Bytecode + Metadata UI.
3.  **Sandbox Runtime**: Eksekusi di lingkungan terisolasi untuk stabilitas militer (Quantum Fortress).
4.  **Organic Rendering**: Driver `OrganicUI` melakukan *Surface Morphing* untuk koreksi distorsi fisik secara real-time.

---

## 🛠️ 3. Panduan Pengembangan & Instalasi

### Untuk Pengguna (Runtime)
OmniLang **sudah terintegrasi native** di kernel AetherOS. Pengguna tidak perlu instalasi tambahan.
- Verifikasi: `omni --version` di shell AetherOS.

### Untuk Pengembang (SDK)
Instalasi compiler di mesin pengembangan (Windows/Mac/Linux):
```bash
curl -sSL https://get.aetheros.dev/sdk | bash
omnic --version # OmniLang Compiler v1.0
```

### Format Paket: .apkg
Aplikasi dibungkus dalam kontainer imutabel `.apkg`:
- **Atomic**: Gagal 0% atau Sukses 100%.
- **Signed**: Penandatanganan kriptografis untuk integritas Zero-Trust.

---

## 🎨 4. Konsep UI Organik (Code IS Interface)
Contoh deklarasi antarmuka yang adaptif:

```swift
app StatusMonitor {
    @distributed(priority: .high)
    ui View {
        VStack(spacing: 15) {
            Text("System Vitality")
                .effect(.levitate(5.mm))
                
            Graph(source: Mesh.heartbeat())
                .style(.glass(opacity: 0.2))
                .onGaze { focus -> focus.scale(1.1) }
        }
        .background(.adaptive) // Menyesuaikan dengan warna dinding
    }
}
```

---

## 🔄 5. Strategi Sinkronisasi & Porting
Kernel AetherOS menanamkan logika OmniLang secara native via `no_std` adaptation.
- **Upstream**: `https://github.com/HaKaTo99/OmniLang`
- **Bridge**: `kernel/src/runtime/omnilang.rs` bertindak sebagai adapter untuk menghubungkan logika bahasa dengan Quantum Bus dan Organic UI.

---
*"Code the reality you want to inhabit."* 🔥
