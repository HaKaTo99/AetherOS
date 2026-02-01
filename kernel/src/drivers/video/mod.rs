//! Video Driver Subsystem
//! Abstraction for Framebuffer and Graphics Support

pub mod vga;
pub mod font; // [NEW] Embedded Font

#[cfg(target_arch = "aarch64")]
pub mod simplefb;

// pub mod vc4; // Future: RPi4 GPU driver

/// RGB Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
    
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// Convert to 32-bit ARGB (assuming 255 alpha)
    pub fn to_u32(&self) -> u32 {
        0xFF000000 | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

/// 2D Point coordinate
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Framebuffer Trait - Interface for video drivers
pub trait Framebuffer: Send + Sync {
    /// Initialize the framebuffer
    fn init(&mut self);
    
    /// Clear screen with specific color
    fn clear(&mut self, color: Color);
    
    /// Draw a single pixel
    fn draw_pixel(&mut self, p: Point, color: Color);
    
    /// Get screen width
    fn width(&self) -> usize;
    
    /// Get screen height
    fn height(&self) -> usize;
    
    /// Flush/Swap buffers (if double buffered)
    fn flush(&mut self) {}
    
    // --- High-level primitives (default implementations) ---
    
    /// Draw a filled rectangle
    fn draw_rect(&mut self, p: Point, width: usize, height: usize, color: Color) {
        for y in p.y..(p.y + height) {
            for x in p.x..(p.x + width) {
                if x < self.width() && y < self.height() {
                    self.draw_pixel(Point::new(x, y), color);
                }
            }
        }
    }

    /// Draw a character using embedded font
    fn draw_char(&mut self, p: Point, c: char, color: Color) {
        if (c as u32) > 0x7F { return; } // Only ASCII for now
        
        let font_idx = (c as usize) * 8;
        if font_idx + 8 > crate::drivers::video::font::FONT_8X8.len() { return; }
        
        let bitmap = &crate::drivers::video::font::FONT_8X8[font_idx..font_idx+8];
        
        for y in 0..8 {
            let row = bitmap[y];
            for x in 0..8 {
                if (row >> (7 - x)) & 1 == 1 {
                    self.draw_pixel(Point::new(p.x + x, p.y + y), color);
                }
            }
        }
    }
    
    /// Draw a string
    fn draw_string(&mut self, p: Point, s: &str, color: Color) {
        let mut x = p.x;
        let mut y = p.y;
        
        for c in s.chars() {
            if c == '\n' {
                x = p.x;
                y += 10; // Line height
                continue;
            }
            
            self.draw_char(Point::new(x, y), c, color);
            x += 8; // Char width
        }
    }
}

// Global Video Driver instance (single active display)
// Uses spinlock for thread safety
use spin::Mutex;

static VIDEO_DRIVER: Mutex<Option<&'static mut dyn Framebuffer>> = Mutex::new(None);

/// Register a video driver as the global display
pub fn register_driver(driver: &'static mut dyn Framebuffer) {
    let mut driver_guard = VIDEO_DRIVER.lock();
    *driver_guard = Some(driver);
}

/// Get access to the video driver to draw
pub fn draw<F>(f: F) 
where
    F: FnOnce(&mut dyn Framebuffer),
{
    let mut driver_guard = VIDEO_DRIVER.lock();
    if let Some(driver) = driver_guard.as_mut() {
        f(*driver);
    }
}
