#![no_std]
#![no_main]

use aetheros_kernel::{kernel_init, kernel_tick, BOOT_PARAMS};

// entry_point!(kernel_main);

// fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
//     // Initialize kernel with no DTB pointer (legacy path)
//     kernel_init(0);

//     // Main kernel loop
//     loop {
//         kernel_tick();
//     }
// }

#[no_mangle]
pub extern "C" fn kernel_main_grub(magic: u32, info_ptr: usize) -> ! {
    if magic != 0x36d76289 {
        panic!("Not booted by Multiboot2");
    }

    let cmdline = unsafe { aetheros_kernel::boot::cmdline::find_multiboot2_cmdline(info_ptr) }.unwrap_or("");
    let params = aetheros_kernel::boot::cmdline::parse_cmdline(cmdline);
    {
        let mut lock = BOOT_PARAMS.lock();
        *lock = params;
    }

    kernel_init(0);
    loop {
        kernel_tick();
    }
}

