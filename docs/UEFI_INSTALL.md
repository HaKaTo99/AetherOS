# UEFI Installer Guide (Prototype)

Steps to create a UEFI-bootable installer ISO and install AetherOS on a PC:

1. Build kernel and initramfs
2. Prepare filesystem layout for ISO (EFI/BOOT, /boot, rootfs)
3. Include `shimx64.efi` and `grubx64.efi` signed appropriately
4. Use `xorriso` / `mkisofs` to create UEFI ISO (see `scripts/mkiso.sh`)
5. Boot ISO on target, run installer to write image to disk

Installer should support:
- Partitioning and formatting
- Secure Boot key enrollment (if requested)
- Package selection and optional OEM drivers
