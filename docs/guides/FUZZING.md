# Fuzzing Guide (SMME)

Purpose:
- Provide a minimal plan to fuzz `SMME` (`kernel/src/memory/smme.rs`) to find memory corruption and logic bugs.

Requirements:
- Linux or WSL, `cargo`, `rustc` (nightly if necessary), `honggfuzz` or `cargo-fuzz` and `qemu` for VM-based tests.

Quickstart using `honggfuzz`:
1. Install `honggfuzz` and `cargo-hfuzz` per your platform instructions.
2. Add a small harness crate that links against kernel memory module and exposes allocation/deallocation operations.
3. Run `honggfuzz` targeting harness and provide mutational input sequences that encode: reserve,size,commit,deallocate cycles.

Notes:
- Start with unit-style harness that exercises small/medium/large allocations and randomly mixes frees and allocations.
- Run fuzzing in a VM (QEMU) for safety; use ASAN/LSAN if possible.

Suggested scripts: `tools/fuzz/run_fuzz_smme.sh` (helper to invoke container/VM).
