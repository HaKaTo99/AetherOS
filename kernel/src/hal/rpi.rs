//! Raspberry Pi 4 Platform Implementation

use super::Platform;

pub struct RPiPlatform;

impl Platform for RPiPlatform {
    fn init(&self) {
        // RPi GPIO & UART init
    }

    fn shutdown(&self) {
        // Watchdog reset
    }

    fn get_ticks(&self) -> u64 {
        // Read system timer
        0
    }

    fn sleep_ms(&self, _ms: u64) {
        // Timer delay
    }

    fn put_char(&self, _c: u8) {
        // Write to UART1
    }
}

unsafe impl Sync for RPiPlatform {}
