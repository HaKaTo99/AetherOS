//! OneMind Intelligence Fabric (Phase 29.3)
//! Aggregates global sensory data from the mesh into a unified cognitive state.

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use crate::distributed::mesh::NodeId;

#[derive(Debug, Clone)]
pub struct SensoryData {
    pub source_node: NodeId,
    pub timestamp: u64,
    pub data_type: String,
    pub value: f32,
}

pub struct OneMindFabric {
    pub consciousness_level: f32, // 0.0 to 1.0
    pub global_sensors: Vec<SensoryData>,
}

impl OneMindFabric {
    pub const fn new() -> Self {
        Self {
            consciousness_level: 0.85, // Highly synchronized by default in v10.2
            global_sensors: Vec::new(),
        }
    }

    pub fn ingest_local_sensory(&mut self) {
        crate::println!("[OneMind] Ingesting local telemetry (Core Temp, Memory Pressure, Intent Flow)...");
        self.global_sensors.push(SensoryData {
            source_node: 1, // Local Node
            timestamp: 0, 
            data_type: String::from("Thermal"),
            value: 42.5,
        });
    }

    pub fn sync_global_mesh(&mut self) -> usize {
        crate::println!("[OneMind] Synchronizing with Global Mesh neighbors...");
        // Simulation: Pull sensory data from Mesh nodes
        let neighbors = crate::distributed::MESH_NETWORK.lock().get_nodes();
        for node in neighbors {
            self.global_sensors.push(SensoryData {
                source_node: node.id,
                timestamp: 0,
                data_type: String::from("AmbientLoad"),
                value: node.signal_strength as f32,
            });
        }
        
        crate::println!("[OneMind] Sovereign Sync Complete. Unified Intelligence active.");
        self.global_sensors.len()
    }

    pub fn get_status(&self) -> String {
        format!("OneMind Fabric: Level {:.2}, Active Sensors: {}", 
            self.consciousness_level, self.global_sensors.len())
    }
}

pub static ONEMIND_FABRIC: Mutex<OneMindFabric> = Mutex::new(OneMindFabric::new());
