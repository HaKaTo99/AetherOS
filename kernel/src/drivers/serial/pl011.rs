use crate::drivers::{Driver, DriverType};
use core::ptr::{read_volatile, write_volatile};

// Register offsets
const DR: usize = 0x00;     // Data Register
const FR: usize = 0x18;     // Flag Register
const IBRD: usize = 0x24;   // Integer Baud Rate Divisor
const FBRD: usize = 0x28;   // Fractional Baud Rate Divisor
const LCRH: usize = 0x2C;   // Line Control Register
const CR: usize = 0x30;     // Control Register

// Flag Register bits
const FR_TXFF: u32 = 1 << 5; // Transmit FIFO full
const FR_RXFE: u32 = 1 << 4; // Receive FIFO empty

// Control Register bits
const CR_UARTEN: u32 = 1 << 0;  // UART enable
const CR_TXE: u32 = 1 << 8;     // Transmit enable
const CR_RXE: u32 = 1 << 9;     // Receive enable

// Line Control Register bits
const LCRH_WLEN_8BIT: u32 = 0b11 << 5; // 8-bit word length
const LCRH_FEN: u32 = 1 << 4;          // Enable FIFOs

pub struct Pl011Uart {
    base_addr: usize,
    clock_rate: u32,
    baud_rate: u32,
}

impl Pl011Uart {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            clock_rate: 48_000_000, // Default RPi4 UART clock
            baud_rate: 115200,      // Default baud rate
        }
    }

    pub fn set_baud(&mut self, baud: u32) {
        self.baud_rate = baud;
    }

    // --- Helper methods for register access ---
    #[inline]
    unsafe fn read_reg(&self, offset: usize) -> u32 {
        read_volatile((self.base_addr + offset) as *const u32)
    }

    #[inline]
    unsafe fn write_reg(&self, offset: usize, value: u32) {
        write_volatile((self.base_addr + offset) as *mut u32, value);
    }
}

impl Driver for Pl011Uart {
    fn compatible(&self) -> &str {
        "arm,pl011"
    }

    unsafe fn init(&mut self) -> Result<(), &'static str> {
        // 1. Disable UART
        self.write_reg(CR, 0);

        // 2. Wait for current transmission to finish
        while (self.read_reg(FR) & FR_TXFF) != 0 {}

        // 3. Flush FIFOs by disabling them
        self.write_reg(LCRH, 0);

        // 4. Set baud rate
        let divisor = (self.clock_rate * 4) / self.baud_rate; // Fixed point with 6 fractional bits
        let ibrd = divisor >> 6;
        let fbrd = divisor & 0x3F;

        self.write_reg(IBRD, ibrd);
        self.write_reg(FBRD, fbrd);

        // 5. Set line control: 8N1, enable FIFOs
        self.write_reg(LCRH, LCRH_WLEN_8BIT | LCRH_FEN);

        // 6. Enable UART, TX, and RX
        self.write_reg(CR, CR_UARTEN | CR_TXE | CR_RXE);

        Ok(())
    }

    fn device_type(&self) -> DriverType {
        DriverType::Serial
    }
}

// Implement fmt::Write for convenient printing
impl core::fmt::Write for Pl011Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            unsafe {
                // Wait until TX FIFO is not full
                while (self.read_reg(FR) & FR_TXFF) != 0 {}
                // Write character
                if byte == b'\n' {
                     self.write_reg(DR, b'\r' as u32);
                     while (self.read_reg(FR) & FR_TXFF) != 0 {}
                }
                self.write_reg(DR, byte as u32);
            }
        }
        Ok(())
    }
}
