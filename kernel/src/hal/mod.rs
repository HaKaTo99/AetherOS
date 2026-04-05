//! Hardware Abstraction Layer (HAL) v2.0
//! Dynamic Trait-based abstraction for multi-platform support

pub mod stub;
pub mod neural_v2; // [NEW] Phase 27.7
#[cfg(target_arch = "aarch64")]
pub mod rpi;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;


/// Core Platform Trait - must be implemented by all hardware backends
pub trait Platform: Sync {
    fn init(&self);
    fn shutdown(&self);
    
    // Timer & Entropy support
    fn get_ticks(&self) -> u64;
    fn sleep_ms(&self, ms: u64);
    fn get_entropy(&self) -> u64;
    
    // Serial/Keyboard support
    fn put_char(&self, c: u8);
    fn get_char(&self) -> u8;
    fn has_data(&self) -> bool;
    fn clear(&self);

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

/// Helper for println! macro
pub struct ConsoleWriter;

impl core::fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // [SOVEREIGN TEE] 1. Serial/Physical Console
        get_platform().puts(s);

        // 2. UI/Visual Terminal Buffer
        crate::ui::terminal::log_to_terminal(s);
        
        Ok(())
    }
}

/// Renders an ASCII progress bar to the console.
pub fn print_boot_progress(current: usize, total: usize) {
    use core::fmt::Write;
    let mut writer = ConsoleWriter;
    let percentage = (current * 100) / total;
    let width = 40;
    let filled = (current * width) / total;

    let _ = write!(writer, "\r\x1B[36m[AetherOS Loading] [");
    for _ in 0..filled { let _ = writer.write_str("="); }
    for _ in filled..width { let _ = writer.write_str("-"); }
    let _ = write!(writer, "] {}%\x1B[0m", percentage);
    
    if current == total {
        let _ = writer.write_str("\r\n");
    }
}

pub mod spatial; // [NEW] Phase 28.3 Holographic Mapping
