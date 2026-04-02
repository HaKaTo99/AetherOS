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

    fn get_entropy(&self) -> u64 {
        // [SIMULATION NOISE]
        42
    }

    fn put_char(&self, _c: u8) {
        // Mock output
    }

    fn get_char(&self) -> u8 {
        // Mock input
        0
    }

    fn has_data(&self) -> bool {
        false
    }

    fn clear(&self) {}
}

// Marker for Sync since it's a stateless stub
unsafe impl Sync for StubPlatform {}
