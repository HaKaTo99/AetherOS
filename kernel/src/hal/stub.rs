//! Stub HAL implementation — no-op safe defaults for host build and testing
#![no_std]

use super::{CryptoEngine, DeviceManager, PowerController, PerfMode};

pub struct StubHal;

impl StubHal {
    pub const fn new() -> Self {
        StubHal
    }
}

impl PowerController for StubHal {
    fn set_performance_mode(&self, _mode: PerfMode) {
        // no-op stub
    }

    fn battery_level(&self) -> u8 {
        100 // assume full battery in stub
    }
}

impl CryptoEngine for StubHal {
    fn encrypt_in_place(&self, _buf: &mut [u8]) -> bool {
        // no-op: pretend success
        true
    }

    fn decrypt_in_place(&self, _buf: &mut [u8]) -> bool {
        true
    }
}

impl DeviceManager for StubHal {
    fn probe(&self) -> usize {
        0 // no devices in stub
    }
}
