//! Device Mesh Network (Phase 17.1)
//! Implements decentralized node discovery and routing.

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;
/// Node ID Type for the Mesh
pub type NodeId = u64;

/// A node in the AetherOS mesh
#[derive(Debug, Clone)]
pub struct MeshNode {
    pub id: NodeId,
    pub name: String,
    pub capabilities: Vec<String>,
    pub signal_strength: u8, // 0-100
}

/// The Mesh Network Manager
pub struct MeshNetwork {
    local_node: MeshNode,
    neighbors: BTreeMap<u64, MeshNode>,
    routes: BTreeMap<u64, Vec<u64>>, // Dest ID -> Path
}

impl MeshNetwork {
    pub const fn new() -> Self {
        Self {
            local_node: MeshNode {
                id: 1, // Phase 17: Static ID for kernel
                name: String::new(),
                capabilities: Vec::new(),
                signal_strength: 100,
            },
            neighbors: BTreeMap::new(),
            routes: BTreeMap::new(),
        }
    }

    pub fn init(&mut self) {
        self.local_node.name = String::from("AetherOS-Kernel-Node");
        self.local_node.capabilities.push(String::from("compute"));
        self.local_node.capabilities.push(String::from("storage"));
        
        crate::println!("[Mesh] Network Initialized. Node ID: {}", self.local_node.id);
    }

    /// Simulate discovery of nearby devices
    pub fn discover(&mut self) -> usize {
        // Simulation: Add fake neighbors
        let node2 = MeshNode {
            id: 2,
            name: String::from("AetherPad-Tablet"),
            capabilities: alloc::vec![String::from("display"), String::from("touch")],
            signal_strength: 85,
        };
        
        let node3 = MeshNode {
            id: 3,
            name: String::from("AetherBox-Server"),
            capabilities: alloc::vec![String::from("compute-heavy"), String::from("ai-inference")],
            signal_strength: 92,
        };

        self.neighbors.insert(node2.id, node2.clone());
        self.neighbors.insert(node3.id, node3.clone());

        // Update routing table (simple direct routes)
        self.routes.insert(2, alloc::vec![2]);
        self.routes.insert(3, alloc::vec![3]);

        crate::println!("[Mesh] Discovered {} neighbors.", self.neighbors.len());
        self.neighbors.len()
    }

    /// Route a packet to a destination
    pub fn send_packet(&self, dest_id: u64, payload: &[u8]) -> Result<(), &'static str> {
        if let Some(route) = self.routes.get(&dest_id) {
            crate::println!("[Mesh] Routing {} bytes to Node {} via {:?}", payload.len(), dest_id, route);
            // In a real implementation, this would use the HAL radio driver
            Ok(())
        } else {
            Err("Host unreachable")
        }
    }

    /// List all known nodes
    pub fn get_nodes(&self) -> Vec<MeshNode> {
        self.neighbors.values().cloned().collect()
    }
}

pub static MESH_NETWORK: Mutex<MeshNetwork> = Mutex::new(MeshNetwork::new());
