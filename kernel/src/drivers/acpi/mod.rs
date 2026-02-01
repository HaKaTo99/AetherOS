//! ACPI Driver
//! Advanced Configuration and Power Interface Support

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RsdpDescriptor {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

impl RsdpDescriptor {
    /// Validate checksum
    pub fn is_valid(&self) -> bool {
        let ptr = self as *const _ as *const u8;
        let mut sum: u8 = 0;
        let len = if self.revision == 0 { 20 } else { 36 };
        
        for i in 0..len {
            unsafe { sum = sum.wrapping_add(*ptr.add(i)); }
        }
        
        sum == 0
    }
}

/// Search for RSDP in BIOS memory area (0xE0000 - 0xFFFFF)
pub unsafe fn find_rsdp() -> Option<&'static RsdpDescriptor> {
    // 0xE0000 to 0xFFFFF
    let start_addr = 0x000E0000;
    let end_addr = 0x000FFFFF;
    
    // Scan every 16 bytes
    let mut addr = start_addr;
    while addr < end_addr {
        let ptr = addr as *const u8;
        
        // Check for signature "RSD PTR "
        if *ptr == b'R' && *ptr.add(1) == b'S' && *ptr.add(2) == b'D' && *ptr.add(3) == b' ' &&
           *ptr.add(4) == b'P' && *ptr.add(5) == b'T' && *ptr.add(6) == b'R' && *ptr.add(7) == b' ' 
        {
            let rsdp = &*(addr as *const RsdpDescriptor);
            if rsdp.is_valid() {
                return Some(rsdp);
            }
        }
        
        addr += 16;
    }
    
    None
}

/// Initialize ACPI subsystem
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        if let Some(rsdp) = find_rsdp() {
            // Found ACPI RSDP, can proceed to parse tables
            // For Phase 3, detecting is enough proof of work
            // Since we don't have a logger ready here easily without crate linkage, 
            // valid detection is the goal.
            let _ = rsdp; 
        }
    }
}
