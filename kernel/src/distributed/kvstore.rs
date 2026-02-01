//! Distributed Key-Value Store
//! Primary-backup replication with eventual consistency

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Timestamp for conflict resolution
pub type Timestamp = u64;

/// KV Store entry
#[derive(Debug, Clone)]
pub struct KvEntry {
    pub value: Vec<u8>,
    pub timestamp: Timestamp,
}

/// Replication role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaRole {
    Primary,
    Backup,
    Standalone,
}

/// Distributed Key-Value Store
pub struct KvStore {
    storage: BTreeMap<String, KvEntry>,
    role: ReplicaRole,
    backup_device_id: Option<u32>,
    current_time: Timestamp,
}

impl KvStore {
    pub const fn new() -> Self {
        Self {
            storage: BTreeMap::new(),
            role: ReplicaRole::Standalone,
            backup_device_id: None,
            current_time: 0,
        }
    }

    pub fn init(&mut self) {
        self.role = ReplicaRole::Standalone;
    }

    /// Set replication role
    pub fn set_role(&mut self, role: ReplicaRole, backup_id: Option<u32>) {
        self.role = role;
        self.backup_device_id = backup_id;
    }

    /// Put key-value (local write)
    pub fn put(&mut self, key: String, value: Vec<u8>) {
        self.current_time += 1;
        let entry = KvEntry {
            value,
            timestamp: self.current_time,
        };
        self.storage.insert(key, entry);

        // TODO: Replicate to backup if role == Primary
    }

    /// Get value by key
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.storage.get(key).map(|e| &e.value)
    }

    /// Handle remote write (replication or conflict resolution)
    pub fn put_remote(&mut self, key: String, value: Vec<u8>, timestamp: Timestamp) {
        if let Some(existing) = self.storage.get(&key) {
            // Conflict resolution: last-write-wins
            if timestamp > existing.timestamp {
                self.storage.insert(key, KvEntry { value, timestamp });
            }
        } else {
            self.storage.insert(key, KvEntry { value, timestamp });
        }
    }

    /// Sync with primary/backup (stub)
    pub fn sync(&mut self) -> Result<(), &'static str> {
        // TODO: Implement sync via Quantum Bus KvSync RPC
        Ok(())
    }

    /// Get all keys for debugging
    pub fn keys(&self) -> alloc::vec::Vec<String> {
        self.storage.keys().cloned().collect()
    }
}

// Global KV Store
pub static mut KV_STORE: KvStore = KvStore::new();
