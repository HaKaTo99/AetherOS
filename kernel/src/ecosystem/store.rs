//! Aether Store & Package Infrastructure - v7.0
//! 
//! Decentralized portal for OmniLang apps, AI Agents, and System Tools.

use alloc::string::String;
use alloc::vec::Vec;
use crate::runtime::apm::{PACKAGE_MANAGER, Package, PackageManifest};
use alloc::collections::BTreeMap;

pub struct AetherStore;

impl AetherStore {
    /// Search for applications in the decentralized mesh repository
    pub fn search(query: &str) -> Vec<String> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        // Simulated Mesh Catalog
        if query_lower.contains("finance") || query_lower.contains("omnilang") {
            results.push(String::from("Aether Finance (OmniLang UI)"));
            results.push(String::from("OmniLang SDK"));
        }
        
        if query_lower.contains("game") {
            results.push(String::from("Quantum Quest (ECS Game)"));
        }

        if query_lower.contains("ai") || query_lower.contains("agent") {
            results.push(String::from("Butler AI (Agentic Tool)"));
        }

        results
    }

    /// Purchase and Install application via APM
    pub fn install_app(app_name: &str) -> Result<(), &'static str> {
        let mut apm = PACKAGE_MANAGER.lock();
        
        // Mocking the creation of a .apkg from the store
        let manifest = PackageManifest {
            name: String::from(app_name),
            version: String::from("1.0.0-Stable"),
            description: String::from("Universal Application for AetherOS"),
            category: String::from("UI"),
            dependencies: BTreeMap::new(),
            binaries: vec![String::from("main.omni")],
        };

        let pkg = Package { manifest, data: Vec::new() };
        apm.install(pkg)
    }
}
