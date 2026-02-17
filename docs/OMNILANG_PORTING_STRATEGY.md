# OmniLang Integration Strategy (HaKaTo99/OmniLang)

**Status**: Active Integration
**Target**: `kernel/src/runtime/omnilang.rs`

---

## 🏗️ Mekanisme Integrasi

Untuk menjalankan [HaKaTo99/OmniLang](https://github.com/HaKaTo99/OmniLang) di dalam AetherOS, kita menggunakan strategi **Embedded Runtime Adaptation**.

### Langkah 1: Porting Core Logic
Kernel AetherOS tidak menjalankan OmniLang sebagai proses eksternal, melainkan **menanamkan** logikanya langsung ke dalam kernel (sebagai modul `aetheros_kernel::runtime::omnilang`).

1.  **Clone Repository**: Ambil source code dari repo resmi.
2.  **no_std Adaptation**: Modifikasi krate OmniLang agar kompatibel dengan lingkungan `no_std` (kernel mode).
    - Ganti `std::string` dengan `alloc::string`.
    - Ganti `std::vec` dengan `alloc::vec`.
3.  **Module Embedding**:
    - Pindahkan `src/lexer` -> `kernel/src/runtime/omnilang/lexer.rs`
    - Pindahkan `src/parser` -> `kernel/src/runtime/omnilang/parser.rs`
    - Pindahkan `src/backend` -> `kernel/src/runtime/omnilang/codegen.rs`

### Langkah 2: Kernel Binding
Agar OmniLang bisa mengakses fitur OS (seperti Mesh dan UI):

```rust
// kernel/src/runtime/omnilang.rs

// Import logika dari repo eksternal (setelah di-port)
use omnilang_core::{Compiler, Extensions};

pub struct OmniLangRuntime;

impl OmniLangRuntime {
    pub fn execute(source: &str) {
        // 1. Compile menggunakan engine HaKaTo99
        let ast = Compiler::parse(source);
        
        // 2. Inject AetherOS features (Mesh, UI)
        let ctx = Extensions::new()
            .with_mesh(Mesh::get_instance())
            .with_ui(OrganicUIDriver::get_instance());
            
        // 3. Run safely
        ctx.run(ast);
    }
}
```

---

## 🔄 Sinkronisasi Versi
- **Upstream**: https://github.com/HaKaTo99/OmniLang
- **Downstream**: `kernel/src/runtime/omnilang.rs`
- **Sync Policy**: Setiap rilis major di repo OmniLang akan di-merge ke kernel AetherOS dalam waktu <24 jam melalui CI/CD pipeline.

---

## ✅ Status Saat Ini
Repo `HaKaTo99/OmniLang` sudah diakui sebagai **Official Upstream**. Kode di `kernel/src/runtime/omnilang.rs` sekarang bertindak sebagai *adapter* untuk menghubungkan logika bahasa tersebut dengan sistem operasi.
