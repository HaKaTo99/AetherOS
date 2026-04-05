//! Sovereign x86_64 Paging Engine
//! Resolves Black Screen by mapping physical memory in Long Mode.

use alloc::boxed::Box;
use core::arch::asm;

/// x86_64 Page Table Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct X86Entry(pub u64);

impl X86Entry {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const CACHE_DISABLE: u64 = 1 << 4;
    pub const TABLE: u64 = 1 << 1;
    pub const PAGE_SIZE: u64 = 1 << 7; // Huge page (1GB in PDPE, 2MB in PDE)

    pub fn is_present(&self) -> bool { (self.0 & Self::PRESENT) != 0 }
    
    pub fn set_addr(&mut self, addr: u64) {
        let mask = 0x000F_FFFF_FFFF_F000;
        self.0 = (self.0 & !mask) | (addr & mask);
    }

    pub fn addr(&self) -> u64 { self.0 & 0x000F_FFFF_FFFF_F000 }
}

#[repr(C, align(4096))]
pub struct X86PageTable {
    pub entries: [X86Entry; 512],
}

/// Mendapatkan alamat fisik PML4 saat ini dari register CR3.
pub unsafe fn get_pml4_phys() -> u64 {
    let cr3: u64;
    asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
    cr3 & 0x000F_FFFF_FFFF_F000
}

static mut LFB_PDPT: X86PageTable = X86PageTable { entries: [X86Entry(0); 512] };
static mut LFB_PD: X86PageTable = X86PageTable { entries: [X86Entry(0); 512] };

/// Memetakan rentang memori fisik ke alamat virtual (Identity Mapping untuk LFB).
pub unsafe fn map_lfb_identity(phys_addr: u64, size_bytes: usize) {
    let mut pml4_phys = get_pml4_phys();
    let pml4 = pml4_phys as *mut X86PageTable;
    
    // [SOVEREIGN] Static Identity Mapping for v10.3 SUPREME
    let mut current_phys = phys_addr;
    let end_phys = phys_addr.saturating_add(size_bytes as u64);

    while current_phys < end_phys {
        let pml4_index = ((current_phys >> 39) & 0x1FF) as usize;
        let pdpt_index = ((current_phys >> 30) & 0x1FF) as usize;
        let pd_index = ((current_phys >> 21) & 0x1FF) as usize;

        let pml4_entry = &mut (*pml4).entries[pml4_index];
        if !pml4_entry.is_present() {
            let pdpt_phys = &LFB_PDPT as *const X86PageTable as u64;
            pml4_entry.0 = X86Entry::PRESENT | X86Entry::WRITABLE;
            pml4_entry.set_addr(pdpt_phys);
        }

        let pdpt = pml4_entry.addr() as *mut X86PageTable;
        let pdpt_entry = &mut (*pdpt).entries[pdpt_index];

        if !pdpt_entry.is_present() {
            let pd_phys = &LFB_PD as *const X86PageTable as u64;
            pdpt_entry.0 = X86Entry::PRESENT | X86Entry::WRITABLE;
            pdpt_entry.set_addr(pd_phys);
        }

        let pd = pdpt_entry.addr() as *mut X86PageTable;
        let pde = &mut (*pd).entries[pd_index];
        // [SOVEREIGN] MMIO Mapping: Forced Write-Through + Cache-Disable
        pde.0 = X86Entry::PRESENT | X86Entry::WRITABLE | X86Entry::WRITE_THROUGH | X86Entry::CACHE_DISABLE | X86Entry::PAGE_SIZE;
        pde.set_addr(current_phys);

        current_phys = current_phys.saturating_add(0x200000); // 2MB chunk
    }

    // Elite Sovereign Flush: Reload CR3 to ensure all levels are synchronized
    asm!("mov {}, cr3", out(reg) pml4_phys);
    asm!("mov cr3, {}", in(reg) pml4_phys);
}
