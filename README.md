# AetherOS

**Version 4.0.0** - Cross-Platform Distributed Enterprise OS

![AetherOS](https://img.shields.io/badge/AetherOS-v4.0_Enterprise-blueviolet)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen)
![Status](https://img.shields.io/badge/Status-Stable-green)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-x86__64%20%7C%20aarch64-blue.svg)](https://github.com/HaKaTo99/AetherOS)

AetherOS is a microkernel operating system written in Rust, designed for distributed enterprise computing. V4.0 introduces role-based access control, global mesh networking, and cloud-native capabilities.

---

##  Features

### Core Kernel
- **SMME Memory Allocator**: Three-tier pool architecture (64KB + 2MB + 16MB)
- **Active Object Scheduler**: Priority-based preemptive multitasking
- **Quantum Bus RPC**: Lightweight cross-device communication
- **Capability System**: Token-based security and access control

### Distributed Computing
- **Global Mesh**: Kademlia DHT for internet-wide device discovery (v5.0 Preview)
- **Task Migration**: Migrate Active Objects between devices
- **Distributed KV Store**: Eventually-consistent key-value storage with replication
- **Load Balancing**: Metrics-based task placement and auto-migration

### Enterprise & Cloud (v4.0)
- **RBAC**: Role-Based access control for fleet management
- **Telemetry**: Real-time metrics and health monitoring
- **Cloud-Init**: Auto-configuration for AWS/GCP/Azure environments

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

##  Project Structure

```
AetherOS/
├── boot/                  # Boot configuration (GRUB)
├── bsp/                   # Board Support Packages
├── compiler/              # AetherScript compiler
├── docs/                  # Documentation
│   ├── archive/           #   Historical docs (v1.x-v3.x)
│   ├── guides/            #   Developer & deployment guides
│   ├── reference/         #   API & capabilities reference
│   ├── reports/           #   Implementation & test reports
│   ├── MASTER_TODO.md     #   Full development roadmap
│   └── VERSION_HISTORY.md #   v1.0 → v4.0 evolution
├── examples/              # AetherScript example apps
├── kernel/                # Kernel source code
│   ├── src/
│   │   ├── ai/            #   AI Inference & NPU (v5.0)
│   │   ├── distributed/   #   Mesh, DHT, Market, Storage
│   │   ├── drivers/       #   Device drivers (BCI, USB, etc.)
│   │   ├── enterprise/    #   RBAC, Cloud, Telemetry (v4.0)
│   │   ├── quantum/       #   Quantum Simulation (v5.0)
│   │   ├── runtime/       #   Universal Runtimes (WASM, POSIX)
│   │   ├── scheduler/     #   Active Object scheduler
│   │   ├── security/      #   Capabilities, KASLR
│   │   └── ui/            #   Window manager, widgets
│   ├── Cargo.toml
│   └── build.rs
├── scripts/               # Build & deployment scripts
├── tools/                 # Build tools (ISO creator)
├── CHANGELOG.md           # Detailed change log
├── CONTRIBUTING.md        # Contribution guidelines
├── README.md              # ← You are here
├── RELEASE_NOTES.md       # v4.0.0 release notes
└── SECURITY.md            # Security policy
```

---

##  Documentation

- **[Version History](docs/VERSION_HISTORY.md)**: Complete v1.0 → v4.0 evolution
- **[Capabilities v4.0](docs/reference/CAPABILITIES_v2.0.md)**: Full feature overview
- **[Developer Guide](docs/guides/DEVELOPER_GUIDE.md)**: Architecture, build, debugging
- **[Deployment Guide](docs/guides/DEPLOYMENT_GUIDE.md)**: USB boot, RPi4 setup, Cloud-Init
- **[API Reference](docs/reference/API_REFERENCE.md)**: Rustdoc overview
- **[CHANGELOG](CHANGELOG.md)**: Full v4.0 release notes

---

##  Architecture

```
┌─────────────────────────────────────────────────────────┐
│                 AetherOS Kernel v4.0                     │
├─────────────────────────────────────────────────────────┤
│  Core       │ Scheduler  │ IPC        │ Security (RBAC)│
│  Network    │ Graphics   │ UI         │ Input/Media    │
│  Enterprise │ Cloud      │ Telemetry  │ Distributed    │
└─────────────────────────────────────────────────────────┘
         │                    │                   │
    x86_64 PC          Raspberry Pi 4        Cloud VM
         │                    │                   │
    ┌────┴────────────────┴─────────────────┴────┐
    │  Compatibility: POSIX │ ART │ WASM │ OCI  │
    └────────────────────────────────────────────┘
```

### Key Components

- **SMME**: Symbian-Modern Memory Engine with two-phase allocation
- **Active Objects**: Message-passing concurrency model
- **Global Mesh**: DHT-based discovery for internet-scale clusters
- **Capability Market**: Trade computational resources (TFLOPS for Tokens)
- **Quantum-Ready**: Simulation stubs for Qubits and Superposition

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

**Current Status**: v4.0.0 Enterprise Release with RBAC and Audit Logging.

---

##  Vision: Internet of Abilities (v5.0)

AetherOS v5.0 is currently in active development ("Phase 19").

### v5.0 Roadmap
1. **Global Device Mesh**: Implemented (Phase 19.1)
2. **AI-Native OS**: NPU Driver & Federated Learning (Phase 19.2)
3. **Quantum Computing**: Simulator & Logic Gates (Phase 19.3)
4. **Brain-Computer Interface**: NeuralLink Integration (Phase 19.4)
5. **The Singularity**: Unified Boot Sequence (Phase 19.5)

---

##  Project Status

| Phase | Status | Version |
|-------|--------|---------|
| Core Foundation | ✅ Complete | v1.0 |
| Scheduler & IPC | ✅ Complete | v1.2 |
| Multi-Platform | ✅ Complete | v1.4 |
| Security & Power | ✅ Complete | v1.5 |
| Distributed System | ✅ Complete | v1.6 |
| Testing | ✅ Complete | v1.6.1 |
| Framework Services | ✅ Complete | v1.7 |
| Distributed Finalization | ✅ Complete | v1.8 |
| Documentation | ✅ Complete | v1.9 |
| Pre-Release Stabilization | ✅ Complete | v2.0 |
| **Production Hardening** | ✅ **Complete** | **v2.0.x** |
| **Network & Distributed** | ✅ **Complete** | **v2.1** |
| **Enhanced UX** | ✅ **Complete** | **v2.2** |
| **Ecosystem Foundation** | ✅ **Complete** | **v2.5** |
| **Cross-Platform Bridge** | ✅ **Complete** | **v3.0** |
| **Enterprise & Cloud** | ✅ **Complete** | **v4.0** |
| **Internet of Abilities** | 🚧 **In Progress** | **v5.0** |

**Overall**: 18/18 Phases Complete (v4.0), Phase 19 In Progress.

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
