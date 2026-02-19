#![no_std]
#![no_main]

use aetheros_kernel::{kernel_init, kernel_tick};
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // Initialize kernel with no DTB pointer (legacy path)
    kernel_init(0);

    // Main kernel loop
    loop {
        kernel_tick();
    }
}
