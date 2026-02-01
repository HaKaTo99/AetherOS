//! UI Layout Engine
//! Simple Flex-like layout system

use crate::ui::widget::Rect;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

/// Flex Layout Engine
pub struct FlexLayout {
    pub direction: Direction,
    pub justify_content: Alignment,
    pub align_items: Alignment,
    pub padding: usize,
    pub gap: usize,
}

impl FlexLayout {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            justify_content: Alignment::Start,
            align_items: Alignment::Start,
            padding: 0,
            gap: 0,
        }
    }

    /// Calculate layout for a list of items within a container area
    /// Returns a list of Rects corresponding to each item
    pub fn layout(&self, container: Rect, items: &[Rect]) -> Vec<Rect> {
        let mut result = Vec::new();
        let mut current_pos = match self.direction {
            Direction::Row => container.x + self.padding,
            Direction::Column => container.y + self.padding,
        };

        for item in items {
            let (x, y, w, h) = match self.direction {
                Direction::Row => {
                    // In Row: x increases, y is determined by alignment
                    let item_x = current_pos;
                    let item_y = match self.align_items {
                        Alignment::Start => container.y + self.padding,
                        Alignment::Center => container.y + (container.height - item.height) / 2,
                        Alignment::End => container.y + container.height - item.height - self.padding,
                    };
                    current_pos += item.width + self.gap;
                    (item_x, item_y, item.width, item.height)
                },
                Direction::Column => {
                    // In Column: y increases, x is determined by alignment
                    let item_y = current_pos;
                    let item_x = match self.align_items {
                        Alignment::Start => container.x + self.padding,
                        Alignment::Center => container.x + (container.width - item.width) / 2,
                        Alignment::End => container.x + container.width - item.width - self.padding,
                    };
                    current_pos += item.height + self.gap;
                    (item_x, item_y, item.width, item.height)
                }
            };
            
            result.push(Rect::new(x, y, w, h));
        }

        result
    }
}
