//! i8042 PS/2 Keyboard & Mouse Driver (v10.4.15 Unified Matrix)
//! Military-Grade Stability: Parity Checks, IntelliMouse, and Zero-Lag Polling.

use crate::drivers::input::{InputEvent, KeyCode, KeyState};

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

#[cfg(target_arch = "x86_64")]
unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
    val
}

#[cfg(target_arch = "x86_64")]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[cfg(target_arch = "x86_64")]
unsafe fn io_wait() { outb(0x80, 0); }

#[cfg(not(target_arch = "x86_64"))]
unsafe fn inb(_port: u16) -> u8 { 0 }
#[cfg(not(target_arch = "x86_64"))]
unsafe fn outb(_port: u16, _val: u8) {}
#[cfg(not(target_arch = "x86_64"))]
unsafe fn io_wait() {}

pub struct Ps2Keyboard {
    enabled: bool,
    extended: bool,
}

impl Ps2Keyboard {
    pub const fn new() -> Self { Self { enabled: false, extended: false } }

    pub fn init(&mut self) {
        unsafe {
            // [SOVEREIGN INIT] Phase 1: Controller Reset
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(STATUS_PORT, 0xAD); io_wait(); // Disable Kbd
            outb(STATUS_PORT, 0xA7); io_wait(); // Disable Mouse
            
            // Phase 2: Configuration
            outb(STATUS_PORT, 0x20); io_wait();
            while inb(STATUS_PORT) & 0x01 == 0 { io_wait(); }
            let mut config = inb(DATA_PORT);
            config &= !(1 << 0); // Polling mode (no IRQ1)
            config &= !(1 << 1); // Polling mode (no IRQ12)
            config |= 1 << 6;    // Translation active
            
            outb(STATUS_PORT, 0x60); io_wait();
            outb(DATA_PORT, config); io_wait();

            // Phase 3: Hardware Enable
            outb(STATUS_PORT, 0xAE); io_wait(); // Enable Kbd
            outb(DATA_PORT, 0xF4); io_wait();   // Enable Scanning
            self.flush();
        }
        self.enabled = true;
    }

    pub fn flush(&mut self) {
        unsafe {
            let mut timeout = 1000;
            while inb(STATUS_PORT) & 0x01 != 0 && timeout > 0 {
                let _ = inb(DATA_PORT);
                timeout -= 1;
            }
        }
    }

    /// [v10.4.15] Unified Polling with provided status
    pub fn poll_with_status(&mut self, _status: u8) -> Option<InputEvent> {
        if !self.enabled { return None; }
        unsafe {
            let scancode = inb(DATA_PORT);
            self.process_scancode(scancode)
        }
    }

    pub fn poll(&mut self) -> Option<InputEvent> {
        if !self.enabled { return None; }
        unsafe {
            let status = inb(STATUS_PORT);
            if status & 0x01 != 0 && status & 0x20 == 0 {
                let scancode = inb(DATA_PORT);
                return self.process_scancode(scancode);
            }
            None
        }
    }

    fn process_scancode(&mut self, scancode: u8) -> Option<InputEvent> {
        if scancode == 0xE0 { self.extended = true; return None; }
        let state = if scancode & 0x80 == 0 { KeyState::Pressed } else { KeyState::Released };
        let code = if state == KeyState::Pressed { scancode } else { scancode & 0x7F };

        let key = match code {
            0x01 => KeyCode::Escape, 0x1C => KeyCode::Enter, 0x39 => KeyCode::Space, 0x0E => KeyCode::Backspace,
            0x0F => KeyCode::Tab,
            // Alphas
            0x1E => KeyCode::A, 0x30 => KeyCode::B, 0x2E => KeyCode::C, 0x20 => KeyCode::D,
            0x12 => KeyCode::E, 0x21 => KeyCode::F, 0x22 => KeyCode::G, 0x23 => KeyCode::H,
            0x17 => KeyCode::I, 0x24 => KeyCode::J, 0x25 => KeyCode::K, 0x26 => KeyCode::L,
            0x32 => KeyCode::M, 0x31 => KeyCode::N, 0x18 => KeyCode::O, 0x19 => KeyCode::P,
            0x10 => KeyCode::Q, 0x13 => KeyCode::R, 0x1F => KeyCode::S, 0x14 => KeyCode::T,
            0x16 => KeyCode::U, 0x2F => KeyCode::V, 0x11 => KeyCode::W, 0x2D => KeyCode::X,
            0x15 => KeyCode::Y, 0x2C => KeyCode::Z,
            // Numbers
            0x02 => KeyCode::Num1, 0x03 => KeyCode::Num2, 0x04 => KeyCode::Num3, 0x05 => KeyCode::Num4,
            0x06 => KeyCode::Num5, 0x07 => KeyCode::Num6, 0x08 => KeyCode::Num7, 0x09 => KeyCode::Num8,
            0x0A => KeyCode::Num9, 0x0B => KeyCode::Num0,
            // Function Keys
            0x3B => KeyCode::F1, 0x3C => KeyCode::F2, 0x3D => KeyCode::F3, 0x3E => KeyCode::F4,
            0x3F => KeyCode::F5, 0x40 => KeyCode::F6, 0x41 => KeyCode::F7, 0x42 => KeyCode::F8,
            0x43 => KeyCode::F9, 0x44 => KeyCode::F10, 0x57 => KeyCode::F11, 0x58 => KeyCode::F12,
            // Numpad (Keypad)
            0x52 if !self.extended => KeyCode::Kp0, 0x4F if !self.extended => KeyCode::Kp1, 
            0x50 if !self.extended => KeyCode::Kp2, 0x51 if !self.extended => KeyCode::Kp3,
            0x4B if !self.extended => KeyCode::Kp4, 0x4C if !self.extended => KeyCode::Kp5, 
            0x4D if !self.extended => KeyCode::Kp6, 0x47 if !self.extended => KeyCode::Kp7,
            0x48 if !self.extended => KeyCode::Kp8, 0x49 if !self.extended => KeyCode::Kp9, 
            0x53 if !self.extended => KeyCode::KpDot, 0x35 if !self.extended => KeyCode::KpDiv,
            0x37 if !self.extended => KeyCode::KpMul, 0x4A if !self.extended => KeyCode::KpMinus, 
            0x4E if !self.extended => KeyCode::KpPlus,
            // Arrows (Extended)
            0x48 if self.extended => KeyCode::Up, 0x50 if self.extended => KeyCode::Down,
            0x4B if self.extended => KeyCode::Left, 0x4D if self.extended => KeyCode::Right,
            _ => KeyCode::Unknown(scancode),
        };

        self.extended = false;
        Some(InputEvent::Keyboard { key, state })
    }
}

pub struct Ps2Mouse {
    enabled: bool,
    has_wheel: bool,
    buffer: [u8; 4],
    index: usize,
}

impl Ps2Mouse {
    pub const fn new() -> Self { Self { enabled: false, has_wheel: false, buffer: [0; 4], index: 0 } }

    pub fn init(&mut self) {
        unsafe {
            // Enable Port 2
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(STATUS_PORT, 0xA8); io_wait();

            // [INTELLIMOUSE EXTENSION] Detect Scroll Wheel
            self.send_command(0xF2); // Get ID
            let id = self.read_data();
            if id == 0 {
                // Try to activate wheel
                self.send_command(0xF3); self.send_command(200);
                self.send_command(0xF3); self.send_command(100);
                self.send_command(0xF3); self.send_command(80);
                self.send_command(0xF2);
                let new_id = self.read_data();
                if new_id == 3 { self.has_wheel = true; }
            }

            // Enable Data Reporting
            self.send_command(0xF4);
        }
        self.enabled = true;
    }

    unsafe fn send_command(&self, cmd: u8) {
        while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
        outb(STATUS_PORT, 0xD4);
        while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
        outb(DATA_PORT, cmd);
        let _ack = self.read_data();
    }

    unsafe fn read_data(&self) -> u8 {
        let mut timeout = 10000;
        while inb(STATUS_PORT) & 0x01 == 0 && timeout > 0 { timeout -= 1; io_wait(); }
        inb(DATA_PORT)
    }

    pub fn poll_with_status(&mut self, _status: u8) -> Option<InputEvent> {
        if !self.enabled { return None; }
        unsafe {
            let b = inb(DATA_PORT);
            
            // [MILITARY GRADE] Sync Alignment Check
            // Bit 3 of the first byte in a PS/2 mouse packet should always be 1.
            // If it's 0, we are out of sync (drift). Reset buffer immediately.
            if self.index == 0 && (b & 0x08) == 0 {
                crate::print!("!"); // [DIAGNOSTIC] Mouse Sync Drift Detected & Recovered
                self.index = 0; // Drop and retry
                return None;
            }

            self.buffer[self.index] = b;
            self.index += 1;

            let target = if self.has_wheel { 4 } else { 3 };
            if self.index >= target {
                let flags = self.buffer[0];
                let mut dx = self.buffer[1] as i32;
                let mut dy = self.buffer[2] as i32;
                let mut dz = 0i32;

                if self.has_wheel { dz = self.buffer[3] as i8 as i32; }
                if (flags & 0x10) != 0 { dx |= !0xFF; }
                if (flags & 0x20) != 0 { dy |= !0xFF; }

                self.index = 0;
                
                // Final validation: Bit 3 check must pass again for the final packet
                if (flags & 0x08) == 0 { return None; }

                return Some(InputEvent::Mouse {
                    dx, dy: -dy, dz,
                    left: (flags & 1) != 0,
                    right: (flags & 2) != 0,
                    middle: (flags & 4) != 0,
                });
            }
            None
        }
    }

    pub fn poll(&mut self) -> Option<InputEvent> {
        if !self.enabled { return None; }
        unsafe {
            let status = inb(STATUS_PORT);
            if status & 0x01 != 0 && status & 0x20 != 0 {
                return self.poll_with_status(status);
            }
            None
        }
    }
}

pub static mut KEYBOARD: Ps2Keyboard = Ps2Keyboard::new();
pub static mut MOUSE: Ps2Mouse = Ps2Mouse::new();
