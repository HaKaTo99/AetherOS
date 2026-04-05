# AetherOS Production Guide v10.2

## Build Kernel

### Development Build
```powershell
# Jalankan dari root repositori
cd kernel
cargo build --release --target x86_64-unknown-none
```

### Production Build (Recommended)
Gunakan skrip kanonikal untuk memastikan build marker dan ISO tercipta dengan benar:

```powershell
.\Aether.ps1 -Action build
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

### Unified Launcher (Recommended)
```powershell
# Cara termudah untuk build dan run
.\Aether.ps1
```

### Legacy Launchers (Deprecated)
Skrip berikut telah diubah menjadi wrapper dan akan dialihkan ke Aether.ps1:
- `.\run_aetheros_production.ps1`
- `.\PRODUCTION_LAUNCHER.ps1`
- `.\run_aetheros.cmd`

### Command Line
```
cmd
cmd /c D:\GitHub\AetherOS\run_aetheros.cmd
```

## Create ISO Image

### Standard Build Tool
Gunakan alat terpadu untuk membangun ISO di lingkungan Windows (memerlukan WSL):

```powershell
.\tools\rebuild_vm_iso.ps1 -WslDistro Ubuntu
```

Output akan selalu berada di: `out\aetheros.iso`

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

## Production Readiness Checklist

1. `cargo +nightly -Z bindeps build --release --manifest-path kernel/Cargo.toml` succeeds with zero errors.
2. `powershell .\Aether.ps1 -Action smoke` completes with marker `AetherShell>` or successful output.
3. `powershell .\Aether.ps1 -Action stress` passes with no panics (>5 min).
4. `powershell .\Aether.ps1 -Action verifikasi` completes, log contains `CERTIFIED SUPREME`.
5. AetherOS kernel subsystems in `kernel/src/ui` are included (display/dashboard/splash/file_manager/organic_ui).
6. All critical warnings in stable release are audited & mitigated.

## Requirements

- QEMU 7.0+
- Rust 1.70+
- 4GB RAM minimum
- 20GB disk space
