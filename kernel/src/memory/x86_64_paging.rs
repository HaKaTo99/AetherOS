//! Sovereign x86_64 Paging Engine
//! Resolves Black Screen by mapping physical memory in Long Mode.


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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// [v10.5.9] Sovereign PT Allocator (Elite Capacity: 2048 tables = 8MB)
static mut LFB_PT_POOL: [X86PageTable; 2048] = [X86PageTable { entries: [X86Entry(0); 512] }; 2048];
static mut NEXT_PT_INDEX: usize = 0;

unsafe fn alloc_page_table() -> *mut X86PageTable {
    if NEXT_PT_INDEX >= 2048 { return core::ptr::null_mut(); }
    let ptr = &mut LFB_PT_POOL[NEXT_PT_INDEX] as *mut X86PageTable;
    NEXT_PT_INDEX += 1;
    // Ensure clean table
    (*ptr).entries.fill(X86Entry(0));
    ptr
}

/// [v10.5.6] Flexible Sovereign Mapping
pub unsafe fn map_range_identity(phys_addr: u64, size_bytes: usize, flags: u64) {
    let pml4_phys = get_pml4_phys();
    let pml4 = pml4_phys as *mut X86PageTable;
    
    let aligned_phys = phys_addr & !0xFFF;
    let aligned_size = (size_bytes + (phys_addr as usize & 0xFFF) + 0xFFF) & !0xFFF;

    let mut current_phys = aligned_phys;
    let end_phys = aligned_phys.saturating_add(aligned_size as u64);

    while current_phys < end_phys {
        let pml4_index = ((current_phys >> 39) & 0x1FF) as usize;
        let pdpt_index = ((current_phys >> 30) & 0x1FF) as usize;
        let pd_index = ((current_phys >> 21) & 0x1FF) as usize;
        let pt_index = ((current_phys >> 12) & 0x1FF) as usize;

        // 1. PML4 -> PDPT
        let pml4_entry = &mut (*pml4).entries[pml4_index];
        if !pml4_entry.is_present() {
            let pdpt_ptr = alloc_page_table();
            if pdpt_ptr.is_null() { break; }
            pml4_entry.0 = X86Entry::PRESENT | X86Entry::WRITABLE;
            pml4_entry.set_addr(pdpt_ptr as u64);
        }
    
        // 2. PDPT -> PD
        let pdpt = pml4_entry.addr() as *mut X86PageTable;
        let pdpt_entry = &mut (*pdpt).entries[pdpt_index];
        if !pdpt_entry.is_present() {
            let pd_ptr = alloc_page_table();
            if pd_ptr.is_null() { break; }
            pdpt_entry.0 = X86Entry::PRESENT | X86Entry::WRITABLE;
            pdpt_entry.set_addr(pd_ptr as u64);
        }
    
        // 3. PD -> PT
        let pd = pdpt_entry.addr() as *mut X86PageTable;
        let pde = &mut (*pd).entries[pd_index];
        if !pde.is_present() {
            let pt_ptr = alloc_page_table();
            if pt_ptr.is_null() { break; }
            pde.0 = X86Entry::PRESENT | X86Entry::WRITABLE;
            pde.set_addr(pt_ptr as u64);
        }
    
        // 4. PT -> 4KB Page
        let pt = pde.addr() as *mut X86PageTable;
        let pte = &mut (*pt).entries[pt_index];
        
        pte.0 = flags;
        pte.set_addr(current_phys);

        current_phys = current_phys.saturating_add(0x1000);
    }
}

/// [v10.5.10] Sovereign Activation: Commits the constructed page tables to hardware.
pub unsafe fn activate_paging() {
    let pml4_phys = get_pml4_phys();
    asm!("mov cr3, {}", in(reg) pml4_phys);
}

pub unsafe fn map_lfb_identity(phys_addr: u64, size_bytes: usize) {
    let flags = X86Entry::PRESENT | X86Entry::WRITABLE | X86Entry::WRITE_THROUGH | X86Entry::CACHE_DISABLE;
    map_range_identity(phys_addr, size_bytes, flags);
}

pub unsafe fn map_heap_identity() {
    let flags = X86Entry::PRESENT | X86Entry::WRITABLE;
    map_range_identity(0x0800_0000, 128 * 1024 * 1024, flags);
}
