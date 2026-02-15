# BSP Guide — Creating a Board Support Package for AetherOS

This guide explains what a BSP for AetherOS should include and provides templates/checklist for maintainers.

Minimum BSP contents:
- Kernel config and patches
- Device tree (DT) or ACPI tables
- Bootloader config (U-Boot, rpi firmware)
- Minimal root filesystem (Buildroot/Yocto) and packaging scripts
- HAL implementation (Q-HAL) that maps kernel drivers to `kernel/src/hal` traits
- Driver & firmware blobs (documented and licensed)

Checklist:
- [ ] Kernel bootable on target (serial + framebuffer)
- [ ] Device drivers for network, storage, and GPU (or software fallback)
- [ ] HAL implementation (Power, Crypto, DeviceManager)
- [ ] Image creation scripts and flashing instructions
- [ ] CI job to build BSP images and smoke-test in QEMU or device farm

Example: mapping HAL
```
// kernel/src/hal/mod.rs
pub trait PowerController { fn set_performance_mode(&self, mode: u8); fn battery_level(&self) -> u8; }
// BSP implements these methods and registers global instance
```

For more details, see `bsp/rpi/README.md` and the `docs/PORTING_ROADMAP.md` roadmap.
