//! Aether Package Manager (APM) - v5.4
//! 
//! Handles .apkg package installation, dependency resolution, and removal.
//! 
//! Format: .apkg (Tarball + manifest.json + signature)

use alloc::string::String;
use alloc::vec::Vec;

pub struct Package {
    pub name: String,
    pub version: String,
    pub installed: bool,
}

pub struct Apm {
    registry: Vec<Package>,
}

impl Apm {
    pub fn new() -> Self {
        Apm {
            registry: Vec::new(),
        }
    }

    pub fn install(&mut self, package_name: &str) -> Result<String, &'static str> {
        // [SIMULATION]
        self.registry.push(Package {
            name: String::from(package_name),
            version: String::from("1.0.0"),
            installed: true,
        });
        Ok(format!("Successfully installed {} v1.0.0", package_name))
    }

    pub fn list_installed(&self) -> usize {
        self.registry.len()
    }
}

pub static GLOBAL_APM: spin::Mutex<Apm> = spin::Mutex::new(Apm { registry: Vec::new() });
