#![no_std]
#![no_main]

extern crate alloc;

use libaether::{open, exit};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Open the net scheme
    let net_fd = open("net:main", 0);
    if net_fd < 0 {
        exit(1);
    }

    // 2. [v10.4 Stage-2 Factorization] Initialize smoltcp in userspace
    // Thisdaemon will handle all TCP/IP logic, offloading the kernel.
    
    loop {
        // [MILITARY GRADE] Network polling loop
        // read(net_fd, rx_buffer);
        // interface.poll(rx_buffer);
        
        // Simulating the network heartbeat
        for _ in 0..5_000_000 {} 
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
