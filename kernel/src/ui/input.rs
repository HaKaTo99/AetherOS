//! Input Management for Complete Desktop Environment
//! Handles mouse, keyboard, and touch events like Ubuntu/Windows/Mac

// Define input event types since they may not exist in drivers::input
#[derive(Clone, Copy)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub button: MouseButton,
    pub pressed: bool,
    pub dx: i32,
    pub dy: i32,
}

#[derive(Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy)]
pub enum KeyCode {
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    LeftShift,
    RightShift,
    Tab,
    D,
    N,
    Q,
    F11,
    Escape,
    Char(char),
    Backspace,
    Enter,
}

#[derive(Clone)]
pub struct KeyboardEvent {
    pub keycode: KeyCode,
    pub pressed: bool,
}

use crate::ui::desktop::{DesktopManager, MouseButton as DesktopMouseButton, MouseEventType, KeyCode as DesktopKeyCode, KeyModifiers};

/// Global input state
pub struct InputManager {
    pub desktop: DesktopManager,
    pub mouse_x: usize,
    pub mouse_y: usize,
    pub mouse_buttons: [bool; 3], // Left, Right, Middle
    pub keyboard_modifiers: KeyModifiers,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            desktop: DesktopManager::new(),
            mouse_x: 0,
            mouse_y: 0,
            mouse_buttons: [false; 3],
            keyboard_modifiers: KeyModifiers {
                ctrl: false,
                alt: false,
                shift: false,
            },
        }
    }

    /// Initialize complete desktop environment
    pub fn initialize(&mut self) {
        self.desktop.initialize_complete_desktop();
    }

    /// Handle mouse input
    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        self.mouse_x = event.x as usize;
        self.mouse_y = event.y as usize;

        match event.button {
            MouseButton::Left => {
                if event.pressed && !self.mouse_buttons[0] {
                    self.desktop.handle_mouse_event(self.mouse_x, self.mouse_y, DesktopMouseButton::Left, MouseEventType::Press);
                } else if !event.pressed && self.mouse_buttons[0] {
                    self.desktop.handle_mouse_event(self.mouse_x, self.mouse_y, DesktopMouseButton::Left, MouseEventType::Release);
                }
                self.mouse_buttons[0] = event.pressed;
            }
            MouseButton::Right => {
                if event.pressed && !self.mouse_buttons[1] {
                    self.desktop.handle_mouse_event(self.mouse_x, self.mouse_y, DesktopMouseButton::Right, MouseEventType::Press);
                }
                self.mouse_buttons[1] = event.pressed;
            }
            MouseButton::Middle => {
                if event.pressed && !self.mouse_buttons[2] {
                    self.desktop.handle_mouse_event(self.mouse_x, self.mouse_y, DesktopMouseButton::Middle, MouseEventType::Press);
                }
                self.mouse_buttons[2] = event.pressed;
            }
        }

        // Handle mouse movement
        if event.dx != 0 || event.dy != 0 {
            self.desktop.handle_mouse_event(self.mouse_x, self.mouse_y, DesktopMouseButton::Left, MouseEventType::Move);
        }
    }

    /// Handle keyboard input
    pub fn handle_keyboard_event(&mut self, event: KeyboardEvent) {
        // Update modifiers
        match event.keycode {
            KeyCode::LeftCtrl | KeyCode::RightCtrl => {
                self.keyboard_modifiers.ctrl = event.pressed;
            }
            KeyCode::LeftAlt | KeyCode::RightAlt => {
                self.keyboard_modifiers.alt = event.pressed;
            }
            KeyCode::LeftShift | KeyCode::RightShift => {
                self.keyboard_modifiers.shift = event.pressed;
            }
            _ => {}
        }

        // Convert to desktop key codes
        if event.pressed {
            let desktop_key = match event.keycode {
                KeyCode::Tab => DesktopKeyCode::Tab,
                KeyCode::D => DesktopKeyCode::D,
                KeyCode::N => DesktopKeyCode::N,
                KeyCode::Q => DesktopKeyCode::Q,
                KeyCode::F11 => DesktopKeyCode::F11,
                KeyCode::Escape => DesktopKeyCode::Escape,
                KeyCode::Char(c) => DesktopKeyCode::Char(c),
                KeyCode::Backspace => DesktopKeyCode::Backspace,
                KeyCode::Enter => DesktopKeyCode::Enter,
                _ => return, // Ignore unsupported keys
            };

            self.desktop.handle_keyboard_event(desktop_key, self.keyboard_modifiers);
        }
    }

    /// Render the desktop
    pub fn render(&mut self, fb: &mut dyn crate::drivers::video::Framebuffer) {
        self.desktop.render(fb);
    }

    /// Update desktop state (notifications, etc.)
    pub fn update(&mut self) {
        // Clean up expired notifications
        self.desktop.notifications.retain(|_n| {
            // Simplified: assume notifications don't expire in this demo
            true
        });
    }
}

/// Global input manager instance
pub static mut INPUT_MANAGER: Option<InputManager> = None;

/// Initialize the input system
pub fn init() {
    unsafe {
        INPUT_MANAGER = Some(InputManager::new());
        if let Some(manager) = &mut INPUT_MANAGER {
            manager.initialize();
        }
    }
}

/// Get mutable reference to input manager
pub fn get_manager() -> &'static mut InputManager {
    unsafe {
        INPUT_MANAGER.as_mut().unwrap()
    }
}