//! Hardware Abstraction Layer (HAL) v2.0
//! Dynamic Trait-based abstraction for multi-platform support

pub mod stub;
#[cfg(target_arch = "aarch64")]
pub mod rpi;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;


/// Core Platform Trait - must be implemented by all hardware backends
pub trait Platform: Sync {
    fn init(&self);
    fn shutdown(&self);
    
    // Timer support
    fn get_ticks(&self) -> u64;
    fn sleep_ms(&self, ms: u64);
    
    // Serial support (debug output)
    fn put_char(&self, c: u8);

    // Default implementation for string output
    fn puts(&self, s: &str) {
        for c in s.bytes() {
            self.put_char(c);
        }
    }

    // Power management
    fn cpu_relax(&self) {
        // Default: busy loop hint
        core::hint::spin_loop();
    }

    /// Halt CPU indefinitely (for shutdown)
    fn cpu_halt(&self) -> ! {
        loop {
            self.cpu_relax();
        }
    }

    /// Enter low-power idle state (can be woken by interrupts)
    /// Uses WFI (Wait For Interrupt) or WFE (Wait For Event)
    fn enter_idle_state(&self) {
        self.cpu_relax();
    }

    /// Set power state for a specific domain (device specific)
    /// Returns Ok(true) if state is ON, Ok(false) if OFF
    fn set_power_state(&self, _domain_id: usize, _on: bool) -> Result<bool, ()> {
        // Default implementation does nothing
        Err(())
    }

    /// Get power state for a specific domain
    fn get_power_state(&self, _domain_id: usize) -> Result<bool, ()> {
        // Default implementation does nothing
        Err(())
    }
}

/// Global platform instance
static mut PLATFORM: Option<&'static dyn Platform> = None;

/// Initialize the global platform
pub fn init_platform(p: &'static dyn Platform) {
    unsafe {
        PLATFORM = Some(p);
        p.init();
    }
}

/// Get access to global platform
pub fn get_platform() -> &'static dyn Platform {
    unsafe {
        PLATFORM.expect("Platform not initialized!")
    }
}

/// Try to get platform (for panic handler)
pub unsafe fn try_get_platform() -> Option<&'static dyn Platform> {
    PLATFORM
}
