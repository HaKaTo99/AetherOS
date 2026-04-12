//! Sovereign Nebula Generator
//! High-Precision Visual Asset Generation for AetherOS v10.3 SUPREME

use crate::drivers::video::{Color, Point, Framebuffer};

pub struct NebulaGenerator;

impl NebulaGenerator {
    /// Merender latar belakang Galaksi (Nebula) ke framebuffer secara dinamis (Integer Math Only).
    pub fn render(fb: &mut dyn Framebuffer) {
        let w = fb.width();
        let h = fb.height();
        
        // 1. Bright Sovereign Gradient Base (Significantly brighter for visibility)
        for y in 0..h {
            let ratio_255 = (y * 255) / h;
            // Base: Deep Purplish-Blue but visible
            let r = (40 + (160 * ratio_255 / 255)) as u8;
            let g = (10 + (60 * ratio_255 / 255)) as u8;
            let b = (70 + (185 * ratio_255 / 255)) as u8;
            let row_color = Color::new(r, g, b);
            fb.draw_rect(Point::new(0, y), w, 1, row_color);
        }

        // 2. Render Nebula Clouds (Recursive-like noise for depth)
        for y in (0..h).step_by(3) {
            for x in (0..w).step_by(3) {
                let n1 = Self::pseudo_noise(x, y);
                let n2 = Self::pseudo_noise(x / 2, y / 2);
                let combined = (n1 as u16 + n2 as u16) / 2;
                
                if combined > 140 {
                    let intensity = (combined - 140) as u8;
                    // Multi-hued gas (Violet, Blue, Cyan)
                    let r = intensity.saturating_mul(2);
                    let g = (intensity / 3) as u8;
                    let b = intensity.saturating_mul(3);
                    let cloud = Color::new(r, g, b);
                    
                    fb.draw_pixel(Point::new(x, y), cloud);
                    if x + 1 < w { fb.draw_pixel(Point::new(x + 1, y), cloud); }
                }
            }
        }

        // 3. Render High-Intensity Stars
        for i in 0..400 {
            let x = (Self::pseudo_noise(i, 11) as usize * w / 255) % w;
            let y = (Self::pseudo_noise(i, 22) as usize * h / 255) % h;
            fb.draw_pixel(Point::new(x, y), Color::WHITE);
            // Glow effect
            if x + 1 < w { fb.draw_pixel(Point::new(x + 1, y), Color::new(150, 180, 255)); }
            if y + 1 < h { fb.draw_pixel(Point::new(x, y + 1), Color::new(150, 180, 255)); }
        }
    }

    fn pseudo_noise(x: usize, y: usize) -> u8 {
        let mut n = x.wrapping_add(y.wrapping_mul(57));
        n = (n << 13) ^ n;
        let t = n.wrapping_mul(n.wrapping_mul(n).wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589);
        (t & 0xFF) as u8
    }
}
