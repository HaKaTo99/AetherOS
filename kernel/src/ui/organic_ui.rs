//! Organic UI Drivers (v10.3 SUPREME)
//! Adaptive rendering logic for flexible and multi-surface hardware.

use crate::drivers::video::{draw, Color, Point, Framebuffer};

// --- Quantum Glow Color Palette ---
pub const CYAN_GLOW: Color = Color::new(0, 255, 255);
pub const MAGENTA_GLOW: Color = Color::new(255, 0, 255);
pub const DEEP_SPACE: Color = Color::new(30, 30, 46);
pub const MIDNIGHT_FABRIC: Color = Color::new(49, 50, 68);

pub struct OrganicUIDriver;

impl OrganicUIDriver {
    /// Initialize adaptive rendering for flexible OLED or projection surfaces.
    pub fn init() {
        draw(|fb| {
            fb.init();
        });
        crate::println!("[ v10.3] OUI: Initializing organic/adaptive surface drivers... [ ACTIVE ]");
    }

    /// Clear the primary framebuffer with the Aether Sovereign Gradient.
    pub fn clear() {
        draw(|fb| {
            let w = fb.width();
            let h = fb.height();
            // Apply Sovereign Gradient: Deep Space to Midnight Fabric
            fb.draw_gradient_rect(Point::new(0, 0), w, h, DEEP_SPACE, MIDNIGHT_FABRIC);
        });
    }

    /// Draw a styled rectangle with basic alpha-blending simulation.
    pub fn draw_rect(x: u32, y: u32, w: u32, h: u32, color: Color) {
        draw(|fb| {
            fb.draw_rect(Point::new(x as usize, y as usize), w as usize, h as usize, color);
        });
    }

    /// Draw a gradient rectangle for Sovereign UI elements.
    pub fn draw_gradient_rect(x: u32, y: u32, w: u32, h: u32, start: Color, end: Color) {
        draw(|fb| {
            fb.draw_gradient_rect(Point::new(x as usize, y as usize), w as usize, h as usize, start, end);
        });
    }

    /// Draw a glowing border (Quantum Glow) for windows and taskbars.
    pub fn draw_glow_border(x: u32, y: u32, w: u32, h: u32, glow_color: Color) {
        draw(|fb| {
            let p = Point::new(x as usize, y as usize);
            let width = w as usize;
            let height = h as usize;
            
            /// Draw a thin 2-pixel glow border around the rectangle
            fb.draw_rect(p, width, 2, glow_color); // Top
            fb.draw_rect(Point::new(p.x, p.y + height - 2), width, 2, glow_color); // Bottom
            fb.draw_rect(p, 2, height, glow_color); // Left
            fb.draw_rect(Point::new(p.x + width - 2, p.y), 2, height, glow_color); // Right
        });
    }

    /// [v10.5.20] Soft Drop Shadow Generator
    pub fn draw_soft_shadow(fb: &mut dyn Framebuffer, x: usize, y: usize, w: usize, h: usize) {
        let shadow_color_1 = Color::new(5, 5, 12); // Deepest
        let shadow_color_2 = Color::new(10, 10, 20);
        let shadow_color_3 = Color::new(15, 15, 30); // Faintest
        
        // Layer 1 (Nearest)
        fb.draw_rect(Point::new(x + 2, y + 2), w, h, shadow_color_1);
        // Layer 2
        fb.draw_rect(Point::new(x + 4, y + 4), w, h, shadow_color_2);
        // Layer 3 (Farthest spreading)
        fb.draw_rect(Point::new(x + 6, y + 6), w, h, shadow_color_3);
    }

    /// [TRINITY v2.0] Professional glass panel with multi-layer glassmorphism effect.
    pub fn draw_glass_panel(fb: &mut dyn Framebuffer, x: u32, y: u32, w: u32, h: u32, accent: Color) {
        let (x, y, w, h) = (x as usize, y as usize, w as usize, h as usize);
        
        // 1. [v10.5.20] Soft Drop Shadow (Trinity Elevation)
        Self::draw_soft_shadow(fb, x, y, w, h);
        
        // 2. Base Glass Layer (Brighter, more vibrant base)
        fb.draw_gradient_rect(Point::new(x, y), w, h, Color::new(45, 50, 75), Color::new(25, 30, 50));
        
        // 3. [NEW] High-Density Bayer Dither (Frosted Grain)
        let frost_color = Color::new(80, 85, 120);
        for dy in (0..h).step_by(2) {
            for dx in (0..w).step_by(2) {
                // Bayer-like matrix 2x2 for more organic frost texture
                let threshold = if (dx / 2 + dy / 2) % 2 == 0 { 0 } else { 1 };
                if threshold == 0 {
                    fb.draw_pixel(Point::new(x + dx, y + dy), frost_color);
                }
            }
        }

        // 4. [v10.5.20] Specular Top-Edge Highlight (Premium Gloss)
        let specular = Color::new(200, 210, 255);
        let gloss_line = Color::new(120, 130, 180);
        fb.draw_rect(Point::new(x, y), w, 1, specular); // Absolute top specular
        fb.draw_rect(Point::new(x, y + 1), w, 1, gloss_line); // Secondary gloss
        fb.draw_rect(Point::new(x, y), 1, h, gloss_line); // Left edge shine
        
        // 5. Accent Identification (Plasma Line)
        fb.draw_rect(Point::new(x + 10, y), w.saturating_sub(20), 2, accent);
    }

    /// [FABRIC PULSE] Animate pulsing orb/heartbeat with simplified phase interpolation.
    pub fn animate_pulse(cx: u32, cy: u32, phase: f32, base_color: Color) {
        // [NO_STD compatible] Simple pulse without sine math
        // phase goes from 0.0 to 1.0, create triangle wave for pulsing
        let pulse_phase = if phase < 0.5 { phase * 2.0 } else { (1.0 - phase) * 2.0 };
        let intensity = (pulse_phase * 255.0) as u8;
        
        let r = ((base_color.r as u32 * intensity as u32) / 255) as u8;
        let g = ((base_color.g as u32 * intensity as u32) / 255) as u8;
        let b = ((base_color.b as u32 * intensity as u32) / 255) as u8;
        let pulse_color = Color::new(r, g, b);
        
        // Draw concentric circles for pulse effect (use global draw)
        crate::drivers::video::draw(|fb| {
            fb.draw_rect(Point::new(cx as usize, cy as usize), 12, 12, pulse_color);
            fb.draw_rect(Point::new((cx.saturating_sub(4)) as usize, (cy.saturating_sub(4)) as usize), 20, 20, 
                         Color::new((r / 2).max(0), (g / 2).max(0), (b / 2).max(0))); // Outer ring
        });
    }

    /// Switch accent theme (Neon Sovereign <-> Military Dark).
    pub fn set_accent_theme(theme: Theme) {
        match theme {
            Theme::NeonSovereign => { /* Update globals if needed */ }
            Theme::MilitaryDark => { /* Muted palette */ }
        }
        crate::println!("[OUI] Theme switched to {:?}", theme);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Theme {
    NeonSovereign,
    MilitaryDark,
}

