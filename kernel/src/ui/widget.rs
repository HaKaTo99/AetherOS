//! UI Widget System
//! Retained mode widget primitives

use crate::drivers::video::{Framebuffer, Point, Color};
use alloc::string::String;

/// Geometric Rectangle
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}

/// Base Trait for all UI Widgets
pub trait Widget {
    /// Draw the widget onto the framebuffer
    fn draw(&self, fb: &mut dyn Framebuffer);
    
    /// Get the area occupied by the widget
    fn area(&self) -> Rect;
}

/// Text Label Widget
pub struct Label {
    pub text: String,
    pub area: Rect,
    pub color: Color,
}

impl Label {
    pub fn new(text: &str, x: usize, y: usize, color: Color) -> Self {
        Self {
            text: String::from(text),
            area: Rect::new(x, y, text.len() * 8, 8), // Auto-calculate width based on 8x8 font
            color,
        }
    }
}

impl Widget for Label {
    fn draw(&self, fb: &mut dyn Framebuffer) {
        fb.draw_string(Point::new(self.area.x, self.area.y), &self.text, self.color);
    }
    
    fn area(&self) -> Rect {
        self.area
    }
}

/// Button Widget (Clickable)
pub struct Button {
    pub label: String,
    pub area: Rect,
    pub bg_color: Color,
    pub text_color: Color,
}

impl Button {
    pub fn new(label: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            label: String::from(label),
            area: Rect::new(x, y, width, height),
            bg_color: Color::new(100, 100, 100), // Gray
            text_color: Color::WHITE,
        }
    }
}

impl Widget for Button {
    fn draw(&self, fb: &mut dyn Framebuffer) {
        // Draw background
        fb.draw_rect(
            Point::new(self.area.x, self.area.y),
            self.area.width, 
            self.area.height, 
            self.bg_color
        );
        
        // Center text (Simple centering)
        let text_width = self.label.len() * 8;
        let text_x = if self.area.width > text_width {
            self.area.x + (self.area.width - text_width) / 2
        } else {
            self.area.x
        };
        
        let text_y = if self.area.height > 8 {
            self.area.y + (self.area.height - 8) / 2
        } else {
            self.area.y
        };
        
        fb.draw_string(Point::new(text_x, text_y), &self.label, self.text_color);
    }
    
    fn area(&self) -> Rect {
        self.area
    }
}
