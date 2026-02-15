//! Window Manager (Phase 13.1)
//! Compositor for safe multi-window rendering

use crate::ui::Rect;
use alloc::vec::Vec;
use spin::Mutex;

/// Window ID type
pub type WindowId = usize;

/// Window structure
#[derive(Clone)]
pub struct Window {
    pub id: WindowId,
    pub rect: Rect,
    pub z_order: i32,
    pub visible: bool,
    pub title: &'static str,
}

impl Window {
    pub fn new(id: WindowId, rect: Rect, title: &'static str) -> Self {
        Self {
            id,
            rect,
            z_order: 0,
            visible: true,
            title,
        }
    }
}

/// Window Manager
pub struct WindowManager {
    windows: Vec<Window>,
    next_id: WindowId,
}

impl WindowManager {
    pub const fn new() -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
        }
    }

    /// Create a new window
    pub fn create_window(&mut self, rect: Rect, title: &'static str) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;

        let window = Window::new(id, rect, title);
        self.windows.push(window);
        id
    }

    /// Get window by ID
    pub fn get_window(&self, id: WindowId) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }

    /// Get mutable window by ID
    pub fn get_window_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    /// Set window z-order
    pub fn set_z_order(&mut self, id: WindowId, z: i32) {
        if let Some(window) = self.get_window_mut(id) {
            window.z_order = z;
        }
        // Sort by z-order
        self.windows.sort_by_key(|w| w.z_order);
    }

    /// Get all visible windows sorted by z-order
    pub fn visible_windows(&self) -> impl Iterator<Item = &Window> {
        self.windows.iter().filter(|w| w.visible)
    }

    /// Close window
    pub fn close_window(&mut self, id: WindowId) {
        self.windows.retain(|w| w.id != id);
    }

    /// Get window count
    pub fn count(&self) -> usize {
        self.windows.len()
    }
}

/// Global window manager
pub static WINDOW_MANAGER: Mutex<WindowManager> = Mutex::new(WindowManager::new());
