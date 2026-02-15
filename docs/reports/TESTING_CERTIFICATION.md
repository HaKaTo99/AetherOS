# Testing & Certification Plan

Outline for automated testing, device farm, and certification processes.

## Automated CI
- Unit tests: `cargo test` for crates that run on host
- Cross-target smoke tests in QEMU for boot/bootlog checks
- Nightly benchmarks and performance regressions

## Device Farm
- Self-hosted or cloud-managed devices for Raspberry Pi, Pixel, x86_64 laptops
- Automated flashing, boot, and smoke-test scripts

## Security Audits
- Static analysis of `unsafe` usage
- Fuzzing critical interfaces (IPC, driver IO)
- External penetration testing and code audits for release

## Compliance
- Maintain SBOM, export control, and licensing documentation
- Certification plan for enterprise (FIPS, Common Criteria) — roadmap item
