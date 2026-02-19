# Audit: build & launch scripts (select)

Files examined:
- `scripts/cross-build.sh`
- `scripts/mkiso.sh`
- `scripts/aetheros-launch.sh`
- `scripts/run_aetheros_x86.ps1`
- `scripts/run_virtualbox.ps1`
- `security/generate_keys.ps1`

Summary:
- Scripts cover cross-builds, ISO creation, QEMU/VirtualBox launch, Android BSP helpers, and key generation. They include helpful checks and fallbacks for missing tools.

Risks/Findings:
- `scripts/aetheros-launch.sh` performs external network health checks and queries GitHub; this may disclose local environment status or cause launch failures in air-gapped environments.
- `security/generate_keys.ps1` creates MOCK keys when OpenSSL not found — which is useful for dev but dangerous if accidentally used in release images.
- Several bash scripts assume `xorriso`/`grub` exist; Windows users must use WSL or adjust paths.

Recommendations:
1. Add an explicit `--offline` or `--no-network` flag to `aetheros-launch.sh` and equivalent helpers.
2. Change `generate_keys.ps1` to error out in release mode; provide a `--dev` flag to allow mock key creation for local testing only.
3. Document required tools in `CONTRIBUTING.md` and make scripts idempotent and explicit about side effects.

References:
- `scripts/aetheros-launch.sh`
- `security/generate_keys.ps1`
