//! x86_64 Platform Implementation (QEMU/PC)

use super::Platform;

use core::arch::asm;

// --- VGA Buffer ---
const VGA_BUFFER: usize = 0xb8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub struct VgaWriter {
    column_position: usize,
    buffer: *mut u16,
}

impl VgaWriter {
    pub const fn new() -> Self {
        Self {
            column_position: 0,
            buffer: VGA_BUFFER as *mut u16,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_WIDTH {
                    self.new_line();
                }

                let row = VGA_HEIGHT - 1;
                let col = self.column_position;
                let color_byte = 0x0F; // White on Black
                
                unsafe {
                    *self.buffer.add(row * VGA_WIDTH + col) = 
                        (color_byte as u16) << 8 | (byte as u16);
                }
                
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                unsafe {
                    let character = *self.buffer.add(row * VGA_WIDTH + col);
                    *self.buffer.add((row - 1) * VGA_WIDTH + col) = character;
                }
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = 0x0F00u16 | (b' ' as u16);
        for col in 0..VGA_WIDTH {
            unsafe {
                *self.buffer.add(row * VGA_WIDTH + col) = blank;
            }
        }
    }
}

// --- Serial Port 0x3F8 ---
pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self { SerialPort }

    pub fn init(&self) {
        unsafe {
            // Disable interrupts
            outb(0x3F8 + 1, 0x00);
            // Enable DLAB (set baud rate divisor)
            outb(0x3F8 + 3, 0x80);
            // Set divisor to 3 (lo byte) 38400 baud
            outb(0x3F8 + 0, 0x03);
            outb(0x3F8 + 1, 0x00);
            // 8 bits, no parity, one stop bit
            outb(0x3F8 + 3, 0x03);
            // Enable FIFO, clear them, with 14-byte threshold
            outb(0x3F8 + 2, 0xC7);
            // IRQs enabled, RTS/DSR set
            outb(0x3F8 + 4, 0x0B);
        }
    }

    pub fn send(&self, byte: u8) {
        unsafe {
            // Wait for transmit empty
            while (inb(0x3F8 + 5) & 0x20) == 0 {}
            outb(0x3F8, byte);
        }
    }
}

// --- Platform Implementation ---
pub struct X86Platform {
    // We use UnsafeCell/Mutex in real code, but for single-threaded init this is "ok" 
    // for MVP. To make it Send/Sync for static, we assume single core or appropriate lock.
    // Making it zero-sized for now and creating instances on the fly or using static mut 
    // internally would be cleaner, but let's stick to the pattern.
}

static mut VGA: VgaWriter = VgaWriter::new();
static SERIAL: SerialPort = SerialPort::new();

impl X86Platform {
    pub const fn new() -> Self {
        Self {}
    }
}

// IO Helpers
#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nostack, nomem, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port, options(nostack, nomem, preserves_flags));
    ret
}

impl Platform for X86Platform {
    fn init(&self) {
        SERIAL.init();
        self.puts("X86_64 HAL Initialized\n");
    }

    fn shutdown(&self) {
        // QEMU shutdown hack (older style)
        unsafe {
             outb(0xf4, 0x00);
        }
    }

    fn get_ticks(&self) -> u64 {
        let rax: u64;
        let rdx: u64;
        unsafe {
            asm!("rdtsc", out("rax") rax, out("rdx") rdx);
        }
        (rdx << 32) | rax
    }

    fn sleep_ms(&self, ms: u64) {
        // Very rough busy loop approximation
        // 1ms ~ 1000000 cycles on modern CPU?
        let steps = ms * 1000000;
        let start = self.get_ticks();
        while self.get_ticks() - start < steps {
            core::hint::spin_loop();
        }
    }

    fn put_char(&self, c: u8) {
        unsafe {
            SERIAL.send(c);
            VGA.write_byte(c);
        }
    }
}

unsafe impl Sync for X86Platform {}
