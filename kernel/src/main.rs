#![no_std]
#![no_main]

use aetheros_kernel::{kernel_init, kernel_tick, BOOT_PARAMS};

// Multiboot2 header (manual)
#[link_section = ".multiboot2_header"]
#[used]
static MULTIBOOT2_HEADER: [u8; 16] = [
    0x02, 0x00, 0x00, 0x00,  // magic
    0x00, 0x00, 0x00, 0x00,  // architecture (i386)
    0x10, 0x00, 0x00, 0x00,  // header length
    0x00, 0x00, 0x00, 0x00,  // checksum
];

#[no_mangle]
pub extern "C" fn kernel_main_grub(magic: u32, info_ptr: usize) -> ! {
    if magic != 0x36d76289 {
        panic!("Not booted by Multiboot2");
    }

    let cmdline = unsafe { aetheros_kernel::boot::cmdline::find_multiboot2_cmdline(info_ptr) }.unwrap_or("");
    let params = aetheros_kernel::boot::cmdline::parse_cmdline(cmdline);
    
    // Captured LFB Info for High-Res Graphics
    let _fb_info = unsafe { aetheros_kernel::boot::cmdline::find_multiboot2_framebuffer(info_ptr) };

    {
        let mut lock = BOOT_PARAMS.lock();
        *lock = params;
    }

    // Pass the Multiboot2 Info Pointer for full tag discovery
    kernel_init(info_ptr);
    
    loop {
        kernel_tick();
    }
}
