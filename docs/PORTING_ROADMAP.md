# PORTING ROADMAP — AetherOS

## Tujuan
Dokumen ini merinci strategi praktis untuk membawa AetherOS ke perangkat seluler (Android, iPhone), PC, dan perangkat embedded/IoT. Termasuk target perangkat referensi, batasan utama, toolchain, CI, dan checklist implementasi.

## Target Perangkat Prioritas
1. Reference SBCs
   - Raspberry Pi 4/5 (ARM64) — referensi utama untuk ARM SoC, mudah diakses.
2. Android phones (unlocked bootloader preferred)
   - Google Pixel (generic Android GSI friendliness)
   - OnePlus / Fairphone (developer-friendly OEMs)
   - Selected Android OEMs with active bootloader unlock programs
3. Laptops & Desktops
   - x86_64 UEFI (Intel/AMD) — standard PC installer via UEFI/GRUB
   - ARM64 laptops (Surface Pro X class) — UEFI on ARM
4. RISC-V Platforms
   - SiFive HiFive / VisionFive — exploratory support
5. iPhone (iOS devices)
   - Note: Strong OEM restrictions — practical support requires partnership or jailbroken devices
6. IoT / Instruments
   - Yocto/Buildroot targets for constrained ARM/MIPS/RISC-V devices

## High-Level Strategy
- Start small: choose 3 initial platforms to fully support (suggest: Raspberry Pi 4, Pixel (GSI/PBR), x86_64 laptop). Use these to validate HAL, installer, and distribution mechanisms.
- Build `Q-HAL` (Quantum HAL) abstraction layer that isolates platform-specific drivers and exposes unified APIs for power, devices, graphics, crypto, and networking.
- Implement BSPs (Board Support Packages) per target containing kernel config, device tree (DTB), vendor driver glue, boot scripts, and packaging.
- Provide compatibility layers: Android ART runtime (for APK execution), WASM runtime, and POSIX compatibility shim.
- Use Buildroot/Yocto for embedded/IoT images; use standard kernel + modules for PC/phone targets.

## Platform-specific Notes
- Android
  - Use Project Treble / GSI approach where possible.
  - Workflow: unlock bootloader → install boot image/kernel → vendor blobs for Wi‑Fi/GPU as needed → ART compatibility.
  - For closed drivers, negotiate OEM/vendor licensing OR provide fallback software paths (LLVM/OpenCL drivers where feasible).
- iPhone
  - Apple enforces signed firmware; non-jailbreak installation infeasible without Apple cooperation. Focus on research, jailbreak support only for development and demos.
- PC (x86_64 / ARM64)
  - Provide UEFI installer/ISO, support Secure Boot (signing keys), driver packaging (NVIDIA/Intel/AMD via vendor-provided modules), and fallback software renderers.
- IoT / Instruments
  - Minimal images with Buildroot/Yocto, real-time kernels when needed, hardware abstraction for sensors and comms.

## Toolchain & CI
- Cross-toolchains
  - x86_64-unknown-linux-gnu (host build)
  - aarch64-unknown-linux-gnu (ARM64)
  - armv7-unknown-linux-gnueabihf (ARM32)
  - riscv64-unknown-elf (RISC-V)
- CI matrix (GitHub Actions / self-hosted runners)
  - Build artifacts per target, run unit tests in emulation (QEMU) where possible, and smoke-test images on device farm.
- QEMU-based tests
  - Boot validation, kernel log checks, basic networking and IPC tests.

## BSP & Driver Strategy
- Open-source drivers preferred; for closed-source blobs:
  - Maintain minimal user-space glue and vendor module loading facility.
  - Document license & redistribution constraints, provide instructions for users to obtain vendor blobs.
- Encourage OEM partnerships for signed drivers and Secure Boot keys.

## Security & Signing
- Secure Boot: provide key generation/signing workflow and documented instructions for OEMs.
- Verified boot & measured boot for enterprise devices.
- Memory encryption & TPM integration via Q-HAL primitives.

## Testing & Validation
- Device farm (self-hosted or cloud): automated flashing, boot tests, performance and power profiling, regression tests.
- Security audit pipeline: static analysis of `unsafe` usage, fuzzing critical subsystems, vulnerability scanning.

## Roadmap & Milestones (example)
- Week 0–4: Reference targets selection, cross-toolchain setup, Raspberry Pi BSP
- Week 4–8: Pixel GSI PoC (boot kernel + basic drivers), UEFI ISO prototype for x86_64
- Week 8–12: ART runtime integration, Buildroot minimal IoT image
- Week 12–20: Device farm setup, Secure Boot workflow, OEM outreach

## Checklist (actionable)
- [ ] Select 3 reference devices and procure hardware
- [ ] Add cross-toolchains to CI and produce nightly artifacts
- [ ] Implement Q-HAL core API and reference implementations for RPi and x86_64
- [ ] Create BSP repo templates and kernel configs
- [ ] Prototype UEFI installer + ISO generation
- [ ] Implement ART compatibility layer prototype for APK execution
- [ ] Define vendor driver handling policy and licensing docs
- [ ] Establish automated device farm flashing & test harness
- [ ] Draft OEM partnership outreach template and security/compliance docs

---

This roadmap is intended sebagai titik awal; saya dapat menguraikan langkah berikutnya menjadi issues/prioritized tasks, menambahkan CI jobs, atau membuat contoh BSP untuk Raspberry Pi sekarang jika Anda setuju.
