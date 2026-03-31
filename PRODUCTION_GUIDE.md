# AetherOS Production Guide v10.2

## Build Kernel

### Development Build
```
powershell
cd D:\GitHub\AetherOS\kernel
cargo build --release --target x86_64-unknown-none
```

### Production Build (Recommended)
```
powershell
cd D:\GitHub\AetherOS
cargo build --profile production --target x86_64-unknown-none
```

Production build includes:
- LTO fat (Link-Time Optimization)
- Maximum optimization (opt-level 3)
- Strip symbols
- No debug info
- Panic = abort

## Boot Parameters

AetherOS supports boot parameters for flexible startup:

### Available Parameters

| Parameter | Description |
|-----------|-------------|
| `toram` | Load entire system to RAM for faster execution |
| `toram=trim` | Load only essential modules to RAM |
| `load=mod1,mod2` | Explicitly load specific modules |
| `noload=mod1,mod2` | Skip specific modules |
| `verbose` | Enable verbose boot logging |
| `insecure` | Disable security features (testing only) |

### Example Usage

```
bash
# QEMU with toram mode
qemu-system-x86_64 -kernel aetheros-kernel -append "toram verbose"

# GRUB config
menuentry "AetherOS" {
    linux /boot/aetheros-kernel toram verbose
}
```

## Run with QEMU

### Development
```
powershell
powershell -ExecutionPolicy Bypass -File D:\GitHub\AetherOS\QUICKSTART_AETHEROS.ps1
```

### Production Binary
```
powershell
powershell -ExecutionPolicy Bypass -File D:\GitHub\AetherOS\run_aetheros_production.ps1 -Action dev
```

### Production Headless
```
powershell
powershell -ExecutionPolicy Bypass -File D:\GitHub\AetherOS\run_aetheros_production.ps1 -Action headless
```

### Build Kernel (Production Profile)
```
powershell
powershell -ExecutionPolicy Bypass -File D:\GitHub\AetherOS\run_aetheros_production.ps1 -Action build
```

### Command Line
```
cmd
cmd /c D:\GitHub\AetherOS\run_aetheros.cmd
```

## Create ISO Image

```
powershell
# Using Makefile
make iso-image

# Or manual
mkdir -p iso/boot/grub
cp target/x86_64-unknown-none/production/aetheros-kernel iso/boot/aetheros_kernel
grub-mkrescue -o aetheros.iso iso
```

## VirtualBox Setup

1. Open VirtualBox
2. New VM → Name: AetherOS
3. Type: Other, Version: Other/Unknown (64-bit)
4. Memory: 2048MB
5. Create Virtual Hard Disk → 20GB
6. Settings → Storage → Add ISO
7. Start VM

## AetherShell Commands

```
help           # Show all commands
status         # System status
memory         # Memory info
exit           # Exit
```

## Production Metrics Target

| Metric | Target |
|--------|--------|
| Uptime tanpa reboot | >90 hari |
| Boot failure rate | <0.01% |
| Memory leak per hari | <0.1KB |
| Driver failure rate | <0.05% |
| Recovery time | <500ms |

## Stability Features

AetherOS v10.2 includes these stability mechanisms:

- **Watchdog Timer**: Auto-recovery from hangs
- **Stack Canary**: Stack smashing detection
- **SMME Memory**: Predictive cleanup with self-audit
- **Priority Scheduler**: Deadlock prevention via priority inheritance

## Requirements

- QEMU 7.0+
- Rust 1.70+
- 4GB RAM minimum
- 20GB disk space
