//! x86_64 Platform Implementation (QEMU/PC)

use super::Platform;
use crate::drivers::input::{InputEvent, KeyCode, KeyState};

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
            b'\r' => {
                self.column_position = 0;
            }
            8 => {
                if self.column_position > 0 {
                    self.column_position -= 1;
                    let row = VGA_HEIGHT - 1;
                    let col = self.column_position;
                    let color_byte = self.color_attribute;
                    unsafe {
                        *self.buffer.add(row * VGA_WIDTH + col) =
                            (color_byte as u16) << 8 | (b' ' as u16);
                    }
                }
            }
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
        self.update_hardware_cursor();
    }

    fn update_hardware_cursor(&self) {
        let row = VGA_HEIGHT - 1;
        let pos = (row * VGA_WIDTH + self.column_position) as u16;
        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
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
        self.update_hardware_cursor();
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

    pub fn has_data(&self) -> bool {
        unsafe { (inb(0x3F8 + 5) & 1) != 0 }
    }

    pub fn receive(&self) -> u8 {
        unsafe { inb(0x3F8) }
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

    fn init_ps2_keyboard_minimal(&self) {
        // [SUPREME COMPATIBILITY] Use the hardened driver init sequence
        unsafe {
            crate::drivers::input::ps2::KEYBOARD.init();
        }
    }

    /// Pompa data dari hardware ke internal buffer (Phase 10.0 Harmony)
    pub fn poll_hardware(&self) {
        // Path A: official PS/2 driver events (set-1)
        unsafe {
            let mut event_limit = 0;
            while event_limit < 32 {
                match crate::drivers::input::ps2::KEYBOARD.poll() {
                    Some(InputEvent::Keyboard { key, state }) if state == KeyState::Pressed => {
                        if let Some(c) = self.map_keycode_to_ascii(key) {
                            self.process_input_byte(c);
                        }
                        event_limit += 1;
                    }
                    Some(InputEvent::Raw(c)) => {
                        self.process_input_byte(c);
                        event_limit += 1;
                    }
                    Some(_) => {
                        event_limit += 1;
                    }
                    None => break,
                }
            }
        }

        // Path B: Serial COM1 (for QEMU stdio / terminal interaction)
        while SERIAL.has_data() {
            let c = SERIAL.receive();
            self.process_input_byte(c);
        }
    }

    fn map_keycode_to_ascii(&self, key: KeyCode) -> Option<u8> {
        match key {
            KeyCode::Enter => Some(b'\n'),
            KeyCode::Space => Some(b' '),
            KeyCode::Backspace => Some(8),
            KeyCode::Num0 => Some(b'0'), KeyCode::Num1 => Some(b'1'), KeyCode::Num2 => Some(b'2'),
            KeyCode::Num3 => Some(b'3'), KeyCode::Num4 => Some(b'4'), KeyCode::Num5 => Some(b'5'),
            KeyCode::Num6 => Some(b'6'), KeyCode::Num7 => Some(b'7'), KeyCode::Num8 => Some(b'8'),
            KeyCode::Num9 => Some(b'9'),
            KeyCode::A => Some(b'a'), KeyCode::B => Some(b'b'), KeyCode::C => Some(b'c'),
            KeyCode::D => Some(b'd'), KeyCode::E => Some(b'e'), KeyCode::F => Some(b'f'),
            KeyCode::G => Some(b'g'), KeyCode::H => Some(b'h'), KeyCode::I => Some(b'i'),
            KeyCode::J => Some(b'j'), KeyCode::K => Some(b'k'), KeyCode::L => Some(b'l'),
            KeyCode::M => Some(b'm'), KeyCode::N => Some(b'n'), KeyCode::O => Some(b'o'),
            KeyCode::P => Some(b'p'), KeyCode::Q => Some(b'q'), KeyCode::R => Some(b'r'),
            KeyCode::S => Some(b's'), KeyCode::T => Some(b't'), KeyCode::U => Some(b'u'),
            KeyCode::V => Some(b'v'), KeyCode::W => Some(b'w'), KeyCode::X => Some(b'x'),
            KeyCode::Y => Some(b'y'), KeyCode::Z => Some(b'z'),
            KeyCode::Minus => Some(b'-'), KeyCode::Equal => Some(b'='),
            KeyCode::LBracket => Some(b'['), KeyCode::RBracket => Some(b']'),
            KeyCode::Backslash => Some(b'\\'), KeyCode::Semicolon => Some(b';'),
            KeyCode::Quote => Some(b'\''), KeyCode::Comma => Some(b','),
            KeyCode::Period => Some(b'.'), KeyCode::Slash => Some(b'/'),
            _ => None,
        }
    }

    fn process_input_byte(&self, c: u8) {
        // [MILITARY GRADE DIAGNOSTICS] Log Ring 0 Input (Uncomment for deep audit)
        // crate::enterprise::AUDIT_LOGGER.lock().log_raw(format!("Input: 0x{:02x}", c));

        // Normal char goes to queue
        INPUT_QUEUE.lock().push(c);
    }

    #[allow(dead_code)]
    fn map_ps2_to_ascii(&self, scancode: u8) -> Option<u8> {
        // Deterministic mode: default to Set-1 make codes only.
        if scancode == 0xE0 || scancode == 0xF0 || scancode & 0x80 != 0 {
            return None;
        }

        // Full US QWERTY Set 1 scancode to ASCII (unshifted)
        match scancode {
            0x1C => Some(b'\n'), 0x39 => Some(b' '), 0x0E => Some(8),
            0x02 => Some(b'1'), 0x03 => Some(b'2'), 0x04 => Some(b'3'), 0x05 => Some(b'4'),
            0x06 => Some(b'5'), 0x07 => Some(b'6'), 0x08 => Some(b'7'), 0x09 => Some(b'8'),
            0x0A => Some(b'9'), 0x0B => Some(b'0'),
            0x0C => Some(b'-'), 0x0D => Some(b'='),
            0x10 => Some(b'q'), 0x11 => Some(b'w'), 0x12 => Some(b'e'), 0x13 => Some(b'r'),
            0x14 => Some(b't'), 0x15 => Some(b'y'), 0x16 => Some(b'u'), 0x17 => Some(b'i'),
            0x18 => Some(b'o'), 0x19 => Some(b'p'),
            0x1A => Some(b'['), 0x1B => Some(b']'),
            0x1E => Some(b'a'), 0x1F => Some(b's'), 0x20 => Some(b'd'), 0x21 => Some(b'f'),
            0x22 => Some(b'g'), 0x23 => Some(b'h'), 0x24 => Some(b'j'), 0x25 => Some(b'k'),
            0x26 => Some(b'l'), 0x27 => Some(b';'), 0x28 => Some(b'\''),
            0x29 => Some(b'`'),
            0x2B => Some(b'\\'),
            0x2C => Some(b'z'), 0x2D => Some(b'x'), 0x2E => Some(b'c'), 0x2F => Some(b'v'),
            0x30 => Some(b'b'), 0x31 => Some(b'n'), 0x32 => Some(b'm'),
            0x33 => Some(b','), 0x34 => Some(b'.'), 0x35 => Some(b'/'),
            // Numpad and fallback for symbols
            0x37 => Some(b'*'), 0x4E => Some(b'+'), 0x4A => Some(b'-'),
            // Extended/fallback for missing symbols (assign to unused scancodes or numpad/fn keys)
            0x56 => Some(b'|'), // <\|> key (ISO)
            0x73 => Some(b'_'), // Fallback for _
            0x7D => Some(b'{'), // Fallback for {
            0x7E => Some(b'}'), // Fallback for }
            0x41 => Some(b','), // Numpad ,
            0x5B => Some(b'~'), // F11 (as ~ fallback)
            0x5C => Some(b'!'), // F12 (as ! fallback)
            0x5D => Some(b'@'), // F13 (as @ fallback)
            0x5E => Some(b'#'), // F14 (as # fallback)
            0x5F => Some(b'$'), // F15 (as $ fallback)
            0x60 => Some(b'%'), // F16 (as % fallback)
            0x61 => Some(b'^'), // F17 (as ^ fallback)
            0x62 => Some(b'&'), // F18 (as & fallback)
            0x63 => Some(b'*'), // F19 (as * fallback)
            0x64 => Some(b'('), // F20 (as ( fallback)
            0x65 => Some(b')'), // F21 (as ) fallback)
            0x66 => Some(b'?'), // F22 (as ? fallback)
            0x67 => Some(b'"'), // F23 (as " fallback)
            0x68 => Some(b':'), // Fallback for :
            0x69 => Some(b';'), // Fallback for ;
            0x6A => Some(b'<'), // Fallback for <
            0x6B => Some(b'>'), // Fallback for >
            0x6C => Some(b'['), // Fallback for [
            0x6D => Some(b']'), // Fallback for ]
            0x6E => Some(b'/'), // Fallback for /
            0x6F => Some(b'\\'), // Fallback for \
            0x70 => Some(b'+'), // Fallback for +
            0x71 => Some(b'='), // Fallback for =
            // Shifted symbols are not handled here (would require shift state tracking)
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
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port, options(nostack, nomem, preserves_flags));
    ret
}

/// Military Grade RAII Interrupt Guard (State Aware)
struct InterruptGuard {
    enabled: bool,
}

impl InterruptGuard {
    fn new() -> Self {
        let rflags: u64;
        unsafe {
            asm!("pushfq; pop {}", out(reg) rflags, options(nomem, preserves_flags));
            asm!("cli", options(nomem, nostack));
        }
        // IF bit is bit 9
        Self { enabled: (rflags & (1 << 9)) != 0 }
    }
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        if self.enabled {
            unsafe { asm!("sti", options(nomem, nostack)); }
        }
    }
}


impl Platform for X86Platform {
    fn init(&self) {
        SERIAL.init();
        // Very-early diagnostic banner to verify serial/VGA output
        self.puts("[EARLY] HAL init: serial/VGA initialized\r\n");
        unsafe { crate::drivers::input::ps2::KEYBOARD.init(); }
        self.init_ps2_keyboard_minimal();
        self.puts("X86_64 HAL Initialized (v10.3 Supreme Grade)\n");
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

    fn get_entropy(&self) -> u64 {
        // [MILITARY GRADE NOISE] Using TSC (Time Stamp Counter) as fallback entropy.
        // In full TPM/HW-RNG mode, this would call RDRAND.
        self.get_ticks()
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
            if let Some(c) = INPUT_QUEUE.lock().pop() {
                return c;
            }

            self.poll_hardware();

            // Keep input path deterministic and low-latency while waiting.
            core::hint::spin_loop();
        }
    }

    fn has_data(&self) -> bool {
        self.poll_hardware();
        !INPUT_QUEUE.lock().is_empty()
    }

    fn clear(&self) {
        unsafe {
            let vga_ptr = core::ptr::addr_of_mut!(VGA);
            (*vga_ptr).clear_with_color(0x0F);
        }
        let mut queue = INPUT_QUEUE.lock();
        queue.head = 0;
        queue.tail = 0;
    }

    fn cpu_relax(&self) {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)); }
    }
}

unsafe impl Sync for X86Platform {}
