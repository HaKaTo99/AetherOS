# xAetherOS v5.1 "Foundation" - Release Notes
**Date**: 2026-02-16
**Codename**: Foundation

## 🚀 Highlights
This release marks the transition from "Operating System" to "Intelligent Fabric". It introduces the **Quantum Fortress** security model (Early Access from v6.0) and significant Developer Experience improvements.

### 🛡️ Security (Quantum Fortress)
- **Post-Quantum Cryptography**: All default communications now use **Kyber-768** (KEM) and **Dilithium-3** (Signatures).
- **Immutable Core**: Atomic A/B partition updates implementation (Stub).
- **Homomorphic Encryption**: Privacy-preserving AI stub allows operations on encrypted data.
- **Zero-Trust**: Continuous kernel self-attestation.

### 🦊 Consumer Experience
- **Secure Browser**: Firefox Container stub with PQC-TLS handshake.
- **Enhanced File Manager**: Drag & Drop capability securely integrated with permission model.
- **Multi-Monitor Safety**: UI updates now respect physical display bounds.

### 💻 Developer Experience
- **Rustdoc**: 100% coverage provided for public APIs.
- **Templates**: New `simple-cli` and `distributed-service` project templates.
- **Debugging**: DWARF symbol generation for AetherScript.

## ⚠️ Known Issues
- Real-time PQC performance on RPi4 is unoptimized (simulated).
- FHE module is a stub and should not be used for production health data yet.

## 📦 Download
- **Raspi 4 Image**: `aetheros-v5.1-rpi4.img.xz`
- **QEMU Image**: `aetheros-v5.1-qemu-x86.iso`
- **Source Code**: `v5.1.tar.gz`

---
*Built with ❤️ by the AetherOS Team*
