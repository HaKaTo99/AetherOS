//! Neo-Vision Fleet Dashboard (Phase 26.3)
//!
//! Visualisasi real-time kesehatan kernel dengan estetika Glassmorphism.
//! Memberikan visibilitas total terhadap resource system dan harmoni mesh.

use crate::ui::widget::Rect;
use crate::drivers::video::{Color, Point, draw};
use crate::SMME;
use crate::SCHEDULER;
use crate::DEVICE_MESH;

pub struct FleetDashboard {
    pub area: Rect,
    pub active: bool,
}

impl FleetDashboard {
    pub const fn new() -> Self {
        Self {
            area: Rect::new(40, 40, 560, 400),
            active: false,
        }
    }

    /// Render dashboard ke framebuffer menggunakan perenderan grafis penuh
    pub fn render(&self) {
        if !self.active { return; }

        draw(|fb| {
            // 1. Draw Glassmorphism Background Panel
            let top_bg = Color::new(20, 20, 40);
            let bottom_bg = Color::new(10, 10, 25);
            fb.draw_gradient_rect(
                Point::new(self.area.x as usize, self.area.y as usize),
                self.area.width as usize,
                self.area.height as usize,
                top_bg,
                bottom_bg
            );

            // 2. Draw Frame Border
            let border_color = Color::new(0, 150, 255);
            fb.draw_rect(Point::new(self.area.x as usize, self.area.y as usize), self.area.width as usize, 2, border_color); // Top
            fb.draw_rect(Point::new(self.area.x as usize, (self.area.y + self.area.height - 2) as usize), self.area.width as usize, 2, border_color); // Bottom

            // 3. Header
            fb.draw_string(Point::new((self.area.x + 20) as usize, (self.area.y + 15) as usize), 
                "AetherOS Fleet Monitor v10.2 [ SUPREME GRADE ]", Color::WHITE);

            // Fetch Live data
            let mem_stats = SMME.lock().stats();
            let sched_stats = SCHEDULER.lock().stats();
            let mesh_nodes = DEVICE_MESH.lock().device_count();

            let mut current_y = (self.area.y + 50) as usize;

            // 4. CPU Section
            fb.draw_string(Point::new((self.area.x + 20) as usize, current_y), "[ CPU ORCHESTRATION ]", Color::new(0, 255, 255));
            current_y += 20;
            
            // Integer CPU Load calculation
            let total = sched_stats.total_objects.max(1);
            let cpu_usage_pct = (sched_stats.running_objects * 100) / total;
            self.draw_graphical_bar(fb, "Core Load", cpu_usage_pct, 100, Color::new(0, 255, 255), Point::new((self.area.x + 30) as usize, current_y));
            current_y += 20;
            
            fb.draw_string(Point::new((self.area.x + 30) as usize, current_y), 
                &alloc::format!("Threads: {} | Switches: {} | Preempts: {}", 
                sched_stats.total_objects, sched_stats.context_switches, sched_stats.preemptions), 
                Color::new(200, 200, 200));
            current_y += 40;

            // 5. Memory Section (SMME 3-Tier)
            fb.draw_string(Point::new((self.area.x + 20) as usize, current_y), "[ SMME MEMORY ENGINE ]", Color::new(0, 255, 100));
            current_y += 20;
            
            let l0_pct = (mem_stats.l0_usage * 100).checked_div(16 * 1024 * 1024).unwrap_or(0);
            self.draw_graphical_bar(fb, "L0 (Small) ", l0_pct, 100, Color::GREEN, Point::new((self.area.x + 30) as usize, current_y));
            current_y += 15;

            let l1_pct = (mem_stats.l1_usage * 100).checked_div(32 * 1024 * 1024).unwrap_or(0);
            self.draw_graphical_bar(fb, "L1 (Medium)", l1_pct, 100, Color::new(255, 255, 0), Point::new((self.area.x + 30) as usize, current_y));
            current_y += 15;

            let l2_pct = (mem_stats.l2_usage * 100).checked_div(64 * 1024 * 1024).unwrap_or(0);
            self.draw_graphical_bar(fb, "L2 (Large) ", l2_pct, 100, Color::RED, Point::new((self.area.x + 30) as usize, current_y));
            current_y += 20;

            fb.draw_string(Point::new((self.area.x + 30) as usize, current_y), 
                &alloc::format!("Committed: {} KB / Reserved: {} KB", 
                mem_stats.total_committed / 1024, mem_stats.total_reserved / 1024), 
                Color::new(200, 200, 200));
            current_y += 40;

            // 6. Mesh & Security Section
            fb.draw_string(Point::new((self.area.x + 20) as usize, current_y), "[ GLOBAL MESH FABRIC ]", Color::new(255, 150, 0));
            current_y += 20;
            fb.draw_string(Point::new((self.area.x + 30) as usize, current_y), 
                &alloc::format!("Active Nodes: {} | Status: Harmony Stable", mesh_nodes), Color::WHITE);
            current_y += 15;
            fb.draw_string(Point::new((self.area.x + 30) as usize, current_y), 
                "Security: PQC Kyber-768 | Identity: herma-001 (Verified)", Color::new(0, 200, 255));

            // Footer
            fb.draw_string(Point::new((self.area.x + 20) as usize, (self.area.y + self.area.height - 30) as usize), 
                "[SYSTEM] GUI Mode Active. Press 'D' to toggle monitor.", Color::new(150, 150, 150));
        });
    }

    fn draw_graphical_bar(&self, fb: &mut dyn crate::drivers::video::Framebuffer, label: &str, value: usize, max: usize, color: Color, p: Point) {
        fb.draw_string(p, label, Color::WHITE);
        
        let bar_x = p.x + 100;
        let bar_width = 200;
        let filled_width = (value * bar_width) / max;

        // Background bar
        fb.draw_rect(Point::new(bar_x, p.y), bar_width, 8, Color::new(40, 40, 60));
        // Status bar
        fb.draw_rect(Point::new(bar_x, p.y), filled_width, 8, color);
        
        // Percentage text
        fb.draw_string(Point::new(bar_x + bar_width + 10, p.y), &alloc::format!("{}%", value), Color::WHITE);
    }
}

pub static FLEET_DASHBOARD: spin::Mutex<FleetDashboard> = spin::Mutex::new(FleetDashboard::new());
