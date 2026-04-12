//! Linear Framebuffer (LFB) Video Driver
//! High-Resolution Graphics for xAetherOS v10.3 SUPREME

use super::{Framebuffer, Color, Point};
use crate::boot::cmdline::FramebufferInfo;
use core::ptr::write_volatile;
use alloc::vec::Vec;
use alloc::boxed::Box;

#[derive(Debug, Clone, Copy)]
struct DirtyRegion {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    active: bool,
}

impl DirtyRegion {
    fn new() -> Self {
        Self { min_x: 2000, min_y: 2000, max_x: 0, max_y: 0, active: false }
    }

    fn update(&mut self, x: usize, y: usize) {
        if x < self.min_x { self.min_x = x; }
        if y < self.min_y { self.min_y = y; }
        if x > self.max_x { self.max_x = x; }
        if y > self.max_y { self.max_y = y; }
        self.active = true;
    }

    fn reset(&mut self) {
        self.min_x = 2000; self.min_y = 2000;
        self.max_x = 0; self.max_y = 0;
        self.active = false;
    }
}

pub struct LfbVideoDriver {
    info: FramebufferInfo,
    back_buffer: Option<Box<[u32]>>,
    dirty: DirtyRegion,
}

// --- Software Cursor & UI State ---
static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut LFB_VIRTUAL_ADDR: usize = 0; // [NEW] Phase 41: Sovereign Virtual Pointer

impl LfbVideoDriver {
    pub fn new(info: FramebufferInfo) -> Self {
        Self { 
            info, 
            back_buffer: None,
            dirty: DirtyRegion::new(),
        }
    }

    fn buffer_size(&self) -> usize {
        (self.info.width as usize) * (self.info.height as usize)
    }

    #[allow(dead_code)]
    #[inline(always)]
    fn index_for(&self, x: usize, y: usize) -> usize {
        y * (self.info.width as usize) + x
    }

    /// Calculate the memory offset for a specific (x, y) coordinate.
    #[inline(always)]
    fn get_offset(&self, x: usize, y: usize) -> usize {
        (y * self.info.pitch as usize) + (x * (self.info.bpp as usize / 8))
    }

    /// Merender jendela profesional dengan tombol kontrol kedaulatifan [R][Y][G]
    fn draw_window_internal(&mut self, title: &str, x: usize, y: usize, w: usize, h: usize, border_color: Color) {
        self.draw_rect(Point::new(x, y), w, h, Color::new(0, 5, 15));
        self.draw_rect(Point::new(x + 2, y + 2), w - 4, h - 4, Color::new(0, 2, 5));
        self.draw_gradient_rect(Point::new(x + 2, y + 2), w - 4, 30, Color::new(0, 60, 80), Color::BLACK);
        
        let btn_y = y + 10;
        let btn_right = x + w - 70;
        self.draw_rect(Point::new(btn_right + 40, btn_y), 12, 12, Color::new(255, 60, 60));
        self.draw_rect(Point::new(btn_right + 20, btn_y), 12, 12, Color::new(255, 200, 40));
        self.draw_rect(Point::new(btn_right, btn_y), 12, 12, Color::new(40, 255, 40));
        
        self.draw_string(Point::new(x + 15, y + 10), title, Color::WHITE);
        
        self.draw_rect(Point::new(x, y), w, 2, border_color); 
        self.draw_rect(Point::new(x, y + h - 2), w, 2, border_color);
        self.draw_rect(Point::new(x, y), 2, h, border_color);
        self.draw_rect(Point::new(x + w - 2, y), 2, h, border_color);
    }

    /// Internal logic for drawing the high-resolution dashboard
    pub fn render_dashboard(&mut self) {
        let w = self.info.width as usize;
        let h = self.info.height as usize;
        let cyan = Color::new(0, 255, 255);
        let magenta = Color::new(255, 0, 255);

        // [SOVEREIGN UI] 1. Render Galaxy Background (Nebula)
        use crate::drivers::video::nebula::NebulaGenerator;
        NebulaGenerator::render(self);

        // 2. Render Windowings (The "Windows" Experience)
        self.draw_window_internal("SYSTEM STATUS", 30, 80, 400, 250, cyan);
        self.draw_window_internal("SECURITY PROTOCOLS", 550, 80, 400, 350, magenta);
        self.draw_window_internal("AETHER CONNECT", 150, 400, 700, 300, cyan);

        // 3. Top HUD (Time & Global Status)
        self.draw_window_internal("HUD", w/2 - 100, 10, 200, 50, Color::WHITE);
        self.draw_string(Point::new(w/2 - 35, 25), "23:48", Color::WHITE);

        // 4. Bottom Taskbar (Icon Simulation)
        self.draw_rect(Point::new(0, h - 60), w, 60, Color::new(0, 15, 30));
        self.draw_rect(Point::new(0, h - 60), w, 2, cyan);
        self.draw_string(Point::new(w/2 - 150, h - 40), "[BROWSER] [FILES] [SETTINGS] [TERMINAL]", Color::WHITE);
    }
}

// Safety: Framebuffer access is a unique system resource managed by Mutex in mod.rs
unsafe impl Send for LfbVideoDriver {}
unsafe impl Sync for LfbVideoDriver {}

impl Framebuffer for LfbVideoDriver {
    fn init(&mut self) {
        unsafe {
            LFB_VIRTUAL_ADDR = self.info.address as usize;
            
            // [v10.5.18] Segmented Physical Scrub: Clear LFB row-by-row to remove VGA artifacts
            // This prevents a single massive write from hanging the memory bus.
            if LFB_VIRTUAL_ADDR != 0 {
                let pitch = self.info.pitch as usize;
                for y in 0..self.info.height as usize {
                    let row_ptr = (LFB_VIRTUAL_ADDR + y * pitch) as *mut u8;
                    core::ptr::write_bytes(row_ptr, 0, pitch);
                }
            }
        }
        
        // [v10.5.17] Visibility Extraction: Back-buffer will be allocated LAZILY on first draw
        self.back_buffer = None;
        
        self.dirty.min_x = 0;
        self.dirty.min_y = 0;
        self.dirty.max_x = (self.info.width as usize).saturating_sub(1);
        self.dirty.max_y = (self.info.height as usize).saturating_sub(1);
        self.dirty.active = true;
    }

    fn clear(&mut self, color: Color) {
        for y in 0..self.info.height as usize {
            for x in 0..self.info.width as usize {
                self.draw_pixel(Point::new(x, y), color);
            }
        }
    }

    fn draw_pixel(&mut self, p: Point, color: Color) {
        let width = self.info.width as usize;
        let height = self.info.height as usize;
        if p.x >= width || p.y >= height { return; }

        // [v10.5.18] Lazy Logic: Allocation attempt happens outside hot pixel-loop normally,
        // but here we check for presence to maintain absolute stability.
        if let Some(ref mut buf) = self.back_buffer {
            let idx = p.y * width + p.x;
            buf[idx] = color.to_u16_rgb565() as u32; // Fallback to safe 16-bit or 32-bit as needed
            if self.info.bpp == 32 {
                buf[idx] = color.to_u32();
            }
            self.dirty.update(p.x, p.y);
        } else {
            let offset = self.get_offset(p.x, p.y);
            unsafe {
                let addr = (LFB_VIRTUAL_ADDR + offset) as *mut u32;
                if LFB_VIRTUAL_ADDR != 0 {
                    write_volatile(addr, color.to_u32());
                }
            }
        }
    }

    fn draw_rect(&mut self, p: Point, width: usize, height: usize, color: Color) {
        let max_w = self.info.width as usize;
        let max_h = self.info.height as usize;
        if p.x >= max_w || p.y >= max_h { return; }
        let end_x = (p.x + width).min(max_w);
        let end_y = (p.y + height).min(max_h);
        let val = color.to_u32();

        if let Some(ref mut buf) = self.back_buffer {
            for y in p.y..end_y {
                let base = y * (self.info.width as usize);
                for x in p.x..end_x {
                    let idx = base + x;
                    buf[idx] = val;
                }
            }
            self.dirty.update(p.x, p.y);
            self.dirty.update(end_x.saturating_sub(1), end_y.saturating_sub(1));
            self.dirty.active = true;
        } else {
            unsafe {
                for y in p.y..end_y {
                    let mut addr = (LFB_VIRTUAL_ADDR + self.get_offset(p.x, y)) as *mut u32;
                    for _x in p.x..end_x {
                        write_volatile(addr, val);
                        addr = addr.add(1);
                    }
                }
            }
        }
    }

    fn flush(&mut self) {
        if !self.dirty.active { return; }
        if let Some(ref buf) = self.back_buffer {
            unsafe {
                if LFB_VIRTUAL_ADDR == 0 { return; }
                let fb_base = LFB_VIRTUAL_ADDR as *mut u32;
                let width = self.info.width as usize;
                let height = self.info.height as usize;
                let pitch_pixels = self.info.pitch as usize / 4;

                let start_y = self.dirty.min_y;
                let end_y = (self.dirty.max_y + 1).min(height);
                let start_x = self.dirty.min_x;
                let end_x = (self.dirty.max_x + 1).min(width);
                let copy_width = end_x - start_x;

                // [v10.5.16] Sovereign Recovery Path: Optimized Block Copy if stride permits
                if start_x == 0 && end_x == width && (width * 4) == self.info.pitch as usize {
                    let total_pixels = width * (end_y - start_y);
                    core::ptr::copy_nonoverlapping(
                        buf.as_ptr().add(start_y * width),
                        fb_base.add(start_y * width),
                        total_pixels
                    );
                } else {
                    // Precision Path: Row-by-row copy for non-standard pitches or partial regions
                    for y in start_y..end_y {
                        let dest_row = fb_base.add(y * pitch_pixels + start_x);
                        let src_row = buf.as_ptr().add(y * width + start_x);
                        core::ptr::copy_nonoverlapping(src_row, dest_row, copy_width);
                    }
                }
            }
            self.dirty.reset();
        }
    }

    fn width(&self) -> usize { self.info.width as usize }
    fn height(&self) -> usize { self.info.height as usize }

    fn set_cursor_pos(&mut self, _p: Point) {
        unsafe { CURSOR_X = _p.x; CURSOR_Y = _p.y; }
    }

    fn write_char(&mut self, c: char, color: Color) {
        unsafe {
            if c == '\n' {
                CURSOR_X = 0;
                CURSOR_Y += 10;
            } else if c == '\r' {
                CURSOR_X = 0;
            } else {
                self.draw_char(Point::new(CURSOR_X * 8, CURSOR_Y * 10), c, color);
                CURSOR_X += 1;
                if CURSOR_X * 8 >= self.info.width as usize - 160 {
                    CURSOR_X = 0;
                    CURSOR_Y += 10;
                }
            }
            if CURSOR_Y * 10 >= (self.info.height as usize - 40) {
                CURSOR_Y = 4;
                self.render_dashboard();
            }
        }
    }

    fn draw_dashboard(&mut self) { self.render_dashboard(); }
    fn get_fb_ptr(&self) -> usize { self.info.address as usize }

    fn draw_char(&mut self, p: Point, c: char, color: Color) {
        let code = c as usize;
        // [SUPREME CALIBRATION] Final offset: 16
        if code < 32 || code >= 127 { return; }
        let char_offset = (code - 16) * 8;
        let font_data = &crate::drivers::video::font::FONT_8X8;
        for row in 0..8 {
            let row_data = font_data[char_offset + row];
            for col in 0..8 {
                if (row_data & (1 << (7 - col))) != 0 {
                    self.draw_pixel(Point::new(p.x + col, p.y + row), color);
                }
            }
        }
    }
}
