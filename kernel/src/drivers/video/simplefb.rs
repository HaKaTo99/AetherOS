//! Simple Framebuffer Driver
//! Uses metadata passed from bootloader (DTB/UEFI)

use super::{Framebuffer, Color, Point};
use crate::drivers::dtb::DeviceTree;

pub struct SimpleFramebuffer {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: usize,
    pub buffer: *mut u8,
}

// Safety: Framebuffer memory is mmio
unsafe impl Send for SimpleFramebuffer {}
unsafe impl Sync for SimpleFramebuffer {}

static mut SIMPLE_FB: Option<SimpleFramebuffer> = None;

pub fn init(dt: &DeviceTree) {
    // TODO: Parse "simple-framebuffer" node from DTB
    // For now, stub implementation
    unsafe {
        SIMPLE_FB = Some(SimpleFramebuffer {
            width: 1920,
            height: 1080,
            pitch: 1920 * 4,
            bpp: 32,
            buffer: 0x3E000000 as *mut u8, // Stub address
        });
        
        if let Some(fb) = SIMPLE_FB.as_mut() {
            super::register_driver(fb);
        }
    }
}

impl Framebuffer for SimpleFramebuffer {
    fn init(&mut self) {
        // No-op for simplefb, already set up by bootloader
    }
    
    fn clear(&mut self, color: Color) {
        let pixel = color.to_u32();
        // Naive implementation
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(Point::new(x, y), color);
            }
        }
    }
    
    fn draw_pixel(&mut self, p: Point, color: Color) {
        if p.x >= self.width || p.y >= self.height {
            return;
        }
        
        let offset = p.y * self.pitch + p.x * 4;
        let pixel = color.to_u32(); // ARGB
        
        unsafe {
            // Assume 32-bit BGR/RGB
            let ptr = self.buffer.add(offset) as *mut u32;
            core::ptr::write_volatile(ptr, pixel);
        }
    }
    
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }

    fn set_cursor_pos(&mut self, _p: Point) {
        // SimpleFB has no hardware cursor
    }
}
