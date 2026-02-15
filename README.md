# AetherOS

**Version 3.0.0** - Cross-Platform Distributed Operating System

![AetherOS](https://img.shields.io/badge/AetherOS-v4.0_Enterprise-blueviolet)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen)
![Status](https://img.shields.io/badge/Status-Stable-green)
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

##  Project Structure

```
AetherOS/
├── boot/                  # Boot configuration (GRUB)
├── bsp/                   # Board Support Packages
│   ├── android/           #   Android device BSP
│   ├── buildroot/         #   Buildroot integration
│   ├── rpi/               #   Raspberry Pi 4 BSP
│   └── yocto/             #   Yocto Linux BSP
├── compiler/              # AetherScript compiler
├── docs/                  # Documentation
│   ├── archive/           #   Historical docs (v1.x)
│   ├── guides/            #   Developer & deployment guides
│   ├── porting/           #   Platform porting guides
│   ├── reference/         #   API & capabilities reference
│   ├── reports/           #   Implementation & test reports
│   ├── MASTER_TODO.md     #   Full development roadmap
│   └── VERSION_HISTORY.md #   v1.0 → v3.0 evolution
├── examples/              # AetherScript example apps
├── installer/             # OS installer
├── kernel/                # Kernel source code
│   ├── src/
│   │   ├── ai/            #   Oracle AI predictor
│   │   ├── arch/          #   Architecture (aarch64, x86_64)
│   │   ├── bus/           #   Quantum Bus RPC
│   │   ├── distributed/   #   KV store, migration, load balancer
│   │   ├── drivers/       #   Device drivers (input, video, etc.)
│   │   ├── events/        #   Event router & processor
│   │   ├── hal/           #   Hardware Abstraction Layer
│   │   ├── ipc/           #   Inter-Process Communication
│   │   ├── loader/        #   ELF loader & user mode
│   │   ├── memory/        #   SMME allocator, MMU, paging
│   │   ├── net/           #   Network stack (BCM GENET, VirtIO)
│   │   ├── oracle/        #   AI inference engine
│   │   ├── runtime/       #   POSIX, Android, WASM, containers
│   │   ├── scheduler/     #   Active Object scheduler
│   │   ├── security/      #   Capabilities, KASLR, hardening
│   │   ├── syscall/       #   System call interface
│   │   ├── testing/       #   Benchmarks & perf metrics
│   │   ├── tests/         #   Unit & stress tests
│   │   ├── ui/            #   Window manager, widgets, toolkit
│   │   └── virt/          #   Virtualization
│   ├── Cargo.toml
│   └── build.rs
├── scripts/               # Build & deployment scripts
├── security/              # Keys & signing infrastructure
├── tools/                 # Build tools (ISO creator)
├── website/               # Landing page
├── CHANGELOG.md           # Detailed change log
├── CONTRIBUTING.md        # Contribution guidelines
├── Dockerfile             # Container build
├── Makefile               # Build automation
├── README.md              # ← You are here
├── RELEASE_NOTES.md       # v3.0.0 release notes
└── SECURITY.md            # Security policy
```

---

##  Documentation

- **[Version History](docs/VERSION_HISTORY.md)**: Complete v1.0 → v3.0 evolution
- **[Capabilities v3.0](docs/reference/CAPABILITIES_v2.0.md)**: Full feature overview (Phase 1-15)
- **[Developer Guide](docs/guides/DEVELOPER_GUIDE.md)**: Architecture, build, debugging
- **[Deployment Guide](docs/guides/DEPLOYMENT_GUIDE.md)**: USB boot, RPi4 setup
- **[API Reference](docs/reference/API_REFERENCE.md)**: Rustdoc overview
- **[CHANGELOG](CHANGELOG.md)**: Full v3.0 release notes

---

##  Architecture

```
┌─────────────────────────────────────────────────────────┐
│                 AetherOS Kernel v3.0                     │
├─────────────────────────────────────────────────────────┤
│  Core       │ Scheduler  │ IPC        │ Security       │
│  Network    │ Graphics   │ UI         │ Input/Media    │
│  Ecosystem  │ Runtimes   │ Containers │ WASM           │
└─────────────────────────────────────────────────────────┘
         │                    │                   │
    x86_64 PC          Raspberry Pi 4        Android
         │                    │                   │
    ┌────┴────────────────┴─────────────────┴────┐
    │  Compatibility: POSIX │ ART │ WASM │ OCI  │
    └────────────────────────────────────────────┘
```

### Key Components

- **SMME**: Symbian-Modern Memory Engine with two-phase allocation
- **Active Objects**: Message-passing concurrency model
- **Quantum Bus**: TLS-encrypted RPC for distributed computing
- **AetherScript**: Custom language with WASM codegen
- **Cross-Platform**: POSIX, Android ART, WASM, and Container runtimes

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

**Current Status**: v3.0.0 cross-platform release with KASLR, TLS, SecureChannel, WASM sandboxing.

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

**Overall**: 15/15 phases complete (100%) 

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
