#!/bin/bash
# Script to create Android boot.img
# Usage: ./build_bootimg.sh <kernel_binary> <output_boot.img>

KERNEL=$1
OUTPUT=$2

if [ -z "$KERNEL" ] || [ -z "$OUTPUT" ]; then
    echo "Usage: $0 <kernel_binary> <output_boot.img>"
    exit 1
fi

# Kernel load address (standard for Pixel/Qualcomm)
# Base: 0x10000000 -> Kernel: 0x10008000
BASE=0x10000000
KERNEL_OFFSET=0x00008000
RAMDISK_OFFSET=0x01000000
TAGS_OFFSET=0x00000100

# Command line
CMDLINE="console=tty0 earlycon=efifb keep_bootcon root=/dev/ram0 init=/init"

# Check if mkbootimg exists
if ! command -v mkbootimg &> /dev/null; then
    echo "Error: mkbootimg not found. Please install it:"
    echo "  pip install mkbootimg"
    exit 1
fi

echo "Creating boot.img..."
mkbootimg \
    --kernel "$KERNEL" \
    --ramdisk /dev/null \
    --base "$BASE" \
    --kernel_offset "$KERNEL_OFFSET" \
    --ramdisk_offset "$RAMDISK_OFFSET" \
    --tags_offset "$TAGS_OFFSET" \
    --cmdline "$CMDLINE" \
    --os_version 12.0.0 \
    --os_patch_level 2024-01-01 \
    --header_version 2 \
    -o "$OUTPUT"

echo "Done: $OUTPUT"
echo "To flash: fastboot boot $OUTPUT"
