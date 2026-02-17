# AetherOS: Onboarding Flow 🌐✨

Dokumen ini menjelaskan bagaimana pengguna baru atau perangkat baru dapat bergabung dengan ekosistem **xAetherOS Distributed Mesh Fabric**.

## 1. Tahap Discovery (Penemuan)
Ketika perangkat baru dinyalakan dengan AetherOS, sistem akan secara otomatis memindai jaringan lokal (Local Mesh) menggunakan **Quantum Channel (QC)**.
*   **Protocols**: UDP Broadcast / Multicast Discovery.
*   **Action**: Perangkat mengirim paket `DiscoveryRequest` mencari Peer aktif.

## 2. Tahap Attestation (Verifikasi Keamanan)
Sebelum bergabung, perangkat baru harus membuktikan integritasnya melalui **Quantum Fortress Safe-Boot**.
*   **Identity Check**: Node lama memverifikasi tanda tangan firmware perangkat baru.
*   **Trust Model**: Zero-Trust. Perangkat baru dianggap "untusted" sampai melewati audit peer-to-peer.

## 3. Tahap Identity Creation (Pendaftaran)
Setelah perangkat diverifikasi secara hardware, pengguna perlu membuat identitas digital.
*   **Global Identity**: Akun tidak disimpan di satu server, melainkan didistribusikan di **DHT (Distributed HashTable)**.
*   **RBAC Registration**: Pengguna baru diberikan role `User` secara default oleh sistem keamanan yang dipimpin oleh akun Admin (seperti `root` atau `herman`).

## 4. Tahap Synchronization (Sinkronisasi Fabric)
Setelah login berhasil, perangkat akan mulai menarik data-state dari mesh.
*   **State Sync**: Wallpaper, pengaturan, dan aplikasi (APM) disinkronkan.
*   **Workload Sharing**: Perangkat baru mulai mengiklankan kapabilitas CPU/AI-nya ke **Capability Market** agar node lain bisa menitipkan tugas.

## 5. Tahap Explorer (Aether Store)
Pengguna kini memiliki akses penuh ke **Aether Store** untuk menginstal aplikasi OmniLang dan mulai berkontribusi dalam ekonomi mesh.

---
> [!TIP]
> **Flow bagi Pengguna Umum**:
> Hidupkan Perangkat -> Pilih "Join Mesh" -> Masukkan Identity Key -> Menunggu Sync -> AetherOS Siap Digunakan.
