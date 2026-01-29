//! Device Tree Blob (DTB) / Flattened Device Tree (FDT) Parser
//! Implements a minimal zero-allocation parser for the FDT format.
//! Specification: Devicetree Specification v0.4



const FDT_MAGIC: u32 = 0xd00dfeed;

// Basic FDT Token types
const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

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

    /// Return an iterator over the device tree structure
    pub fn nodes(&self) -> DtbIterator<'a> {
        let struct_offset = u32::from_be(self.header.off_dt_struct) as usize;
        let strings_offset = u32::from_be(self.header.off_dt_strings) as usize;
        
        // Ensure offsets are within bounds
        if struct_offset >= self.base.len() || strings_offset >= self.base.len() {
             return DtbIterator {
                base: self.base,
                offset: self.base.len(),
                strings_start: 0,
            };
        }

        DtbIterator {
            base: self.base,
            offset: struct_offset,
            strings_start: strings_offset,
        }
    }
}

pub struct DtbIterator<'a> {
    base: &'a [u8],
    offset: usize,
    strings_start: usize,
}

#[derive(Debug)]
pub enum DtbItem<'a> {
    BeginNode(&'a str),
    EndNode,
    Property { name: &'a str, value: &'a [u8] },
    End,
    Error,
}

impl<'a> DtbIterator<'a> {
    fn read_u32(&mut self) -> Option<u32> {
        if self.offset + 4 > self.base.len() {
            return None;
        }
        
        let val = unsafe {
            let ptr = self.base.as_ptr().add(self.offset) as *const u32;
            u32::from_be(*ptr)
        };
        self.offset += 4;
        Some(val)
    }
    
    fn align_4(&mut self) {
        self.offset = (self.offset + 3) & !3;
    }
    
    fn get_string(&self, offset: usize) -> Option<&'a str> {
        let abs_offset = self.strings_start + offset;
        if abs_offset >= self.base.len() {
            return None;
        }
        let mut end = abs_offset;
        while end < self.base.len() && self.base[end] != 0 {
            end += 1;
        }
        if end >= self.base.len() { return None; }
        core::str::from_utf8(&self.base[abs_offset..end]).ok()
    }
}

impl<'a> Iterator for DtbIterator<'a> {
    type Item = DtbItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.align_4();
        let token = self.read_u32()?;
        match token {
            FDT_BEGIN_NODE => {
                let start = self.offset;
                let mut end = start;
                while end < self.base.len() && self.base[end] != 0 {
                    end += 1;
                }
                if end >= self.base.len() { return Some(DtbItem::Error); }
                let name = match core::str::from_utf8(&self.base[start..end]) {
                    Ok(s) => s,
                    Err(_) => return Some(DtbItem::Error),
                };
                self.offset = end + 1;
                self.align_4();
                Some(DtbItem::BeginNode(name))
            },
            FDT_END_NODE => Some(DtbItem::EndNode),
            FDT_PROP => {
                let len = self.read_u32()? as usize;
                let name_off = self.read_u32()? as usize;
                let name = self.get_string(name_off)?;
                if self.offset + len > self.base.len() { return Some(DtbItem::Error); }
                let value = &self.base[self.offset..self.offset + len];
                self.offset += len;
                self.align_4();
                Some(DtbItem::Property { name, value })
            },
            FDT_NOP => self.next(),
            FDT_END => Some(DtbItem::End),
            _ => Some(DtbItem::Error),
        }
    }
}



