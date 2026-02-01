//! PS/2 Keyboard Driver (x86_64)
//! Simple polling-based implementation for stability.

use crate::drivers::input::{InputEvent, KeyCode, KeyState};

// Standard PS/2 Ports
const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

// Helper to read port (x86 only)
#[cfg(target_arch = "x86_64")]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    core::arch::asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    ret
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn inb(_port: u16) -> u8 { 0 }

/// PS/2 Keyboard Driver (Polling-based)
pub struct Ps2Keyboard {
    enabled: bool,
    extended: bool, // 0xE0 prefix
}

impl Ps2Keyboard {
    pub const fn new() -> Self {
        Self {
            enabled: false,
            extended: false,
        }
    }

    pub fn init(&mut self) {
        self.enabled = true;
    }

    /// Poll for keyboard input
    pub fn poll(&mut self) -> Option<InputEvent> {
        if !self.enabled {
            return None;
        }

        unsafe {
            let status = inb(STATUS_PORT);
            if status & 0x01 != 0 { // Output buffer full
                let scancode = inb(DATA_PORT);
                self.process_scancode(scancode)
            } else {
                None
            }
        }
    }

    /// Process a raw scancode
    fn process_scancode(&mut self, scancode: u8) -> Option<InputEvent> {
        if scancode == 0xE0 {
            self.extended = true;
            return None;
        }

        let pressed = (scancode & 0x80) == 0;
        let code = scancode & 0x7F;

        let key = if self.extended {
            self.extended = false;
            match code {
                0x48 => KeyCode::Up,
                0x50 => KeyCode::Down,
                0x4B => KeyCode::Left,
                0x4D => KeyCode::Right,
                _ => KeyCode::Unknown(scancode),
            }
        } else {
            match code {
                0x01 => KeyCode::Escape,
                0x02 => KeyCode::Num1,
                0x03 => KeyCode::Num2,
                0x04 => KeyCode::Num3,
                0x05 => KeyCode::Num4,
                0x06 => KeyCode::Num5,
                0x07 => KeyCode::Num6,
                0x08 => KeyCode::Num7,
                0x09 => KeyCode::Num8,
                0x0A => KeyCode::Num9,
                0x0B => KeyCode::Num0,
                0x0E => KeyCode::Backspace,
                0x0F => KeyCode::Tab,
                0x10 => KeyCode::Q,
                0x11 => KeyCode::W,
                0x12 => KeyCode::E,
                0x13 => KeyCode::R,
                0x14 => KeyCode::T,
                0x15 => KeyCode::Y,
                0x16 => KeyCode::U,
                0x17 => KeyCode::I,
                0x18 => KeyCode::O,
                0x19 => KeyCode::P,
                0x1C => KeyCode::Enter,
                0x1D => KeyCode::LCtrl,
                0x1E => KeyCode::A,
                0x1F => KeyCode::S,
                0x20 => KeyCode::D,
                0x21 => KeyCode::F,
                0x22 => KeyCode::G,
                0x23 => KeyCode::H,
                0x24 => KeyCode::J,
                0x25 => KeyCode::K,
                0x26 => KeyCode::L,
                0x2A => KeyCode::LShift,
                0x2C => KeyCode::Z,
                0x2D => KeyCode::X,
                0x2E => KeyCode::C,
                0x2F => KeyCode::V,
                0x30 => KeyCode::B,
                0x31 => KeyCode::N,
                0x32 => KeyCode::M,
                0x36 => KeyCode::RShift,
                0x38 => KeyCode::LAlt,
                0x39 => KeyCode::Space,
                0x3B => KeyCode::F1,
                0x3C => KeyCode::F2,
                0x3D => KeyCode::F3,
                0x3E => KeyCode::F4,
                0x3F => KeyCode::F5,
                0x40 => KeyCode::F6,
                0x41 => KeyCode::F7,
                0x42 => KeyCode::F8,
                0x43 => KeyCode::F9,
                0x44 => KeyCode::F10,
                0x57 => KeyCode::F11,
                0x58 => KeyCode::F12,
                _ => KeyCode::Unknown(scancode),
            }
        };

        Some(InputEvent::Keyboard {
            key,
            state: if pressed { KeyState::Pressed } else { KeyState::Released },
        })
    }
}

// Global PS/2 Keyboard instance
pub static mut KEYBOARD: Ps2Keyboard = Ps2Keyboard::new();

