//! MMU (Memory Management Unit) Setup for AArch64
//! Enables virtual memory with W^X protection

use crate::memory::paging::{PageTable, PageDescriptor, Mapper};
use core::arch::asm;

// Linker symbols
extern "C" {
    static __text_start: usize;
    static __text_end: usize;
    static __rodata_start: usize;
    static __rodata_end: usize;
    static __data_start: usize;
    static __data_end: usize;
    static __bss_start: usize;
    static __bss_end: usize;
}

// Memory layout constants
const KERNEL_BASE: usize = 0x80000;           
const PERIPHERAL_BASE: usize = 0xFE000000;    
const PERIPHERAL_SIZE: usize = 0x01800000;    

// Page table attributes
const ATTR_DEVICE: u64 = 0x04 | ATTR_UXN | ATTR_PXN;     // Device: nGnRnE, No Exec
const ATTR_NORMAL: u64 = 0x44;                           // Normal cacheable

// Access Permissions and Execute Never bits
const ATTR_UXN: u64 = 1 << 54; // Unprivileged Execute Never
const ATTR_PXN: u64 = 1 << 53; // Privileged Execute Never
const ATTR_RO: u64 = 1 << 7;   // Read-Only (AP[2]=1)

// Derived attributes for W^X
// Derived attributes for W^X
const ATTR_CODE: u64   = ATTR_NORMAL | ATTR_RO | ATTR_UXN;  // RX (Kernel EL1 only), User NX
const ATTR_RODATA: u64 = ATTR_NORMAL | ATTR_RO | ATTR_UXN | ATTR_PXN; // RO, NX
const ATTR_DATA: u64   = ATTR_NORMAL | ATTR_UXN | ATTR_PXN;           // RW, NX

// Translation Control Register (TCR_EL1) bits
const TCR_T0SZ: u64 = 16;          // 48-bit VA for TTBR0
const TCR_T1SZ: u64 = 16 << 16;    // 48-bit VA for TTBR1
const TCR_TG0_4K: u64 = 0 << 14;   // 4KB granule for TTBR0
const TCR_TG1_4K: u64 = 2 << 30;   // 4KB granule for TTBR1

// Memory Attribute Indirection Register (MAIR_EL1)
const MAIR_DEVICE_N_GN_RN_E: u64 = 0x00;  // Device memory
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
    /// Initialize and enable MMU with W^X protection.
    ///
    /// # Safety
    /// This function:
    /// - Modifies system control registers (SCTLR, TCR, MAIR, TTBR0).
    /// - Enables the MMU, changing memory access semantics globally.
    /// - Requires exclusive access to memory setup sequence.
    /// - Must be called only once during early boot.
    pub unsafe fn init() {
        Self::setup_page_tables();
        Self::configure_mmu();
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

        // Identity map kernel space with granular permissions
        let l1_kernel_ptr = &mut tables.l1_tables[0] as *mut PageTable;
        let mut mapper = Mapper::new(&mut *l1_kernel_ptr);
        
        // Get section addresses from linker symbols
        let text_start = &__text_start as *const _ as usize;
        let text_end = &__text_end as *const _ as usize;
        let rodata_start = &__rodata_start as *const _ as usize;
        let rodata_end = &__rodata_end as *const _ as usize;
        let data_start = &__data_start as *const _ as usize;

        let bss_end = &__bss_end as *const _ as usize;

        // 1. Text (RX)
        for addr in (text_start..text_end).step_by(4096) {
            mapper.map_memory(addr, addr, ATTR_CODE | PageDescriptor::ACCESS);
        }

        // 2. Read-Only Data (RO, NX)
        for addr in (rodata_start..rodata_end).step_by(4096) {
            mapper.map_memory(addr, addr, ATTR_RODATA | PageDescriptor::ACCESS);
        }

        // 3. Data + BSS (RW, NX)
        // Combine Data and BSS ranges (assuming contiguous)
        let rw_start = data_start;
        let rw_end = bss_end;
        for addr in (rw_start..rw_end).step_by(4096) {
            mapper.map_memory(addr, addr, ATTR_DATA | PageDescriptor::ACCESS);
        }
        
        // 4. Map remaining kernel space (Heap/Stack area) as RW NX
        // Up to 2MB mark or further
        let heap_start = (bss_end + 4095) & !4095;
        let kernel_limit = KERNEL_BASE + 0x200000; // 2MB total
        if heap_start < kernel_limit {
            for addr in (heap_start..kernel_limit).step_by(4096) {
                mapper.map_memory(addr, addr, ATTR_DATA | PageDescriptor::ACCESS);
            }
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
            (MAIR_DEVICE_N_GN_RN_E << 0) |  // Index 0: Device
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
        // Make sure WXN (Write Execute Never) is set to enforce W^X globally if desired
        // sctlr |= (1 << 19); // WXN bit

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
