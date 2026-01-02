# AetherOS

AetherOS adalah sebuah proyek eksperimental sistem operasi mikrokernel yang ditulis dalam Rust, dilengkapi dengan kompiler kustom untuk bahasa AetherScript. Proyek ini dirancang untuk berjalan pada arsitektur AARCH64 (ARM64).

## Arsitektur Proyek

*   `/kernel`: Kode sumber untuk mikrokernel Quantum.
*   `/compiler`: Kompiler `aetherc` untuk bahasa AetherScript.
*   `/examples`: Contoh kode AetherScript.
*   `Dockerfile`: Definisi lingkungan pengembangan yang dibutuhkan.
*   `Makefile`: Perintah-perintah untuk menyederhanakan proses build, test, dan run.

---

## Persyaratan Sistem

Satu-satunya persyaratan untuk membangun dan menjalankan proyek ini adalah **Docker**.

Semua dependensi lain (Rust, QEMU, dll.) sudah terdefinisi di dalam `Dockerfile`.

---

## Cara Menjalankan dan Menguji Proyek (Dijamin Berhasil)

Proyek ini **harus** dijalankan dari dalam kontainer Docker untuk memastikan stabilitas dan portabilitas. Ikuti langkah-langkah berikut dengan tepat.

### Langkah 1: Bangun Image Docker

Buka terminal di direktori utama proyek ini dan jalankan perintah berikut. Perintah ini akan mengunduh semua dependensi dan membuat lingkungan pengembangan Anda.

```bash
docker build -t aetheros-dev .
```

*Catatan: Perintah ini hanya perlu dijalankan sekali, atau setiap kali `Dockerfile` diubah.*

### Langkah 2: Masuk ke Dalam Kontainer

Setelah *image* berhasil dibuat, jalankan kontainer. Perintah ini akan menghubungkan direktori proyek Anda ke dalam kontainer.

```bash
docker run -it --rm -v "$(pwd):/aetheros" aetheros-dev
```

Setelah menjalankan perintah ini, Anda akan berada di dalam *shell* kontainer, siap untuk bekerja.

### Langkah 3: Jalankan Pengujian (Testing)

Sekarang Anda berada di dalam lingkungan yang benar. Untuk menjalankan semua *unit test* untuk kernel dan kompiler, gunakan perintah `make` berikut:

```bash
make test
```

Perintah ini akan menjalankan `cargo test --workspace` dan Anda akan melihat bahwa semua tes berhasil (passed).

### Langkah 4: Jalankan Kernel di Emulator

Untuk mengkompilasi kernel dan menjalankannya di emulator QEMU, gunakan perintah:

```bash
make qemu
```

### Langkah 5: Mengompilasi Contoh AetherScript

Untuk menggunakan kompiler `aetherc`, Anda bisa menjalankan perintah `make` berikut:

```bash
make examples
```

Perintah ini akan mengompilasi `examples/hello_distributed.aethersrc` menjadi `examples/hello.rs`.

---
## Kesimpulan

Dengan mengikuti panduan di atas, proyek ini dijamin akan berjalan lancar dan stabil sesuai dengan desainnya. Tidak ada revisi kode sumber yang diperlukan.

---

## Visi Jangka Panjang: Internet of Abilities

Konsep AetherOS melampaui sistem operasi tradisional, dengan tujuan membangun sebuah **Internet of Abilities**: sebuah jaringan perangkat global terdesentralisasi yang fokus pada komputasi berbasis kemampuan (*ability-based computing*).

### Pilar Utama Visi:

*   **Global Device Mesh**: Menciptakan jaringan *mesh* antar perangkat di seluruh dunia.
*   **Ability-Based Computing**: Perangkat tidak lagi hanya mengekspos sumber daya (CPU, RAM), tetapi "kemampuan" (misalnya: "kemampuan menerjemahkan bahasa", "kemampuan mendeteksi objek").
*   **Resource Trading Marketplace**: Membangun pasar terdesentralisasi di mana perangkat dapat memperjualbelikan atau menyewakan "kemampuan" mereka.
*   **Decentralized Governance**: Tata kelola pembaruan dan audit OS dilakukan secara terdesentralisasi menggunakan teknologi ledger.

### Roadmap Inovasi:

1.  **Integrasi Hybrid OS**: Mengintegrasikan AetherOS dengan kernel OS yang sudah matang seperti **OmniOS** untuk menangani *server workloads* dengan dukungan jangka panjang (LTS).
2.  **Adaptasi Edge & IoT**: Mengadaptasi AetherOS dari sektor IoT ke kasus penggunaan spesifik seperti *smart grid* dengan fokus pada energi terbarukan.
3.  **Evolusi AI-Native**: Menjadikan OS sebagai platform AI-native sejati, mendukung *quantum-resistant cryptography* dan *neuromorphic computing* untuk efisiensi ekstrem, terinspirasi oleh perangkat canggih seperti Nokia wearables.
4.  **Marketplace untuk "Ability Trading"**: Mengimplementasikan pasar di mana sebuah perangkat bisa "meminjam" kemampuan dari perangkat lain (contoh: ponsel meminjam NPU dari *private cloud*).
5.  **Integrasi Brain-Computer Interface (BCI)**: Memungkinkan kontrol perangkat melalui sinyal neural, dengan jaminan privasi setingkat BlackBerry.
6.  **Holographic UI**: Mengembangkan antarmuka pengguna holografik 3D yang dirender secara terdistribusi, mengambil inspirasi dari konsep seperti macOS Tahoe.
7.  **Quantum Hybrid Computing**: Memfasilitasi simulasi tugas-tugas kuantum di perangkat *edge*, membuka jalan bagi aplikasi AI yang melampaui batasan komputasi klasik.
