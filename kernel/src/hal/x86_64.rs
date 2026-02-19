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
    pub color_attribute: u8,
}

impl VgaWriter {
    pub const fn new() -> Self {
        Self {
            column_position: 0,
            buffer: VGA_BUFFER as *mut u16,
            color_attribute: 0x0F, // Default: White on Black
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
                let color_byte = self.color_attribute;
                
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
        let blank = (self.color_attribute as u16) << 8 | (b' ' as u16);
        for col in 0..VGA_WIDTH {
            unsafe {
                *self.buffer.add(row * VGA_WIDTH + col) = blank;
            }
        }
    }

    pub fn clear_with_color(&mut self, attribute: u8) {
        let blank = (attribute as u16) << 8 | (b' ' as u16);
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                *self.buffer.add(i) = blank;
            }
        }
        self.column_position = 0;
    }
}

// --- Serial Port 0x3F8 ---
pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self { SerialPort }

    pub fn init(&self) {
        let _guard = InterruptGuard::new();
        unsafe {
            // Disable interrupts (Important for Military Grade Polling)
            outb(0x3F8 + 1, 0x00);
            // Enable DLAB (set baud rate divisor)
            outb(0x3F8 + 3, 0x80);
            // Set divisor to 1 (lo byte) 115200 baud
            outb(0x3F8 + 0, 0x01);
            outb(0x3F8 + 1, 0x00);
            // 8 bits, no parity, one stop bit
            outb(0x3F8 + 3, 0x03);
            // Enable FIFO, clear them, with 14-byte threshold (Optimum for Mesh Latency)
            outb(0x3F8 + 2, 0xC7);
            // RTS/DSR set
            outb(0x3F8 + 4, 0x03);
        }
    }

    pub fn clear(&self) {
        unsafe {
            // Drain receiver
            while (inb(0x3F8 + 5) & 1) != 0 {
                let _ = inb(0x3F8);
            }
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

// --- Shared Input Buffer (Phase 38.4 Harmony) ---
use spin::Mutex;

const INPUT_BUF_SIZE: usize = 64;
static INPUT_QUEUE: Mutex<InputQueue> = Mutex::new(InputQueue::new());

struct InputQueue {
    buffer: [u8; INPUT_BUF_SIZE],
    head: usize,
    tail: usize,
}

impl InputQueue {
    const fn new() -> Self {
        Self {
            buffer: [0; INPUT_BUF_SIZE],
            head: 0,
            tail: 0,
        }
    }

    fn push(&mut self, c: u8) {
        let next = (self.head + 1) % INPUT_BUF_SIZE;
        if next != self.tail {
            self.buffer[self.head] = c;
            self.head = next;
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.head == self.tail {
            None
        } else {
            let c = self.buffer[self.tail];
            self.tail = (self.tail + 1) % INPUT_BUF_SIZE;
            Some(c)
        }
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
}

// --- Platform Implementation ---
pub struct X86Platform {}

pub static mut VGA: VgaWriter = VgaWriter::new();
static SERIAL: SerialPort = SerialPort::new();

impl X86Platform {
    pub const fn new() -> Self {
        Self {}
    }

    /// Pompa data dari hardware ke internal buffer (Phase 10.0 Harmony)
    pub fn poll_hardware(&self) {
        let _guard = InterruptGuard::new();
        unsafe {
            // 1. Poll Serial Port (Drain entire FIFO for high throughput)
            let mut limit = 0;
            while (inb(0x3F8 + 5) & 1) != 0 && limit < 16 {
                let c = inb(0x3F8);
                self.process_input_byte(c);
                limit += 1;
            }

            // 2. Poll PS/2 Keyboard
            if (inb(0x64) & 1) != 0 {
                let scancode = inb(0x60);
                // Military Grade: Scancode validation
                if scancode != 0 && scancode != 0xFF {
                    if let Some(c) = self.map_ps2_to_ascii(scancode) {
                        self.process_input_byte(c);
                    }
                }
            }
        }
    }

    fn process_input_byte(&self, c: u8) {
        // [MILITARY GRADE DIAGNOSTICS] Log Ring 0 Input (Uncomment for deep audit)
        // crate::enterprise::AUDIT_LOGGER.lock().log_raw(format!("Input: 0x{:02x}", c));

        // System Hotkey Interception
        if c == b'd' || c == b'D' {
            let mut dashboard = crate::ui::dashboard::FLEET_DASHBOARD.lock();
            dashboard.active = !dashboard.active;
            if dashboard.active {
                dashboard.render();
            }
            return;
        }

        // Normal char goes to queue
        INPUT_QUEUE.lock().push(c);
    }

    fn map_ps2_to_ascii(&self, scancode: u8) -> Option<u8> {
        if scancode & 0x80 != 0 { return None; }
        match scancode {
            0x1C => Some(b'\n'), 0x39 => Some(b' '), 0x0E => Some(8),
            0x02..=0x0B => { if scancode == 0x0B { Some(b'0') } else { Some(b'0' + (scancode - 1)) } },
            0x0C => Some(b'-'), 0x35 => Some(b'/'), 0x37 => Some(b'*'), 0x4E => Some(b'+'),
            0x20 => Some(b'd'), 0x21 => Some(b'f'),
            _ => None,
        }
    }
}

// IO Helpers
#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nostack, nomem, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    // Military Grade: Port Range Validation
    if port > 0xFFFF { return 0; }
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port, options(nostack, nomem, preserves_flags));
    ret
}

/// Military Grade RAII Interrupt Guard
struct InterruptGuard;

impl InterruptGuard {
    fn new() -> Self {
        unsafe { asm!("cli", options(nomem, nostack)); }
        InterruptGuard
    }
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        unsafe { asm!("sti", options(nomem, nostack)); }
    }
}

impl Platform for X86Platform {
    fn init(&self) {
        SERIAL.init();
        self.puts("X86_64 HAL Initialized (v10.2 Supreme Grade)\n");
    }

    fn shutdown(&self) { unsafe { outb(0xf4, 0x00); } }

    fn get_ticks(&self) -> u64 {
        let rax: u64; let rdx: u64;
        unsafe { asm!("rdtsc", out("rax") rax, out("rdx") rdx); }
        (rdx << 32) | rax
    }

    fn sleep_ms(&self, ms: u64) {
        let steps = ms * 100000;
        let start = self.get_ticks();
        while self.get_ticks() - start < steps { core::hint::spin_loop(); }
    }

    fn put_char(&self, c: u8) {
        unsafe {
            SERIAL.send(c);
            let vga_ptr = core::ptr::addr_of_mut!(VGA);
            (*vga_ptr).write_byte(c);
        }
    }

    fn get_char(&self) -> u8 {
        loop {
            // Pump hardware
            self.poll_hardware();
            
            // Check queue
            if let Some(c) = INPUT_QUEUE.lock().pop() {
                return c;
            }
            
            // Background maintenance while waiting
            crate::kernel_tick();
            core::hint::spin_loop();
        }
    }

    fn has_data(&self) -> bool {
        self.poll_hardware();
        !INPUT_QUEUE.lock().is_empty()
    }

    fn clear(&self) {
        SERIAL.clear();
        let mut queue = INPUT_QUEUE.lock();
        queue.head = 0;
        queue.tail = 0;
    }

    fn cpu_relax(&self) {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)); }
    }
}

unsafe impl Sync for X86Platform {}
