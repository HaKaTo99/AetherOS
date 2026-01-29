//! MMU (Memory Management Unit) Setup for AArch64
//! Enables virtual memory with identity mapping

use crate::memory::paging::{PageTable, PageDescriptor, Mapper};
use core::arch::asm;

// Memory layout constants
const KERNEL_BASE: usize = 0x80000;           // Kernel load address
const KERNEL_SIZE: usize = 0x200000;          // 2MB kernel space
const PERIPHERAL_BASE: usize = 0xFE000000;    // RPi4 peripherals
const PERIPHERAL_SIZE: usize = 0x01800000;    // 24MB peripheral space

// Page table attributes
const ATTR_DEVICE: u64 = 0x04;     // Device memory (peripherals)
const ATTR_NORMAL: u64 = 0x44;     // Normal cacheable memory

// Translation Control Register (TCR_EL1) bits
const TCR_T0SZ: u64 = 16;          // 48-bit VA for TTBR0
const TCR_T1SZ: u64 = 16 << 16;    // 48-bit VA for TTBR1
const TCR_TG0_4K: u64 = 0 << 14;   // 4KB granule for TTBR0
const TCR_TG1_4K: u64 = 2 << 30;   // 4KB granule for TTBR1

// Memory Attribute Indirection Register (MAIR_EL1)
const MAIR_DEVICE_nGnRnE: u64 = 0x00;  // Device memory
const MAIR_NORMAL_NC: u64 = 0x44;      // Normal non-cacheable
const MAIR_NORMAL: u64 = 0xFF;         // Normal cacheable

// System Control Register (SCTLR_EL1) bits
const SCTLR_MMU_ENABLED: u64 = 1 << 0;    // MMU enable
const SCTLR_CACHE_ENABLED: u64 = 1 << 2;  // Data cache enable
const SCTLR_ICACHE_ENABLED: u64 = 1 << 12; // Instruction cache enable

// Static page tables (must be 4KB aligned)
#[repr(C, align(4096))]
struct PageTables {
    l0_table: PageTable,
    l1_tables: [PageTable; 4],
}

static mut PAGE_TABLES: PageTables = PageTables {
    l0_table: PageTable::new(),
    l1_tables: [PageTable::new(); 4],
};

pub struct Mmu;

impl Mmu {
    /// Initialize and enable MMU
    pub unsafe fn init() {
        // 1. Setup page tables
        Self::setup_page_tables();

        // 2. Configure MMU registers
        Self::configure_mmu();

        // 3. Enable MMU
        Self::enable_mmu();
    }

    unsafe fn setup_page_tables() {
        let tables = &mut *core::ptr::addr_of_mut!(PAGE_TABLES);
        
        // Setup L0 table (points to L1 tables)
        for i in 0..4 {
            let l1_addr = &tables.l1_tables[i] as *const _ as usize;
            tables.l0_table.entries[i].0 = 
                PageDescriptor::VALID | 
                PageDescriptor::TABLE | 
                (l1_addr as u64 & 0x0000_FFFF_FFFF_F000);
        }

        // Identity map kernel space (0x80000 - 0x280000)
        // Use raw pointer to avoid borrow checker issues
        let l1_kernel_ptr = &mut tables.l1_tables[0] as *mut PageTable;
        let mut mapper = Mapper::new(&mut *l1_kernel_ptr);
        
        // Map kernel as normal cacheable memory
        for addr in (KERNEL_BASE..KERNEL_BASE + KERNEL_SIZE).step_by(0x200000) {
            mapper.map_memory(addr, addr, ATTR_NORMAL | PageDescriptor::ACCESS);
        }

        // Identity map peripherals (0xFE000000+)
        let peripheral_l1_idx = (PERIPHERAL_BASE >> 30) & 0x3;
        let l1_peripheral_ptr = &mut tables.l1_tables[peripheral_l1_idx] as *mut PageTable;
        let mut peripheral_mapper = Mapper::new(&mut *l1_peripheral_ptr);
        
        for addr in (PERIPHERAL_BASE..PERIPHERAL_BASE + PERIPHERAL_SIZE).step_by(0x200000) {
            peripheral_mapper.map_memory(addr, addr, ATTR_DEVICE | PageDescriptor::ACCESS);
        }
    }

    unsafe fn configure_mmu() {
        let tables = &*core::ptr::addr_of!(PAGE_TABLES);
        let ttbr0 = &tables.l0_table as *const _ as u64;

        // Set Translation Table Base Register 0 (TTBR0_EL1)
        asm!("msr ttbr0_el1, {}", in(reg) ttbr0);

        // Set Translation Control Register (TCR_EL1)
        let tcr = TCR_T0SZ | TCR_T1SZ | TCR_TG0_4K | TCR_TG1_4K;
        asm!("msr tcr_el1, {}", in(reg) tcr);

        // Set Memory Attribute Indirection Register (MAIR_EL1)
        let mair = 
            (MAIR_DEVICE_nGnRnE << 0) |  // Index 0: Device
            (MAIR_NORMAL_NC << 8) |      // Index 1: Normal non-cacheable
            (MAIR_NORMAL << 16);         // Index 2: Normal cacheable
        asm!("msr mair_el1, {}", in(reg) mair);

        // Ensure all previous writes are visible
        asm!("dsb sy");
        asm!("isb");
    }

    unsafe fn enable_mmu() {
        // Read current SCTLR_EL1
        let mut sctlr: u64;
        asm!("mrs {}, sctlr_el1", out(reg) sctlr);

        // Enable MMU, caches, and instruction cache
        sctlr |= SCTLR_MMU_ENABLED | SCTLR_CACHE_ENABLED | SCTLR_ICACHE_ENABLED;

        // Write back SCTLR_EL1
        asm!("msr sctlr_el1, {}", in(reg) sctlr);

        // Ensure MMU is enabled before continuing
        asm!("dsb sy");
        asm!("isb");
    }

    /// Check if MMU is enabled
    pub fn is_enabled() -> bool {
        let sctlr: u64;
        unsafe {
            asm!("mrs {}, sctlr_el1", out(reg) sctlr);
        }
        (sctlr & SCTLR_MMU_ENABLED) != 0
    }

    /// Unmap a page (for stack guards)
    pub unsafe fn unmap_page(virt_addr: usize) {
        let tables = &mut *core::ptr::addr_of_mut!(PAGE_TABLES);
        
        // Get L1 index
        let l1_idx = (virt_addr >> 30) & 0x3;
        let l1_offset = (virt_addr >> 21) & 0x1FF;
        
        // Mark as invalid
        tables.l1_tables[l1_idx].entries[l1_offset].0 = 0;
        
        // Flush TLB for this address
        asm!("tlbi vaae1, {}", in(reg) (virt_addr >> 12));
        asm!("dsb sy");
        asm!("isb");
    }

    /// Setup stack guard page
    pub unsafe fn setup_stack_guard(stack_base: usize, stack_size: usize) {
        let guard_addr = stack_base + stack_size;
        
        // Align to page boundary
        let guard_page = (guard_addr + 0xFFF) & !0xFFF;
        
        // Unmap guard page
        Self::unmap_page(guard_page);
    }
}
