//! USB Subsystem (Phase 11)
//! 
//! Handles USB Host Controllers (xHCI/EHCI) and device enumeration.

pub mod xhci;

pub use xhci::XhciController;
