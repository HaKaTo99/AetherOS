//! Device Tree Blob (DTB) / Flattened Device Tree (FDT) Parser
//! Implements a minimal zero-allocation parser for the FDT format.
//! Specification: Devicetree Specification v0.4

use core::mem::size_of;

const FDT_MAGIC: u32 = 0xd00dfeed;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FdtHeader {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceTree<'a> {
    base: &'a [u8],
    header: &'a FdtHeader,
}

impl<'a> DeviceTree<'a> {
    /// Create a new DeviceTree parser from a raw pointer
    pub unsafe fn from_raw(ptr: *const u8) -> Option<Self> {
        // Basic alignment check
        if ptr as usize % 4 != 0 {
            return None;
        }

        let header = &*(ptr as *const FdtHeader);
        let header_magic = u32::from_be(header.magic);

        if header_magic != FDT_MAGIC {
            return None;
        }

        let total_size = u32::from_be(header.totalsize) as usize;
        let slice = core::slice::from_raw_parts(ptr, total_size);

        Some(Self {
            base: slice,
            header,
        })
    }
    
    pub fn total_size(&self) -> usize {
        u32::from_be(self.header.totalsize) as usize
    }

    // Helper to byte-swap (big endian to little endian)
    fn be_to_le(val: u32) -> u32 {
        u32::from_be(val)
    }
}

// Basic FDT Token types
const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;
