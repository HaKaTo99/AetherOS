# Contributing to AetherOS — Developer Preview v0.9

Terima kasih telah tertarik berkontribusi ke AetherOS!
Proyek ini dalam pengembangan aktif dan **membutuhkan kontributor** di banyak area.

> Baca [STATUS.md](STATUS.md) dulu untuk memahami mana yang sudah selesai
> dan mana yang masih perlu dikerjakan, agar kontribusi Anda tepat sasaran.

---

## Contributor License Agreement (CLA)

**By submitting any contribution (code, documentation, bug reports, or other materials) to AetherOS, you agree to the terms of our [Contributor License Agreement](CLA.md).**

Please read the full [CLA.md](CLA.md) before contributing. In summary:

- You grant the Project a perpetual, worldwide license to use, modify, and distribute your contributions.
- You retain ownership of your contributions.
- The Project may relicense your contributions as part of the Project (e.g., under MIT or a commercial license) in the future.

If you have questions, please open an issue.

---

## Cara Berkontribusi

### 1. Setup Lingkungan

```powershell
git clone https://github.com/HaKaTo99/AetherOS.git
cd AetherOS

# Install Rust nightly + target
rustup toolchain install nightly
rustup target add x86_64-unknown-none --toolchain nightly
rustup component add rust-src --toolchain nightly

# Build
.\Aether.ps1 -Action build

# Jalankan
.\Aether.ps1 -Action run
```

**Prasyarat:** QEMU for Windows, PowerShell 7+, `llvm-tools-preview`

---

### 2. Area yang Paling Butuh Kontribusi

Berdasarkan [STATUS.md](STATUS.md), ini area prioritas tertinggi:

| Area | Kesulitan | Dampak |
|------|-----------|--------|
| Perbaikan silent success di `posix.rs` | 🟢 Mudah | Tinggi |
| VirtIO TX/RX DMA nyata | 🟡 Sedang | Tinggi |
| QuickJS integration | 🟡 Sedang | Sangat Tinggi |
| Intel E1000 NIC driver | 🔴 Sulit | Tinggi |
| APIC timer untuk preemption | 🔴 Sulit | Tinggi |

---

### 3. Standar Kode

#### Tidak Boleh Ada Silent Success
```rust
// ❌ JANGAN — mengembalikan sukses tanpa melakukan apapun
pub fn do_something() -> Result<()> {
    Ok(()) // stub
}

// ✅ HARUS — kembalikan error eksplisit jika belum diimplementasikan
pub fn do_something() -> Result<()> {
    Err(AetherError::NotImplemented("do_something: pending implementation"))
}
```

#### Tidak Boleh Ada Nilai Hardcoded untuk Metrik
```rust
// ❌ JANGAN
scheduler_latency_us: 50, // Target: <50μs (BUKAN PENGUKURAN!)

// ✅ HARUS
scheduler_latency_us: measure_scheduler_latency_us(),
```

#### Docstring Jujur
```rust
// ❌ JANGAN
/// Mengirim paket melalui jaringan dengan latensi <1ms.
pub fn transmit(&mut self, packet: &[u8]) -> NetResult<()> {
    Ok(()) // belum diimplementasikan
}

// ✅ HARUS
/// Mengirim paket melalui VirtIO TX ring.
/// **Status:** Implementasi DMA pending. Saat ini mengembalikan `NotImplemented`.
pub fn transmit(&mut self, packet: &[u8]) -> NetResult<()> {
    Err(NetError::NotImplemented)
}
```

---

### 4. Alur Pull Request

1. **Fork** repositori dan buat branch: `git checkout -b feat/nama-fitur`
2. **Tulis kode** dengan mengikuti standar di atas
3. **Verifikasi build:** `cargo build --target x86_64-unknown-none -Z build-std=core,alloc`
4. **Jalankan clippy:** `cargo clippy --target x86_64-unknown-none -Z build-std=core,alloc`
5. **Update STATUS.md** jika fitur Anda mengubah status komponen
6. **Buat PR** dengan deskripsi jelas:
   - Apa yang diubah
   - Bagaimana cara mengujinya
   - Apakah ada breaking change

---

### 5. Pelaporan Bug

Gunakan GitHub Issues dengan template:

```markdown
**Versi AetherOS:** Developer Preview v0.9
**Platform:** QEMU x86_64 / VirtualBox / Hardware fisik
**Langkah reproduksi:**
1. ...
2. ...
**Hasil aktual:** ...
**Hasil yang diharapkan:** ...
**Log serial:** (lampirkan output dari `target/qemu-smoke.log`)
```

---

### 6. Standar Komit

```
feat: Implement QuickJS eval in AetherShell omni bridge
fix: Replace silent Ok(()) in posix_read with ENOSYS
docs: Update STATUS.md — mark EventQueue as complete
bench: Add RDTSC-based context switch measurement
```

---

## Kode Etik

- Jujur tentang status implementasi — jangan klaim selesai jika belum.
- Hormati kontributor lain — kritik kode, bukan orangnya.
- Terbuka terhadap review — semua kode melewati proses review.

---

**"One Mind. One Mesh. Zero Compromise."** 🔥
*Dimulai dengan zero compromise pada standar kode.*
