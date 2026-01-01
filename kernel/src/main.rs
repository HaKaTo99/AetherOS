//! AetherOS Quantum Microkernel
//! Week 2 Implementation - Entry Point

#![no_std]
#![no_main]
#![feature(asm_const)]

mod memory;
mod scheduler;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Initialize SMME
    let mut smme = memory::smme::SymbianModernMemoryEngine::new(1 << 30); // 1GB
    
    // Test allocation
    match smme.reserve(1 << 20) {
        Ok(_) => {
            // Success: Reserved 1MB
            smme.commit(512 << 10).expect("Commit failed");
        }
        Err(_) => {
            // Handle OOM
        }
    }
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
