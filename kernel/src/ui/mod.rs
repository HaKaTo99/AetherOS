//! User Interface Framework
//!
//! A lightweight UI framework for AetherOS featuring a widget system and flexible layout engine.
//!
//! # Components
//!
//! - **Display**: Framebuffer abstraction and distributed rendering
//! - **Widget**: Core UI components (Label, Button, Panel, TextBox)
//! - **Layout**: FlexBox-inspired layout engine for responsive UIs
//!
//! # Example
//!
//! ```no_run
//! use aetheros_kernel::ui::{Label, Button, FlexLayout, Rect};
//!
//! // Create a simple UI scene
//! let mut label = Label::new(Rect::new(10, 10, 200, 30), "Hello AetherOS");
//! let mut button = Button::new(Rect::new(10, 50, 100, 30), "Click Me");
//!
//! // Use FlexLayout for responsive layout
//! let layout = FlexLayout::new(Rect::new(0, 0, 640, 480));
//! layout.row(); // Horizontal layout
//! ```
//!
//! # Design Philosophy
//!
//! The UI framework is designed to be minimal yet functional, prioritizing:
//! - **Simplicity**: Easy to understand widget system
//! - **Flexibility**: Layout engine adapts to different screen sizes
//! - **Performance**: Lightweight rendering suitable for embedded systems

pub mod display;
pub mod widget; // [NEW] Widget system
pub mod layout; // [NEW] Layout engine
pub mod window; // [NEW] Window Manager (Phase 13.1)
pub mod components; // [NEW] Menu, FilePicker, Notifications (Phase 13.1)
pub mod toolkit; // [NEW] App UI Toolkit (Phase 14.2)
pub mod file_manager; // [NEW] Secure FileManager (Phase 20.2)
pub mod organic_ui; // [NEW] Organic UI Drivers (Phase 25.4)
pub mod dashboard; // [NEW] Fleet Monitor Dashboard (Phase 26.3)
pub mod desktop; // [NEW] AetherDesktop Seed (Tahap III)
pub mod splash; // [NEW] Supreme Splash (v10.2)

pub use display::*;
pub use layout::*;
pub use window::{Window, WindowManager, WINDOW_MANAGER};

pub use display::{DistributedFramebuffer, UIUpdate, PixelFormat, VectorRenderer};
pub use widget::{Widget, Rect, Label, Button};
pub use layout::FlexLayout;
