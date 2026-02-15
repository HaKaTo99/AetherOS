# AetherOS

**Version 2.0.0** - Production-Ready Distributed Operating System

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-x86__64%20%7C%20aarch64-blue.svg)](https://github.com/HaKaTo99/AetherOS)

AetherOS is a microkernel operating system written in Rust, designed for distributed computing across multiple devices. Inspired by Symbian OS design patterns and modern distributed systems, AetherOS enables seamless task migration, resource sharing, and collaborative computing.

---

##  Features

### Core Kernel
- **SMME Memory Allocator**: Three-tier pool architecture (64KB + 2MB + 16MB)
- **Active Object Scheduler**: Priority-based preemptive multitasking
- **Quantum Bus RPC**: Lightweight cross-device communication
- **Capability System**: Token-based security and access control

### Distributed Computing
- **Task Migration**: Migrate Active Objects between devices
- **Distributed KV Store**: Eventually-consistent key-value storage with replication
- **Load Balancing**: Metrics-based task placement and auto-migration

### User Interface
- **Widget System**: Label, Button, Panel, TextBox components
- **FlexLayout Engine**: Responsive layout system
- **Graphics Stack**: VGA driver (x86_64), SimpleFB (aarch64)
- **Input Handling**: PS/2 keyboard (polled)

### Multi-Platform Support
- **x86_64**: PC with VGA output, bootable via USB
- **aarch64**: Raspberry Pi 4 with UART console, bootable via SD card

---

##  Quick Start

### Prerequisites

- **Rust Toolchain** (nightly):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup default nightly
  rustup component add llvm-tools-preview
  ```

- **Cross-Compilation Targets**:
  ```bash
  rustup target add x86_64-unknown-none
  rustup target add aarch64-unknown-none
  ```

- **QEMU** (for testing):
  - Linux: `sudo apt install qemu-system-x86 qemu-system-aarch64`
  - macOS: `brew install qemu`
  - Windows: Download from [qemu.org](https://www.qemu.org/download/)

### Building

```bash
# Clone repository
git clone https://github.com/HaKaTo99/AetherOS.git
cd AetherOS/kernel

# Build for x86_64
cargo build --release --target x86_64-unknown-none

# Build for Raspberry Pi 4
cargo build --release --target aarch64-unknown-none
```

### Running in QEMU

```bash
# x86_64
qemu-system-x86_64 \
  -kernel target/x86_64-unknown-none/release/aetheros-kernel \
  -serial stdio

# Raspberry Pi 4
qemu-system-aarch64 \
  -M raspi4b \
  -kernel target/aarch64-unknown-none/release/aetheros-kernel \
  -serial stdio
```

---

##  Documentation

- **[Capabilities v2.0](docs/reference/CAPABILITIES_v2.0.md)**: Latest feature overview
- **[Developer Guide](docs/guides/DEVELOPER_GUIDE.md)**: Architecture, build instructions, debugging
- **[Deployment Guide](docs/guides/DEPLOYMENT_GUIDE.md)**: USB boot, RPi4 setup, troubleshooting
- **[API Reference](docs/reference/API_REFERENCE.md)**: Rustdoc overview
- **[Porting Roadmap](docs/porting/PORTING_ROADMAP.md)**: Porting strategy
- **[CHANGELOG](CHANGELOG.md)**: Full v2.0 release notes

---

##  Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    AetherOS Kernel                      │
├─────────────────────────────────────────────────────────┤
│  Memory (SMME) │ Scheduler │ IPC (Quantum Bus)          │
│  Distributed   │ Graphics  │ UI Framework               │
│  Security      │ Network   │ Device Drivers             │
└─────────────────────────────────────────────────────────┘
         │                    │                   │
    x86_64 PC          Raspberry Pi 4        Android (Future)
```

### Key Components

- **SMME**: Symbian-Modern Memory Engine with two-phase allocation
- **Active Objects**: Message-passing concurrency model
- **Quantum Bus**: RPC framework for distributed computing
- **FlexLayout**: Responsive UI layout engine

---

##  Testing

```bash
# Run unit tests
cd kernel
cargo test

# Build verification (both targets)
cargo build --release --target x86_64-unknown-none
cargo build --release --target aarch64-unknown-none

# Generate documentation
cargo doc --no-deps --open
```

---

##  Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Before submitting**:
- Run `cargo fmt` (code formatting)
- Run `cargo clippy` (linting)
- Ensure all tests pass
- Update documentation

---

##  Security

For security vulnerabilities, please see [SECURITY.md](SECURITY.md).

**Current Status**: v2.0.0 production release with comprehensive security hardening.

---

##  Vision: Internet of Abilities

AetherOS aims to create a **decentralized network of devices** that share capabilities, not just resources.

### Future Roadmap

1. **Hybrid OS Integration**: Integration with mature kernels (OmniOS for server workloads)
2. **Edge & IoT Adaptation**: Smart grid, renewable energy applications
3. **AI-Native Platform**: Quantum-resistant cryptography, neuromorphic computing
4. **Ability Marketplace**: Trade computational capabilities between devices
5. **Brain-Computer Interface**: Neural control with privacy guarantees
6. **Holographic UI**: Distributed 3D rendering
7. **Quantum Hybrid Computing**: Edge quantum simulation

---

##  Project Status

| Phase | Status | Version |
|-------|--------|---------|
| Core Foundation |  Complete | v1.0 |
| Scheduler & IPC |  Complete | v1.2 |
| Multi-Platform |  Complete | v1.4 |
| Security & Power |  Complete | v1.5 |
| Distributed System |  Complete | v1.6 |
| Testing |  Complete | v1.6.1 |
| Framework Services |  Complete | v1.7 |
| Distributed Finalization |  Complete | v1.8 |
| Documentation |  Complete | v1.9 |
| **Pre-Release Stabilization** |  **Complete** | **v2.0** |

**Overall**: 10/10 phases complete (100%) 

---

##  License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

---

##  Acknowledgments

- Built with Rust 
- Inspired by Symbian OS, Zircon, and seL4
- Community contributors and testers

---

##  Contact

- **Issues**: [GitHub Issues](https://github.com/HaKaTo99/AetherOS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/HaKaTo99/AetherOS/discussions)
- **Documentation**: [docs/](docs/)

---

**Star this repo if you find it interesting!**
