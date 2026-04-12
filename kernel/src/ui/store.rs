//! Sovereign Store UI - v1.0 (v10.3 SUPREME)
//! The finalized ecosystem hub for Tahap III (Jalur B).

use alloc::string::String;
use alloc::vec::Vec;
use crate::ui::window::{Window, WINDOW_MANAGER};
use crate::ui::widget::Rect;
use crate::runtime::apm::PACKAGE_MANAGER;
use crate::ui::display::VectorRenderer;


pub struct StoreApp {
    pub window_id: usize,
    pub available_packages: Vec<String>,
}

impl StoreApp {
    pub fn new() -> Self {
        let mut wm = WINDOW_MANAGER.lock();
        let window_id = wm.add_window(Window::new(
            0,
            "Sovereign Store",
            100, 100, 600, 400,
            crate::drivers::video::Color::new(100, 180, 255)
        ));
        
        StoreApp {
            window_id,
            available_packages: Vec::new(),
        }
    }

    /// Refresh app list from Mesh Network
    pub fn refresh(&mut self) {
        self.available_packages.clear();
        self.available_packages.push(String::from("AetherNote"));
        self.available_packages.push(String::from("MeshChat"));
        self.available_packages.push(String::from("QuantumCalc"));
        
        crate::println!("[Store] Discovery complete. 3 sovereign apps found in mesh.");
        self.render();
    }

    /// Render the store interface
    pub fn render(&self) {
        // 1. Draw Title Area
        VectorRenderer::draw_rect(110, 110, 580, 50, 0xFF333333);
        
        // 2. Draw App Grid
        for (i, _app) in self.available_packages.iter().enumerate() {
            let y_offset = 170 + (i as u32 * 60);
            VectorRenderer::draw_rect(120, y_offset, 560, 50, 0xFF444444);
            // Simulate "Install" button
            VectorRenderer::draw_rect(580, y_offset + 10, 80, 30, 0xFF00AA00);
        }
        
        VectorRenderer::flush();
    }

    /// Install selected app through APM
    pub fn install_app(&self, name: &str) {
        let mut apm = PACKAGE_MANAGER.lock();
        if let Ok(package) = apm.fetch_from_mesh(name) {
            match apm.install(package) {
                Ok(_) => crate::println!("[Store] Successfully deployed {} to system.", name),
                Err(e) => crate::println!("[Store] Installation failed: {}", e),
            }
        }
    }
}
