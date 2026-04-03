//! Window Manager - v2.0 "Organic" (v10.3 SUPREME)
//! Advanced compositor with Focus Management and Event Routing.

use crate::ui::Rect;
use alloc::vec::Vec;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};

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
    pub focused: bool,
}

impl Window {
    pub fn new(id: WindowId, rect: Rect, title: &'static str) -> Self {
        Self {
            id,
            rect,
            z_order: 0,
            visible: true,
            title,
            focused: false,
        }
    }
}

/// Window Manager
pub struct WindowManager {
    windows: Vec<Window>,
    next_id: WindowId,
    focused_id: Option<WindowId>,
}

impl WindowManager {
    pub const fn new() -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
            focused_id: None,
        }
    }

    /// Create a new window and bring to front
    pub fn create_window(&mut self, rect: Rect, title: &'static str) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;

        let mut window = Window::new(id, rect, title);
        window.z_order = self.windows.len() as i32;
        self.windows.push(window);
        
        self.focus_window(id);
        id
    }

    /// Bring window to top of stack and give focus
    pub fn focus_window(&mut self, id: WindowId) {
        let mut highest_z = 0;
        for w in &self.windows {
            if w.z_order > highest_z { highest_z = w.z_order; }
        }

        for w in &mut self.windows {
            if w.id == id {
                w.z_order = highest_z + 1;
                w.focused = true;
                self.focused_id = Some(id);
            } else {
                w.focused = false;
            }
        }
        
        // Dynamic Re-sorting for Compositor
        self.windows.sort_by_key(|w| w.z_order);
    }

    /// Get current focused window
    pub fn get_focused_window(&self) -> Option<&Window> {
        self.focused_id.and_then(|id| self.windows.iter().find(|w| w.id == id))
    }

    pub fn get_window_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    /// Optimized: Get all visible windows sorted by z-order
    pub fn visible_windows(&self) -> impl Iterator<Item = &Window> {
        self.windows.iter().filter(|w| w.visible)
    }

    pub fn close_window(&mut self, id: WindowId) {
        if self.focused_id == Some(id) { self.focused_id = None; }
        self.windows.retain(|w| w.id != id);
    }

    pub fn count(&self) -> usize { self.windows.len() }
}

/// Global window manager
pub static WINDOW_MANAGER: Mutex<WindowManager> = Mutex::new(WindowManager::new());
