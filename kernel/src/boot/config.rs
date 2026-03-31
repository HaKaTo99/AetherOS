//! Boot Configuration Parser
//! 
//! Supports boot parameters like:
//! - toram: Load entire system to RAM
//! - toram=trim: Load only essential modules  
//! - load=module1,module2: Specific modules to load
//! - noload=module1,module2: Modules to skip

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Boot mode options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootMode {
    /// Boot directly from media (normal)
    Direct,
    /// Load entire system to RAM (portable mode)
    Toram,
    /// Load partial to RAM, rest from media
    Hybrid,
}

impl Default for BootMode {
    fn default() -> Self {
        BootMode::Direct
    }
}

/// Boot configuration structure
#[derive(Debug, Clone)]
pub struct BootConfig {
    /// Current boot mode
    pub mode: BootMode,
    /// Modules to explicitly load
    pub load_modules: Vec<String>,
    /// Modules to skip
    pub noload_modules: Vec<String>,
    /// Use minimal modules (toram=trim)
    pub trim: bool,
    /// Verbose boot logging
    pub verbose: bool,
    /// Disable security features (for testing)
    pub insecure: bool,
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            mode: BootMode::Direct,
            load_modules: Vec::new(),
            noload_modules: Vec::new(),
            trim: false,
            verbose: false,
            insecure: false,
        }
    }
}

impl BootConfig {
    /// Parse command line string (from GRUB, QEMU -append, etc.)
    pub fn parse(cmdline: &str) -> Self {
        let mut config = BootConfig::default();

        for part in cmdline.split_whitespace() {
            // Basic modes
            if part == "toram" {
                config.mode = BootMode::Toram;
            } else if part == "hybrid" {
                config.mode = BootMode::Hybrid;
            } else if part.starts_with("toram=") {
                config.mode = BootMode::Toram;
                let val = &part[6..];
                if val == "trim" {
                    config.trim = true;
                }
            } else if part.starts_with("load=") {
                let val = &part[5..];
                for module in val.split(',') {
                    if !module.is_empty() {
                        config.load_modules.push(module.to_string());
                    }
                }
            } else if part.starts_with("noload=") {
                let val = &part[7..];
                for module in val.split(',') {
                    if !module.is_empty() {
                        config.noload_modules.push(module.to_string());
                    }
                }
            } else if part == "verbose" || part == "v" {
                config.verbose = true;
            } else if part == "insecure" {
                config.insecure = true;
            }
            // Unknown parameter, ignore
        }

        config
    }

    /// Check if a specific module should be loaded
    pub fn should_load(&self, module_name: &str) -> bool {
        // If explicit load list exists, only load those modules
        if !self.load_modules.is_empty() {
            return self.load_modules.iter().any(|m| m == module_name);
        }
        // If noload list exists, skip those modules
        if !self.noload_modules.is_empty() {
            return !self.noload_modules.iter().any(|m| m == module_name);
        }
        // Default: load all
        true
    }

    /// Get the boot mode as a string
    pub fn mode_str(&self) -> &'static str {
        match self.mode {
            BootMode::Direct => "direct",
            BootMode::Toram => "toram",
            BootMode::Hybrid => "hybrid",
        }
    }
}
