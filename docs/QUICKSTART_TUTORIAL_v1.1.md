# QUICK START TUTORIAL v1.1 — Developer Guide

Last updated: 2026-01-02

## 5-Minute Quick Start

### Step 1: Installation

macOS / Linux

```bash
curl -sSL https://get.aetheros.dev | bash
```

Windows (WSL2)

```powershell
wsl --install Ubuntu
curl -sSL https://get.aetheros.dev | bash
```

Docker

```bash
docker run -it aetheros/dev:latest
```

### Step 2: Verify Installation

```bash
aether --version
aether --help
aether doctor
```

### Create your first AetherScript app

```bash
aether new hello-world --template=distributed
cd hello-world
```

Example `src/main.aethersrc` and build/run commands included in full doc.

### Build & Run

Local:

```bash
aether build
aether run
```

Distributed:

```bash
aether devices discover
aether run --distributed
```

### Packaging & Deployment

```bash
aether package
aether package --sign --certificate=mycert.pem
aether deploy --device=raspberry-pi-4
```

### Development workflow

- `aether dev` (hot reload)
- `aether test`, `aether debug`, `aether profile`

## Learning Resources

- `aether learn` interactive tutorials
- `aether docs` to open local documentation
