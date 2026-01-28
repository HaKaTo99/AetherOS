#![no_std]
#![no_main]

use aetheros_kernel::{kernel_init, kernel_tick};

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Initialize all subsystems
    kernel_init();

    // Main kernel loop
    loop {
        kernel_tick();
    }
}
