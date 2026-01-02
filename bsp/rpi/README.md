# Raspberry Pi BSP (Reference)

This directory contains guidance and templates to create a Board Support Package (BSP) for Raspberry Pi (4/5) as a reference platform for AetherOS.

Contents & Steps

1. Kernel
   - Use a mainline Linux kernel or a minimally patched kernel compatible with AetherOS subsystems.
   - Provide a device tree blob (DTB) for the target Pi revision.

2. Bootloader
   - Use U-Boot or the Pi's firmware to load a kernel image and initramfs.
   - Provide `boot/config.txt` and `cmdline.txt` templates.

3. Rootfs
   - Create minimal rootfs using Buildroot or a small BusyBox-based image for testing.
   - Include AetherOS userland or compatibility layers.

4. Drivers
   - For full hardware support, include vendor blobs where licensing allows (GPU, Wi-Fi).
   - Prefer open-source drivers where available.

5. Q-HAL
   - Implement `kernel/src/hal` traits (PowerController, CryptoEngine, DeviceManager) using platform services (VPU, PMIC, etc.).

6. Flashing & Testing
   - Steps to write image to SD card and boot:
     - `dd if=aetheros-rpi.img of=/dev/sdX bs=4M conv=fsync`
     - Insert SD card, power on, monitor serial console.

Notes
- This BSP is intended as a reference to get AetherOS running on an ARM SoC. For production support, additional work on Secure Boot, driver licensing, and performance tuning is required.
