# AetherOS Script Unification Blueprint

Date: 2026-03-31
Scope: root scripts, scripts/, tools/, and documented run paths

## 1. Current State Summary

There are multiple overlapping script families:
- Root legacy launchers and test scripts (many absolute Windows paths).
- scripts/ utilities (generally better structured, mixed legacy/modern).
- tools/ workflows (contains the most consistent single-ISO path).

Observed consistency issues:
- Hardcoded absolute paths are widespread.
- ISO output path diverges between aetheros.iso and out/aetheros.iso.
- Multiple launchers with similar responsibilities increase operator ambiguity.

## 2. Canonical Operational Path (Recommended)

Adopt one canonical workflow for day-to-day use:
1. Build + stamp + single ISO rebuild:
   - tools/rebuild_vm_iso.ps1
   - output invariant: out/aetheros.iso
2. QEMU smoke verification:
   - scripts/qemu-smoke.ps1 (Windows)
   - scripts/qemu-smoke.sh (Linux)
3. Optional debug boot:
   - scripts/qemu-debug.sh

Why this path:
- deterministic output location,
- explicit build marker stamping,
- marker-based verification logic for boot confidence,
- lower ambiguity than legacy root launchers.

## 3. Deprecation Candidate Set

Candidate scripts to mark as legacy/deprecated (not immediate deletion):
- run_aetheros_production.ps1
- PRODUCTION_LAUNCHER.ps1
- run_aetheros.cmd
- run_qemu_test.ps1
- run_qemu_auto_test.ps1
- TEST_QEMU.ps1 / TEST_ALL_PLATFORMS.ps1 (if replaced by smoke + focused tests)
- root build_iso.ps1 (if scripts/build_iso.ps1 + tools/rebuild_vm_iso.ps1 fully adopted)

## 4. Refactor Principles

1. No absolute repo paths.
- derive paths from script root and repo root.

2. Tool discovery via command probing.
- qemu-system-x86_64, VBoxManage, grub-mkrescue detected at runtime.

3. One ISO convention.
- standardize to out/aetheros.iso for all docs and scripts.

4. Single source for launcher behavior.
- root launcher should delegate to tools/rebuild_vm_iso.ps1 and scripts/qemu-smoke.ps1.

5. Explicit profile flags.
- dev, smoke, debug profiles via parameters/environment, not separate duplicate scripts.

## 5. Suggested Migration Plan

Phase A: Documentation first
- Update README and guides to canonical path only.
- Add "Legacy scripts" section with compatibility note.

Phase B: Wrapper consolidation
- Keep legacy script filenames but convert internals into wrappers that call canonical scripts.
- Emit warning line: "legacy wrapper invoked".

Phase C: Path and output harmonization
- Replace hardcoded D:\GitHub\AetherOS references.
- Standardize ISO path to out/aetheros.iso.

Phase D: Test stabilization
- Make smoke tests the mandatory CI gate.
- Keep platform-specific tests as optional/manual suites.

## 6. Minimal Acceptance Criteria

- All primary docs reference one canonical build/run path.
- All primary scripts are path-portable (no hardcoded repo absolute path).
- All boot artifacts resolve to out/aetheros.iso unless explicitly overridden.
- Smoke tests pass on both Windows and Linux script variants.

## 7. Immediate Low-Risk Improvements

1. Add a root entry script that only dispatches to canonical tools path.
2. Add one shared helper for path resolution and tool checks.
3. Add a short "script status matrix" table (canonical vs legacy) in README.
