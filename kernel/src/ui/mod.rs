//! UI Subsystem - Sovereing Isolation Mode [SDE v2.0]
//! Complete desktop environment like Ubuntu, Windows, and macOS

pub mod desktop;      // [SDE v2.0] Complete Desktop Manager
pub mod window;       // Window management with z-ordering
pub mod components;   // UI components (buttons, text fields, etc.)
pub mod dashboard;    // System dashboard
pub mod display;      // Vector rendering and GPU orchestration
pub mod file_manager; // File browser integration
pub mod layout;       // Layout managers
pub mod organic_ui;   // Advanced visual effects
pub mod splash;       // Splash screens
pub mod store;        // Application store UI
pub mod toolkit;      // UI toolkit utilities
pub mod widget;       // Core widgets
pub mod input;        // Input management for desktop
pub mod terminal;     // [v10.3] Sovereign Terminal Buffer

// Re-export core APIs for ergonomic access and compatibility
pub use widget::Rect;
pub use window::WINDOW_MANAGER;
pub use desktop::{DesktopManager, Window, MouseEventType, MouseButton};
pub use input::{InputManager, init as init_input, get_manager as get_input_manager};
pub use terminal::TERMINAL_LOG;
