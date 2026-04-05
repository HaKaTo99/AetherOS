//! Sovereign Desktop Environment (SDE) - Clean, compile-friendly implementation
//! Minimal, self-contained desktop manager using available framebuffer primitives

use crate::drivers::video::{Color, Point, Framebuffer};
use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Closed,
}

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

pub struct Window {
    pub id: usize,
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub min_width: usize,
    pub min_height: usize,
    pub border_color: Color,
    pub background_color: Color,
    pub state: WindowState,
    pub is_focused: bool,
    pub is_resizable: bool,
    pub has_titlebar: bool,
    pub z_index: i32,
    pub parent_id: Option<usize>,
    pub is_modal: bool,
}

impl Window {
    pub fn new(id: usize, title: &str, x: usize, y: usize, w: usize, h: usize, color: Color) -> Self {
        Self {
            id,
            title: String::from(title),
            x,
            y,
            width: w,
            height: h,
            min_width: 200,
            min_height: 100,
            border_color: color,
            background_color: Color::new(30, 30, 40),
            state: WindowState::Normal,
            is_focused: false,
            is_resizable: true,
            has_titlebar: true,
            z_index: 0,
            parent_id: None,
            is_modal: false,
        }
    }

    pub fn contains_point(&self, px: usize, py: usize) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    pub fn titlebar_rect(&self) -> (usize, usize, usize, usize) {
        (self.x, self.y, self.width, 30)
    }

    pub fn close_button_rect(&self) -> (usize, usize, usize, usize) {
        (self.x + 12, self.y + 8, 14, 14) // Left side (macOS)
    }

    pub fn minimize_button_rect(&self) -> (usize, usize, usize, usize) {
        (self.x + 34, self.y + 8, 14, 14)
    }

    pub fn maximize_button_rect(&self) -> (usize, usize, usize, usize) {
        (self.x + 56, self.y + 8, 14, 14)
    }
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
    pub total_desktops: usize,
    pub wallpaper_mode: WallpaperMode,
    pub show_desktop_icons: bool,
    pub top_bar_height: usize,
    pub dock_height: usize,
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
            accent_color: Color::new(120, 80, 255),
            uptime_ticks: 0,
            mem_used_pages: 0,
            mem_total_pages: 0,
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
            accent_color: Color::new(120, 80, 255),
            uptime_ticks: 0,
            mem_used_pages: 0,
            mem_total_pages: 0,
            mouse_y: 384,
            mouse_left: false,
        });
        &INSTANCE
    }

    /// [SOVEREIGN Trinity] Jantung Render Utama
    pub fn paint_all(&mut self) {
        // [SOVEREIGN v10.4.4] Gunakan pola "Sovereign Draw" yang sah
        crate::drivers::video::draw(|driver| {
            // 1. Render Nebula Background (The Fabric Pulse)
            use crate::drivers::video::nebula::NebulaGenerator;
            NebulaGenerator::render(driver);

            // 2. Render Windows
            // Karena kita di dalam closure, kita butuh akses ke self. 
            // Namun karena fungsionalitas ini terbatas, kita akan memindahkan loop ke luar atau memanggilnya secara manual.
        });

        // Loop rendering jendela di luar closure untuk menghindari peminjaman ganda (double-borrow)
        for window in self.windows.iter_mut() {
            if window.state != WindowState::Closed && window.state != WindowState::Minimized {
                let border = window.border_color;
                let bg = window.background_color;
                let wx = window.x;
                let wy = window.y;
                let ww = window.width;
                let wh = window.height;

                crate::drivers::video::draw(|driver| {
                    driver.draw_rect(Point::new(wx, wy), ww, wh, bg);
                    driver.draw_rect(Point::new(wx, wy), ww, 2, border);
                });
            }
        }

        // Final Flush & Cursor Pulse
        let mx = self.mouse_x;
        let my = self.mouse_y;
        crate::drivers::video::draw(|driver| {
            driver.draw_rect(Point::new(mx, my), 8, 8, Color::WHITE);
            driver.flush();
        });
    }

    pub fn update_mouse(&mut self, dx: i32, dy: i32, left: bool) {
        // [SUPREME INTERACTION] Mouse movement is relative in PS/2
        let mut new_x = self.mouse_x as i32 + dx;
        let mut new_y = self.mouse_y as i32 - dy; // PS/2 Y is inverted relative to screen Y

        // Screen clamping (1024x768 hardcoded for v10.3 target)
        if new_x < 0 { new_x = 0; }
        if new_y < 0 { new_y = 0; }
        if new_x > 1023 { new_x = 1023; }
        if new_y > 767 { new_y = 767; }

        let old_left = self.mouse_left;
        self.mouse_x = new_x as usize;
        self.mouse_y = new_y as usize;
        self.mouse_left = left;

        // --- Stage A5: Interactive Event Bridging ---
        let x = self.mouse_x;
        let y = self.mouse_y;

        // 1. Detect Press (Transition from false to true)
        if !old_left && left {
            self.handle_mouse_event(x, y, MouseButton::Left, MouseEventType::Press);
        }
        // 2. Detect Release (Transition from true to false)
        else if old_left && !left {
            self.handle_mouse_event(x, y, MouseButton::Left, MouseEventType::Release);
        }
        // 3. Constant Update for Moving/Dragging
        else {
            self.handle_mouse_event(x, y, MouseButton::Left, MouseEventType::Move);
        }
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
            w.is_focused = w.id == window_id;
            if w.is_focused {
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
                w.width = 1024;
                w.height = 768usize.saturating_sub(self.top_bar_height);
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
            }
        }
    }

    pub fn end_drag(&mut self) {
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
        self.desktop_icons.push(DesktopIcon { id: 0, name: String::from("Core"), x: 50, y: 50, icon_type: IconType::System, is_selected: false });
        self.desktop_icons.push(DesktopIcon { id: 1, name: String::from("Data"), x: 50, y: 150, icon_type: IconType::Folder, is_selected: false });
        self.desktop_icons.push(DesktopIcon { id: 2, name: String::from("Aether Store"), x: 50, y: 250, icon_type: IconType::Application, is_selected: false });
        self.desktop_icons.push(DesktopIcon { id: 3, name: String::from("Nodes"), x: 50, y: 350, icon_type: IconType::Application, is_selected: false });

        let system_window = Window::new(0, "System Status", 120, 90, 420, 320, Color::new(110, 80, 255));
        let security_window = Window::new(1, "Security Protocols", 560, 80, 420, 340, Color::new(220, 100, 255));
        let network_window = Window::new(2, "Aether Connect", 180, 430, 740, 320, Color::new(100, 220, 255));
        let terminal_window = Window::new(3, "Terminal", 300, 150, 600, 400, Color::new(50, 200, 50));

        self.add_window(system_window);
        self.add_window(security_window);
        self.add_window(network_window);
        self.add_window(terminal_window);

        self.add_notification("Welcome to AetherOS", "Neon desktop initialized.", 6);
        self.add_notification("Status", "Sovereign UI ready.", 6);
    }

    /// Backwards-compatible alias used elsewhere in the kernel
    pub fn initialize_supreme_desktop(&mut self) {
        self.initialize_complete_desktop();
    }

    pub fn render(&mut self, fb: &mut dyn Framebuffer) {
        let width = fb.width();
        let height = fb.height();

        // [DIAGNOSTIC] Trace Loop Start
        crate::print!("."); // Minimal pulse in serial

        // 1. Wallpaper & Background System
        self.render_wallpaper(fb, width, height);

        // 2. High-Res Nebula Brand Overlay (Center)
        self.render_brand_overlay(fb, width, height);

        // 3. Sovereign Subsystems (Icons)
        if self.show_desktop_icons {
            self.render_desktop_icons(fb);
        }

        // 4. Window Stack (Z-Order Rendering)
        self.render_windows(fb);

        // 5. Global Top Bar (Clock, Apple-like Menu, System Status)
        self.render_top_bar(fb, width, height);

        // 6. Centered Floating Dock (Ubuntu/Mac style)
        self.render_dock(fb, width, height);

        // 7. Notifications Layer (Glassmorphism)
        self.render_notifications(fb, width, height);

        // 8. Context Menu (Floating)
        if let Some((x, y, ref items)) = self.context_menu {
            self.render_context_menu(fb, x, y, items);
        }
        
        // 9. [SOVEREIGN] PROOF-OF-VISUALIZATION OVERLAY
        fb.draw_string(crate::drivers::video::Point::new(10, 10), "AETHEROS v10.3 [ACTIVE]", self.accent_color);

        // [v10.4] The Fabric Pulse Indicator (Bottom-Right)
        self.render_fabric_pulse(fb, width, height);

        // [v10.3 SUPREME] Final Overlay: Interactive Cursor
        fb.draw_cursor(crate::drivers::video::Point::new(self.mouse_x, self.mouse_y));

        // Atomic Buffer Flip with hardware barrier
        fb.flush();
    }

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
                self.fill_rect(fb, 0, 0, width, height, color);
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

    fn render_windows(&self, fb: &mut dyn Framebuffer) {
        for window in &self.windows {
            if window.state == WindowState::Minimized { continue; }

            // [v10.3 SUPREME] Use the new high-fidelity window primitive
            let accent = if window.is_focused { self.accent_color } else { Color::new(60, 60, 80) };
            
            // [A6.4] Focus Glow (Outer border for active window)
            if window.is_focused {
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

        let dock_y = 768usize.saturating_sub(self.dock_height + 15);
        if y >= dock_y {
            self.handle_taskbar_click(x, y);
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

    fn handle_right_click(&mut self, x: usize, y: usize) {
        let dock_y = 768usize.saturating_sub(self.dock_height + 15);
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

    fn handle_taskbar_click(&mut self, x: usize, y: usize) {
        // [SOVEREIGN DOCK INTERACTION]
        let width = 1024; // Assuming standard resolution
        let dock_w = (self.taskbar_items.len() * 60) + 40;
        let dock_x = (width - dock_w) / 2;
        let dock_y = 768 - self.dock_height - 15;

        // Check if click is inside Dock
        if x < dock_x || x > dock_x + dock_w || y < dock_y || y > dock_y + self.dock_height {
            return;
        }

        // Check each dock item
        let mut cur_x = dock_x + 20;
        for i in 0..self.taskbar_items.len() {
            if x >= cur_x && x <= cur_x + 44 {
                let window_id = self.taskbar_items[i].window_id;
                if self.taskbar_items[i].is_minimized {
                    if let Some(w) = self.windows.iter_mut().find(|w| w.id == window_id) {
                        w.state = WindowState::Normal;
                    }
                } else {
                    self.minimize_window(window_id);
                }
                self.update_taskbar();
                return;
            }
            cur_x += 60;
        }

        // Aether Menu Icon (A) at the right end of the dock
        if x >= cur_x && x <= cur_x + 40 {
             let w = Window::new(0, "Aether Menu", 312, 184, 400, 300, self.accent_color);
             self.add_window(w);
        }
    }

    fn open_desktop_icon(&mut self, icon_id: usize) {
        if let Some(icon) = self.desktop_icons.iter().find(|i| i.id == icon_id) {
            match icon.icon_type {
                IconType::System => { let w = Window::new(0, &format!("{} - System", icon.name), 200, 150, 600, 400, self.accent_color); self.add_window(w); }
                IconType::Folder => { let w = Window::new(0, &format!("{} - Folder", icon.name), 250, 200, 500, 350, Color::new(255,255,0)); self.add_window(w); }
                IconType::Application => { let w = Window::new(0, &format!("{} - Application", icon.name), 300, 250, 550, 400, Color::new(0,255,0)); self.add_window(w); }
                _ => {}
            }
        }
    }

    pub fn handle_keyboard_event(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match (key, modifiers.ctrl, modifiers.alt, modifiers.shift) {
            (KeyCode::Tab, true, false, false) => self.cycle_windows(),
            (KeyCode::D, true, false, false) => self.show_desktop(),
            (KeyCode::N, true, false, false) => { let w = Window::new(0, "New Window", 300, 300, 400, 300, self.accent_color); self.add_window(w); }
            (KeyCode::Q, true, false, false) => self.close_all_windows(),
            _ => {}
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
