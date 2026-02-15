# AetherOS Deployment Guide

**Version:** 3.0.0  
**Last Updated:** February 15, 2026

This guide covers deploying AetherOS to physical hardware: x86_64 PCs (USB boot) and Raspberry Pi 4 (SD card).

---

## Table of Contents

1. [x86_64 Deployment (Bootable USB)](#x86_64-deployment)
2. [Raspberry Pi 4 Deployment (SD Card)](#raspberry-pi-4-deployment)
3. [Troubleshooting](#troubleshooting)
4. [Next Steps](#next-steps)

---

## x86_64 Deployment

### Prerequisites

- AetherOS kernel binary (built for x86_64-unknown-none)
- USB flash drive (minimum 512MB)
- `dd` utility (Linux/macOS) or Rufus/balenaEtcher (Windows)

### Step 1: Build Bootable Image

**Windows (PowerShell)**:
```powershell
cd d:\GitHub\AetherOS\kernel
.\tools\build_iso.ps1
```

**Linux/macOS**:
```bash
cd kernel
cargo bootimage --release --target x86_64-unknown-none
```

This creates `target/x86_64-unknown-none/release/bootimage-aetheros-kernel.bin`

### Step 2: Flash to USB

**Linux**:
```bash
# Identify USB device (e.g., /dev/sdb)
lsblk

# Flash image (CAUTION: This will erase all data on the USB!)
sudo dd if=target/x86_64-unknown-none/release/bootimage-aetheros-kernel.bin \
        of=/dev/sdX \
        bs=4M \
        status=progress \
        conv=fdatasync

# Replace /dev/sdX with your USB device
```

**macOS**:
```bash
# Identify USB device
diskutil list

# Unmount (e.g., /dev/disk2)
diskutil unmountDisk /dev/diskX

# Flash image
sudo dd if=target/x86_64-unknown-none/release/bootimage-aetheros-kernel.bin \
        of=/dev/rdiskX \
        bs=4m \
        status=progress
```

**Windows**:
1. Install [Rufus](https://rufus.ie/) or [balenaEtcher](https://www.balena.io/etcher/)
2. Select the `.bin` file
3. Select your USB drive
4. Click "Start"

### Step 3: Boot from USB

1. Insert USB into target PC
2. Restart and enter BIOS/UEFI (usually F2, F12, or Del)
3. Set USB as first boot device
4. Save and exit

**Expected Output** (VGA or serial):
```
[AetherOS] Kernel v3.0.0 booting...
[AetherOS] SMME initialized (3-tier pools)
[AetherOS] Scheduler: Priority-based Active Objects
[AetherOS] Distributed computing ready
[AetherOS] UI Framework loaded
```

---

## Raspberry Pi 4 Deployment

### Prerequisites

- AetherOS kernel binary (built for aarch64-unknown-none)
- MicroSD card (minimum 4GB, FAT32 formatted)
- USB-to-TTL serial adapter (for UART output)
- Raspberry Pi 4 Model B

### Step 1: Prepare SD Card

**Format as FAT32**:
- **Windows**: Right-click on SD card → Format → FAT32
- **Linux**: `sudo mkfs.vfat -F 32 /dev/sdX1`
- **macOS**: Disk Utility → Erase → MS-DOS (FAT)

### Step 2: Copy Kernel and Bootloader Files

**Build kernel**:
```bash
cd kernel
cargo build --release --target aarch64-unknown-none
```

**Copy to SD card**:
```bash
# Mount SD card (e.g., /media/boot or /Volumes/BOOT)
cp target/aarch64-unknown-none/release/aetheros-kernel /media/boot/kernel8.img
cp config.txt /media/boot/
```

**Create `config.txt`** (if not exists):
```
arm_64bit=1
kernel=kernel8.img
core_freq=250

# UART configuration
enable_uart=1
uart_baud=115200

# Disable GPU
gpu_mem=16
```

### Step 3: Connect UART (Optional but Recommended)

Connect USB-to-TTL adapter to RPi4 GPIO:
- **TX** → GPIO 14 (Pin 8)
- **RX** → GPIO 15 (Pin 10)
- **GND** → Ground (Pin 6)

**Open serial terminal** (115200 baud):
- **Linux**: `screen /dev/ttyUSB0 115200`
- **macOS**: `screen /dev/cu.usbserial 115200`
- **Windows**: Use PuTTY or TeraTerm

### Step 4: Boot Raspberry Pi 4

1. Insert SD card into RPi4
2. Connect power (USB-C, 5V 3A)
3. Monitor serial output

**Expected Output**:
```
[AetherOS] RPi4 BSP initialized
[AetherOS] UART: PL011 @ 115200 baud
[AetherOS] SMME: 3-tier memory pools ready
[AetherOS] Scheduler: 8 priority levels
[AetherOS] Network: BCM GENET ready (DHCP client active)
[SIM] High Load Simulated! Triggering Migration...
```

---

## Troubleshooting

### x86_64 Issues

**Q: System doesn't boot from USB**  
**A**: 
- Check BIOS boot order (USB should be first)
- Try disabling Secure Boot in UEFI settings
- Verify USB flash was successful: `sudo fdisk -l /dev/sdX`

**Q: Kernel panic immediately**  
**A**:
- Enable verbose logging: Rebuild with `RUSTFLAGS="--cfg debug_assertions"`
- Check serial output for panic message
- Verify build target is `x86_64-unknown-none`

**Q: No VGA output**  
**A**:
- Connect serial console to see boot logs
- VGA text mode requires legacy BIOS, not UEFI
- Try QEMU first to verify kernel functionality

### Raspberry Pi 4 Issues

**Q: No UART output**  
**A**:
- Verify UART wiring (TX ↔ RX, Ground connected)
- Check`enable_uart=1` in `config.txt`
- Ensure baud rate is 115200 on both sides

**Q: Kernel doesn't start**  
**A**:
- Verify SD card is FAT32 formatted
- Check `kernel8.img` exists in boot partition
- Ensure `arm_64bit=1` in `config.txt`
- Try official RPi firmware files (start4.elf, fixup4.dat)

**Q: RPi4 not responding**  
**A**:
- Check power supply (5V 3A minimum)
- Verify SD card is not corrupted
- Try rebuilding kernel: `cargo clean && cargo build --release`

### Build Issues

**Q: `cargo bootimage` fails**  
**A**:
```bash
# Install bootimage tool
cargo install bootimage

# Add llvm-tools-preview
rustup component add llvm-tools-preview
```

**Q: Linker errors**  
**A**:
```bash
# Update Rust nightly
rustup update nightly

# Clean and rebuild
cargo clean
cargo build --release --target <TARGET>
```

---

## Next Steps

After successful deployment:

1. **Explore distributed features**: Connect multiple devices for task migration
2. **Test UI**: VGA output shows widget demo on x86_64
3. **Network testing**: Enable Ethernet on RPi4 for device discovery
4. **Contribute**: Report issues or submit PRs on [GitHub](https://github.com/HaKaTo99/AetherOS)

---

## Support

- **Developer Guide**: `docs/DEVELOPER_GUIDE.md` - Build and debug instructions
- **API Reference**: Run `cargo doc --open` for rustdoc
- **Issues**: [GitHub Issues](https://github.com/HaKaTo99/AetherOS/issues)

---

**Happy Deploying!** 🎉
