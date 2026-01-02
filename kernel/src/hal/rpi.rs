//! RPi HAL implementation (minimal, uses stub primitives where platform access is unavailable)
#![no_std]

use super::{CryptoEngine, DeviceManager, PowerController, PerfMode};

pub struct RpiHal;

impl RpiHal {
    pub const fn new() -> Self { RpiHal }
}

impl PowerController for RpiHal {
    fn set_performance_mode(&self, mode: PerfMode) {
        // Map perf mode to simple governor settings via platform interface (placeholder)
        let _ = mode;
    }

    fn battery_level(&self) -> u8 { 100 }
}

impl CryptoEngine for RpiHal {
    fn encrypt_in_place(&self, _buf: &mut [u8]) -> bool { true }
    fn decrypt_in_place(&self, _buf: &mut [u8]) -> bool { true }
}

impl DeviceManager for RpiHal {
    fn probe(&self) -> usize { 1 }
}
