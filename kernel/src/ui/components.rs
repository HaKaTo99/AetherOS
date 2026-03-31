//! UI Components (Phase 13.1)
//! Menu system, file picker, notification system

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

// ===========================
// Menu System
// ===========================

/// Menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub id: usize,
    pub enabled: bool,
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(label: &str, id: usize) -> Self {
        Self {
            label: String::from(label),
            id,
            enabled: true,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, item: MenuItem) {
        self.children.push(item);
    }
}

/// Context menu
pub struct ContextMenu {
    pub items: Vec<MenuItem>,
    pub visible: bool,
    pub position: (i32, i32),
}

impl ContextMenu {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            visible: false,
            position: (0, 0),
        }
    }

    pub fn show(&mut self, x: i32, y: i32) {
        self.position = (x, y);
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }
}

/// Menu bar (top of window)
pub struct MenuBar {
    pub menus: Vec<(String, ContextMenu)>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self { menus: Vec::new() }
    }

    pub fn add_menu(&mut self, label: &str, menu: ContextMenu) {
        self.menus.push((String::from(label), menu));
    }
}

// ===========================
// File Picker
// ===========================

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: usize,
}

/// File picker dialog
pub struct FilePicker {
    pub current_path: String,
    pub entries: Vec<FileEntry>,
    pub selected: Option<usize>,
    pub visible: bool,
}

impl FilePicker {
    pub fn new() -> Self {
        Self {
            current_path: String::from("/"),
            entries: Vec::new(),
            selected: None,
            visible: false,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn select(&mut self, index: usize) {
        if index < self.entries.len() {
            self.selected = Some(index);
        }
    }

    pub fn get_selected(&self) -> Option<&FileEntry> {
        self.selected.and_then(|i| self.entries.get(i))
    }
}

// ===========================
// Notification System
// ===========================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub level: NotificationLevel,
    pub ttl_ms: u64, // Time to live
}

pub struct NotificationManager {
    notifications: Vec<Notification>,
    next_id: usize,
}

impl NotificationManager {
    pub const fn new() -> Self {
        Self {
            notifications: Vec::new(),
            next_id: 1,
        }
    }

    pub fn notify(&mut self, title: &str, msg: &str, level: NotificationLevel) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.notifications.push(Notification {
            id,
            title: String::from(title),
            message: String::from(msg),
            level,
            ttl_ms: 5000,
        });
        id
    }

    pub fn dismiss(&mut self, id: usize) {
        self.notifications.retain(|n| n.id != id);
    }

    pub fn active(&self) -> &[Notification] {
        &self.notifications
    }
}

pub static NOTIFICATION_MANAGER: Mutex<NotificationManager> = Mutex::new(NotificationManager::new());
