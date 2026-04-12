#![no_std]
#![no_main]

extern crate alloc;

mod desktop;

use libaether::{open, exit, Color};
use crate::desktop::{DesktopManager, Window};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Open the display scheme
    let display_fd = open("display:main", 0);
    if display_fd < 0 {
        exit(1);
    }

    // 2. [v10.4 Stage-1 Factorization] Map the framebuffer (Simulasi mmap via physical address)
    // In actual implementation, we'd use a real mmap syscall to get a virtual pointer.
    // For now, we simulate the 'Orbital' pulse logic here in Ring 3.
    
    let mut desktop = DesktopManager::new(1920, 1200);
    
    // Add some initial windows
    desktop.windows.push(Window {
        id: 1,
        title: alloc::string::String::from("Sovereign Terminal"),
        x: 100, y: 100, width: 600, height: 400,
        accent: Color::new(140, 100, 255),
        focused: true,
    });

    loop {
        // [MILITARY GRADE] User-space rendering loop
        // desktop.paint_all(fb); // fb is the mapped memory
        
        // Simulating the pulse to indicate userspace is active
        for _ in 0..10_000_000 {} 
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
