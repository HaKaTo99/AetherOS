//! Video Driver Subsystem
//! Abstraction for Framebuffer and Graphics Support

pub mod vga;
pub mod font; // [NEW] Embedded Font
pub mod lfb; // [SOVEREIGN] Linear Framebuffer
pub mod nebula; // [NEW] Sovereign Nebula Generator

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
    
    /// Mark a region as dirty (for optimized flushing)
    fn mark_dirty(&mut self, _p: Point, _w: usize, _h: usize) {}
    
    /// Get screen width
    fn width(&self) -> usize;
    
    /// Get screen height
    fn height(&self) -> usize;
    
    /// Update hardware cursor position (if supported)
    fn set_cursor_pos(&mut self, p: Point);

    /// Flush/Swap buffers (if double buffered)
    fn flush(&mut self) {}

    /// Write a character to the screen (handles cursor management in software)
    fn write_char(&mut self, _c: char, _color: Color) {}

    /// Trigger the full graphical dashboard frame (Neo-Vision)
    fn draw_dashboard(&mut self) {}
    
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
    
    /// Draw a high-end gradient rectangle (Glassmorphism simulation)
    /// Uses integer lerp for kernel stability (avoiding floats)
    fn draw_gradient_rect(&mut self, p: Point, w: usize, h: usize, start: Color, end: Color) {
        let max_w = self.width();
        let max_h = self.height();
        
        for y in 0..h {
            if p.y + y >= max_h { break; }
            
            // Integer LERP: start + (end - start) * y / h
            let r = start.r as i32 + ((end.r as i32 - start.r as i32) * y as i32 / h.max(1) as i32);
            let g = start.g as i32 + ((end.g as i32 - start.g as i32) * y as i32 / h.max(1) as i32);
            let b = start.b as i32 + ((end.b as i32 - start.b as i32) * y as i32 / h.max(1) as i32);
            let current_color = Color::new(r as u8, g as u8, b as u8);
            
            for x in 0..w {
                if p.x + x >= max_w { break; }
                self.draw_pixel(Point::new(p.x + x, p.y + y), current_color);
            }
        }
    }

    /// Draw a tactical progress ring (Procedural Circle)
    fn draw_circle(&mut self, center: Point, radius: usize, color: Color, thickness: usize) {
        let r = radius as i32;
        let t = thickness as i32;
        let r_inner = r - t;
        
        for y in -r..=r {
            for x in -r..=r {
                let dist_sq = x * x + y * y;
                if dist_sq <= r * r && dist_sq >= r_inner * r_inner {
                    let px = (center.x as i32 + x) as usize;
                    let py = (center.y as i32 + y) as usize;
                    if px < self.width() && py < self.height() {
                        self.draw_pixel(Point::new(px, py), color);
                    }
                }
            }
        }
    }

    /// Draw a string using the embedded 8x8 font
    fn draw_string(&mut self, p: Point, s: &str, color: Color) {
        let mut x = p.x;
        let mut y = p.y;
        let max_w = self.width();
        let max_h = self.height();
        
        for c in s.chars() {
            if c == '\n' {
                x = p.x;
                y += if max_h <= 25 { 1 } else { 10 }; // Line height aware
                continue;
            }
            
            if x < max_w && y < max_h {
                self.draw_char(Point::new(x, y), c, color);
            }
            x += if max_w <= 80 { 1 } else { 8 }; // Char width aware
        }
    }

    /// Draw a filled rectangle with rounded corners (8px default)
    fn draw_rounded_rect(&mut self, p: Point, w: usize, h: usize, radius: usize, color: Color) {
        let max_w = self.width();
        let max_h = self.height();
        let r = radius as i32;

        for y in 0..h {
            for x in 0..w {
                let px = p.x + x;
                let py = p.y + y;
                if px >= max_w || py >= max_h { continue; }

                let mut draw = true;
                let dx = x as i32;
                let dy = y as i32;
                let fw = w as i32;
                let fh = h as i32;

                // Top-left corner
                if dx < r && dy < r {
                    if (dx - r) * (dx - r) + (dy - r) * (dy - r) > r * r { draw = false; }
                }
                // Top-right corner
                else if dx >= fw - r && dy < r {
                    if (dx - (fw - r)) * (dx - (fw - r)) + (dy - r) * (dy - r) > r * r { draw = false; }
                }
                // Bottom-left corner
                else if dx < r && dy >= fh - r {
                    if (dx - r) * (dx - r) + (dy - (fh - r)) * (dy - (fh - r)) > r * r { draw = false; }
                }
                // Bottom-right corner
                else if dx >= fw - r && dy >= fh - r {
                    if (dx - (fw - r)) * (dx - (fw - r)) + (dy - (fh - r)) * (dy - (fh - r)) > r * r { draw = false; }
                }

                if draw {
                    self.draw_pixel(Point::new(px, py), color);
                }
            }
        }
    }

    /// [v10.3 SUPREME] Draw a high-fidelity Sovereign Window with rounded corners and traffic lights
    fn draw_sovereign_window(&mut self, title: &str, x: usize, y: usize, w: usize, h: usize, accent: Color) {
        // 1. Shadow/Outer Glow (Simulated with a slightly larger rounded rect)
        self.draw_rounded_rect(Point::new(x, y), w, h, 10, Color::new(5, 5, 10));
        
        // 2. Main Body (Dark Glass / Deep Space)
        self.draw_rounded_rect(Point::new(x + 1, y + 1), w - 2, h - 2, 8, Color::new(10, 15, 25));
        
        // 3. Title Bar (Gradient)
        self.draw_gradient_rect(Point::new(x + 2, y + 2), w - 4, 32, Color::new(25, 30, 45), Color::new(10, 15, 25));

        // 4. [macOS STYLE] Traffic Light Controls (Red, Yellow, Green)
        let btn_y = y + 12;
        let btn_start_x = x + 15;
        self.draw_circle(Point::new(btn_start_x, btn_y), 6, Color::new(255, 95, 87), 6);    // Close (Red)
        self.draw_circle(Point::new(btn_start_x + 20, btn_y), 6, Color::new(255, 189, 46), 6); // Min (Yellow)
        self.draw_circle(Point::new(btn_start_x + 40, btn_y), 6, Color::new(40, 201, 64), 6);  // Max (Green)

        // 5. Title Text (Centered Crystal)
        let title_x = x + (w / 2) - (title.len() * 4);
        self.draw_string(Point::new(title_x, y + 12), title, Color::WHITE);
        
        // 6. Accent Border (Top Glow)
        self.draw_rect(Point::new(x + 10, y), w - 20, 1, accent);
    }

    /// [v10.3 SUPREME] Draw a modern futuristic mouse cursor (Hybrid Arrow/Core)
    fn draw_cursor(&mut self, p: Point) {
        let white = Color::new(255, 255, 255);
        let accent = Color::new(100, 255, 255);
        let x = p.x;
        let y = p.y;
        let (max_w, max_h) = (self.width(), self.height());

        // Modern Arrow Shape (Software Rendered)
        for i in 0..15 {
            for j in 0..i {
                if x + j < max_w && y + i < max_h {
                    self.draw_pixel(Point::new(x + j, y + i), white);
                }
            }
        }
        // Futuristic Core (Neon Cyan)
        for i in 4..8 {
            for j in 1..i-2 {
                if x + j < max_w && y + i < max_h {
                    self.draw_pixel(Point::new(x + j, y + i), accent);
                }
            }
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
