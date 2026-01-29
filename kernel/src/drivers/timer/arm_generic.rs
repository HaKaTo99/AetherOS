use crate::drivers::{Driver, DriverType};
use core::arch::asm;

pub struct ArmGenericTimer {
    interval_ms: u64,
}

impl ArmGenericTimer {
    pub const fn new() -> Self {
        Self { interval_ms: 10 } // Default 10ms tick
    }

    pub fn set_interval(&mut self, ms: u64) {
        self.interval_ms = ms;
    }

    /// Get timer frequency (Hz)
    pub fn get_frequency(&self) -> u64 {
        let freq: u64;
        unsafe {
            asm!("mrs {}, cntfrq_el0", out(reg) freq);
        }
        freq
    }

    /// Get current counter value
    pub fn get_counter(&self) -> u64 {
        let count: u64;
        unsafe {
            asm!("mrs {}, cntpct_el0", out(reg) count);
        }
        count
    }

    /// Get current ticks (us)
    pub fn get_ticks_us(&self) -> u64 {
        let freq = self.get_frequency();
        let count = self.get_counter();
        (count * 1_000_000) / freq
    }

    /// Reload timer for next tick
    pub unsafe fn reload(&self) {
        let freq = self.get_frequency();
        let ticks = (freq * self.interval_ms) / 1000;
        asm!("msr cntp_tval_el0, {}", in(reg) ticks);
    }
}

impl Driver for ArmGenericTimer {
    fn compatible(&self) -> &str {
        "arm,armv8-timer"
    }

    unsafe fn init(&mut self) -> Result<(), &'static str> {
        // Enable timer interrupt (10ms tick by default)
        let freq = self.get_frequency();
        let ticks = (freq * self.interval_ms) / 1000;

        // Set timer value (countdown)
        asm!("msr cntp_tval_el0, {}", in(reg) ticks);

        // Enable timer and unmask interrupt
        // Bit 0: Enable, Bit 1: Mask (0 = not masked)
        asm!("msr cntp_ctl_el0, {}", in(reg) 1u64);

        Ok(())
    }

    fn device_type(&self) -> DriverType {
        DriverType::Timer
    }

    fn handle_irq(&mut self, _irq_num: u32) {
        // Ack/Reload timer
        unsafe { self.reload() };
    }
}
