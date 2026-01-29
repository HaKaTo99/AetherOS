#![no_std]
#![no_main]

use aetheros_kernel::{kernel_init, kernel_tick};

#[no_mangle]
pub extern "C" fn kernel_main(dtb_ptr: usize, _x1: usize, _x2: usize, _x3: usize) -> ! {
    // Initialize all subsystems with hardware context
    kernel_init(dtb_ptr);

    // Main kernel loop
    loop {
        kernel_tick();
    }
}
