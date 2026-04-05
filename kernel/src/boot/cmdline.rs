//! Boot command line parsing and Multiboot2 helpers.
//! Minimal, no allocation.

#[derive(Debug, Clone, Copy, Default)]
pub struct BootParams {
    pub toram: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FramebufferInfo {
    pub address: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub fb_type: u8,
}

/// Parse whitespace-separated command line into `BootParams`.
pub fn parse_cmdline(cmdline: &str) -> BootParams {
    let mut params = BootParams::default();
    for token in cmdline.split_whitespace() {
        match token {
            "toram" => params.toram = true,
            "debug" => params.debug = true,
            _ => {}
        }
    }
    params
}

/// Extract command line string from a Multiboot2 info structure (if present).
/// Safety: `info_ptr` must point to a valid Multiboot2 info block provided by the bootloader.
pub unsafe fn find_multiboot2_cmdline(info_ptr: usize) -> Option<&'static str> {
    if info_ptr == 0 {
        return None;
    }
    let base = info_ptr as *const u8;
    let total_size = (base as *const u32).read() as usize;
    let mut offset = 8usize; // skip total_size and reserved

    while offset + 8 <= total_size {
        let tag = base.add(offset);
        let tag_type = (tag as *const u32).read();
        let tag_size = (tag.add(4) as *const u32).read() as usize;

        if tag_size < 8 || offset + tag_size > total_size {
            break;
        }

        if tag_type == 1 {
            let data_ptr = tag.add(8);
            let mut data_len = tag_size.saturating_sub(8);
            while data_len > 0 && *data_ptr.add(data_len - 1) == 0 {
                data_len -= 1;
            }
            if let Ok(s) = core::str::from_utf8(core::slice::from_raw_parts(data_ptr, data_len)) {
                return Some(s);
            }
        }

        if tag_type == 0 {
            break;
        }
        offset = ((offset + tag_size) + 7) & !7; // 8-byte alignment
    }
    None
}

/// Extract Framebuffer info from a Multiboot2 info structure (Tag Type 8).
pub unsafe fn find_multiboot2_framebuffer(info_ptr: usize) -> Option<FramebufferInfo> {
    if info_ptr == 0 { return None; }
    let base = info_ptr as *const u8;
    let total_size = (base as *const u32).read() as usize;
    let mut offset = 8usize;

    while offset + 8 <= total_size {
        let tag = base.add(offset);
        let tag_type = (tag as *const u32).read();
        let tag_size = (tag.add(4) as *const u32).read() as usize;

        if tag_size < 8 || offset + tag_size > total_size { break; }

        if tag_type == 8 {
            return Some(FramebufferInfo {
                address: (tag.add(8) as *const u64).read(),
                pitch: (tag.add(16) as *const u32).read(),
                width: (tag.add(20) as *const u32).read(),
                height: (tag.add(24) as *const u32).read(),
                bpp: *tag.add(28),
                fb_type: *tag.add(29),
            });
        }

        if tag_type == 0 { break; }
        offset = ((offset + tag_size) + 7) & !7;
    }
    None
}
