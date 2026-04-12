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

    /// Get the physical/virtual address of the framebuffer (for MMAP)
    fn get_fb_ptr(&self) -> usize { 0 }
    
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
        
        let font_idx = (c as usize).saturating_sub(16) * 8;
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

    /// Draw a filled rectangle with rounded corners (Optimized Path)
    /// Draw a filled rectangle with rounded corners (Optimized Path)
    fn draw_rounded_rect(&mut self, p: Point, w: usize, h: usize, radius: usize, color: Color) {
        let max_w = self.width();
        let max_h = self.height();
        let r = radius as i32;
        let fw = w as i32;
        let fh = h as i32;

        for y in 0..h {
            let py = p.y + y;
            if py >= max_h { break; }
            let dy = y as i32;

            for x in 0..w {
                let px = p.x + x;
                if px >= max_w { break; }
                let dx = x as i32;

                let mut draw = true;
                
                // Only perform expensive circle math within the corner areas
                let in_top_zone = dy < r;
                let in_bottom_zone = dy >= fh - r;
                let in_left_zone = dx < r;
                let in_right_zone = dx >= fw - r;

                if in_top_zone && in_left_zone {
                    if (dx - r) * (dx - r) + (dy - r) * (dy - r) > r * r { draw = false; }
                } else if in_top_zone && in_right_zone {
                    if (dx - (fw - r)) * (dx - (fw - r)) + (dy - r) * (dy - r) > r * r { draw = false; }
                } else if in_bottom_zone && in_left_zone {
                    if (dx - r) * (dx - r) + (dy - (fh - r)) * (dy - (fh - r)) > r * r { draw = false; }
                } else if in_bottom_zone && in_right_zone {
                    if (dx - (fw - r)) * (dx - (fw - r)) + (dy - (fh - r)) * (dy - (fh - r)) > r * r { draw = false; }
                }

                if draw {
                    self.draw_pixel(Point::new(px, py), color);
                }
            }
        }
    }

    /// [v10.3 SUPREME] Draw a high-fidelity Sovereign Window with rounded corners and traffic lights
    fn draw_sovereign_window(&mut self, title: &str, x: usize, y: usize, w: usize, h: usize, accent: Color) where Self: Sized {
        // [v10.5.20] Trinity Composition Bridge: Use high-end glass panel logic
        crate::ui::organic_ui::OrganicUIDriver::draw_glass_panel(self, x as u32, y as u32, w as u32, h as u32, accent);
        
        // [macOS STYLE] Traffic Light Controls (Refined Colors & Anti-aliased simulation)
        let btn_y = y + 14;
        let btn_start_x = x + 18;
        self.draw_circle(Point::new(btn_start_x, btn_y), 6, Color::new(255, 95, 87), 6);    // Close
        self.draw_circle(Point::new(btn_start_x + 22, btn_y), 6, Color::new(255, 189, 46), 6); // Minimize
        self.draw_circle(Point::new(btn_start_x + 44, btn_y), 6, Color::new(40, 201, 64), 6);  // Maximize

        // Title Text (Crystal White - Centered on the title bar area)
        let title_x = x + (w / 2) - (title.len() * 4);
        self.draw_string(Point::new(title_x, y + 14), title, Color::new(220, 240, 255));
    }

    /// [v10.3 SUPREME] Draw a modern futuristic mouse cursor (Hybrid Arrow/Core)
    fn draw_cursor(&mut self, p: Point) {
        let core = Color::WHITE;
        let pneu = Color::new(0, 255, 255); // Neon Cyan energy
        let x = p.x;
        let y = p.y;
        let (max_w, max_h) = (self.width(), self.height());

        // [v10.5.20] Energy Core Arrow (Cyberpunk tactical)
        for i in 0..16 {
            for j in 0..i {
                if x + j < max_w && y + i < max_h {
                    // Outer glow/plasma shadow
                    if j == i - 1 || j == 0 || i == 15 {
                        self.draw_pixel(Point::new(x + j, y + i), pneu);
                    } else if j < 4 && i < 8 {
                        self.draw_pixel(Point::new(x + j, y + i), core);
                    } else {
                        // Semi-transparent trailing effect simulation (dithered)
                        if (x + j + y + i) % 2 == 0 {
                            self.draw_pixel(Point::new(x + j, y + i), Color::new(0, 100, 100));
                        }
                    }
                }
            }
        }
        
        // Center spine glow
        for i in 0..12 {
            if x < max_w && y + i < max_h {
                self.draw_pixel(Point::new(x, y + i), core);
            }
        }
    }
}

// Global Video Driver instance (single active display)
// Uses spinlock for thread safety
use spin::Mutex;

// Wrapper to make raw pointers compatible with Send + Sync requirements
struct SendableFb(*mut dyn Framebuffer);
unsafe impl Send for SendableFb {}
unsafe impl Sync for SendableFb {}

static VIDEO_DRIVER: Mutex<Option<SendableFb>> = Mutex::new(None);

/// Register a video driver as the global display
pub fn register_driver(driver: &'static mut dyn Framebuffer) {
    let mut driver_guard = VIDEO_DRIVER.lock();
    *driver_guard = Some(SendableFb(driver as *mut dyn Framebuffer));
}

/// Get access to the video driver to draw
pub fn draw<F>(f: F) 
where
    F: FnOnce(&mut dyn Framebuffer),
{
    let driver_guard = VIDEO_DRIVER.lock();
    if let Some(SendableFb(driver_ptr)) = *driver_guard {
        // SAFETY: The pointer is valid as long as the driver is registered and we hold the lock
        unsafe {
            if !driver_ptr.is_null() {
                f(&mut *driver_ptr);
            }
        }
    }
}
