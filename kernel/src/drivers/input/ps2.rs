//! i8042 PS/2 Keyboard Driver (v10.2 Supreme Grade)

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

/// Standard I/O delay for older/emulated hardware
#[cfg(target_arch = "x86_64")]
unsafe fn io_wait() {
    outb(0x80, 0);
}

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
    pub const fn new() -> Self {
        Self { enabled: false, extended: false }
    }

    pub fn init(&mut self) {
        unsafe {
            // [IRON-CLAD STEP 1] Disable Devices
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(STATUS_PORT, 0xAD); io_wait();
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(STATUS_PORT, 0xA7); io_wait();
            
            self.flush();
            
            // [IRON-CLAD STEP 3-4] Controller Config
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(STATUS_PORT, 0x20); io_wait();
            while inb(STATUS_PORT) & 0x01 == 0 { io_wait(); }
            let mut config = inb(DATA_PORT); io_wait();
            
            config &= !(1 << 0); // Disable IRQ1 (we poll)
            config &= !(1 << 1); // Disable IRQ2
            config &= !(1 << 4); // [SUPREME POWER] Enable Port 1 Clock
            config &= !(1 << 5); // [SUPREME POWER] Enable Port 2 Clock
            config |= (1 << 6);  // Enable Translation (Set 2 to 1)
            
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(STATUS_PORT, 0x60); io_wait();
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(DATA_PORT, config); io_wait();
            
            // [IRON-CLAD STEP 6] Enable Keyboard
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(STATUS_PORT, 0xAE); io_wait();
            
            // [IRON-CLAD STEP 8] Enable Scanning
            while inb(STATUS_PORT) & 0x02 != 0 { io_wait(); }
            outb(DATA_PORT, 0xF4); io_wait();
            
            self.flush();
        }
        self.enabled = true;
    }

    pub fn flush(&mut self) {
        unsafe {
            let mut timeout = 1000;
            while inb(STATUS_PORT) & 0x01 != 0 && timeout > 0 {
                let _ = inb(DATA_PORT);
                io_wait();
                timeout -= 1;
            }
        }
    }

    pub fn poll(&mut self) -> Option<InputEvent> {
        if !self.enabled { return None; }
        unsafe {
            let status = inb(STATUS_PORT);
            if status & 0x01 != 0 {
                if status & 0x20 != 0 {
                    let _ = inb(DATA_PORT);
                    return None;
                }
                let scancode = inb(DATA_PORT);
                return self.process_scancode(scancode);
            }
            None
        }
    }

    /// Process a raw scancode (Hybrid Set 1/2 support)
    pub fn process_scancode(&mut self, scancode: u8) -> Option<InputEvent> {
        if scancode == 0xE0 {
            self.extended = true;
            return None;
        }

        // Detect State (Set 1 break codes have bit 7 set)
        let state = if scancode & 0x80 == 0 { KeyState::Pressed } else { KeyState::Released };
        let code = if state == KeyState::Pressed { scancode } else { scancode & 0x7F };

        // [SUPREME HYBRID] Try Set 1 first, then fallback to Set 2
        let key = match code {
            // Set 1 Alpha
            0x1E => KeyCode::A, 0x30 => KeyCode::B, 0x2E => KeyCode::C, 0x20 => KeyCode::D,
            0x12 => KeyCode::E, 0x21 => KeyCode::F, 0x22 => KeyCode::G, 0x23 => KeyCode::H,
            0x17 => KeyCode::I, 0x24 => KeyCode::J, 0x25 => KeyCode::K, 0x26 => KeyCode::L,
            0x32 => KeyCode::M, 0x31 => KeyCode::N, 0x18 => KeyCode::O, 0x19 => KeyCode::P,
            0x10 => KeyCode::Q, 0x13 => KeyCode::R, 0x1F => KeyCode::S, 0x14 => KeyCode::T,
            0x16 => KeyCode::U, 0x2F => KeyCode::V, 0x11 => KeyCode::W, 0x2D => KeyCode::X,
            0x15 => KeyCode::Y, 0x2C => KeyCode::Z,
            // Set 1 Numbers
            0x02 => KeyCode::Num1, 0x03 => KeyCode::Num2, 0x04 => KeyCode::Num3, 0x05 => KeyCode::Num4,
            0x06 => KeyCode::Num5, 0x07 => KeyCode::Num6, 0x08 => KeyCode::Num7, 0x09 => KeyCode::Num8,
            0x0A => KeyCode::Num9, 0x0B => KeyCode::Num0,
            // Set 1 Spec
            0x01 => KeyCode::Escape, 0x1C => KeyCode::Enter, 0x39 => KeyCode::Space, 0x0E => KeyCode::Backspace,
            // Fallback: [Set 2 Standards]
            0x5A => KeyCode::Enter, // Set 2 Enter
            0x66 => KeyCode::Backspace, // Set 2 Backspace
            0x16 => KeyCode::Num1, // Set 2 Num1 (conflicts with Set 1 'U' if translation fails)
            0x1E => KeyCode::Num2,
            0x26 => KeyCode::Num3,
            0x25 => KeyCode::Num4,
            0x2E => KeyCode::Num5,
            0x36 => KeyCode::Num6,
            0x3D => KeyCode::Num7,
            0x3E => KeyCode::Num8,
            0x46 => KeyCode::Num9,
            0x45 => KeyCode::Num0,
            _ => KeyCode::Unknown(scancode),
        };
        
        // Final fallback for A-Z in Set 2 if the Set 1 mapping gave Unknown
        let final_key = if let KeyCode::Unknown(_) = key {
            match scancode {
                0x1C => KeyCode::A, 0x32 => KeyCode::B, 0x21 => KeyCode::C, 0x23 => KeyCode::D,
                0x24 => KeyCode::E, 0x2B => KeyCode::F, 0x34 => KeyCode::G, 0x33 => KeyCode::H,
                0x43 => KeyCode::I, 0x3B => KeyCode::J, 0x42 => KeyCode::K, 0x4B => KeyCode::L,
                0x3A => KeyCode::M, 0x31 => KeyCode::N, 0x44 => KeyCode::O, 0x4D => KeyCode::P,
                0x15 => KeyCode::Q, 0x2D => KeyCode::R, 0x1B => KeyCode::S, 0x2C => KeyCode::T,
                0x3C => KeyCode::U, 0x2A => KeyCode::V, 0x1D => KeyCode::W, 0x22 => KeyCode::X,
                0x35 => KeyCode::Y, 0x1A => KeyCode::Z,
                _ => key,
            }
        } else { key };

        self.extended = false;
        Some(InputEvent::Keyboard { key: final_key, state })
    }

    /// Trigger a visual pulse on the VGA screen to prove hardware communication
    unsafe fn trigger_heartbeat(&self) {
        let vga_buf = 0xB8000 as *mut u16;
        let pos = 79;
        let current = vga_buf.add(pos).read_volatile();
        let new_char = if current & 0xFF == b'*' as u16 {
            0x0F00 | b' ' as u16
        } else {
            0x0F00 | b'*' as u16
        };
        vga_buf.add(pos).write_volatile(new_char);
    }
}

pub static mut KEYBOARD: Ps2Keyboard = Ps2Keyboard::new();
