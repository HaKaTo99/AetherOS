//! Organic UI Drivers (v10.3 SUPREME)
//! Adaptive rendering logic for flexible and multi-surface hardware.

use crate::drivers::video::{draw, Color, Point};

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
            
            // Draw a thin 2-pixel glow border around the rectangle
            fb.draw_rect(p, width, 2, glow_color); // Top
            fb.draw_rect(Point::new(p.x, p.y + height - 2), width, 2, glow_color); // Bottom
            fb.draw_rect(p, 2, height, glow_color); // Left
            fb.draw_rect(Point::new(p.x + width - 2, p.y), 2, height, glow_color); // Right
        });
    }

    /// Morph UI elements based on physical surface topology.
    pub fn morph_interface(_surface_curvature: f32) {
        // Future: Planar to Spherical UV mapping
    }
}
