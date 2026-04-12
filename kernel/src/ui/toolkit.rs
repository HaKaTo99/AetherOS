//! UI Toolkit for Third-Party Apps (Phase 14.2)
//! Simplified widget API for app developers

use crate::ui::widget::Rect;
use crate::ui::window::{Window, WINDOW_MANAGER};
use alloc::string::String;
use alloc::vec::Vec;

/// App-facing widget types
#[derive(Debug, Clone)]
pub enum AppWidget {
    Label { text: String, rect: Rect },
    Button { text: String, rect: Rect, on_click_id: usize },
    TextInput { text: String, rect: Rect, placeholder: String },
    Image { path: String, rect: Rect },
    List { items: Vec<String>, rect: Rect, selected: Option<usize> },
    Slider { min: i32, max: i32, value: i32, rect: Rect },
    Checkbox { label: String, checked: bool, rect: Rect },
}

/// App UI builder (fluent API)
pub struct AppUI {
    pub window_title: &'static str,
    pub widgets: Vec<AppWidget>,
    pub width: usize,
    pub height: usize,
}

impl AppUI {
    pub fn new(title: &'static str, width: usize, height: usize) -> Self {
        Self {
            window_title: title,
            widgets: Vec::new(),
            width,
            height,
        }
    }

    pub fn label(mut self, text: &str, x: usize, y: usize, w: usize, h: usize) -> Self {
        self.widgets.push(AppWidget::Label {
            text: String::from(text),
            rect: Rect::new(x, y, w, h),
        });
        self
    }

    pub fn button(mut self, text: &str, x: usize, y: usize, w: usize, h: usize, id: usize) -> Self {
        self.widgets.push(AppWidget::Button {
            text: String::from(text),
            rect: Rect::new(x, y, w, h),
            on_click_id: id,
        });
        self
    }

    pub fn text_input(mut self, placeholder: &str, x: usize, y: usize, w: usize, h: usize) -> Self {
        self.widgets.push(AppWidget::TextInput {
            text: String::new(),
            rect: Rect::new(x, y, w, h),
            placeholder: String::from(placeholder),
        });
        self
    }

    pub fn checkbox(mut self, label: &str, checked: bool, x: usize, y: usize) -> Self {
        self.widgets.push(AppWidget::Checkbox {
            label: String::from(label),
            checked,
            rect: Rect::new(x, y, 200, 24),
        });
        self
    }

    /// Build and register the window
    pub fn build(self) -> usize {
        let mut wm = WINDOW_MANAGER.lock();
        let w = Window::new(
            0,
            self.window_title,
            50, 50, self.width, self.height,
            crate::drivers::video::Color::new(150, 150, 180)
        );
        wm.add_window(w)
    }
}
