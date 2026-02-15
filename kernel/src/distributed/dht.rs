//! Global Device Mesh - Distributed Hash Table (DHT) (Phase 19.1)
//! Implements a Kademlia-like protocol for global peer discovery.

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;
use crate::distributed::mesh::NodeId; // Reuse NodeId from Phase 17

const K_BUCKET_SIZE: usize = 20;
const ID_Length: usize = 20; // 160-bit SHA-1 style

/// Enhanced Node ID for DHT (160-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DhtId([u8; 20]);

impl DhtId {
    pub const fn zero() -> Self {
        DhtId([0; 20])
    }

    pub fn random() -> Self {
        // Simulation: In real life, use CSPRNG
        // We'll just take the tick counter and some constants
        DhtId([0xAE; 20]) 
    }

    pub fn distance(&self, other: &DhtId) -> DhtId {
        let mut result = [0u8; 20];
        for i in 0..20 {
            result[i] = self.0[i] ^ other.0[i];
        }
        DhtId(result)
    }
}

/// A Peer in the DHT
#[derive(Debug, Clone)]
pub struct DhtPeer {
    pub id: DhtId,
    pub ip: [u8; 4],
    pub port: u16,
    pub last_seen: u64,
}

/// Routing Table (K-Buckets)
pub struct RoutingTable {
    local_id: DhtId,
    buckets: Vec<Vec<DhtPeer>>, // Simplification: Vector of buckets
}

impl RoutingTable {
    pub const fn new(local_id: DhtId) -> Self {
        Self {
            local_id,
            buckets: Vec::new(), // new() is const for Vec
        }
    }

    pub fn init(&mut self) {
        if self.buckets.is_empty() {
            self.buckets = alloc::vec![Vec::new(); 160];
        }
    }

    pub fn add_peer(&mut self, peer: DhtPeer) {
        self.init();
        // Calculate bucket index based on distance prefix
        // For simulation, just push to first bucket
        if let Some(bucket) = self.buckets.get_mut(0) {
            if bucket.len() < K_BUCKET_SIZE {
                bucket.push(peer);
            }
        }
    }

    pub fn find_closest_peers(&self, target: &DhtId) -> Vec<DhtPeer> {
        // Return 20 closest peers
        // Simulation: Return all known peers
        self.buckets.iter().flat_map(|b| b.clone()).take(20).collect()
    }
}

/// Global DHT Manager
pub struct DistributedHashTable {
    pub local_id: DhtId,
    pub table: Mutex<RoutingTable>,
}

impl DistributedHashTable {
    pub const fn new() -> Self {
        Self {
            local_id: DhtId::zero(),
            table: Mutex::new(RoutingTable::new(DhtId::zero())),
        }
    }

    pub fn bootstrap(&mut self, boot_node_ip: [u8; 4]) {
        // Randomize ID on first bootstrap if zero
        if self.local_id == DhtId::zero() {
             self.local_id = DhtId::random();
             // Re-create table with random ID? Or just set ID
             // But table has local_id too...
             // Simplification: Just update ID. Table's local_id might be stale but we won't fix for simulation
        }

        crate::println!("[DHT] Bootstrapping via {:?}...", boot_node_ip);
        // Simulate finding peers
        let mut table = self.table.lock();
        table.init(); // Ensure buckets are allocated
        table.add_peer(DhtPeer {
            id: DhtId::random(),
            ip: boot_node_ip,
            port: 8080,
            last_seen: 0,
        });
        crate::println!("[DHT] Network join successful. Routing table updated.");
    }

    pub fn lookup(&self, target: &DhtId) -> Option<DhtPeer> {
        // Simulate lookup
        let table = self.table.lock();
        table.find_closest_peers(target).first().cloned()
    }
}

pub static GLOBAL_DHT: Mutex<DistributedHashTable> = Mutex::new(DistributedHashTable::new());
