//! Supreme Boot Splash Screen (Phase 10.1)
//! 
//! High-end graphical boot experience with Quantum Sync progress ring.
//! Inspired by the Supreme Grade aesthetics.

use crate::drivers::video::{Color, Point, draw};

pub struct BootSplash;

impl BootSplash {
    /// Render the background and basic branding
    pub fn render_base() {
        draw(|fb| {
            // 1. Clear with Deep Tactical Blue Gradient
            let top_color = Color::new(5, 5, 15);
            let bottom_color = Color::new(10, 20, 60);
            fb.draw_gradient_rect(Point::new(0, 0), fb.width(), fb.height(), top_color, bottom_color);
            
            // 2. Draw 'A' Logo Mockup (Procedural)
            let center_x = fb.width() / 2;
            let center_y = fb.height() / 2;
            
            // Minimalist 'A' (Resolution Aware)
            let cyan = Color::new(0, 255, 255);
            let size = if fb.width() <= 80 { 2 } else { 30 };
            
            fb.draw_rect(Point::new(center_x.saturating_sub(size), center_y.saturating_sub(20)), 2, 40, cyan);
            fb.draw_rect(Point::new(center_x.saturating_add(size), center_y.saturating_sub(20)), 2, 40, cyan);
            
            // 3. Branding Text (Resolution Aware)
            let text_y = center_y.saturating_add(if fb.height() <= 25 { 5 } else { 100 });
            let text_x = center_x.saturating_sub(if fb.width() <= 80 { 15 } else { 80 });
            
            fb.draw_string(Point::new(text_x, text_y), "AetherOS v10.2 SUPREME", Color::WHITE);
        });
    }

    /// Update the Quantum Sync progress ring
    pub fn update_progress(percent: usize, status_text: &str) {
        draw(|fb| {
            let center_x = fb.width() / 2;
            let center_y = fb.height() / 2 + 120;
            
            // 1. Clear progress area (Mini Gradient patch)
            fb.draw_rect(Point::new(center_x - 150, center_y - 10), 300, 100, Color::new(10, 20, 60));

            // 2. Draw Progress Ring
            let ring_color = Color::new(0, 180, 255);
            fb.draw_circle(Point::new(center_x, center_y + 40), 30, Color::new(30, 30, 80), 3); // Background
            
            // Procedural ring growth simulation (Simple for no-alloc)
            fb.draw_circle(Point::new(center_x, center_y + 40), 32, ring_color, 4);
            
            // 3. Status Label
            fb.draw_string(Point::new(center_x - (status_text.len() * 4), center_y + 80), status_text, Color::new(0, 255, 200));
            fb.draw_string(Point::new(center_x - 15, center_y + 35), &alloc::format!("{}%", percent), Color::WHITE);
        });
    }
}
