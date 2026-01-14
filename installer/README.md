# Installer Skeleton — AetherOS

This directory contains the skeleton for an installer/ISO generator for AetherOS on PC (UEFI) targets.

Planned components:
- `scripts/mkiso.sh` — create UEFI-bootable ISO
- `installer/efi/` — shim and GRUB config for UEFI Secure Boot
- `packages/` — packaging scripts for kernel, initramfs, and modules

Usage (prototype):
```bash
# Build kernel and rootfs
# Create ISO
./scripts/mkiso.sh --kernel target/x86_64/kernel --rootfs out/rootfs
```

Notes:
- Secure Boot signing requires key management; see `docs/SECURE_BOOT.md`.
