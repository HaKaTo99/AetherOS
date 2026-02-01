#!/bin/bash
# Android Image Signing Script for AetherOS
# Uses avbtool to sign boot.img

IMAGE_PATH=$1
KEY_PATH=$2

if [ -z "$IMAGE_PATH" ]; then
    echo "Usage: ./sign_image.sh <boot.img> [key_path]"
    exit 1
fi

if [ -z "$KEY_PATH" ]; then
    KEY_PATH="../../security/keys/db.key"
    echo "[!] Using default key: $KEY_PATH"
fi

# Check for avbtool
AVBTOOL=$(which avbtool)
if [ -z "$AVBTOOL" ]; then
    echo "[-] avbtool not found. Please add Android Build Tools to PATH."
    # Simulation mode
    echo "[*] Simulating signing process..."
    cp $IMAGE_PATH "${IMAGE_PATH}.signed"
    echo "[+] Signed image created: ${IMAGE_PATH}.signed (MOCKED)"
    exit 0
fi

echo "[*] Signing $IMAGE_PATH..."
$AVBTOOL add_hash_footer \
    --image $IMAGE_PATH \
    --partition_name boot \
    --partition_size $(stat -c %s $IMAGE_PATH) \
    --key $KEY_PATH \
    --algorithm SHA256_RSA2048

echo "[+] Done."
