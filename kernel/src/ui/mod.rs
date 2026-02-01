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

pub use display::{DistributedFramebuffer, UIUpdate, PixelFormat, VectorRenderer};
pub use widget::{Widget, Rect, Label, Button};
pub use layout::FlexLayout;
