//! Virtual Memory Paging (AArch64 VMSA)
//! Supports 4KB granule, 48-bit PA

use core::marker::PhantomData;

/// Page Table Entry (Descriptor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageDescriptor(pub u64);

impl PageDescriptor {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn is_valid(&self) -> bool {
        (self.0 & 1) != 0
    }

    pub fn set_addr(&mut self, addr: usize) {
        // Clear old addr (bits 47:12) and set new
        // 0x0000_FFFF_FFFF_F000
        let mask = 0x0000_FFFF_FFFF_F000;
        self.0 = (self.0 & !mask) | ((addr as u64) & mask);
    }
    
    pub fn addr(&self) -> usize {
        (self.0 & 0x0000_FFFF_FFFF_F000) as usize
    }

    pub fn set_flags(&mut self, flags: u64) {
        self.0 |= flags;
    }
    
    pub const VALID: u64 = 1 << 0;
    pub const TABLE: u64 = 1 << 1; 
    pub const BLOCK: u64 = 0 << 1; 
    pub const ACCESS: u64 = 1 << 10;
}

/// A 4KB Page Table containing 512 entries
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageDescriptor; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        Self {
            entries: [PageDescriptor(0); 512],
        }
    }
}

/// Generic Page Mapper
pub struct Mapper {
    root_table: &'static mut PageTable,
}

impl Mapper {
    pub fn new(root: &'static mut PageTable) -> Self {
        Self { root_table: root }
    }
    
    /// Map a virtual address to a physical address (L2/2MB Block usually, simplified to L1/1GB here for v1.2)
    pub fn map_memory(&mut self, virt: usize, phys: usize, flags: u64) {
        // Simplified VMSA: Assuming using 1GB blocks at Level 1 for kernel space
        // Virt Addr: [...][L1 Index 9 bits][Block Offset 30 bits]
        // 30 bits = 1GB
        let l1_index = (virt >> 30) & 0x1FF;
        
        let entry = &mut self.root_table.entries[l1_index];
        // Set flags: Valid(1) | Block(0) | Custom Flags
        // Note: Block bit is actually 0 in bit 1 for L0/L1/L2 blocks, Table is 1.
        // Wait, for L1:
        // 0: Invalid
        // 1: Block (for L1/L2) -> 1GB/2MB
        // 1: Table (for L0/L1/L2) -> Pointer to next level
        // Actually AArch64:
        // Entry[1] == 0 => Block
        // Entry[1] == 1 => Table
        // Entry[0] == 1 => Valid
        
        // Let's assume we want a Block entry: Valid(1) | NotTable(0)
        // entry = flags | phys | 1
        
        // Clear everything
        entry.0 = 0;
        
        // Set Valid
        entry.0 |= PageDescriptor::VALID;
        
        // Set as Block (ensure bit 1 is 0)
        entry.0 &= !PageDescriptor::TABLE; 
        
        // Set Address
        entry.set_addr(phys);
        
        // Set other flags (Access, etc)
        entry.set_flags(flags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_descriptor_manipulation() {
        let mut desc = PageDescriptor::empty();
        assert!(!desc.is_valid());

        desc.0 = PageDescriptor::VALID;
        assert!(desc.is_valid());

        let phys_addr = 0x4000_0000;
        desc.set_addr(phys_addr);
        assert_eq!(desc.addr(), phys_addr);
        assert!(desc.is_valid()); // Should still be valid
    }

    #[test]
    fn test_mapping_logic() {
        // Create a fake page table on stack (or heap if test allows)
        // Since it is 4KB aligned, might be tricky on stack. 
        // Box might guarantee alignment or just use a static/global or carefully constructed struct.
        // For unit test, alignment is less critical unless HW uses it. Rust struct alignment helps.
        
        let mut table = PageTable::new();
        let mut mapper = Mapper::new(&mut table);

        let virt = 0x4000_0000; // 1GB boundary
        let phys = 0x8000_0000;
        let flags = PageDescriptor::ACCESS;

        mapper.map_memory(virt, phys, flags);

        let index = (virt >> 30) & 0x1FF;
        let entry = table.entries[index];

        assert!(entry.is_valid());
        assert_eq!(entry.addr(), phys);
        assert_eq!(entry.0 & PageDescriptor::ACCESS, PageDescriptor::ACCESS);
    }
}
