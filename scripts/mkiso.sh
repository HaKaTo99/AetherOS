#!/usr/bin/env bash
# mkiso.sh — simple ISO builder skeleton for UEFI boot
set -euo pipefail

usage(){
  echo "Usage: $0 --kernel <vmlinuz> --rootfs <rootfs_dir> --efi <BOOTX64.EFI> --out <out.iso>"
  exit 1
}

KERNEL=""
ROOTFS=""
EFI_BIN=""
OUT="aetheros.iso"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --kernel) KERNEL="$2"; shift 2;;
    --rootfs) ROOTFS="$2"; shift 2;;
    --efi) EFI_BIN="$2"; shift 2;;
    --out) OUT="$2"; shift 2;;
    *) echo "Unknown arg: $1"; usage;;
  esac
done

if [[ -z "${KERNEL}" || -z "${ROOTFS}" || -z "${EFI_BIN}" ]]; then
  usage
fi

if ! command -v xorriso >/dev/null 2>&1; then
  echo "[error] xorriso not found"
  exit 1
fi

tmpdir=$(mktemp -d)
mkdir -p "$tmpdir/EFI/BOOT"
cp "$KERNEL" "$tmpdir/vmlinuz"
cp -r "$ROOTFS"/* "$tmpdir/"
cp "$EFI_BIN" "$tmpdir/EFI/BOOT/BOOTX64.EFI"

cat > "$tmpdir/EFI/BOOT/grub.cfg" <<'EOF'
set timeout=0
set default=0
menuentry "AetherOS" {
  multiboot2 /vmlinuz
  boot
}
EOF

# Create ISO (requires xorriso)
xorriso -as mkisofs -iso-level 3 -o "$OUT" \
    -volid "AETHEROS" -eltorito-alt-boot \
    -e EFI/BOOT/BOOTX64.EFI -no-emul-boot "$tmpdir"

echo "ISO created: $OUT"
