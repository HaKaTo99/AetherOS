//! Quantum HAL (Q-HAL) - hardware abstraction layer for AetherOS kernel
#![no_std]

pub mod stub;

/// Power performance modes (0..=255)
pub type PerfMode = u8;

/// Minimal HAL traits. Implementations live in BSPs or platform crates.
pub trait PowerController {
    /// Set performance/power mode (0 = low power, 255 = max perf)
    fn set_performance_mode(&self, mode: PerfMode);
    /// Get battery level percentage (0-100)
    fn battery_level(&self) -> u8;
}

pub trait CryptoEngine {
    /// Encrypt data in place. Returns true on success.
    fn encrypt_in_place(&self, buf: &mut [u8]) -> bool;
    /// Decrypt data in place. Returns true on success.
    fn decrypt_in_place(&self, buf: &mut [u8]) -> bool;
}

pub trait DeviceManager {
    /// Probe devices and initialize drivers. Returns number of devices found.
    fn probe(&self) -> usize;
}

/// Provide a default stub implementation (safe no-op) for builds that don't
/// have platform-specific HAL yet. BSPs should replace this with real code.
pub use stub::StubHal;
