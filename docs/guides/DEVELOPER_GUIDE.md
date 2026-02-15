# AetherOS Developer Guide

**Version:** 2.0.0  
**Last Updated:** February 2, 2026

Welcome to the AetherOS Developer Guide! This document will help you understand, build, debug, and contribute to the AetherOS kernel.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Getting Started](#getting-started)
3. [Building from Source](#building-from-source)
4. [Running in QEMU](#running-in-qemu)
5. [Debugging Guide](#debugging-guide)
6. [Contributing](#contributing)

---

## Architecture Overview

AetherOS is a microkernel operating system written in Rust, designed for distributed computing across multiple platforms (RPi4, x86_64, Android).

### Core Components

```
┌─────────────────────────────────────────────────────────┐
│                    AetherOS Kernel                      │
├─────────────────────────────────────────────────────────┤
│  Phase 1-2: Foundation                                  │
│  ├─ SMME (Memory Allocator)                            │
│  ├─ Active Object Scheduler (Priority-based)           │
│  ├─ Synchronization Primitives (Mutex, RwLock, etc.)   │
│  └─ IPC (Quantum Channel RPC)                          │
├─────────────────────────────────────────────────────────┤
│  Phase 3-5: Platform & Distribution                     │
│  ├─ HAL (RPi4, x86_64, Android)                        │
│  ├─ Network Stack (smoltcp)                             │
│  ├─ Device Discovery (Beacon protocol)                  │
│  └─ AI Inference (Oracle predictor)                     │
├─────────────────────────────────────────────────────────┤
│  Phase 7-8: Services & Distribution                     │
│  ├─ Graphics (VGA, SimpleFB)                            │
│  ├─ UI Framework (Widget + FlexLayout)                  │
│  ├─ Input (PS/2 Keyboard - polling)                     │
│  ├─ Task Migration                                      │
│  ├─ Distributed KV Store                                │
│  ├─ Load Balancing                                      │
│  └─ Internal Simulation (Auto-Stress Test)              │
└─────────────────────────────────────────────────────────┘
```

### Design Philosophy

- **Symbian DNA**: Inspired by Symbian OS design patterns (Active Objects, two-phase construction)
- **Rust Safety**: Memory-safe by default, `unsafe` only where necessary
- **Microkernel**: Minimal kernel, services in userspace (future)
- **Distributed-First**: Multi-device computing as a core feature

---

## Getting Started

### Prerequisites

1. **Rust Toolchain** (nightly required)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default nightly
   ```

2. **Cross-Compilation Targets**
   ```bash
   rustup target add x86_64-unknown-none
   rustup target add aarch64-unknown-none
   ```

3. **QEMU** (for testing)
   - **Linux**: `sudo apt install qemu-system-x86 qemu-system-aarch64`
   - **macOS**: `brew install qemu`
   - **Windows**: Download from [qemu.org](https://www.qemu.org/download/)

4. **Optional Tools**
   - `gdb` or `lldb` for debugging
   - `cargo-bootimage` for x86_64 bootable images

---

## Building from Source

### Clone the Repository

```bash
git clone https://github.com/HaKaTo99/AetherOS.git
cd AetherOS/kernel
```

### Build for x86_64

```bash
# PowerShell (Windows)
$env:RUSTFLAGS="-C relocation-model=static"
cargo build --release --target x86_64-unknown-none

# Bash (Linux/macOS)
export RUSTFLAGS="-C relocation-model=static"
cargo build --release --target x86_64-unknown-none
```

**Output**: `target/x86_64-unknown-none/release/aetheros-kernel`

### Build for aarch64 (Raspberry Pi 4)

```bash
cargo check --target aarch64-unknown-none
```

**Output**: `target/aarch64-unknown-none/release/aetheros-kernel`

### Common Build Issues

**Error**: `cannot find crate for 'std'`  
**Solution**: Ensure you're using a `no_std` compatible target

**Error**: `linker 'rust-lld' not found`  
**Solution**: Update Rust nightly: `rustup update nightly`

---

## Running in QEMU

### x86_64

```bash
qemu-system-x86_64 \
  -kernel target/x86_64-unknown-none/release/aetheros-kernel \
  -serial stdio \
  -display none

# With VGA output
qemu-system-x86_64 \
  -kernel target/x86_64-unknown-none/release/aetheros-kernel \
  -serial stdio
```

### aarch64 (Raspberry Pi 4)

```bash
qemu-system-aarch64 \
  -M raspi4b \
  -kernel target/aarch64-unknown-none/release/aetheros-kernel \
  -serial stdio \
  -display none
```

**Expected Output**:
```
[AetherOS] Booting kernel...
[AetherOS] SMME initialized
[AetherOS] Scheduler initialized
[AetherOS] Distributed computing ready
```

---

## Debugging Guide

### Using GDB with QEMU

1. **Start QEMU in debug mode**:
   ```bash
   qemu-system-x86_64 \
     -kernel target/x86_64-unknown-none/release/aetheros-kernel \
     -s -S \
     -serial stdio
   ```
   - `-s`: Open gdbserver on port 1234
   - `-S`: Freeze CPU at startup

2. **Connect GDB**:
   ```bash
   gdb target/x86_64-unknown-none/release/aetheros-kernel
   (gdb) target remote localhost:1234
   (gdb) break kernel_main
   (gdb) continue
   ```

### Logging

Use the `kprintln!` macro for kernel logging:

```rust
use crate::kprintln;

kprintln!("Task {} scheduled", task_id);
```

Logs appear on the serial console (UART).

### Analyzing Panics

When the kernel panics, it prints a stack trace:

```
[PANIC] at kernel/src/memory/smme.rs:123
Out of memory in L0 pool
Stack backtrace:
  #0: 0xffffffff80001234 - smme::allocate
  #1: 0xffffffff80005678 - scheduler::create_task
```

Check the file and line number to locate the issue.

---

## Contributing

We welcome contributions! Please follow these guidelines:

### Code Style

- **Formatting**: Use `cargo fmt` before committing
- **Linting**: Run `cargo clippy` and fix warnings
- **Documentation**: Add rustdoc comments (`///`) to all public APIs

### Testing

- **Unit Tests**: Add tests for new functionality
- **Build Verification**: Ensure both x86_64 and aarch64 build successfully

### Git Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit changes: `git commit -m "feat: Add new feature"`
4. Push: `git push origin feature/my-feature`
5. Open a Pull Request on GitHub

### PR Requirements

- Clear description of changes
- No breaking changes without discussion
- All tests pass
- Code is formatted and linted

---

## Additional Resources

- **API Documentation**: See `target/doc/aetheros_kernel/index.html` (run `cargo doc --open`)
- **Master TODO**: `docs/MASTER_TODO.md` - Full development roadmap
- **Security Policy**: `SECURITY.md` - Security guidelines
- **Deployment Guide**: `docs/DEPLOYMENT_GUIDE.md` - Hardware deployment

---

## Support

- **Issues**: [GitHub Issues](https://github.com/HaKaTo99/AetherOS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/HaKaTo99/AetherOS/discussions)

---

**Happy Hacking!** 🚀
