//! Mesh Network Module (Phase 25 - v7.0 Global Mesh)
//! Implements self-healing, geographic-aware routing, and ability market.

pub mod self_healing;
pub mod market;
pub mod geo_routing;
pub mod swarm; // [NEW] Phase 28.1 Autonomous Swarm Governance
pub mod tactical; // [NEW] Phase 29.1 Military Tactical Dominance

use spin::Mutex;
use alloc::vec::Vec;
use crate::hal;

/// Global Mesh Controller
pub struct GlobalMeshController {
    _node_id: [u8; 32],
    _is_master: bool,
    _peers: Vec<[u8; 32]>,
}

impl GlobalMeshController {
    pub const fn new() -> Self {
        Self {
            _node_id: [0u8; 32],
            _is_master: false,
            _peers: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        let platform = hal::get_platform();
        platform.puts("[ v7.0 ] Global Mesh: Cluster handshake initiated...\n");
        
        // Phase 25.1: Initialize Heartbeat
        self_healing::SELF_HEALING.record_heartbeat(0);
        
        // Phase 25.2: Sync Market listings
        market::AbilityMarket::advertise_ability(1, 100);

        // Phase 28.1: Autonomous Swarm Governance
        swarm::SWARM_GOVERNANCE.lock().init();
        
        platform.puts("[ v7.0 ] Global Mesh: Harmony Baseline Stable.\n");
    }
    
    /// Tambahkan peer baru dan log event discovery
    pub fn add_peer(&mut self, peer_id: [u8; 32]) {
        let platform = hal::get_platform();
        self._peers.push(peer_id);
        platform.puts("[Mesh] Peer discovered: ");
        for b in &peer_id {
            platform.puts(&format!("{:02X}", b));
        }
        platform.puts("\n");
    }
    
    /// Debug: Print status mesh/peer ke log (bisa dipanggil dari shell)
    pub fn debug_print_status(&self) {
        let platform = hal::get_platform();
        platform.puts("[Mesh] Node ID: ");
        for b in &self._node_id {
            platform.puts(&format!("{:02X}", b));
        }
        platform.puts("\n[Mesh] Peer count: ");
        platform.puts(&format!("{}\n", self._peers.len()));
        for (i, peer) in self._peers.iter().enumerate() {
            platform.puts(&format!("[Mesh] Peer {}: ", i));
            for b in peer {
                platform.puts(&format!("{:02X}", b));
            }
            platform.puts("\n");
        }
    }
}
// end impl GlobalMeshController

pub static GLOBAL_MESH: Mutex<GlobalMeshController> = Mutex::new(GlobalMeshController::new());
