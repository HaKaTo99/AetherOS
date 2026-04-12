//! Sovereign Desktop Environment (SDE) - Clean, compile-friendly implementation
//! Minimal, self-contained desktop manager using available framebuffer primitives

use crate::drivers::video::{Color, Point, Framebuffer};
use crate::ui::window::{Window, WindowState, AppType};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::time::Duration;
use spin::Mutex;

#[derive(Clone)]
pub struct DesktopIcon {
    pub id: usize,
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub icon_type: IconType,
    pub is_selected: bool,
}

#[derive(Clone, Copy)]
pub enum IconType {
    File,
    Folder,
    Application,
    System,
}

#[derive(Clone)]
pub struct TaskbarItem {
    pub window_id: usize,
    pub title: String,
    pub is_minimized: bool,
    pub has_notification: bool,
}

#[derive(Clone)]
pub struct Notification {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub timeout: Duration,
    pub created_at: u64,
}

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
    pub enabled: bool,
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Open,
    Copy,
    Paste,
    Delete,
    Properties,
    NewFolder,
    Refresh,
    Settings,
    About,
    Minimize,
    Maximize,
    Close,
}

#[derive(Clone, Copy)]
pub enum ContextMenuType {
    Desktop,
    Window,
    Taskbar,
    Icon,
}

pub struct DesktopManager {
    pub windows: Vec<Window>,
    pub desktop_icons: Vec<DesktopIcon>,
    pub taskbar_items: Vec<TaskbarItem>,
    pub notifications: Vec<Notification>,
    pub focused_window: Option<usize>,
    pub dragged_window: Option<usize>,
    pub drag_offset: (isize, isize),
    pub context_menu: Option<(usize, usize, Vec<MenuItem>)>,
    pub current_desktop: usize,
    pub start_menu_open: bool,
    pub search_box_active: bool,
    pub start_menu_query: String,
    pub total_desktops: usize,
    pub wallpaper_mode: WallpaperMode,
    pub show_desktop_icons: bool,
    pub top_bar_height: usize,
    pub dock_height: usize,
    pub screen_width: usize,
    pub screen_height: usize,
    pub accent_color: Color,
    // [SOVEREIGN] Kernel Metrics
    pub uptime_ticks: u64,
    pub mem_used_pages: u64,
    pub mem_total_pages: u64,
    // [v10.3 SUPREME] Interactive Mouse State
    pub mouse_x: usize,
    pub mouse_y: usize,
    pub mouse_left: bool,
}

#[derive(Clone, Copy)]
pub enum WallpaperMode {
    Nebula,
    Solid(Color),
    Gradient(Color, Color),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseEventType {
    Press,
    Release,
    Move,
}

#[derive(Clone, Copy)]
pub enum KeyCode {
    Tab,
    D,
    N,
    Q,
    F11,
    Escape,
    Char(char),
    Backspace,
    Enter,
}

#[derive(Clone, Copy)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl DesktopManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            desktop_icons: Vec::new(),
            taskbar_items: Vec::new(),
            notifications: Vec::new(),
            focused_window: None,
            dragged_window: None,
            drag_offset: (0, 0),
            context_menu: None,
            current_desktop: 0,
            total_desktops: 4,
            wallpaper_mode: WallpaperMode::Nebula,
            show_desktop_icons: true,
            top_bar_height: 30,
            dock_height: 64,
            screen_width: 1920,
            screen_height: 1200,
            accent_color: Color::new(120, 80, 255),
            uptime_ticks: 0,
            mem_used_pages: 0,
            mem_total_pages: 0,
            start_menu_open: false,
            search_box_active: false,
            start_menu_query: String::new(),
            mouse_x: 512,
            mouse_y: 384,
            mouse_left: false,
        }
    }

    pub fn get_instance() -> &'static Mutex<Self> {
    static INSTANCE: Mutex<DesktopManager> = Mutex::new(DesktopManager {
            windows: Vec::new(),
            desktop_icons: Vec::new(),
            taskbar_items: Vec::new(),
            notifications: Vec::new(),
            focused_window: None,
            dragged_window: None,
            drag_offset: (0, 0),
            context_menu: None,
            current_desktop: 0,
            total_desktops: 4,
            wallpaper_mode: WallpaperMode::Nebula,
            show_desktop_icons: true,
            top_bar_height: 30,
            dock_height: 64,
            screen_width: 1920,
            screen_height: 1200,
            accent_color: Color::new(120, 80, 255),
            uptime_ticks: 0,
            mem_used_pages: 0,
            mem_total_pages: 0,
            start_menu_open: false,
            search_box_active: false,
            start_menu_query: String::new(),
            mouse_x: 512,
            mouse_y: 384,
            mouse_left: false,
        });
        &INSTANCE
    }

    /// [v10.4.23] paint_all — Sovereign High-Fidelity Pipeline (Direct Access Path).
    pub fn paint_all(&mut self, fb: &mut dyn Framebuffer) {
        // [MILITARY GRADE] Sync logical dimensions with physical hardware
        let w = fb.width();
        let h = fb.height();
        self.screen_width = w;
        self.screen_height = h;

        // 1. Wallpaper: Sovereign Nebula (Galaxy Rendering)
        use crate::drivers::video::nebula::NebulaGenerator;
        NebulaGenerator::render(fb);

        // 2. Top Bar (Glassmorphic)
        let top_h = self.top_bar_height;
        crate::ui::organic_ui::OrganicUIDriver::draw_glass_panel(fb, 0, 0, w as u32, top_h as u32, self.accent_color);
        fb.draw_string(Point::new(12, 10), "AETHEROS v10.4", Color::new(100, 255, 255));
        fb.draw_string(Point::new(w.saturating_sub(180), 10), "SOVEREIGN DESKTOP", Color::new(100, 255, 255));

        // 3. Render Windows
        self.render_windows_internal(fb);

        // 4. Dock (bottom Glass Panel - Trinity v2.0)
        let dock_h = self.dock_height;
        let dock_y = h.saturating_sub(dock_h + 10);
        let dock_w = 600;
        let dock_x = (w.saturating_sub(dock_w)) / 2;
        crate::ui::organic_ui::OrganicUIDriver::draw_glass_panel(fb, dock_x as u32, dock_y as u32, dock_w as u32, dock_h as u32, self.accent_color);

        // Dock items (TRM, SYS, NET, SEC, APP)
        let dock_items = ["TRM", "SYS", "NET", "SEC", "APP"];
        for (i, lbl) in dock_items.iter().enumerate() {
            let ix = dock_x + 30 + (i * 110);
            let iy = dock_y + 10;
            let bg = if self.mouse_x > ix && self.mouse_x < ix + 90 && self.mouse_y > iy && self.mouse_y < iy + 44 {
                Color::new(80, 100, 160)
            } else {
                Color::new(40, 50, 80)
            };

            fb.draw_rect(Point::new(ix, iy), 90, 44, bg);
            fb.draw_rect(Point::new(ix + 1, iy + 1), 88, 42, Color::new(45, 55, 100));
            
            // Holographic inner glow for hovered item
            if bg.r > 40 {
                fb.draw_rect(Point::new(ix + 2, iy + 40), 86, 2, Color::new(0, 255, 255));
            }
            
            fb.draw_string(Point::new(ix + 28, iy + 16), lbl, Color::new(150, 255, 255));
        }

        // 5. [v10.5.20] SOVEREIGN ENERGY CURSOR (Final layer)
        fb.draw_cursor(Point::new(self.mouse_x, self.mouse_y));

        fb.flush();
    }



    /// [B4] Aether-X Launcher (Super+Space: global search popup)
    pub fn toggle_aether_x(&mut self) {
        // Centered glass popup (400x300) with search field
        let popup_x = self.screen_width.saturating_sub(400) / 2;
        let popup_y = self.screen_height.saturating_sub(300) / 2;
        
        // Check if launcher already open
        if let Some(pos) = self.windows.iter().position(|w| w.app_type == AppType::Store && w.title == "Aether-X") {
            let id = self.windows[pos].id;
            self.close_window(id);
            return;
        }
        
        let mut launcher = Window::new(0, "Aether-X Search (Super+Space)", popup_x, popup_y, 400, 300, self.accent_color);
        launcher.is_modal = true;
        launcher.parent_id = None; 
        launcher.app_type = AppType::Store; // Using Store as base for launcher
        self.add_window(launcher);
        
        crate::println!("[Trinity] Aether-X toggled ON - Search apps/files/nodes");
    }

    // === INTERNAL RENDERING FUNCTIONS FOR paint_all() ===

    /// Helper: Fill rectangle on framebuffer
    fn fill_rect_fb(&self, fb: &mut dyn Framebuffer, x: usize, y: usize, w: usize, h: usize, color: Color) {
        fb.draw_rect(Point::new(x, y), w, h, color);
    }

    /// Helper: Draw rectangle outline on framebuffer
    fn draw_rect_outline_fb(&self, fb: &mut dyn Framebuffer, x: usize, y: usize, w: usize, h: usize, color: Color) {
        // Top
        fb.draw_rect(Point::new(x, y), w, 1, color);
        // Bottom
        fb.draw_rect(Point::new(x, y + h - 1), w, 1, color);
        // Left
        fb.draw_rect(Point::new(x, y), 1, h, color);
        // Right
        fb.draw_rect(Point::new(x + w - 1, y), 1, h, color);
    }

    /// Internal desktop icons rendering for paint_all()
    fn render_desktop_icons_internal(&self, fb: &mut dyn Framebuffer) {
        for icon in &self.desktop_icons {
            let bg = if icon.is_selected { Color::new(140, 90, 255) } else { Color::new(30, 30, 55) };
            self.fill_rect_fb(fb, icon.x, icon.y, 64, 64, bg);
            self.draw_rect_outline_fb(fb, icon.x, icon.y, 64, 64, Color::new(140, 120, 255));
            let sym = match icon.icon_type { IconType::File => "F", IconType::Folder => "D", IconType::Application => "A", IconType::System => "S" };
            fb.draw_string(Point::new(icon.x + 18, icon.y + 20), sym, Color::new(200, 220, 255));
            fb.draw_string(Point::new(icon.x + 4, icon.y + 72), &icon.name, Color::new(200, 200, 255));
            self.fill_rect_fb(fb, icon.x + 8, icon.y + 8, 48, 48, Color::new(90, 80, 220));
        }
    }

    /// Internal windows rendering for paint_all()
    fn render_windows_internal(&self, fb: &mut dyn Framebuffer) {
        for window in &self.windows {
            if window.state == WindowState::Minimized { continue; }

            // 1. [v10.4 SUPREME] Draw Sovereign Window Frame & Glass
            let mut accent = if window.focused { self.accent_color } else { Color::new(70, 75, 100) };
            
            // [NEW] Fabric Pulse for focused window
            if window.focused {
                let divisor = 500_000_000u64;
                let phase = (self.uptime_ticks / divisor) % 2;
                if phase == 0 {
                    // Brighten the accent during pulse
                    accent = Color::new(
                        accent.r.saturating_add(40),
                        accent.g.saturating_add(40),
                        accent.b.saturating_add(40)
                    );
                }
            }
            
            fb.draw_sovereign_window(&window.title, window.x, window.y, window.width, window.height, accent);

            // 2. Window Content Area
            let content_y = window.y + 40;
            match window.app_type {
                AppType::Terminal => {
                    let log = crate::ui::terminal::TERMINAL_LOG.lock();
                    let lines = log.get_lines();
                    let start_idx = lines.len().saturating_sub(15);
                    for (i, line) in lines[start_idx..].iter().enumerate() {
                        let color = if line.contains("!") || line.contains("[ERROR]") {
                            Color::new(255, 120, 120)
                        } else if line.contains("[OK]") || line.contains("DONE") {
                            Color::new(140, 255, 140)
                        } else {
                            Color::new(200, 230, 255)
                        };
                        fb.draw_string(Point::new(window.x + 10, content_y + (i * 12)), line, color);
                    }
                }
                AppType::SystemStatus => {
                    fb.draw_string(Point::new(window.x + 10, content_y), "CORE: Sovereign v10.4.16", Color::WHITE);
                    fb.draw_string(Point::new(window.x + 10, content_y + 20), "MEMORY: SMME Active", Color::new(200, 200, 255));
                    fb.draw_string(Point::new(window.x + 10, content_y + 40), &format!("UPTIME: {} ticks", self.uptime_ticks), Color::new(200, 200, 255));
                    
                    // Memory progress bar (real-time)
                    if self.mem_total_pages > 0 {
                        let usage_ratio = self.mem_used_pages as f32 / self.mem_total_pages as f32;
                        let bar_w = (window.width - 20) as f32 * usage_ratio;
                        fb.draw_rect(Point::new(window.x + 10, content_y + 60), window.width - 20, 10, Color::new(40, 40, 60));
                        fb.draw_rect(Point::new(window.x + 10, content_y + 60), bar_w as usize, 10, self.accent_color);
                    }
                }
                AppType::Security => {
                    fb.draw_string(Point::new(window.x + 10, content_y), "[PQC] Post-Quantum Active", Color::new(255, 100, 255));
                    fb.draw_string(Point::new(window.x + 10, content_y + 20), "[SME] Memory Encrypted", Color::new(255, 200, 255));
                    fb.draw_string(Point::new(window.x + 10, content_y + 40), "[BFT] Swarm Consensus: OK", Color::new(100, 255, 255));
                }
                AppType::Store => {
                    // [NEW] Aether-X Launcher Visualization
                    fb.draw_string(Point::new(window.x + 20, content_y + 5), "Search Nodes/Apps:", Color::new(100, 255, 255));
                    fb.draw_rect(Point::new(window.x + 20, content_y + 25), window.width - 40, 2, self.accent_color);
                    
                    let search_results = [
                        " > Omni Kernel v10.5",
                        " > Mesh Explorer",
                        " > Tactical Dashboard",
                    ];
                    for (i, res) in search_results.iter().enumerate() {
                        let ry = content_y + 50 + (i * 30);
                        fb.draw_rect(Point::new(window.x + 15, ry - 5), window.width - 30, 24, Color::new(30, 30, 60));
                        fb.draw_string(Point::new(window.x + 30, ry), res, Color::WHITE);
                    }
                }
                AppType::FileManager => {
                    fb.draw_string(Point::new(window.x + 10, content_y), "Location: /", Color::new(150, 150, 250));
                    fb.draw_rect(Point::new(window.x + 10, content_y + 15), window.width - 20, 1, Color::new(60, 60, 100));
                    
                    // Simulated File Items
                    let items = [("system", "D"), ("apps", "D"), ("users", "D"), ("manifesto.txt", "F")];
                    for (i, (name, itype)) in items.iter().enumerate() {
                        let iy = content_y + 30 + (i * 24);
                        let color = if *itype == "D" { Color::new(255, 230, 100) } else { Color::WHITE };
                        fb.draw_string(Point::new(window.x + 15, iy), &format!("[{}] {}", itype, name), color);
                    }
                }
                _ => {
                    fb.draw_string(Point::new(window.x + 20, content_y + 20), "Application stub", Color::new(150, 150, 150));
                }
            }
        }
    }

    /// Internal top bar rendering for paint_all() - [SDE v2.0 Glassmorphism]
    fn render_top_bar_internal(&self, fb: &mut dyn Framebuffer, width: usize, _height: usize) {
        // Sovereign Glass Bar
        crate::ui::organic_ui::OrganicUIDriver::draw_glass_panel(fb, 10, 10, (width - 20) as u32, self.top_bar_height as u32, self.accent_color);

        // Aether Menu Logo (Stylized)
        fb.draw_string(Point::new(25, 18), "AetherOS", self.accent_color);
        
        // System Metrics Area (Matching Simulation)
        let metrics_x = width / 3;
        // Battery
        fb.draw_string(Point::new(metrics_x, 18), "(88%)", Color::new(100, 255, 100));
        // RAM
        fb.draw_string(Point::new(metrics_x + 80, 18), "14.2 GB / 32 GB", Color::new(200, 200, 255));
        // CPU
        fb.draw_string(Point::new(metrics_x + 220, 18), "34%", Color::new(0, 255, 255));

        // System Tray (Right Side)
        let total_secs = self.uptime_ticks / 2_500_000_000u64;
        let mins = (total_secs / 60) % 60;
        let hours = (total_secs / 3600) % 24;
        let time_str = format!("{:02}:{:02}", hours, mins);
        fb.draw_string(Point::new(width - 120, 18), time_str.as_str(), Color::new(220, 240, 255));
        fb.draw_string(Point::new(width - 220, 18), "Network", Color::new(180, 180, 255));
    }

    /// Internal dock rendering for paint_all()
    fn render_dock_internal(&self, fb: &mut dyn Framebuffer, width: usize, height: usize) {
        // Sovereign Dock Base (Dynamic Glass)
        let dock_w = 600;
        let dock_x = (width - dock_w) / 2;
        let dock_y = height - self.dock_height - 20;
        crate::ui::organic_ui::OrganicUIDriver::draw_glass_panel(fb, dock_x as u32, dock_y as u32, dock_w as u32, self.dock_height as u32, self.accent_color);

        // Render Icons (Mockup of simulation icons)
        let icons = [">_", "F", "C", "M", "</>", "W", "S", "G"];
        for (i, icon) in icons.iter().enumerate() {
            let ix = dock_x + 30 + (i * 70);
            let iy = dock_y + 12;
            fb.draw_rect(Point::new(ix, iy), 40, 40, Color::new(49, 50, 68));
            fb.draw_string(Point::new(ix + 12, iy + 12), icon, Color::WHITE);
        }
    }

    fn render_start_menu_internal(&self, fb: &mut dyn Framebuffer, _width: usize, height: usize) {
        let menu_w = 320;
        let menu_h = 340;
        let x = 16;
        let y = height.saturating_sub(self.dock_height + menu_h + 20);
        self.fill_rect_fb(fb, x, y, menu_w, menu_h, Color::new(18, 20, 35));
        self.draw_rect_outline_fb(fb, x, y, menu_w, menu_h, Color::new(100, 140, 220));
        fb.draw_string(Point::new(x + 16, y + 14), "Start Menu", Color::new(220, 240, 255));
        self.fill_rect_fb(fb, x + 16, y + 38, menu_w - 32, 34, Color::new(30, 35, 55));
        self.draw_rect_outline_fb(fb, x + 16, y + 38, menu_w - 32, 34, Color::new(100, 120, 170));
        fb.draw_string(Point::new(x + 22, y + 46), "Type to search...", Color::new(160, 180, 220));

        let items = ["Terminal", "System Status", "Security", "Aether Store", "Settings"];
        for (i, item) in items.iter().enumerate() {
            let ty = y + 84 + i * 48;
            self.fill_rect_fb(fb, x + 16, ty, menu_w - 32, 38, Color::new(28, 30, 48));
            fb.draw_string(Point::new(x + 22, ty + 10), item, Color::new(220, 220, 255));
        }
    }

    fn is_point_in_start_menu(&self, x: usize, y: usize) -> bool {
        let menu_w = 320;
        let menu_h = 340;
        let mx = 16;
        let my = self.screen_height.saturating_sub(self.dock_height + menu_h + 20);
        x >= mx && x <= mx + menu_w && y >= my && y <= my + menu_h
    }

    fn toggle_start_menu(&mut self) {
        self.start_menu_open = !self.start_menu_open;
        if !self.start_menu_open {
            self.search_box_active = false;
        }
    }

    fn close_start_menu(&mut self) {
        self.start_menu_open = false;
        self.search_box_active = false;
    }

    /// Internal notifications rendering for paint_all()
    fn render_notifications_internal(&self, fb: &mut dyn Framebuffer, width: usize, _height: usize) {
        let mut y = 50usize;
        for n in &self.notifications {
            let w = 300usize; let h = 80usize; let x = width.saturating_sub(w + 20);
            self.fill_rect_fb(fb, x, y, w, h, Color::new(50, 50, 70));
            self.fill_rect_fb(fb, x, y, w, 2, self.accent_color);
            fb.draw_string(Point::new(x + 10, y + 12), n.title.as_str(), Color::WHITE);
            let msg = if n.message.len() > 30 { format!("{}...", &n.message[..27]) } else { n.message.clone() };
            fb.draw_string(Point::new(x + 10, y + 34), msg.as_str(), Color::new(200, 200, 200));
            y += h + 10;
        }
    }

    /// Internal context menu rendering for paint_all()
    fn render_context_menu_internal(&self, fb: &mut dyn Framebuffer, x: usize, y: usize, items: &[MenuItem]) {
        let w = 150usize; let ih = 25usize; let h = items.len() * ih;
        self.fill_rect_fb(fb, x, y, w, h, Color::new(40, 45, 55));
        self.fill_rect_fb(fb, x, y, w, 1, Color::new(100, 100, 120));
        for (i, it) in items.iter().enumerate() {
            let iy = y + i * ih;
            let bg = if it.enabled { Color::new(60, 65, 75) } else { Color::new(40, 40, 50) };
            self.fill_rect_fb(fb, x + 1, iy + 1, w - 2, ih - 2, bg);
            fb.draw_string(Point::new(x + 10, iy + 5), it.label.as_str(), Color::WHITE);
        }
    }


    pub fn update_mouse(&mut self, dx: i32, dy: i32, left: bool) {
        let mut new_x = self.mouse_x as i32 + dx;
        let mut new_y = self.mouse_y as i32 + dy; // Already inverted in driver

        // Screen boundary safety
        new_x = new_x.clamp(0, self.screen_width.saturating_sub(1) as i32);
        new_y = new_y.clamp(0, self.screen_height.saturating_sub(1) as i32);

        let old_left = self.mouse_left;
        self.mouse_x = new_x as usize;
        self.mouse_y = new_y as usize;
        self.mouse_left = left;

        let x = self.mouse_x;
        let y = self.mouse_y;

        // B3 Hover/Snap: Detect near edges for snapping preview (visual feedback)
        if !left && self.dragged_window.is_none() {
            self.update_hover_effects(x, y);
        }

        if !old_left && left {
            self.handle_mouse_event(x, y, MouseButton::Left, MouseEventType::Press);
        } else if old_left && !left {
            self.handle_mouse_event(x, y, MouseButton::Left, MouseEventType::Release);
            
            // [NEW] Dock Interaction: Check for TRM (Terminal) click
            let dock_h = self.dock_height;
            let dock_y = self.screen_height.saturating_sub(dock_h + 10);
            let dock_x = (self.screen_width.saturating_sub(600)) / 2;
            
            if y > dock_y && y < dock_y + dock_h {
                if x > dock_x + 30 && x < dock_x + 120 {
                    // Clicked TRM
                    let mut term = Window::new(0, "Aether Terminal", 100, 100, 600, 420, self.accent_color);
                    term.app_type = AppType::Terminal;
                    self.add_window(term);
                }
            }

            if let Some(id) = self.dragged_window { self.snap_window_to_edge(id, x, y); }
        } else {
            self.handle_mouse_event(x, y, MouseButton::Left, MouseEventType::Move);
        }
    }

    /// [B3] Hover glow/scale preview (1.1x dock/icons)
    fn update_hover_effects(&mut self, mx: usize, my: usize) {
        // Dock hover (simplified size increase via color pulse)
        let dock_y = self.screen_height.saturating_sub(self.dock_height + 1);
        if my > dock_y {
            // Glow intensify logic (already in render_dock via accent)
            crate::print!("H"); // Debug pulse
        }
        // Icon hover
        for icon in &mut self.desktop_icons {
            if (mx.saturating_sub(icon.x) < 64) && (my.saturating_sub(icon.y) < 64) {
                icon.is_selected = true; // Triggers scale/glow in render
            }
        }
    }

    /// [B3] Magnetic window snapping on drag release
    fn snap_window_to_edge(&mut self, id: usize, mx: usize, my: usize) {
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            let margin = 20;
            let screen_w = self.screen_width;
            let screen_h = self.screen_height;
            
            if mx < margin {
                win.x = 0; // Left snap
            } else if mx > screen_w.saturating_sub(win.width + margin) {
                win.x = screen_w.saturating_sub(win.width); // Right
            } else if my < 50 {
                win.y = 0; // Top
            } else if my > screen_h.saturating_sub(win.height + margin) {
                win.y = screen_h.saturating_sub(win.height + self.dock_height);
            }
            
            crate::println!("[Trinity] Window {} snapped!", id);
        }
        self.end_drag();
    }


    fn next_window_id(&self) -> usize {
        self.windows.iter().map(|w| w.id).max().map(|m| m + 1).unwrap_or(0)
    }

    fn get_next_z_index(&self) -> i32 {
        self.windows.iter().map(|w| w.z_index).max().unwrap_or(0) + 1
    }

    fn sort_windows_by_z_index(&mut self) {
        self.windows.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    }

    fn update_taskbar(&mut self) {
        self.taskbar_items.clear();
        for w in &self.windows {
            self.taskbar_items.push(TaskbarItem {
                window_id: w.id,
                title: w.title.clone(),
                is_minimized: w.state == WindowState::Minimized,
                has_notification: false,
            });
        }
    }

    pub fn add_window(&mut self, mut window: Window) {
        let id = self.next_window_id();
        window.id = id;
        window.z_index = self.get_next_z_index();
        self.windows.push(window);
        self.focus_window(id);
        self.update_taskbar();
    }

    pub fn remove_window(&mut self, window_id: usize) {
        self.windows.retain(|w| w.id != window_id);
        if self.focused_window == Some(window_id) {
            self.focused_window = None;
        }
        self.update_taskbar();
    }

    pub fn focus_window(&mut self, window_id: usize) {
        let next_z = self.get_next_z_index();
        for w in &mut self.windows {
            w.focused = w.id == window_id;
            if w.focused {
                w.z_index = next_z;
            }
        }
        self.focused_window = Some(window_id);
        self.sort_windows_by_z_index();
    }

    pub fn minimize_window(&mut self, window_id: usize) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == window_id) {
            w.state = WindowState::Minimized;
        }
        self.update_taskbar();
    }

    pub fn maximize_window(&mut self, window_id: usize) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == window_id) {
            if w.state == WindowState::Maximized {
                w.state = WindowState::Normal;
                w.x = 100;
                w.y = 100;
                w.width = 800;
                w.height = 600;
            } else {
                w.state = WindowState::Maximized;
                w.x = 0;
                w.y = self.top_bar_height;
                w.width = self.screen_width;
                w.height = self.screen_height.saturating_sub(self.top_bar_height);
            }
        }
    }

    pub fn close_window(&mut self, window_id: usize) {
        self.remove_window(window_id);
    }

    pub fn start_drag(&mut self, window_id: usize, mouse_x: usize, mouse_y: usize) {
        if let Some(w) = self.windows.iter().find(|w| w.id == window_id) {
            self.dragged_window = Some(window_id);
            self.drag_offset = (mouse_x as isize - w.x as isize, mouse_y as isize - w.y as isize);
        }
    }

    pub fn update_drag(&mut self, mouse_x: usize, mouse_y: usize) {
        if let Some(id) = self.dragged_window {
            if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
                let nx = mouse_x as isize - self.drag_offset.0;
                let ny = mouse_y as isize - self.drag_offset.1;
                w.x = if nx < 0 { 0 } else { nx as usize };
                w.y = if ny < 0 { 0 } else { ny as usize };
                
                // [NEW] Detect Snap Target for Preview
                let margin = 40;
                if mouse_x < margin || mouse_x > self.screen_width.saturating_sub(margin) 
                   || mouse_y < margin || mouse_y > self.screen_height.saturating_sub(margin) {
                    // Visual feedback: briefly show snap hint (handled in paint_all)
                }
            }
        }
    }

    pub fn end_drag(&mut self) {
        if let Some(id) = self.dragged_window {
            let margin = 40;
            let (mx, my) = (self.mouse_x, self.mouse_y);
            let sw = self.screen_width;
            let sh = self.screen_height;
            let top_h = self.top_bar_height;
            let dock_h = self.dock_height;

            if let Some(w) = self.windows.iter_mut().find(|win| win.id == id) {
                if mx < margin {
                    // [B6] Snap Left
                    w.state = WindowState::Normal;
                    w.x = 0;
                    w.y = top_h;
                    w.width = sw / 2;
                    w.height = sh.saturating_sub(dock_h + top_h);
                } else if mx > sw.saturating_sub(margin) {
                    // [B6] Snap Right
                    w.state = WindowState::Normal;
                    w.x = sw / 2;
                    w.y = top_h;
                    w.width = sw / 2;
                    w.height = sh.saturating_sub(dock_h + top_h);
                } else if my < margin + top_h {
                    // [B6] Snap Maximize
                    w.state = WindowState::Maximized;
                    w.x = 0;
                    w.y = top_h;
                    w.width = sw;
                    w.height = sh.saturating_sub(dock_h + top_h);
                }
            }
        }
        self.dragged_window = None;
    }

    pub fn show_context_menu(&mut self, x: usize, y: usize, menu_type: ContextMenuType) {
        let items = match menu_type {
            ContextMenuType::Desktop => vec![
                MenuItem { label: String::from("New Folder"), action: MenuAction::NewFolder, enabled: true },
                MenuItem { label: String::from("Refresh"), action: MenuAction::Refresh, enabled: true },
                MenuItem { label: String::from("Settings"), action: MenuAction::Settings, enabled: true },
            ],
            ContextMenuType::Window => vec![
                MenuItem { label: String::from("Minimize"), action: MenuAction::Minimize, enabled: true },
                MenuItem { label: String::from("Maximize"), action: MenuAction::Maximize, enabled: true },
                MenuItem { label: String::from("Close"), action: MenuAction::Close, enabled: true },
            ],
            _ => Vec::new(),
        };
        self.context_menu = Some((x, y, items));
    }

    pub fn hide_context_menu(&mut self) {
        self.context_menu = None;
    }

    pub fn execute_menu_action(&mut self, action: MenuAction) {
        match action {
            MenuAction::NewFolder => {
                let id = self.desktop_icons.len();
                self.desktop_icons.push(DesktopIcon {
                    id,
                    name: format!("New Folder {}", id),
                    x: 50 + (id * 100),
                    y: 50 + (id * 100),
                    icon_type: IconType::Folder,
                    is_selected: false,
                });
            }
            MenuAction::Refresh => {
                crate::println!("[Desktop] Refreshing desktop...");
            }
            MenuAction::Settings => {
                let w = Window::new(0, "System Settings", 200, 150, 600, 400, self.accent_color);
                self.add_window(w);
            }
            MenuAction::Minimize => {
                if let Some(id) = self.focused_window {
                    self.minimize_window(id);
                }
            }
            MenuAction::Maximize => {
                if let Some(id) = self.focused_window {
                    self.maximize_window(id);
                }
            }
            MenuAction::Close => {
                if let Some(id) = self.focused_window {
                    self.close_window(id);
                }
            }
            _ => {}
        }
        self.hide_context_menu();
    }

    pub fn add_notification(&mut self, title: &str, message: &str, timeout_secs: u64) {
        let n = Notification {
            id: self.notifications.len(),
            title: String::from(title),
            message: String::from(message),
            timeout: Duration::from_secs(timeout_secs),
            created_at: 0,
        };
        self.notifications.push(n);
    }

    pub fn switch_desktop(&mut self, desktop_id: usize) {
        if desktop_id < self.total_desktops {
            self.current_desktop = desktop_id;
        }
    }

    pub fn toggle_desktop_icons(&mut self) {
        self.show_desktop_icons = !self.show_desktop_icons;
    }

    pub fn initialize_complete_desktop(&mut self) {
        // [v10.4.23] Gold Master Initialization
        self.screen_width = 1920;
        self.screen_height = 1200;
        self.top_bar_height = 32;
        self.dock_height = 64;
        self.wallpaper_mode = WallpaperMode::Nebula;
        self.accent_color = Color::new(0, 255, 255); // Neon Cyan

        // [v10.4.23] Re-enabling terminal for 100% production ready
        let mut term = Window::new(1, "Aether Terminal", 450, 200, 700, 450, self.accent_color);
        term.app_type = AppType::Terminal;
        self.add_window(term);
    }

    /// Backwards-compatible alias used elsewhere in the kernel
    pub fn initialize_supreme_desktop(&mut self) {
        self.initialize_complete_desktop();
    }

    pub fn render(&mut self, fb: &mut dyn Framebuffer) {
        let width = fb.width();
        let height = fb.height();

        self.screen_width = width;
        self.screen_height = height;

        // [DIAGNOSTIC] Trace Loop Start
        crate::print!("."); // Minimal pulse in serial

        // 1. Solid Wallpaper
        self.render_wallpaper(fb, width, height);

        // 2. Desktop Icons
        self.render_desktop_icons_internal(fb);

        // 3. Window Stack
        self.render_windows_internal(fb);

        // 4. Top Bar
        self.render_top_bar_internal(fb, width, height);

        // 5. Dock
        self.render_dock_internal(fb, width, height);

        if self.start_menu_open {
            self.render_start_menu_internal(fb, width, height);
        }

        // 6. Notifications
        self.render_notifications_internal(fb, width, height);

        // 7. Context Menu
        if let Some((x, y, ref items)) = self.context_menu {
            self.render_context_menu_internal(fb, x, y, items.as_slice());
        }

        // 8. Status overlay
        fb.draw_string(crate::drivers::video::Point::new(10, 10), "AETHEROS v10.3 [ACTIVE]", self.accent_color);

        // 9. Pulse indicator
        self.render_fabric_pulse(fb, width, height);

        // 10. Cursor
        fb.draw_cursor(crate::drivers::video::Point::new(self.mouse_x, self.mouse_y));

        // Atomic Buffer Flip
        fb.flush();
    }

    #[allow(dead_code)]
    fn render_top_bar(&self, fb: &mut dyn Framebuffer, width: usize, _height: usize) {
        // [SOVEREIGN TOP BAR] macOS/Ubuntu Style
        fb.draw_gradient_rect(Point::new(0, 0), width, self.top_bar_height, Color::new(10, 10, 20), Color::new(30, 30, 45));
        
        // Aether Menu Logo (Ubuntu/Mac hybrid)
        fb.draw_string(Point::new(10, 8), "AETHER", self.accent_color);
        
        // Active Window Placeholder
        if let Some(top) = self.windows.last() {
             fb.draw_string(Point::new(100, 8), top.title.as_str(), Color::WHITE);
        }

        // System Tray (Right Side) - Aligned for 8px font
        let total_secs = self.uptime_ticks / 2_500_000_000u64; 
        let secs = total_secs % 60;
        let mins = (total_secs / 60) % 60;
        let hours = (total_secs / 3600) % 24;
        let time_str = format!("{:02}:{:02}:{:02}", hours, mins, secs);
        fb.draw_string(Point::new(width - 80, 8), time_str.as_str(), Color::new(0, 255, 255));
        
        // WiFi Icon (Mockup symbols)
        fb.draw_string(Point::new(width - 120, 8), "(|)", Color::new(180, 180, 255));
        
        // Battery (Mockup)
        fb.draw_string(Point::new(width - 170, 8), "[|||]", Color::new(50, 255, 100));

        // [v10.3 SUPREME] Real-time Memory Usage Bar
        let mem_x = width - 300;
        let mem_w = 100;
        fb.draw_rect(Point::new(mem_x, 10), mem_w, 10, Color::new(40, 40, 60)); // BG
        if self.mem_total_pages > 0 {
            let used_w = (self.mem_used_pages * mem_w as u64 / self.mem_total_pages) as usize;
            fb.draw_rect(Point::new(mem_x, 10), used_w.min(mem_w), 10, Color::new(0, 255, 255)); // FG (Neon Cyan)
        }
        fb.draw_string(Point::new(mem_x - 40, 8), "MEM", Color::new(150, 150, 180));
    }

    fn render_wallpaper(&self, fb: &mut dyn Framebuffer, width: usize, height: usize) {
        match self.wallpaper_mode {
            WallpaperMode::Nebula => {
                use crate::drivers::video::nebula::NebulaGenerator;
                NebulaGenerator::render(fb);
                // Add a faint grid overlay for a high-tech feel
                let grid_color = Color::new(40, 30, 80);
                for y in (0..height).step_by(32) {
                    fb.draw_rect(Point::new(0, y), width, 1, grid_color);
                }
                for x in (0..width).step_by(32) {
                    fb.draw_rect(Point::new(x, 0), 1, height, grid_color);
                }
            }
            WallpaperMode::Solid(color) => {
                // [v10.3 PERFECTION] Add a subtle top-to-bottom gradient for depth
                let end_color = Color::new(
                    color.r.saturating_sub(5),
                    color.g.saturating_sub(5),
                    color.b.saturating_sub(10)
                );
                fb.draw_gradient_rect(Point::new(0, 0), width, height, color, end_color);
                
                // Add a very faint scanline/grid overlay for the high-tech Sovereign feel
                let grid_color = Color::new(20, 20, 40);
                for y in (0..height).step_by(64) {
                    fb.draw_rect(Point::new(0, y), width, 1, grid_color);
                }
            }
            WallpaperMode::Gradient(start, end) => {
                for y in 0..height {
                    let ratio = y as f32 / height as f32;
                    let r = (start.r as f32 + (end.r as f32 - start.r as f32) * ratio) as u8;
                    let g = (start.g as f32 + (end.g as f32 - start.g as f32) * ratio) as u8;
                    let b = (start.b as f32 + (end.b as f32 - start.b as f32) * ratio) as u8;
                    let c = Color::new(r, g, b);
                    fb.draw_rect(Point::new(0, y), width, 1, c);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn render_brand_overlay(&self, fb: &mut dyn Framebuffer, width: usize, height: usize) {
        let box_width = 320;
        let box_height = 90;
        let x = width.saturating_sub(box_width + 40) / 2;
        let y = height.saturating_sub(box_height + self.dock_height + 80) / 2;

        fb.draw_gradient_rect(Point::new(x, y), box_width, box_height, Color::new(120, 60, 255), Color::new(30, 10, 90));
        self.fill_rect(fb, x + 2, y + 2, box_width - 4, box_height - 4, Color::new(15, 10, 35));
        fb.draw_string(Point::new(x + 18, y + 28), "AETHEROS", Color::new(220, 220, 255));
        fb.draw_string(Point::new(x + 18, y + 48), "NEON SOVEREIGN SHELL", Color::new(180, 190, 255));
        self.draw_rect_outline(fb, x, y, box_width, box_height, Color::new(150, 120, 255));
    }

    #[allow(dead_code)]
    fn render_desktop_icons(&self, fb: &mut dyn Framebuffer) {
        for icon in &self.desktop_icons {
            let bg = if icon.is_selected { Color::new(140, 90, 255) } else { Color::new(30, 30, 55) };
            self.fill_rect(fb, icon.x, icon.y, 64, 64, bg);
            self.draw_rect_outline(fb, icon.x, icon.y, 64, 64, Color::new(140, 120, 255));
            let sym = match icon.icon_type { IconType::File => "F", IconType::Folder => "D", IconType::Application => "A", IconType::System => "S" };
            fb.draw_string(Point::new(icon.x + 18, icon.y + 20), sym, Color::new(200, 220, 255));
            fb.draw_string(Point::new(icon.x + 4, icon.y + 72), &icon.name, Color::new(200, 200, 255));
            fb.draw_rect(Point::new(icon.x + 8, icon.y + 8), 48, 48, Color::new(90, 80, 220));
        }
    }

    #[allow(dead_code)]
    fn render_windows(&self, fb: &mut dyn Framebuffer) {
        for window in &self.windows {
            if window.state == WindowState::Minimized { continue; }

            // [v10.3 SUPREME] Use the new high-fidelity window primitive
            let accent = if window.focused { self.accent_color } else { Color::new(60, 60, 80) };
            
            // [A6.4] Focus Glow (Outer border for active window)
            if window.focused {
                let glow = Color::new(accent.r / 2, accent.g / 2, accent.b / 2);
                self.draw_rect_outline(fb, window.x - 1, window.y - 1, window.width + 2, window.height + 2, glow);
            }
            
            fb.draw_sovereign_window(&window.title, window.x, window.y, window.width, window.height, accent);

            // [v10.4] Modal Dimming Effect
            // If this window is a parent of an active modal, dim its content
            if let Some(modal_id) = self.get_active_modal_id() {
                if let Some(modal_win) = self.windows.iter().find(|w| w.id == modal_id) {
                    if modal_win.parent_id == Some(window.id) {
                        // Drawing a simple diagonal "disabled" hatch or darkening rect
                        // Since we don't have true alpha blending, we simulate with a grid of dark pixels
                        for dy in 45..window.height-5 {
                            if dy % 2 == 0 {
                                fb.draw_rect(Point::new(window.x + 5, window.y + dy), window.width - 10, 1, Color::new(5, 5, 10));
                            }
                        }
                    }
                }
            }

            // Window Content Specialization (Offset by 45px to clear title bar)
            let content_y = window.y + 45;
            if window.title == "Terminal" {
                // [v10.3 SUPREME] Render Real-time Terminal Log
                let log = crate::ui::terminal::TERMINAL_LOG.lock();
                let lines = log.get_lines();
                let start_idx = lines.len().saturating_sub(20); // Show last 20 lines
                
                for (i, line) in lines[start_idx..].iter().enumerate() {
                    let color = if line.contains("!") || line.contains("[ERROR]") {
                        Color::new(255, 120, 120) // Critical/Error
                    } else if line.contains("[OK]") || line.contains("DONE") {
                        Color::new(140, 255, 140) // Success
                    } else {
                        Color::new(200, 230, 255) // Standard Plasma
                    };
                    fb.draw_string(Point::new(window.x + 15, content_y + (i * 12)), line, color);
                }
                
                // Cursor simulation
                if lines.len() > 0 {
                    let last_line = &lines[lines.len() - 1];
                    let cursor_x = window.x + 15 + (last_line.len() * 8);
                    let cursor_y = content_y + ((lines.len() - start_idx - 1) * 12);
                    if cursor_x < window.x + window.width - 15 {
                        fb.draw_rect(Point::new(cursor_x, cursor_y), 8, 10, self.accent_color);
                    }
                }
            } else if window.title == "System Status" {
                fb.draw_string(Point::new(window.x + 15, content_y), "CORE: Sovereign v10.3", Color::WHITE);
                fb.draw_string(Point::new(window.x + 15, content_y + 20), "MEMORY: SMME Active", Color::new(200, 200, 255));
                
                // Memory Progress Bar
                self.draw_rect_outline(fb, window.x + 15, content_y + 45, window.width - 30, 18, Color::new(120, 80, 255));
                if self.mem_total_pages > 0 {
                    let fill_w = (self.mem_used_pages * (window.width as u64 - 34)) / self.mem_total_pages;
                    self.fill_rect(fb, window.x + 17, content_y + 47, fill_w as usize, 14, self.accent_color);
                }
            } else if window.title == "Security Protocols" {
                fb.draw_string(Point::new(window.x + 15, content_y), "[PQC] Post-Quantum Activated", Color::new(255, 100, 255));
                fb.draw_string(Point::new(window.x + 15, content_y + 20), "[SME] Memory Encrypted", Color::new(255, 200, 255));
                fb.draw_string(Point::new(window.x + 15, content_y + 40), "[BFT] Swarm Consensus: OK", Color::new(100, 255, 255));
            }
        }
    }

    #[allow(dead_code)]
    fn render_dock(&self, fb: &mut dyn Framebuffer, width: usize, height: usize) {
        // [MODERN DOCK] macOS style floating centered launcher
        let dock_w = (self.taskbar_items.len() * 60) + 40;
        let x = (width - dock_w) / 2;
        let y = height - self.dock_height - 15;

        // Glass Dock Background
        fb.draw_gradient_rect(Point::new(x, y), dock_w, self.dock_height, Color::new(20, 20, 40), Color::new(8, 12, 28));
        self.draw_rect_outline(fb, x, y, dock_w, self.dock_height, self.accent_color);

        // Render Icons (Dock Items)
        let mut cur_x = x + 20;
        for item in &self.taskbar_items {
            let bg = if !item.is_minimized { self.accent_color } else { Color::new(45, 45, 80) };
            
            // Icon Square
            self.fill_rect(fb, cur_x, y + 10, 44, 44, bg);
            
            // Notification Dot
            if item.has_notification {
                self.fill_rect(fb, cur_x + 36, y + 6, 8, 8, Color::new(255, 90, 140));
            }
            
            // Active Indicator (Dot below icon)
            if !item.is_minimized {
                self.fill_rect(fb, cur_x + 20, y + 58, 4, 2, Color::WHITE);
            }

            cur_x += 60;
        }
    }

    #[allow(dead_code)]
    fn render_notifications(&self, fb: &mut dyn Framebuffer, width: usize, _height: usize) {
        let mut y = 50usize;
        for n in &self.notifications {
            let w = 300usize; let h = 80usize; let x = width.saturating_sub(w + 20);
            self.fill_rect(fb, x, y, w, h, Color::new(50, 50, 70));
            self.fill_rect(fb, x, y, w, 2, self.accent_color);
            fb.draw_string(Point::new(x + 10, y + 12), n.title.as_str(), Color::WHITE);
            let msg = if n.message.len() > 30 { format!("{}...", &n.message[..27]) } else { n.message.clone() };
            fb.draw_string(Point::new(x + 10, y + 34), msg.as_str(), Color::new(200, 200, 200));
            y += h + 10;
        }
    }

    #[allow(dead_code)]
    fn render_context_menu(&self, fb: &mut dyn Framebuffer, x: usize, y: usize, items: &[MenuItem]) {
        let w = 150usize; let ih = 25usize; let h = items.len() * ih;
        self.fill_rect(fb, x, y, w, h, Color::new(40, 45, 55));
        self.fill_rect(fb, x, y, w, 1, Color::new(100, 100, 120));
        for (i, it) in items.iter().enumerate() {
            let iy = y + i * ih;
            let bg = if it.enabled { Color::new(60, 65, 75) } else { Color::new(40, 40, 50) };
            self.fill_rect(fb, x + 1, iy + 1, w - 2, ih - 2, bg);
            fb.draw_string(Point::new(x + 10, iy + 5), it.label.as_str(), Color::WHITE);
        }
    }

    pub fn handle_mouse_event(&mut self, x: usize, y: usize, button: MouseButton, ev: MouseEventType) {
        // [v10.4] Check for active modals globally before processing standard events
        if let Some(target_id) = self.get_active_modal_id() {
            // Only allow events if they hit the modal (or if it's a move event for dragging the modal)
            if ev == MouseEventType::Press {
                if let Some(w) = self.windows.iter().find(|w| w.id == target_id) {
                    if !w.contains_point(x, y) { return; } // Block press outside modal
                }
            }
        }

        match ev {
            MouseEventType::Press => match button {
                MouseButton::Left => self.handle_left_click(x, y),
                MouseButton::Right => self.handle_right_click(x, y),
                MouseButton::Middle => self.handle_middle_click(x, y),
            },
            MouseEventType::Release => { if self.dragged_window.is_some() { self.end_drag(); } }
            MouseEventType::Move => self.update_drag(x, y),
        }
    }

    fn handle_left_click(&mut self, x: usize, y: usize) {
        if let Some((mx, my, items)) = &self.context_menu {
            let w = 150usize; let ih = 25usize; let h = items.len() * ih;
            if x >= *mx && x <= *mx + w && y >= *my && y <= *my + h {
                let idx = (y - *my) / ih;
                if idx < items.len() && items[idx].enabled {
                    let action = items[idx].action;
                    self.execute_menu_action(action);
                }
            } else {
                self.hide_context_menu();
            }
            return;
        }

        // Top bar interaction (Aether Menu)
        if y < self.top_bar_height {
            if x < 100 { // Clicks on "AETHER" logo area
                self.show_context_menu(10, self.top_bar_height + 5, ContextMenuType::Desktop);
            }
            return;
        }

        if self.start_menu_open && self.handle_start_menu_click(x, y) {
            return;
        }

        let bar_y = self.screen_height.saturating_sub(self.dock_height);
        if y >= bar_y {
            if self.handle_taskbar_click(x, y) {
                return;
            }
            if self.start_menu_open && !self.is_point_in_start_menu(x, y) {
                self.close_start_menu();
            }
            return;
        }

        // Desktop icons: use index-based iteration to allow safe mutable operations
        for i in 0..self.desktop_icons.len() {
            // read-only check first to avoid holding a mutable borrow
            let hit = {
                let icon = &self.desktop_icons[i];
                x >= icon.x && x <= icon.x + 64 && y >= icon.y && y <= icon.y + 64
            };
            if hit {
                let id = self.desktop_icons[i].id;
                {
                    let icon = &mut self.desktop_icons[i];
                    icon.is_selected = !icon.is_selected;
                }
                // [v10.3 SUPREME] Interactive Icon Mapping
                match id {
                    0 => self.focus_window(0), // Core -> System Status
                    1 => self.show_context_menu(x, y, ContextMenuType::Desktop), // Data -> Folder Menu
                    3 => self.focus_window(3), // Terminal -> AetherShell Window
                    _ => self.open_desktop_icon(id),
                }
                return;
            } else {
                let icon = &mut self.desktop_icons[i];
                icon.is_selected = false;
            }
        }

        // Snapshot windows in z-order (topmost first) to avoid borrowing conflicts
        let snapshots: Vec<(usize, usize, usize, usize, usize)> = self.windows.iter().rev().map(|w| (w.id, w.x, w.y, w.width, w.height)).collect();
        for (id, wx, wy, ww, wh) in snapshots {
            if x >= wx && x <= wx + ww && y >= wy && y <= wy + wh {
                // focus and then operate on a mutable reference
                self.focus_window(id);
                if let Some(wm) = self.windows.iter_mut().find(|w| w.id == id) {
                    let (tx, ty, tw, th) = wm.titlebar_rect();
                    if x >= tx && x <= tx + tw && y >= ty && y <= ty + th {
                        self.start_drag(id, x, y);
                        return;
                    }
                    let (cx, cy, cw, ch) = wm.close_button_rect();
                    if x >= cx && x <= cx + cw && y >= cy && y <= cy + ch {
                        self.close_window(id);
                        return;
                    }
                }
                return;
            }
        }
    }

    fn handle_start_menu_click(&mut self, x: usize, y: usize) -> bool {
        if !self.start_menu_open {
            return false;
        }

        let menu_w = 320;
        let menu_h = 340;
        let mx = 16;
        let my = self.screen_height.saturating_sub(self.dock_height + menu_h + 20);

        if x < mx || x > mx + menu_w || y < my || y > my + menu_h {
            return false;
        }

        let search_y = my + 38;
        if y >= search_y && y <= search_y + 34 {
            self.toggle_aether_x();
            return true;
        }

        let items = ["Terminal", "System Status", "Security", "Aether Store", "Settings"];
        let item_start_y = my + 84;
        if y >= item_start_y && y < item_start_y + items.len() * 48 {
            let idx = (y - item_start_y) / 48;
        match idx {
            0 => {
                let w = Window::new(0, "Terminal", 100, 100, 640, 480, self.accent_color)
                             .with_app_type(AppType::Terminal);
                self.add_window(w);
            }
            1 => {
                let w = Window::new(0, "System Status", 150, 120, 400, 300, Color::new(0, 255, 150))
                             .with_app_type(AppType::SystemStatus);
                self.add_window(w);
            }
            2 => {
                let w = Window::new(0, "Security", 260, 220, 500, 340, Color::new(255, 100, 255))
                             .with_app_type(AppType::Security);
                self.add_window(w);
            }
            3 => {
                let w = Window::new(0, "Aether Store", 240, 200, 520, 360, Color::new(100, 180, 255))
                             .with_app_type(AppType::Store);
                self.add_window(w);
            }
            4 => {
                let w = Window::new(0, "Settings", 280, 210, 480, 340, Color::new(180, 120, 255))
                             .with_app_type(AppType::Settings);
                self.add_window(w);
            }
            _ => {}
        }
            self.close_start_menu();
            return true;
        }

        true
    }

    fn handle_right_click(&mut self, x: usize, y: usize) {
        let dock_y = self.screen_height.saturating_sub(self.dock_height + 15);
        if y < dock_y {
            for w in &self.windows {
                if w.contains_point(x, y) {
                    self.show_context_menu(x, y, ContextMenuType::Window);
                    return;
                }
            }
            self.show_context_menu(x, y, ContextMenuType::Desktop);
        }
    }

    fn handle_middle_click(&mut self, _x: usize, _y: usize) {}

    fn get_active_modal_id(&self) -> Option<usize> {
        self.windows.iter().find(|w| w.is_modal).map(|w| w.id)
    }

    fn render_fabric_pulse(&self, fb: &mut dyn Framebuffer, width: usize, height: usize) {
        // Pulse logic based on uptime (simulated sine wave)
        let divisor = 500_000_000u64; // ~0.5s per pulse cycle
        let phase = (self.uptime_ticks / divisor) % 2;
        let brightness = if phase == 0 { 255 } else { 150 };
        
        let x = width - 130;
        let y = height - 100;
        let color = Color::new(0, brightness as u8, (brightness / 2) as u8);
        
        // Draw the glowing orb
        fb.draw_rect(Point::new(x, y), 12, 12, color);
        fb.draw_string(Point::new(x + 18, y + 2), "FABRIC PULSE", Color::new(100, 255, 180));
    }

    fn handle_taskbar_click(&mut self, x: usize, y: usize) -> bool {
        let bar_height = self.dock_height;
        let bar_y = self.screen_height.saturating_sub(bar_height);
        if y < bar_y || y > bar_y + bar_height {
            return false;
        }

        let start_x = 16;
        let start_w = 112;
        let search_x = start_x + start_w + 16;
        let search_w = 220;
        let mut icon_x = search_x + search_w + 22;
        let icon_size = 42;
        let icon_gap = 54;
        let tray_x = self.screen_width.saturating_sub(240);
        let tray_icon_size = 28;
        let tray_gap = 14;

        if x >= start_x && x <= start_x + start_w {
            self.toggle_start_menu();
            return true;
        }

        if x >= search_x && x <= search_x + search_w {
            self.toggle_aether_x();
            return true;
        }

        for i in 0..self.taskbar_items.len() {
            if x >= icon_x && x <= icon_x + icon_size && y >= bar_y + 4 && y <= bar_y + 4 + icon_size {
                let window_id = self.taskbar_items[i].window_id;
                if self.taskbar_items[i].is_minimized {
                    if let Some(w) = self.windows.iter_mut().find(|w| w.id == window_id) {
                        w.state = WindowState::Normal;
                    }
                } else {
                    self.minimize_window(window_id);
                }
                self.update_taskbar();
                return true;
            }
            icon_x += icon_gap;
        }

        let mut tray_pos = tray_x;
        if x >= tray_pos && x <= tray_pos + tray_icon_size {
            self.add_notification("Network", "WiFi connected", 4);
            return true;
        }
        tray_pos += tray_icon_size + tray_gap;

        if x >= tray_pos && x <= tray_pos + tray_icon_size {
            self.add_notification("Sound", "Volume active", 4);
            return true;
        }
        tray_pos += tray_icon_size + tray_gap;

        if x >= tray_pos && x <= tray_pos + tray_icon_size {
            self.add_notification("Battery", "82% remaining", 4);
            return true;
        }

        true
    }

    fn open_desktop_icon(&mut self, icon_id: usize) {
        if let Some(icon) = self.desktop_icons.iter().find(|i| i.id == icon_id) {
            match icon.icon_type {
                IconType::System => { 
                    let w = Window::new(0, &format!("{} - System", icon.name), 200, 150, 600, 400, self.accent_color)
                                 .with_app_type(AppType::SystemStatus); 
                    self.add_window(w); 
                }
                IconType::Folder => { 
                    let w = Window::new(0, &format!("{} - Folder", icon.name), 250, 200, 500, 350, Color::new(255,255,0))
                                 .with_app_type(AppType::FileManager); 
                    self.add_window(w); 
                }
                IconType::Application => { 
                    let w = Window::new(0, &format!("{} - Application", icon.name), 300, 250, 550, 400, Color::new(0,255,0))
                                 .with_app_type(AppType::Generic); 
                    self.add_window(w); 
                }
                _ => {}
            }
        }
    }

    pub fn handle_keyboard_event(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // 1. Handle Global Hotkeys
        match (key, modifiers.ctrl, modifiers.alt, modifiers.shift) {
            (KeyCode::Tab, true, false, false) => { self.cycle_windows(); return; }
            (KeyCode::D, true, false, false) => { self.show_desktop(); return; }
            (KeyCode::N, true, false, false) => { 
                let w = Window::new(0, "New Window", 300, 300, 400, 300, self.accent_color); 
                self.add_window(w); 
                return; 
            }
            (KeyCode::Q, true, false, false) => { self.close_all_windows(); return; }
            _ => {}
        }

        // 2. Route to Active Window
        if let Some(id) = self.focused_window {
            if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
                match win.app_type {
                    AppType::Terminal => {
                        match key {
                            KeyCode::Char(c) => {
                                crate::ui::terminal::log_to_terminal(&c.to_string());
                            }
                            KeyCode::Enter => {
                                crate::ui::terminal::log_to_terminal("\n");
                            }
                            KeyCode::Backspace => {
                                // Simple backspace simulation (remove last char if possible)
                                // In a real system, this would interact with the shell buffer
                                crate::ui::terminal::log_to_terminal("\x08"); 
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn cycle_windows(&mut self) {
        if self.windows.is_empty() { return; }
        let idx = if let Some(f_id) = self.focused_window {
            if let Some(pos) = self.windows.iter().position(|w| w.id == f_id) { (pos + 1) % self.windows.len() } else { 0 }
        } else { 0 };
        let next_id = self.windows[idx].id;
        self.focus_window(next_id);
    }

    fn show_desktop(&mut self) {
        for w in &mut self.windows { w.state = WindowState::Minimized; }
        self.update_taskbar();
    }

    fn close_all_windows(&mut self) {
        self.windows.clear();
        self.update_taskbar();
    }

    // Helper: fill rectangle using draw_pixel
    fn fill_rect(&self, fb: &mut dyn Framebuffer, x: usize, y: usize, width: usize, height: usize, color: Color) {
        fb.draw_rect(Point::new(x, y), width, height, color);
    }

    // Helper: draw rectangle outline using draw_rect for faster writes
    fn draw_rect_outline(&self, fb: &mut dyn Framebuffer, x: usize, y: usize, width: usize, height: usize, color: Color) {
        if width == 0 || height == 0 { return; }
        // Top
        fb.draw_rect(Point::new(x, y), width, 1, color);
        // Bottom
        if height > 1 {
            fb.draw_rect(Point::new(x, y + height - 1), width, 1, color);
        }
        // Left
        if height > 2 {
            fb.draw_rect(Point::new(x, y + 1), 1, height - 2, color);
            // Right
            if width > 1 {
                fb.draw_rect(Point::new(x + width - 1, y + 1), 1, height - 2, color);
            }
        }
    }
}
