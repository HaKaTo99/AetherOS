# Testing AetherOS on Raspberry Pi 4

## Prerequisites
- Raspberry Pi 4 Model B (any RAM variant)
- MicroSD card (8GB+)
- USB-to-TTL serial adapter (3.3V)
- Jumper wires
- Computer with SD card reader

## Hardware Setup

### Serial Console Connection
Connect USB-to-TTL adapter to RPi4 GPIO header:
- **GND** (Pin 6) → Adapter GND
- **GPIO 14 (TXD)** (Pin 8) → Adapter RX
- **GPIO 15 (RXD)** (Pin 10) → Adapter TX

**IMPORTANT**: Do NOT connect 5V/3.3V power pins from adapter!

## SD Card Preparation

### 1. Format SD Card
- Create FAT32 partition
- Label: `AETHEROS`

### 2. Copy Boot Files
```bash
# Build kernel
cargo build --release --target aarch64-unknown-none --package aetheros-kernel

# Copy to SD card (replace X: with your drive letter)
copy target\aarch64-unknown-none\release\aetheros-kernel X:\kernel8.img
copy bsp\rpi\config.txt X:\
copy bsp\rpi\cmdline.txt X:\
```

### 3. Download Firmware Files
Download from https://github.com/raspberrypi/firmware/tree/master/boot:
- `bootcode.bin` (not needed for RPi4, but harmless)
- `start4.elf`
- `fixup4.dat`

Copy these to SD card root.

## Testing Procedure

### 1. Connect Serial Console
- Plug in USB-to-TTL adapter
- Open serial terminal (PuTTY, minicom, screen):
  - Baud rate: **115200**
  - Data bits: 8
  - Parity: None
  - Stop bits: 1
  - Flow control: None

### 2. Boot RPi4
- Insert SD card
- Power on RPi4
- Watch serial console

### Expected Output
```
=================================
  AetherOS v1.3 - Raspberry Pi 4
=================================
HAL initialized successfully
Kernel OK
```

## Troubleshooting

### No Output on Serial
- Check wiring (TX/RX might be swapped)
- Verify baud rate is 115200
- Ensure `enable_uart=1` in config.txt
- Try different USB port

### Kernel Doesn't Boot
- Verify `kernel8.img` is on SD card
- Check `config.txt` has `arm_64bit=1`
- Ensure firmware files are present
- Try reformatting SD card

### Garbage Characters
- Wrong baud rate (should be 115200)
- Incorrect voltage level (use 3.3V adapter)

## Next Steps
Once basic boot works:
1. Implement interrupt handling (GIC-400)
2. Setup timer interrupts for scheduler
3. Enable MMU for virtual memory
4. Test multitasking with multiple tasks

## Resources
- [RPi4 GPIO Pinout](https://pinout.xyz/)
- [BCM2711 Datasheet](https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf)
- [ARM Generic Timer](https://developer.arm.com/documentation/ddi0595/latest/)
