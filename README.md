# AetherOS - The Universal Operating System

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Build Status](https://github.com/AetherOS-Project/aetheros/workflows/CI/badge.svg)](https://github.com/AetherOS-Project/aetheros/actions)

**Status**: Early Development (Week 1-4)  
**Version**: 0.1.0-alpha  
**Target**: Raspberry Pi 4/5 (ARM64)

## Vision

AetherOS menggabungkan DNA terbaik dari lima OS legendaris:
- **Symbian**: Efisiensi memori ekstrem
- **BlackBerry**: Keamanan tingkat militer
- **macOS**: UX premium
- **Android**: Ekosistem terbuka
- **HarmonyOS**: Distributed computing

## Quick Start

```bash
# Clone repository
git clone https://github.com/AetherOS-Project/aetheros
cd aetheros

# Setup development environment
docker-compose up -d dev

# Build kernel + compiler
make all

# Run in QEMU
make run-qemu
```

## Project Structure

```
aetheros/
├── kernel/          # Quantum Microkernel
├── compiler/        # AetherScript Compiler
├── sdk/            # Developer SDK
└── docs/           # Documentation
```

## Documentation

- [Blueprint v1.0](./docs/AETHEROS_BLUEPRINT_V1.md) - Master architecture
- [Quick Start Tutorial](./docs/QUICKSTART_TUTORIAL.md) - Step-by-step guide
- [Contributing](./CONTRIBUTING.md) - How to contribute

## License

Dual licensed:
- **AGPL-3.0** for open source community
- **Commercial** license available for enterprise

## Status (Week 1-4)

- [x] Blueprint finalized
- [ ] Kernel foundation (SMME + Scheduler)
- [ ] Compiler prototype
- [ ] Demo application

**Join us**: [Discord](https://discord.gg/aetheros) | [Twitter](https://twitter.com/AetherOS_Project)
