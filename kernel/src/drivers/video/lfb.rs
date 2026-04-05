//! Linear Framebuffer (LFB) Video Driver
//! High-Resolution Graphics for xAetherOS v10.3 SUPREME

use super::{Framebuffer, Color, Point};
use crate::boot::cmdline::FramebufferInfo;
use core::ptr::write_volatile;
use alloc::vec::Vec;
use alloc::boxed::Box;

pub struct LfbVideoDriver {
    info: FramebufferInfo,
    back_buffer: Option<Box<[u32]>>,
}

// --- Software Cursor & UI State ---
static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut LFB_VIRTUAL_ADDR: usize = 0; // [NEW] Phase 41: Sovereign Virtual Pointer

impl LfbVideoDriver {
    pub fn new(info: FramebufferInfo) -> Self {
        Self { info, back_buffer: None }
    }

    fn buffer_size(&self) -> usize {
        (self.info.width as usize) * (self.info.height as usize)
    }

    #[inline(always)]
    fn index_for(&self, x: usize, y: usize) -> usize {
        y * (self.info.width as usize) + x
    }

    /// Calculate the memory offset for a specific (x, y) coordinate.
    #[inline(always)]
    fn get_offset(&self, x: usize, y: usize) -> usize {
        (y * self.info.pitch as usize) + (x * (self.info.bpp as usize / 8))
    }

    fn draw_char(&mut self, p: Point, c: char, color: Color) {
        let code = c as usize;
        let (x, y) = (p.x, p.y);
        // [SUPREME CALIBRATION] font.rs starts printable chars (ASCII 32) at index 128 (char 16).
        if code < 32 || code >= 127 { return; }
        let char_offset = (code - 16) * 8;
        
        let font_data = &crate::drivers::video::font::FONT_8X8;
        
        for row in 0..8 {
            let row_data = font_data[char_offset + row];
            for col in 0..8 {
                if (row_data & (1 << (7 - col))) != 0 {
                    self.draw_pixel(Point::new(x + col, y + row), color);
                }
            }
        }
    }

    /// Merender jendela profesional dengan tombol kontrol kedaulatifan [R][Y][G]
    fn draw_window(&mut self, title: &str, x: usize, y: usize, w: usize, h: usize, border_color: Color) {
        // [SOVEREIGN UI] 1. Semi-transparent-look Background
        self.draw_rect(Point::new(x, y), w, h, Color::new(0, 5, 15));
        
        // 2. Window Body
        self.draw_rect(Point::new(x + 2, y + 2), w - 4, h - 4, Color::new(0, 2, 5));
        
        // 3. Header Bar with Radiant Gradient
        self.draw_gradient_rect(Point::new(x + 2, y + 2), w - 4, 30, Color::new(0, 60, 80), Color::BLACK);
        
        // 4. Control Buttons (Standard OS Grade: R/Y/G Glow)
        let btn_y = y + 10;
        let btn_right = x + w - 70;
        
        self.draw_rect(Point::new(btn_right + 40, btn_y), 12, 12, Color::new(255, 60, 60));   // [X] Close
        self.draw_rect(Point::new(btn_right + 20, btn_y), 12, 12, Color::new(255, 200, 40));  // [+] Max
        self.draw_rect(Point::new(btn_right, btn_y), 12, 12, Color::new(40, 255, 40));        // [-] Min
        
        // 5. Title Text (White Crystal)
        self.draw_string(Point::new(x + 15, y + 10), title, Color::WHITE);
        
        // 6. Border Glow Architecture
        self.draw_rect(Point::new(x, y), w, 2, border_color); // Top
        self.draw_rect(Point::new(x, y + h - 2), w, 2, border_color); // Bottom
        self.draw_rect(Point::new(x, y), 2, h, border_color); // Left
        self.draw_rect(Point::new(x + w - 2, y), 2, h, border_color); // Right
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
        self.draw_window("SYSTEM STATUS", 30, 80, 400, 250, cyan);
        self.draw_window("SECURITY PROTOCOLS", 550, 80, 400, 350, magenta);
        self.draw_window("AETHER CONNECT", 150, 400, 700, 300, cyan);

        // 3. Top HUD (Time & Global Status)
        self.draw_window("HUD", w/2 - 100, 10, 200, 50, Color::WHITE);
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
        // [MILITARY GRADE] Sovereign Memory Mapping for x86_64 Visuals
        unsafe {
            LFB_VIRTUAL_ADDR = self.info.address as usize; 
            
            #[cfg(target_arch = "x86_64")]
            crate::memory::x86_64_paging::map_lfb_identity(self.info.address, 0x1000_0000); // Map 256MB for 4K potential
        }
        
        crate::println!("[v10.3] LFB: Visual Sovereignty Active at 0x{:X} [ {}x{} ]", 
            self.info.address, self.info.width, self.info.height);
        crate::println!("[MEMORY] LFB Mapping: Phase 0xFD Protected [SUCCESS]");
        
        // --- HIGH-SPEED BUFFER INITIALIZATION ---
        let size = self.buffer_size();
        let mut v: Vec<u32> = Vec::new();
        v.resize(size, Color::new(10, 15, 30).to_u32());
        self.back_buffer = Some(v.into_boxed_slice());
        
        // Final Sync
        self.flush();
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
        if p.x >= width || p.y >= height {
            return;
        }

        if let Some(ref mut buf) = self.back_buffer {
            let idx = p.y * width + p.x;
            buf[idx] = color.to_u32();
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
        if let Some(ref buf) = self.back_buffer {
            unsafe {
                if LFB_VIRTUAL_ADDR == 0 { return; }
                let fb_base = LFB_VIRTUAL_ADDR as *mut u32;
                let width = self.info.width as usize;
                let height = self.info.height as usize;
                let pitch_pixels = self.info.pitch as usize / 4;

                if pitch_pixels == width {
                    // ELITE MODE: Single block copy for maximum performance
                    core::ptr::copy_nonoverlapping(buf.as_ptr(), fb_base, width * height);
                } else {
                    // COMPAT MODE: Row-by-row
                    for y in 0..height {
                        let dest_row = fb_base.add(y * pitch_pixels);
                        let src_row = buf.as_ptr().add(y * width);
                        core::ptr::copy_nonoverlapping(src_row, dest_row, width);
                    }
                }
            }
        }
    }

    fn width(&self) -> usize {
        self.info.width as usize
    }

    fn height(&self) -> usize {
        self.info.height as usize
    }

    fn set_cursor_pos(&mut self, _p: Point) {
        unsafe {
            CURSOR_X = _p.x;
            CURSOR_Y = _p.y;
        }
    }

    fn write_char(&mut self, c: char, color: Color) {
        unsafe {
            if c == '\n' {
                CURSOR_X = 0;
                CURSOR_Y += 10; // 8px font + 2px spacing
            } else if c == '\r' {
                CURSOR_X = 0;
            } else {
                self.draw_char(Point::new(CURSOR_X * 8, CURSOR_Y * 10), c, color);
                CURSOR_X += 1;
                if CURSOR_X * 8 >= self.info.width as usize - 160 { // Wrapping before side HUD
                    CURSOR_X = 0;
                    CURSOR_Y += 10;
                }
            }

            // Simple scroll/reset if we hit bottom
            if CURSOR_Y * 10 >= (self.info.height as usize - 40) {
                CURSOR_Y = 4; // Start back at top within HUD
                self.render_dashboard();
            }
        }
    }

    fn draw_dashboard(&mut self) {
        self.render_dashboard();
    }
}
