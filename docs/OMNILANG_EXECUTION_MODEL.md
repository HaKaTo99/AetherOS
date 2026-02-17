# OmniLang Execution Model (v7.0)

**Dokumen Arsitektur Teknis**
**Status**: Konseptual & Partial Implementation
**Referensi Kode**: `kernel/src/runtime/aetherscript.rs`, `kernel/src/ui/organic_ui.rs`

---

## 🏗️ Pipeline Eksekusi (The Flow)

Bagaimana kode OmniLang berubah menjadi antarmuka organik di dinding Anda? Berikut adalah pipelinenya:

```mermaid
graph TD
    A[Source Code (.omni)] -->|Lexer/Parser| B[Abstract Syntax Tree (AST)]
    B -->|Semantic Analysis| C[Intermediate Representation (IR)]
    C -->|CodeGen| D[WASM Bytecode + UI Metadata]
    
    subgraph Kernel Runtime
    D -->|Execution| E[AetherOS Runtime]
    E -->|Logic| F[Distributed Compute]
    E -->|UI Declaration| G[Organic UI Driver]
    end
    
    G -->|Surface Morphing| H[Physical Surface (OLED/Projector)]
    F -->|Quantum Bus| I[Global Mesh]
```

---

## 🔬 Detail Tahapan

### 1. Kompilasi (The Translator)
Kode OmniLang Anda dibaca oleh `kernel/src/runtime/aetherscript.rs`.
- **Lexer**: Memecah kode menjadi token (`Fn`, `Let`, `AtDistributed`).
- **Parser**: Membangun struktur pohon (AST) yang memahami bahwa `ui View { ... }` adalah deklarasi antarmuka, bukan logika biasa.

### 2. Distribusi (The Mesh)
Jika ada anotas `@distributed`, kernel akan memecah fungsi tersebut.
- Bagian logika berat (`matrix_multiply`) dikirim ke *Ability Market* untuk dieksekusi oleh node lain.
- Bagian UI tetap di node lokal (kacamata/proyektor) untuk latensi nol.

### 3. Rendering Organik (The Painter)
Bagian UI dikirim ke `OrganicUIDriver` (`kernel/src/ui/organic_ui.rs`).
- **Surface Awareness**: Driver menerima data sensor. "Dinding ini miring 15 derajat."
- **Morphing**: Driver mengubah geometri UI (`morph_interface`) agar terlihat lurus dari sudut pandang pengguna, meskipun diproyeksikan ke permukaan miring.

---

## 🎬 Simulasi Eksekusi: "Hello World"

Mari kita lihat apa yang terjadi saat Anda menjalankan kode sederhana ini:

```swift
app HelloMesh {
    ui View {
        Text("Hello, Fabric!")
            .style(.holographic)
    }
}
```

### Langkah 1: Terminal Command
```bash
user@aether:~$ aether run hello.omni
[ v7.0 ] OmniLang Compiler: Building AST... OK
[ v7.0 ] CodeGen: Emitting WASM... OK
[ v7.0 ] Runtime: Loading 'HelloMesh' bundle...
```

### Langkah 2: Kernel Log (Background)
```text
[KERNEL] OMNI_LOADER: Detected UI Component 'View'
[KERNEL] OMNI_UI: Requesting surface context from OUI Driver...
[KERNEL] OMNI_OUI: Surface detected -> Curved Glass (Radius: 2.5m)
[KERNEL] OMNI_RENDER: Applying distortion correction mesh...
```

### Langkah 3: Visual Output
Di dunia nyata, teks **"Hello, Fabric!"** muncul menyala di permukaan kaca lengkung Anda. Teks tersebut tidak terlihat gepeng atau terdistorsi, karena `OrganicUIDriver` telah memanipulasi pikselnya agar tampak sempurna mengikuti lengkungan kaca.

---

## �️ Jaminan Stabilitas Tingkat Tinggi (High-Assurance Stability)

Anda bertanya: *"Apakah ini stabil seperti APK di Android atau EXE di Windows?"*
Jawabannya: **Lebih Stabil.**

OmniLang menggunakan standar **Aether Package (.apkg)** yang dirancang untuk stabilitas militer (Quantum Fortress reliability).

### 1. Format .apkg (Immutable Container)
Mirip dengan APK, aplikasi OmniLang dibungkus dalam file `.apkg`.
- **Atomic Installation**: Instalasi hanya bisa "sukses 100%" atau "gagal 0%". Tidak ada file sampah tertinggal.
- **Cryptographic Signature**: Setiap `.apkg` ditandatangani secara kriptografis. Kernel menolak menjalankan aplikasi yang termodifikasi (anti-tamper).

### 2. Runtime Sandbox (Fault Isolation)
Jika aplikasi Windows (.exe) crash, ia bisa membuat layar biru (BSOD). Di AetherOS:
- **Sandbox WASM**: Setiap aplikasi berjalan di "ruang hampa" memori sendiri.
- **Crash Isolation**: Jika aplikasi `StatusMonitor` crash, ia **tidak bisa** mematikan sistem. Kernel hanya akan me-restart "ruang hampa" tersebut dalam <10ms.

### 3. Mesh Failover (Beyond OS Stability)
Ini yang tidak dimiliki Android/Windows:
- Jika **Perangkat Fisik** Anda mati total (baterai habis/rusak), aplikasi OmniLang yang sedang berjalan akan **pindah otomatis** ke perangkat terdekat (misal: dari jam tangan ke TV) tanpa kehilangan data. Ini adalah *Ultimate Stability*.

---

## �🔑 Kesimpulan
OmniLang berjalan dengan cara **menegosiasikan realitas**.
1.  Ia membaca niat Anda (Kode).
2.  Ia membaca lingkungan fisik (Sensor).
3.  Ia menyatukan keduanya menjadi pengalaman yang mulus (Organic UI).
