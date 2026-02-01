//! Input Subsystem Abstraction
//! Defines events and key codes for specific drivers to translate into.

pub mod ps2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    // Function Keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Special
    Escape, Enter, Space, Backspace, Tab, CapsLock,
    LShift, RShift, LCtrl, RCtrl, LAlt, RAlt,
    Up, Down, Left, Right,
    // Symbols
    Minus, Equal, LBracket, RBracket, Backslash, Semicolon, Quote, Comma, Period, Slash,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Keyboard { key: KeyCode, state: KeyState },
    Mouse { dx: i32, dy: i32, buttons: u8 },
    Touch { x: u32, y: u32, id: u8, pressed: bool },
}

/// Trait for input drivers
pub trait InputDriver {
    /// Poll for pending events
    fn poll(&mut self) -> Option<InputEvent>;
}
