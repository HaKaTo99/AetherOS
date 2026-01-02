#!/usr/bin/env bash
# mkiso.sh — simple ISO builder skeleton for UEFI boot
set -euo pipefail

usage(){
  echo "Usage: $0 --kernel <vmlinuz> --rootfs <rootfs_dir> --out <out.iso>"
  exit 1
}

KERNEL=""
ROOTFS=""
OUT="aetheros.iso"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --kernel) KERNEL="$2"; shift 2;;
    --rootfs) ROOTFS="$2"; shift 2;;
    --out) OUT="$2"; shift 2;;
    *) echo "Unknown arg: $1"; usage;;
  esac
done

if [[ -z "${KERNEL}" || -z "${ROOTFS}" ]]; then
  usage
fi

tmpdir=$(mktemp -d)
mkdir -p "$tmpdir/EFI/BOOT"
cp "$KERNEL" "$tmpdir/vmlinuz"
cp -r "$ROOTFS"/* "$tmpdir/"

# Note: real implementation should add grub/shim and efi bootloader
cat > "$tmpdir/EFI/BOOT/BOOTX64.cfg" <<'EOF'
menuentry "AetherOS" {
  linux /vmlinuz root=/dev/ram0
}
EOF

# Create ISO (requires xorriso)
xorriso -as mkisofs -iso-level 3 -o "$OUT" \
    -volid "AETHEROS" -eltorito-alt-boot \
    -e EFI/BOOT/BOOTX64.EFI -no-emul-boot "$tmpdir"

echo "ISO created: $OUT"
