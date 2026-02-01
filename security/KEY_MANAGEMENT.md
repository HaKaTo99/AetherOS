# AetherOS Key Management Strategy

## UEFI Secure Boot Hierarchy

1.  **Platform Key (PK)**
    *   Root of trust for the hardware platform.
    *   Controls access to KEK database.
    *   *Usage*: Enrollment only.

2.  **Key Exchange Key (KEK)**
    *   Trusted by the platform to manage the Signature Database (db).
    *   *Usage*: Updating db/dbx.

3.  **Signature Database Key (db)**
    *   Used to sign the bootloader (GRUB/Shim) and Kernel.
    *   *Usage*: Daily build signing.

## Android Verified Boot (AVB) 2.0

*   **vbmeta**: Verified Boot Metadata. Contains hashes of partitions.
*   **Key**: RSA key (defaults to test-keys from AOSP).
*   **Chain of Trust**: Bootloader -> vbmeta -> boot/system/vendor.

## Development Workflow
1. Run `generate_keys.ps1` to create self-signed keys in `security/keys`.
2. Use `db.key` to sign kernel binaries during build.
3. For Android, use `avbtool` with generated keys.

> **WARNING**: Do not use these keys for production devices. They are self-signed and strictly for development/testing simulation.
