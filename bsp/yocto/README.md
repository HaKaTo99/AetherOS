# Yocto BSP Guide — Minimal Images for Embedded Devices

Use Yocto to create production-grade images for embedded devices and instruments.

## Quickstart
- Create BSP layer `meta-aetheros` with machine config for your target.
- Add recipes for minimal userland and kernel.

## Recommendations
- Use `meta-openembedded` and `meta-security` for common packages and security features.
- Keep images minimal; prefer Buildroot for very small devices.
