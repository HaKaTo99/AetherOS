//! Raspberry Pi 4 Platform Implementation

use super::Platform;

mod uart;
mod timer;
mod gpio;

use uart::Uart;
use timer::Timer;
use gpio::{Gpio, GpioFunction, GpioPull};

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
    }

    fn shutdown(&self) {
        self.uart.puts("System shutting down...\r\n");
        // In real implementation: halt CPU
        loop {
            unsafe { core::arch::asm!("wfe") };
        }
    }

    fn get_ticks(&self) -> u64 {
        self.timer.get_ticks()
    }

    fn sleep_ms(&self, ms: u64) {
        self.timer.sleep_ms(ms);
    }

    fn put_char(&self, c: u8) {
        self.uart.put_char(c);
    }
}

// Make it Sync for static usage
unsafe impl Sync for RPiPlatform {}
