# Audit: `kernel/src/memory/mmu.rs`

Summary:
- Provides MMU setup for AArch64: page table setup, TCR/MAIR/SCTLR configuration, and helper to unmap pages for stack guards.

Strengths:
- Uses typed `PageTable` and `Mapper` helpers; clearly separates setup/configure/enable stages and documents safety requirements.
- Implements W^X attributes and maps `.text`, `.rodata`, `.data`, heap, and device regions.

Risks/Findings:
- Inline assembly uses system register writes (`msr`, `mrs`) which must be validated across CPU variants; missing error handling if environment lacks required features.
- `PERIPHERAL_BASE` index calculation for `l1` uses shift/and operations; verify architecture-dependent assumptions on translation granule and index ranges.
- `unmap_page()` directly manipulates page table entries without TLB write-back ordering beyond `tlbi` + `dsb`/`isb` — acceptable but should be validated on target hardware for corner cases.
- No explicit checks for overlapping mappings or duplicate mappings; mapper responsibilities must be audited.

Recommendations:
1. Add unit/integration tests under QEMU/CI to validate the MMU init sequence on supported aarch64 targets.
2. Add CPU feature checks and fallbacks in `init()` to ensure the runtime environment supports required page sizes and TCR settings.
3. Consider centralizing mapping validation in `Mapper` (e.g., assert unmapped before mapping) and add invariants checks in debug builds.

References:
- `kernel/src/memory/mmu.rs`
