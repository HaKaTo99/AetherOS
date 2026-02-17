# OmniLang & Organic UI: The Visual Bridge

**Dokumen Konsep UI/UX**
**Versi**: 1.0 (Draft)
**Tanggal**: 16 Februari 2026

---

## 🎨 Filosofi Desain: "Code IS the Interface"

Dalam xAetherOS v7.0, **OmniLang** (sebelumnya AetherScript) tidak hanya mendefinisikan logika, tetapi juga mendefinisikan *perilaku organik* antarmuka. Tidak ada file `.xml` atau `.css` terpisah. UI dideklarasikan sebagai bagian dari logika terdistribusi.

### 1. Deklarasi UI Organik
Kode OmniLang bersifat deklaratif dan adaptif terhadap permukaan (Surface Awareness).

#### Contoh Kode: Holographic Dashboard

```swift
style GlassOrchid {
    material: .glass(opacity: 0.2, blur: 20.px)
    glow: .pulse(color: .neonViolet, rate: 60.bpm)
    behavior: .fluid // Mengalir mengikuti permukaan
}

app StatusMonitor {
    @distributed(priority: .high)
    ui View {
        // Kontainer utama yang "hidup"
        VStack(spacing: 15) {
            
            // Widget Header yang melayang
            Text("System Vitality")
                .font(.biosemi, size: 24)
                .effect(.levitate(height: 5.mm))
            
            // Grafik detak jantung mesh (Real-time)
            Graph(source: Mesh.heartbeat())
                .style(GlassOrchid)
                .frame(height: 120)
                .onGaze { focus -> 
                    // Membesar saat dilirik mata (PUI)
                    focus.scale(1.1)
                    focus.showDetails() 
                }
            
            // Tombol tindakan dengan haptic neural
            Button("Heal Network")
                .onNeuralIntent(.confirm) {
                    Mesh.triggerSelfHealing()
                }
        }
        .padding(20)
        .background(.adaptive) // Menyesuaikan warna dinding/meja
    }
}
```

---

## 🔮 Visualisasi Eksekusi (The Render Pipeline)

Saat kode di atas dijalankan di xAetherOS, inilah yang terjadi:

1.  **Compilation**: OmniLang dikompilasi menjadi *Intermediate Representation (IR)* yang membawa metadata semantik (misal: tombol ini penting).
2.  **Surface Analysis (OUI Driver)**:
    *   Kernel membaca sensor (LiDAR/Kamera): "Permukaan adalah meja kayu melengkung."
    *   Organic UI Driver: "Ubah `VStack` menjadi kurva bezier mengikuti tepi meja."
3.  **Authentication (Continuous Attestation)**:
    *   Sistem memverifikasi bahwa kode `Mesh.triggerSelfHealing()` berasal dari pengguna yang terautentikasi (via Zero-Trust).
4.  **Projection/Rendering**:
    *   Jika menggunakan kacamata AR: UI muncul melayang 30cm di depan mata.
    *   Jika menggunakan proyektor: UI diproyeksikan ke meja, dengan koreksi distorsi otomatis.

---

## 🧠 Interaksi Neural (Mind-Melding)

Salah satu fitur unik OmniLang adalah integrasi **PUI (Perceptual User Interface)**.

*   **Gaze-to-Act**: Tidak perlu pointer mouse. Elemen UI bereaksi saat Anda melihatnya (`.onGaze`).
*   **Intent-Based Clicking**: Tombol `Heal Network` tidak perlu ditekan fisik. Cukup "berniat" menekannya, BCI akan menangkap sinyal motorik korteks Anda, dan tombol akan tertekan secara visual.

---

## 🛠️ Kesimpulan Teknis

OmniLang bukanlah bahasa pemrograman biasa. Ia adalah **mantra** yang memerintahkan Fabric untuk bermanifestasi menjadi alat yang berguna sesuai konteks fisik dan mental pengguna.
