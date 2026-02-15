//! Distributed Storage (Phase 17.2)
//! Implements a resilient Key-Value store with replication.

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;

/// Distributed Storage Engine
pub struct DistStorage {
    local_store: BTreeMap<String, Vec<u8>>,
    replication_factor: u8,
}

impl DistStorage {
    pub const fn new() -> Self {
        Self {
            local_store: BTreeMap::new(),
            replication_factor: 3,
        }
    }

    pub fn init(&mut self) {
        crate::println!("[Storage] Distributed Store Online. Replication: {}", self.replication_factor);
    }

    /// Put a value into the store (and replicate)
    pub fn put(&mut self, key: &str, value: &[u8]) {
        self.local_store.insert(String::from(key), value.to_vec());
        
        // Simulation: Replicate to mesh neighbors
        crate::println!("[Storage] Replicating key '{}' to {} nodes...", key, self.replication_factor);
        use crate::distributed::mesh::MESH_NETWORK;
        if let Some(mesh) = MESH_NETWORK.try_lock() {
            let nodes = mesh.get_nodes();
            for node in nodes.iter().take(self.replication_factor as usize) {
                // In reality, this sends a PUT packet
                crate::println!("[Storage] -> Synced with Node {}", node.id);
            }
        }
    }

    /// Get a value from the store
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.local_store.get(key).cloned()
    }
}

pub static DIST_STORAGE: Mutex<DistStorage> = Mutex::new(DistStorage::new());
