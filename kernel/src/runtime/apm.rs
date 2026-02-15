//! AetherOS Package Manager (apm)
//! Phase 14.1: Package format and management

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: BTreeMap<String, String>, // name -> version constraint
    pub binaries: Vec<String>, // List of binary files
}

/// Package structure (.apkg format)
pub struct Package {
    pub manifest: PackageManifest,
    pub data: Vec<u8>, // Compressed tar.gz data
}

/// Package manager
pub struct PackageManager {
    installed: BTreeMap<String, PackageManifest>,
}

impl PackageManager {
    pub const fn new() -> Self {
        Self {
            installed: BTreeMap::new(),
        }
    }

    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }

    /// Install package
    pub fn install(&mut self, package: Package) -> Result<(), &'static str> {
        // 1. Validate dependencies
        for (dep_name, _version) in &package.manifest.dependencies {
            if !self.is_installed(dep_name) {
                return Err("Missing dependency");
            }
        }

        // 2. Register package
        self.installed.insert(package.manifest.name.clone(), package.manifest);
        Ok(())
    }

    /// Remove package
    pub fn remove(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.is_installed(name) {
            return Err("Package not installed");
        }

        // Check if any package depends on this
        for manifest in self.installed.values() {
            if manifest.dependencies.contains_key(name) {
                return Err("Package is a dependency");
            }
        }

        self.installed.remove(name);
        Ok(())
    }

    /// List installed packages
    pub fn list(&self) -> impl Iterator<Item = &PackageManifest> {
        self.installed.values()
    }
}

/// Global package manager
use spin::Mutex;
pub static PACKAGE_MANAGER: Mutex<PackageManager> = Mutex::new(PackageManager::new());
