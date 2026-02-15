# Secure Boot & Signing Guide (Outline)

This document outlines the signing workflow for Secure Boot integration.

1. Key generation
   - Create an X.509 signing keypair for the organization
   - Protect private keys in HSM or secure vault

2. Sign components
   - Sign EFI shim and GRUB binaries
   - Sign kernel modules and kernel images where required

3. Enrollment
   - Document steps to enroll organization keys into device firmware for OEM partners

4. Development workflow
   - Use developer keys for local testing; rotate to production keys for releases

Notes:
- Secure Boot integration requires coordination with OEMs for preinstalled devices.
- Use established tooling: `sbsign`, `pesign`, `openssl`.
