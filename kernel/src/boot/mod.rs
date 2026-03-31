//! Boot Configuration Module
//! 
//! Supports boot parameters like:
//! - toram: Load entire system to RAM
//! - toram=trim: Load only essential modules  
//! - load=module1,module2: Specific modules to load
//! - noload=module1,module2: Modules to skip

#![allow(dead_code)]

pub mod config;
pub use config::{BootConfig, BootMode};
pub mod cmdline;
