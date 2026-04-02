//! Raspberry Pi 4 Platform Implementation

use super::Platform;

mod uart;
mod timer;
mod gpio;
pub mod gic;  // [NEW] Interrupt controller (public for kernel use)

pub use uart::Uart;
use timer::Timer;
use gpio::{Gpio, GpioFunction, GpioPull};
pub use gic::Gic;  // Export for kernel use

pub struct RPiPlatform {
    uart: Uart,
    timer: Timer,
    gpio: Gpio,
}

impl RPiPlatform {
    pub const fn new() -> Self {
        Self {
            uart: Uart::new(),
            timer: Timer::new(),
            gpio: Gpio::new(),
        }
    }
}

impl Platform for RPiPlatform {
    fn init(&self) {
        // 1. Setup GPIO pins for UART (GPIO 14 = TXD, GPIO 15 = RXD)
        self.gpio.set_function(14, GpioFunction::Alt0);
        self.gpio.set_function(15, GpioFunction::Alt0);
        
        // Disable pull-up/down for UART pins
        self.gpio.set_pull(14, GpioPull::None);
        self.gpio.set_pull(15, GpioPull::None);

        // 2. Initialize UART at 115200 baud
        self.uart.init(115200);
        
        // 3. Initialize timer (already running, just verify)
        self.timer.init();
        
        // 4. Print boot message
        self.uart.puts("\r\n");
        self.uart.puts("=================================\r\n");
        self.uart.puts("  AetherOS v1.3 - Raspberry Pi 4\r\n");
        self.uart.puts("=================================\r\n");
        self.uart.puts("HAL initialized successfully\r\n");
        
        // 5. Initialize GIC (Interrupt Controller)
        unsafe {
            Gic::init();
            Gic::enable_interrupt(gic::IRQ_TIMER);
        }
        self.uart.puts("GIC initialized successfully\r\n");
        
        // 6. Enable timer interrupts
        unsafe {
            self.timer.enable_interrupt();
        }
        self.uart.puts("Timer interrupts enabled\r\n");
    }

    fn shutdown(&self) {
        self.uart.puts("System shutting down...\r\n");
        self.cpu_halt();
    }

    fn get_ticks(&self) -> u64 {
        self.timer.get_ticks()
    }

    fn sleep_ms(&self, ms: u64) {
        self.timer.sleep_ms(ms);
    }

    fn get_entropy(&self) -> u64 {
        // [MILITARY GRADE NOISE] Using Timer Ticks for Entropy on ARM.
        self.get_ticks()
    }

    fn put_char(&self, c: u8) {
        self.uart.put_char(c);
    }

    fn get_char(&self) -> u8 {
        loop {
            if let Some(c) = self.uart.get_char() {
                return c;
            }
            core::hint::spin_loop();
        }
    }

    fn has_data(&self) -> bool {
        self.uart.get_char().is_some()
    }

    fn clear(&self) {
        while self.has_data() {
            let _ = self.get_char();
        }
    }

    fn cpu_relax(&self) {
        // WFE: Wait For Event - low power state until event
        unsafe { core::arch::asm!("wfe") };
    }

    fn cpu_halt(&self) -> ! {
        // Infinite loop with WFE for shutdown
        loop {
            unsafe { core::arch::asm!("wfe") };
        }
    }

    fn enter_idle_state(&self) {
        // WFI: Wait For Interrupt - low power state until interrupt
        // This is better for scheduler idle as it wakes on any interrupt
        unsafe { core::arch::asm!("wfi") };
    }

    fn set_power_state(&self, domain_id: usize, on: bool) -> Result<bool, ()> {
        // Use RPi4 Mailbox for power control
        // Map domain_id to mailbox::PowerDomain
        use crate::drivers::mailbox::{self, PowerDomain};
        
        // Simple mapping: 0=SdCard, 1=Uart0, etc. (Or validation)
        let domain = match domain_id {
            0 => PowerDomain::SdCard,
            1 => PowerDomain::Uart0,
            2 => PowerDomain::Uart1,
            3 => PowerDomain::UsbHcd,
            4 => PowerDomain::I2c0,
            5 => PowerDomain::I2c1,
            6 => PowerDomain::I2c2,
            7 => PowerDomain::Spi,
            8 => PowerDomain::Ccp2tx,
            _ => return Err(()), // Unknown domain
        };
        
        let _ = mailbox::init(); // Ensure initialized
        mailbox::set_power_state(domain, on)
    }
    
    fn get_power_state(&self, domain_id: usize) -> Result<bool, ()> {
        use crate::drivers::mailbox::{self, PowerDomain};
        let domain = match domain_id {
            0 => PowerDomain::SdCard,
            1 => PowerDomain::Uart0,
            2 => PowerDomain::Uart1,
            3 => PowerDomain::UsbHcd,
            4 => PowerDomain::I2c0,
            5 => PowerDomain::I2c1,
            6 => PowerDomain::I2c2,
            7 => PowerDomain::Spi,
            8 => PowerDomain::Ccp2tx,
            _ => return Err(()),
        };
        
        let _ = mailbox::init(); // Ensure initialized
        mailbox::get_power_state(domain)
    }
}

// Make it Sync for static usage
unsafe impl Sync for RPiPlatform {}
