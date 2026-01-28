//! ARM Generic Timer Driver
//! Uses system registers for time measurement

use core::arch::asm;

pub struct Timer;

impl Timer {
    pub const fn new() -> Self {
        Self
    }

    /// Initialize timer
    pub fn init(&self) {
        // Timer is always running on ARM, just ensure it's accessible
        // In real implementation, we'd setup timer interrupts here
    }

    /// Get current tick count (microseconds)
    pub fn get_ticks(&self) -> u64 {
        let freq = self.get_frequency();
        let count = self.get_counter();
        
        // Convert to microseconds
        (count * 1_000_000) / freq
    }

    /// Get raw counter value
    pub fn get_counter(&self) -> u64 {
        let count: u64;
        unsafe {
            asm!("mrs {}, cntpct_el0", out(reg) count);
        }
        count
    }

    /// Get timer frequency (Hz)
    pub fn get_frequency(&self) -> u64 {
        let freq: u64;
        unsafe {
            asm!("mrs {}, cntfrq_el0", out(reg) freq);
        }
        freq
    }

    /// Busy-wait sleep for given milliseconds
    pub fn sleep_ms(&self, ms: u64) {
        let freq = self.get_frequency();
        let start = self.get_counter();
        let ticks = (freq * ms) / 1000;
        
        while (self.get_counter() - start) < ticks {
            // Busy wait
            core::hint::spin_loop();
        }
    }

    /// Busy-wait sleep for given microseconds
    pub fn sleep_us(&self, us: u64) {
        let freq = self.get_frequency();
        let start = self.get_counter();
        let ticks = (freq * us) / 1_000_000;
        
        while (self.get_counter() - start) < ticks {
            core::hint::spin_loop();
        }
    }

    /// Enable timer interrupt (1ms tick)
    pub unsafe fn enable_interrupt(&self) {
        let freq = self.get_frequency();
        let ticks_per_ms = freq / 1000;

        // Set timer value (countdown)
        asm!("msr cntp_tval_el0, {}", in(reg) ticks_per_ms);

        // Enable timer and unmask interrupt
        // Bit 0: Enable, Bit 1: Mask (0 = not masked)
        asm!("msr cntp_ctl_el0, {}", in(reg) 1u64);
    }

    /// Acknowledge timer interrupt
    pub unsafe fn acknowledge_interrupt(&self) {
        // Reload timer for next tick
        let freq = self.get_frequency();
        let ticks_per_ms = freq / 1000;
        asm!("msr cntp_tval_el0, {}", in(reg) ticks_per_ms);
    }
}
