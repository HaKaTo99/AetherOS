//! AetherDesktop (v10.2 SUPREME)
//! Core Graphical Desktop Environment for xAetherOS.
//! Baseline for Tahap III (Desktop Expansion).

use crate::ui::window::WINDOW_MANAGER;
use crate::ui::Rect;
use crate::hal;

pub struct AetherDesktop;

impl AetherDesktop {
    /// Initialize the v10.2 SUPREME Graphical Desktop.
    pub fn init() {
        let platform = hal::get_platform();
        platform.puts("[ v10.2] Desktop: Starting xAetherOS Graphical Seed...\n");

        // 1. Initialize Organic UI FrameBuffer
        super::organic_ui::OrganicUIDriver::init();
        super::organic_ui::OrganicUIDriver::clear();

        // 2. Spawn Sovereign Taskbar
        Self::render_taskbar();

        // 3. Spawn Sovereign Terminal Window (Shell Bridge)
        Self::spawn_terminal();

        platform.puts("[ v10.2] Desktop: Environment Seed [ READY ]\n");
    }

    /// Render the Sovereign Taskbar at the bottom of the screen.
    fn render_taskbar() {
        let mut wm = WINDOW_MANAGER.lock();
        // Placeholder Taskbar: Bottom 40 pixels
        let id = wm.create_window(Rect::new(0, 728, 1024, 40), "Sovereign Taskbar v10.2");
        
        // --- TAHAP III: FUNCTIONAL BASELINE ---
        // Explicitly trigger the software renderer for the taskbar base
        super::organic_ui::OrganicUIDriver::draw_rect(0, 728, 1024, 40, 0xFF1E1E2E);
        
        crate::println!("[v10.2] Desktop: Taskbar [ RENDERED ] -> WindowID: {}", id);
    }

    /// Spawn the main terminal window for AetherShell interaction.
    fn spawn_terminal() {
        let mut wm = WINDOW_MANAGER.lock();
        wm.create_window(Rect::new(100, 100, 640, 480), "AetherShell Terminal");
    }
}
