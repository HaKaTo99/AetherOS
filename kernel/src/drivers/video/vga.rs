//! VGA Text Mode Driver
//! Implements Framebuffer trait for legacy VGA Text Mode (80x25)
//! Mapping: 1 pseudo-pixel = 1 character cell block

use super::{Framebuffer, Color, Point};
use core::ptr::write_volatile;

// Helper to write to I/O port (x86_64 only)
#[cfg(target_arch = "x86_64")]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

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
        let r = c.r;
        let g = c.g;
        let b = c.b;

        if r > 200 && g > 200 && b > 200 { Self::White }
        else if r > 200 && g > 200 { Self::Yellow }
        else if g > 200 && b > 200 { Self::LightCyan }
        else if g > 150 && b > 150 { Self::Cyan }
        else if r > 200 && b > 200 { Self::Pink }
        else if r > 150 { Self::Red }
        else if g > 150 { Self::Green }
        else if b > 150 { Self::Blue }
        else if r > 50 || g > 50 || b > 50 { Self::DarkGray }
        else { Self::Black }
    }
}

impl VgaColor {
    /// Returns the intensity of the color (0-255) for shade selection
    pub fn intensity(&self) -> u8 {
        match self {
            Self::Black => 0,
            Self::DarkGray => 64,
            Self::Blue | Self::Green | Self::Red | Self::Cyan | Self::Magenta | Self::Brown => 128,
            _ => 255,
        }
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
        // Enable blinking block cursor (0-15 scanlines)
        unsafe {
            outb(0x3D4, 0x0A);
            outb(0x3D5, 0x00); // Start scanline 0, bit 5=0 (enable)
            outb(0x3D4, 0x0B);
            outb(0x3D5, 0x0F); // End scanline 15
        }
        
        // Clear screen essentially
        self.clear(Color::BLACK);
        self.set_cursor_pos(Point::new(0, 0));
    }
    
    fn set_cursor_pos(&mut self, p: Point) {
        let pos = (p.y * VGA_WIDTH + p.x) as u16;

        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
        }
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
        
        // Choose shade based on average brightness
        let avg = (color.r as u16 + color.g as u16 + color.b as u16) / 3;
        let char_code = if avg > 200 { 0xDBu8 }      // Full block
                        else if avg > 130 { 0xB2u8 } // Dark shade
                        else if avg > 70 { 0xB1u8 }  // Medium shade
                        else if avg > 20 { 0xB0u8 }  // Light shade
                        else { 0x20u8 };             // Space

        let attrib = vga_color as u8;
        let entry = (attrib as u16) << 8 | (char_code as u16);
        
        unsafe {
            write_volatile(self.buffer.add(offset), entry);
        }
    }

    fn draw_gradient_rect(&mut self, p: Point, w: usize, h: usize, start: Color, end: Color) {
        for row in 0..h {
            let t = (row * 255) / h.max(1);
            let r = ((start.r as usize * (255 - t)) + (end.r as usize * t)) / 255;
            let g = ((start.g as usize * (255 - t)) + (end.g as usize * t)) / 255;
            let b = ((start.b as usize * (255 - t)) + (end.b as usize * t)) / 255;
            let current_color = Color::new(r as u8, g as u8, b as u8);
            
            for col in 0..w {
                self.draw_pixel(Point::new(p.x + col, p.y + row), current_color);
            }
        }
    }
    
    fn width(&self) -> usize {
        VGA_WIDTH
    }
    
    fn height(&self) -> usize {
        VGA_HEIGHT
    }
}
