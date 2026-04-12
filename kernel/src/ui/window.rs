//! Window Manager - v3.0 "Sovereign" (v10.4.16 SUPREME)
//! Complete compositor with Focus Management, Event Routing, and App Lifecycle.

use crate::drivers::video::{Color, Point};
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

/// Application Type for Window-Specific Routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppType {
    Terminal,
    FileManager,
    SystemStatus,
    Security,
    Store,
    Settings,
    Generic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Closed,
}

/// Window structure (Unified v3.0)
#[derive(Clone)]
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
    pub focused: bool,
    pub resizable: bool,
    pub has_titlebar: bool,
    pub z_index: i32,
    pub parent_id: Option<usize>,
    pub is_modal: bool,
    pub app_type: AppType,
    pub opacity: u8, // 0-255 for glassmorphism levels
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
            focused: false,
            resizable: true,
            has_titlebar: true,
            z_index: 0,
            parent_id: None,
            is_modal: false,
            app_type: AppType::Generic,
            opacity: 240, // Slightly translucent
        }
    }

    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.app_type = app_type;
        // Set default titles based on type if needed
        self
    }

    pub fn contains_point(&self, px: usize, py: usize) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    pub fn titlebar_rect(&self) -> (usize, usize, usize, usize) {
        (self.x, self.y, self.width, 34) // 34px standard Aether height
    }

    pub fn close_button_rect(&self) -> (usize, usize, usize, usize) {
        (self.x + 12, self.y + 10, 14, 14) // Left side (macOS/Sovereign style)
    }
}

/// Window Manager (Phase III Refined)
pub struct WindowManager {
    pub windows: Vec<Window>,
    next_id: usize,
    pub focused_id: Option<usize>,
}

impl WindowManager {
    pub const fn new() -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
            focused_id: None,
        }
    }

    pub fn add_window(&mut self, mut window: Window) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        window.id = id;
        window.z_index = self.windows.len() as i32;
        self.windows.push(window);
        self.focus_window(id);
        id
    }

    pub fn focus_window(&mut self, id: usize) {
        let mut highest_z = 0;
        for w in &self.windows {
            if w.z_index > highest_z { highest_z = w.z_index; }
        }

        for w in &mut self.windows {
            if w.id == id {
                w.z_index = highest_z + 1;
                w.focused = true;
                self.focused_id = Some(id);
            } else {
                w.focused = false;
            }
        }
        
        self.windows.sort_by_key(|w| w.z_index);
    }

    pub fn close_window(&mut self, id: usize) {
        if self.focused_id == Some(id) { self.focused_id = None; }
        self.windows.retain(|w| w.id != id);
    }
}

/// Global window manager
pub static WINDOW_MANAGER: Mutex<WindowManager> = Mutex::new(WindowManager::new());

