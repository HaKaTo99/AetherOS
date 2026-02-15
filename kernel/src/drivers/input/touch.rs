//! Touch Gesture Recognition (Phase 13.2)

/// Touch point
#[derive(Debug, Clone, Copy)]
pub struct TouchPoint {
    pub id: u8,
    pub x: i16,
    pub y: i16,
    pub pressure: u8,
    pub active: bool,
}

/// Gesture types
#[derive(Debug, Clone, Copy)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Swipe(SwipeDirection),
    Pinch(i16), // delta pixels
    Rotate(i16), // delta degrees
}

#[derive(Debug, Clone, Copy)]
pub enum SwipeDirection { Up, Down, Left, Right }

/// Multi-touch handler (up to 10 simultaneous touches)
pub struct TouchHandler {
    points: [TouchPoint; 10],
    active_count: usize,
}

impl TouchHandler {
    pub const fn new() -> Self {
        Self {
            points: [TouchPoint { id: 0, x: 0, y: 0, pressure: 0, active: false }; 10],
            active_count: 0,
        }
    }

    pub fn update(&mut self, id: u8, x: i16, y: i16, pressure: u8) {
        if let Some(point) = self.points.get_mut(id as usize) {
            point.id = id;
            point.x = x;
            point.y = y;
            point.pressure = pressure;
            point.active = pressure > 0;
        }
        self.active_count = self.points.iter().filter(|p| p.active).count();
    }

    pub fn detect_gesture(&self) -> Option<GestureType> {
        match self.active_count {
            1 => Some(GestureType::Tap),
            2 => Some(GestureType::Pinch(0)),
            _ => None,
        }
    }
}

/// Input Method Editor (IME) for international text
pub struct InputMethodEditor {
    pub composing: alloc::string::String,
    pub committed: alloc::string::String,
    pub language: &'static str,
}

impl InputMethodEditor {
    pub fn new(language: &'static str) -> Self {
        Self {
            composing: alloc::string::String::new(),
            committed: alloc::string::String::new(),
            language,
        }
    }

    pub fn input(&mut self, ch: char) { self.composing.push(ch); }

    pub fn commit(&mut self) {
        self.committed.push_str(&self.composing);
        self.composing.clear();
    }
}
