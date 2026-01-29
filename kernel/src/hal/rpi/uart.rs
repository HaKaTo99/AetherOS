//! UART PL011 Driver for Raspberry Pi 4
//! Base address: 0xFE201000

use core::ptr::{read_volatile, write_volatile};

const UART_BASE: usize = 0xFE201000;

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

pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new() -> Self {
        Self { base: UART_BASE }
    }

    /// Initialize UART with given baud rate
    pub fn init(&self, baud_rate: u32) {
        unsafe {
            // 1. Disable UART
            self.write_reg(CR, 0);

            // 2. Wait for current transmission to finish
            while (self.read_reg(FR) & FR_TXFF) != 0 {}

            // 3. Flush FIFOs by disabling them
            self.write_reg(LCRH, 0);

            // 4. Set baud rate
            // UART clock = 48 MHz (RPi4)
            // Baud divisor = UART_CLK / (16 * baud_rate)
            // For 115200: divisor = 48000000 / (16 * 115200) = 26.0416
            // IBRD = 26, FBRD = int(0.0416 * 64 + 0.5) = 3
            let divisor = (48_000_000 * 4) / baud_rate; // Fixed point with 6 fractional bits
            let ibrd = divisor >> 6;
            let fbrd = divisor & 0x3F;

            self.write_reg(IBRD, ibrd);
            self.write_reg(FBRD, fbrd);

            // 5. Set line control: 8N1, enable FIFOs
            self.write_reg(LCRH, LCRH_WLEN_8BIT | LCRH_FEN);

            // 6. Enable UART, TX, and RX
            self.write_reg(CR, CR_UARTEN | CR_TXE | CR_RXE);
        }
    }

    /// Write a character to UART
    pub fn put_char(&self, c: u8) {
        unsafe {
            // Wait until TX FIFO is not full
            while (self.read_reg(FR) & FR_TXFF) != 0 {}

            // Write character to data register
            self.write_reg(DR, c as u32);
        }
    }

    /// Read a character from UART (non-blocking)
    pub fn get_char(&self) -> Option<u8> {
        unsafe {
            // Check if RX FIFO is empty
            if (self.read_reg(FR) & FR_RXFE) != 0 {
                None
            } else {
                Some(self.read_reg(DR) as u8)
            }
        }
    }

    /// Write a string to UART
    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            // Convert LF to CRLF for terminal compatibility
            if byte == b'\n' {
                self.put_char(b'\r');
            }
            self.put_char(byte);
        }
    }

    #[inline]
    unsafe fn read_reg(&self, offset: usize) -> u32 {
        read_volatile((self.base + offset) as *const u32)
    }

    #[inline]
    unsafe fn write_reg(&self, offset: usize, value: u32) {
        write_volatile((self.base + offset) as *mut u32, value);
    }
}

// Implement core::fmt::Write for easy formatting
impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.puts(s);
        Ok(())
    }
}
