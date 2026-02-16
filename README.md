# AetherOS

**Version 6.0.0** - Quantum Fortress (Security & AI Release)

![AetherOS](https://img.shields.io/badge/AetherOS-v6.0_Quantum_Fortress-blueviolet)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen)
![Status](https://img.shields.io/badge/Status-Stable-green)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-x86__64%20|%20aarch64%20|%20quantum-blue.svg)](https://github.com/HaKaTo99/AetherOS)

AetherOS is a microkernel operating system written in Rust, designed for the "Internet of Abilities". V6.0 introduces Zero-Trust Security, Homomorphic Encryption stubs, and an advanced AI Oracle Engine.

---

##  🚀 How to Run (Windows/x86_64)

I have prepared a one-click launcher script for the easiest experience.

1.  **Prerequisite**: Install QEMU
    ```powershell
    winget install -e --id SoftwareFreedomConservancy.QEMU
    # Verify installation
    qemu-system-x86_64 --version
    ```
2.  **Launch**:
    ```powershell
    .\scripts\run_aetheros_x86.ps1
    ```
    *(Note: If using CMD, remove the leading `.\`. See [QEMU Documentation](https://www.qemu.org/docs/master/) for troubleshooting)*

---

##  Features

### Core Kernel
- **SMME Memory Allocator**: Three-tier pool architecture (64KB + 2MB + 16MB)
- **Active Object Scheduler**: Priority-based preemptive multitasking
- **Quantum Bus RPC**: Lightweight cross-device communication
- **Capability System**: Token-based security and access control

### Quantum Fortress (v6.0)
- **Zero-Trust**: Continuous attestation stub for verifying system integrity.
- **AI Oracle v2**: Predictive resource management engine.
- **PQC**: Post-Quantum Cryptography (Kyber/Dilithium) integration.
- **Immutable Core**: Atomic update logic.

### Internet of Abilities (v5.0)
- **AI-Native**: NPU drivers with asynchronous job queues for neural processing.
- **Quantum Ready**: Built-in simulator for Qubits, Entanglement, and Quantum Gates.
- **Global Mesh**: Kademlia DHT for decentralized, internet-scale device discovery.
- **Brain Interface**: `NeuralLink` driver for thought-based UI control.

### Enterprise & Cloud (v4.0)
- **RBAC**: Role-Based access control for fleet management
- **Telemetry**: Real-time metrics and health monitoring
- **Cloud-Init**: Auto-configuration for AWS/GCP/Azure environments

### User Interface
- **Widget System**: Label, Button, Panel, TextBox components
- **FlexLayout Engine**: Responsive layout system
- **Graphics Stack**: VGA driver (x86_64), SimpleFB (aarch64)
- **Input Handling**: PS/2 keyboard, USB HID, BCI (Neural)

### Multi-Platform Support
- **x86_64**: PC with VGA output, bootable via USB
- **aarch64**: Raspberry Pi 4 with UART console
- **Simulated**: Quantum Processor Unit (QPU), Neural Processing Unit (NPU)

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

---

##  Project Structure

```
AetherOS/
├── boot/                  # Boot configuration (GRUB)
├── bsp/                   # Board Support Packages
├── compiler/              # AetherScript compiler
├── docs/                  # Documentation
│   ├── archive/           #   Historical docs
│   ├── guides/            #   Developer & deployment guides
│   ├── reference/         #   API & capabilities reference
│   ├── reports/           #   Implementation & test reports
│   ├── MASTER_TODO.md     #   Full development roadmap
│   └── VERSION_HISTORY.md #   v1.0 → v6.0 evolution
├── examples/              # AetherScript example apps
├── kernel/                # Kernel source code
│   ├── src/
│   │   ├── ai/            #   NPU Driver & Job Queue (v5.0)
│   │   ├── distributed/   #   Mesh, DHT, Market (v5.0)
│   │   ├── drivers/       #   Device drivers (BCI, USB, etc.)
│   │   ├── enterprise/    #   RBAC, Cloud, Telemetry
│   │   ├── quantum/       #   Quantum Simulator (v5.0)
│   │   ├── runtime/       #   Universal Runtimes
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
├── RELEASE_NOTES_v6.0.md  # v6.0.0 release notes
└── SECURITY.md            # Security policy
```

---

##  Documentation

- **[Version History](docs/VERSION_HISTORY.md)**: Complete v1.0 → v6.0 evolution
- **[Capabilities v4.0](docs/reference/CAPABILITIES_v4.0.md)**: Feature overview
- **[Developer Guide](docs/guides/DEVELOPER_GUIDE.md)**: Architecture, build, debugging
- **[Deployment Guide](docs/guides/DEPLOYMENT_GUIDE.md)**: USB boot, RPi4 setup
- **[API Reference](docs/reference/API_REFERENCE.md)**: Rustdoc overview

---

##  Architecture

```
┌─────────────────────────────────────────────────────────┐
│              AetherOS Kernel v6.0 (Quantum)             │
├─────────────────────────────────────────────────────────┤
│  Core       │ Scheduler  │ IPC        │ Security (PQC) │
│  Network    │ Graphics   │ UI         │ BCI / Oracle   │
│  Enterprise │ Cloud      │ Telemetry  │ Global Mesh    │
└─────────────────────────────────────────────────────────┘
         │                    │                   │
    x86_64 PC          Raspberry Pi 4        QPU (Sim)
         │                    │                   │
    ┌────┴────────────────┴─────────────────┴────┐
    │  Compatibility: POSIX │ ART │ WASM │ OCI  │
    └────────────────────────────────────────────┘
```

### Key Components

- **SMME**: Symbian-Modern Memory Engine
- **Global Mesh**: DHT-based discovery for internet-scale clusters
- **QuantumSim**: Integrated qubit simulation with entanglement
- **NeuralLink**: Driver for brain-computer interfaces
- **Oracle Engine**: AI-driven resource prediction

---

##  Project Status

| Phase | Status | Version |
|-------|--------|---------|
| Core & Stability | ✅ Complete | v2.0 |
| Distributed & Net | ✅ Complete | v2.1 |
| UX & Ecosystem | ✅ Complete | v2.5 |
| Cross-Platform | ✅ Complete | v3.0 |
| Enterprise & Cloud | ✅ Complete | v4.0 |
| Internet of Abilities | ✅ Complete | v5.0 |
| **Quantum Fortress** | ✅ **Complete** | **v6.0** |

**Overall**: 24/24 Phases Complete (100%).

---

##  License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

---

##  Acknowledgments

- Built with Rust 
- Inspired by Symbian OS, Zircon, and seL4
- Dedicated to the future of distributed computing.

---

**Star this repo if you find it interesting!**
