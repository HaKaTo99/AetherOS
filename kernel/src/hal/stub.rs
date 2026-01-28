//! QEMU Stub Implementation

use super::Platform;

pub struct StubPlatform;

impl Platform for StubPlatform {
    fn init(&self) {
        // QEMU specific initialization
    }

    fn shutdown(&self) {
        // QEMU shutdown command (if needed)
    }

    fn get_ticks(&self) -> u64 {
        // In stub, we just increment a counter or read a register
        // For no_std, we might need assembly or just return 0 for now
        0 
    }

    fn sleep_ms(&self, _ms: u64) {
        // Busy wait
    }

    fn put_char(&self, _c: u8) {
        // Write to UART0 for QEMU
        // Safety: In real impl this writes to 0x09000000
        unsafe {
            // Simulated MMIO for example purposes
            // core::ptr::write_volatile(0x09000000 as *mut u8, c);
        }
    }
}

// Marker for Sync since it's a stateless stub
unsafe impl Sync for StubPlatform {}
