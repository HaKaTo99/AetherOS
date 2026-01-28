//! Hardware Abstraction Layer (HAL) v2.0
//! Dynamic Trait-based abstraction for multi-platform support

pub mod stub;
pub mod rpi;

/// Core Platform Trait - must be implemented by all hardware backends
pub trait Platform: Sync {
    fn init(&self);
    fn shutdown(&self);
    
    // Timer support
    fn get_ticks(&self) -> u64;
    fn sleep_ms(&self, ms: u64);
    
    // Serial support (debug output)
    fn put_char(&self, c: u8);
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
