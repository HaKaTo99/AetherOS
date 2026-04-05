pub mod smme;
pub mod paging; // Paging module for AArch64
pub mod x86_64_paging; // [NEW] Sovereign Paging for x86_64
pub mod mmu;    // MMU setup
pub mod predictive; // Phase 27.6

#[derive(Debug, Clone, Copy)]
pub struct MemoryUsage {
    pub used_pages: u64,
    pub total_pages: u64,
}

/// Mendapatkan statistik penggunaan memori saat ini (simulasi untuk Sovereign v1.0)
pub fn get_usage_stats() -> MemoryUsage {
    // Simulasi beban 25% pada RAM 128MB
    MemoryUsage {
        used_pages: 8192,
        total_pages: 32768,
    }
}
