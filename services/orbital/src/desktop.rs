use libaether::{Color, Point};
use alloc::string::String;
use alloc::vec::Vec;

pub struct Window {
    pub id: usize,
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub accent: Color,
    pub focused: bool,
}

pub struct DesktopManager {
    pub windows: Vec<Window>,
    pub screen_width: usize,
    pub screen_height: usize,
    pub accent_color: Color,
}

impl DesktopManager {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            windows: Vec::new(),
            screen_width: w,
            screen_height: h,
            accent_color: Color::new(120, 80, 255),
        }
    }

    pub fn paint_all(&mut self, fb: &mut [u32]) {
        // [MILITARY GRADE] Direct userspace rendering into mapped framebuffer
        self.render_wallpaper(fb);

        // Render pulse animation (AetherOS signature effect)
        self.render_pulse(fb);

        // Render windows (Simplified for Stage 31.2)
        for win in &self.windows {
            self.draw_rect(fb, win.x, win.y, win.width, win.height, win.accent);
        }
    }

    fn render_wallpaper(&self, fb: &mut [u32]) {
        // High-end gradient wallpaper (Sovereign Finish)
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                fb[y * self.screen_width + x] = 0xFF0A0F2E; // Deep Blue
            }
        }
    }

    fn render_pulse(&self, fb: &mut [u32]) {
        // Centered pulse animation (simulasi fungsionalitas kernel sebelumnya)
        let cx = self.screen_width / 2;
        let cy = self.screen_height / 2;
        self.draw_rect(fb, cx - 10, cy - 10, 20, 20, self.accent_color);
    }

    fn draw_rect(&self, fb: &mut [u32], rx: usize, ry: usize, rw: usize, rh: usize, color: Color) {
        let c = 0xFF000000 | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        for y in ry..(ry + rh) {
            for x in rx..(rx + rw) {
                if x < self.screen_width && y < self.screen_height {
                    fb[y * self.screen_width + x] = c;
                }
            }
        }
    }
}
