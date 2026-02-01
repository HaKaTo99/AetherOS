//! VGA Text Mode Driver
//! Implements Framebuffer trait for legacy VGA Text Mode (80x25)
//! Mapping: 1 pseudo-pixel = 1 character cell block

use super::{Framebuffer, Color, Point};
use core::ptr::write_volatile;

const VGA_BUFFER_ADDR: usize = 0xb8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl From<Color> for VgaColor {
    /// Approximation of RGB to VGA 16-color palette
    fn from(c: Color) -> Self {
        if c.r > 128 && c.g > 128 && c.b > 128 { Self::White }
        else if c.r > 128 && c.g > 128 { Self::Yellow } // Red + Green
        else if c.r > 128 && c.b > 128 { Self::Pink }   // Red + Blue
        else if c.g > 128 && c.b > 128 { Self::LightCyan } // Green + Blue
        else if c.r > 128 { Self::Red }
        else if c.g > 128 { Self::Green }
        else if c.b > 128 { Self::Blue }
        else { Self::Black }
    }
}

pub struct VgaTextDriver {
    buffer: *mut u16,
}

// Safety: VGA buffer is a unique system resource
unsafe impl Send for VgaTextDriver {}
unsafe impl Sync for VgaTextDriver {}

impl VgaTextDriver {
    pub const fn new() -> Self {
        Self {
            buffer: VGA_BUFFER_ADDR as *mut u16,
        }
    }
}

impl Framebuffer for VgaTextDriver {
    fn init(&mut self) {
        // Clear screen essentially
        self.clear(Color::BLACK);
    }
    
    fn clear(&mut self, color: Color) {
        let vga_color = VgaColor::from(color);
        let clear_char = 0x20u8; // Space
        let attrib = (vga_color as u8) << 4; // Background color
        let entry = (attrib as u16) << 8 | (clear_char as u16);
        
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                write_volatile(self.buffer.add(i), entry);
            }
        }
    }
    
    fn draw_pixel(&mut self, p: Point, color: Color) {
        if p.x >= VGA_WIDTH || p.y >= VGA_HEIGHT {
            return;
        }
        
        let offset = p.y * VGA_WIDTH + p.x;
        let vga_color = VgaColor::from(color);
        
        // Draw a "block" character (0xDB is full block code page 437)
        // Foreground color = pixel color
        let char_code = 0xDBu8; 
        let attrib = vga_color as u8;
        let entry = (attrib as u16) << 8 | (char_code as u16);
        
        unsafe {
            write_volatile(self.buffer.add(offset), entry);
        }
    }
    
    fn width(&self) -> usize {
        VGA_WIDTH
    }
    
    fn height(&self) -> usize {
        VGA_HEIGHT
    }
}
