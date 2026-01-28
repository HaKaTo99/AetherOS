//! GPIO Driver for BCM2711 (Raspberry Pi 4)
//! Base address: 0xFE200000

use core::ptr::{read_volatile, write_volatile};

const GPIO_BASE: usize = 0xFE200000;

// Register offsets
const GPFSEL0: usize = 0x00;  // Function Select 0
const GPFSEL1: usize = 0x04;  // Function Select 1
const GPSET0: usize = 0x1C;   // Pin Output Set 0
const GPCLR0: usize = 0x28;   // Pin Output Clear 0
const GPLEV0: usize = 0x34;   // Pin Level 0
const GPPUD: usize = 0x94;    // Pull-up/down Enable
const GPPUDCLK0: usize = 0x98; // Pull-up/down Clock 0

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GpioFunction {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GpioPull {
    None = 0,
    Down = 1,
    Up = 2,
}

pub struct Gpio {
    base: usize,
}

impl Gpio {
    pub const fn new() -> Self {
        Self { base: GPIO_BASE }
    }

    /// Set GPIO pin function
    pub fn set_function(&self, pin: u8, func: GpioFunction) {
        if pin >= 54 {
            return; // Invalid pin
        }

        let reg_index = (pin / 10) as usize;
        let bit_offset = (pin % 10) * 3;

        unsafe {
            let reg_addr = self.base + GPFSEL0 + (reg_index * 4);
            let mut val = read_volatile(reg_addr as *const u32);
            
            // Clear the 3 bits for this pin
            val &= !(0b111 << bit_offset);
            
            // Set new function
            val |= (func as u32) << bit_offset;
            
            write_volatile(reg_addr as *mut u32, val);
        }
    }

    /// Set GPIO pin pull-up/down
    pub fn set_pull(&self, pin: u8, pull: GpioPull) {
        if pin >= 54 {
            return;
        }

        unsafe {
            // 1. Write to GPPUD to set required control signal
            write_volatile((self.base + GPPUD) as *mut u32, pull as u32);

            // 2. Wait 150 cycles (setup time)
            for _ in 0..150 {
                core::hint::spin_loop();
            }

            // 3. Write to GPPUDCLK0 to clock the control signal
            let clock_reg = if pin < 32 { GPPUDCLK0 } else { GPPUDCLK0 + 4 };
            let bit = pin % 32;
            write_volatile((self.base + clock_reg) as *mut u32, 1 << bit);

            // 4. Wait 150 cycles (hold time)
            for _ in 0..150 {
                core::hint::spin_loop();
            }

            // 5. Remove control signal
            write_volatile((self.base + GPPUD) as *mut u32, 0);

            // 6. Remove clock
            write_volatile((self.base + clock_reg) as *mut u32, 0);
        }
    }

    /// Set GPIO pin high
    pub fn set(&self, pin: u8) {
        if pin >= 54 {
            return;
        }

        let reg = if pin < 32 { GPSET0 } else { GPSET0 + 4 };
        let bit = pin % 32;

        unsafe {
            write_volatile((self.base + reg) as *mut u32, 1 << bit);
        }
    }

    /// Set GPIO pin low
    pub fn clear(&self, pin: u8) {
        if pin >= 54 {
            return;
        }

        let reg = if pin < 32 { GPCLR0 } else { GPCLR0 + 4 };
        let bit = pin % 32;

        unsafe {
            write_volatile((self.base + reg) as *mut u32, 1 << bit);
        }
    }

    /// Read GPIO pin level
    pub fn level(&self, pin: u8) -> bool {
        if pin >= 54 {
            return false;
        }

        let reg = if pin < 32 { GPLEV0 } else { GPLEV0 + 4 };
        let bit = pin % 32;

        unsafe {
            let val = read_volatile((self.base + reg) as *const u32);
            (val & (1 << bit)) != 0
        }
    }
}
