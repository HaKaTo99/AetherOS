# Release & Update Process

Outline for releasing images, signing, and delivering updates.

- Nightly builds: automatically produced by CI and uploaded as artifacts
- Beta: tested builds promoted after QA
- Stable: signed and released with changelog and SBOM

Update mechanism:
- Delta updates signed and distributed via OTA
- Rollback support and staged rollouts
