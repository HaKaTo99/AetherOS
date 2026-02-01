#!/bin/bash
# Script to flash AetherOS to an Android device using fastboot
# Usage: ./flash_android.sh <boot.img>

BOOT_IMG=$1

if [ -z "$BOOT_IMG" ]; then
    echo "Usage: $0 <boot.img>"
    echo "Example: $0 aetheros-boot.img"
    exit 1
fi

# Check if fastboot exists
if ! command -v fastboot &> /dev/null; then
    echo "Error: fastboot tool not found."
    echo "Please install android-platform-tools."
    exit 1
fi

echo "Checking for connected Fastboot devices..."
DEVICES=$(fastboot devices)

if [ -z "$DEVICES" ]; then
    echo "No devices found in fastboot mode."
    echo "Please connect your device and boot into bootloader (adb reboot bootloader)."
    exit 1
fi

echo "Found device:"
echo "$DEVICES"

echo ""
echo "WARNING: This will effectively 'boot' the custom image without flashing it permanently."
echo "This is the safest way to test."
echo "Press Ctrl+C to cancel or Enter to continue..."
read

echo "Booting AetherOS..."
fastboot boot "$BOOT_IMG"

# Optional: To flash permanently (DANGEROUS)
# echo "Flashing AetherOS..."
# fastboot flash boot "$BOOT_IMG"
