# Android Porting Guide

This document outlines the strategy to support Android devices (primarily ARM64) for AetherOS.

## Goals
- Run AetherOS as a native OS on developer-friendly Android devices.
- Provide compatibility for Android apps via ART where feasible.
- Provide clear instructions for developers to build and flash images.

## Target Devices (initial)
- Google Pixel (GSI-friendly)
- OnePlus (developer-friendly bootloader)
- Selected AOSP-compatible devices

## Strategies
1. Project Treble / GSI approach
   - Use Generic System Image (GSI) compatibility: build AetherOS system image that can run on Treble-enabled devices.
   - Keep vendor blobs accessible for hardware-specific functionality.

2. Bootloader & Unlock
   - Document steps to unlock bootloader for each target (OEM-specific).
   - Use fastboot to flash boot and vendor images during development.

3. Kernel & Drivers
   - Build kernel with vendor-compatible configs. Use vendor-provided modules for GPU/Wi-Fi when necessary.
   - Provide fallback software renderers if vendor drivers are unavailable.

4. ART Compatibility Layer
   - Integrate ART runtime or provide a compatibility runner to execute APKs in an isolated environment.
   - Support APK sideloading and package manager utilities.

5. Security & Signing
   - Document Secure Boot implications and how to use developer keys during development.
   - For production, pursue OEM partnerships to sign images.

## Developer Workflow
- Build system image (AetherOS system.img) targeting `arm64`.
- Unlock device bootloader → flash boot/system → test via serial/logcat and framebuffer.

## Limitations
- Closed-source vendor drivers may be required for full hardware support.
- Carrier-locked devices and locked bootloaders may be unsupported.
- Non-trivial fragmentation across OEMs.

## Next Steps & Checklist
- [ ] Create device-specific BSPs and flashing scripts
- [ ] Prototype ART runtime integration (APK execution proofs)
- [ ] Automate GSI build and test in CI (QEMU and device farm)
- [ ] Document vendor blob handling and licensing
