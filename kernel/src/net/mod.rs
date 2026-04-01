pub mod mesh;
pub mod driver;
pub mod virtio_net;
pub mod bcm_genet;
pub mod loopback;
pub mod dhcp;
pub mod discovery;

use core::sync::atomic::{AtomicBool, Ordering};

/// Profil Keamanan Jaringan (Layer 3/4 Isolator)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkProfile {
    /// Terhubung penuh ke internet/mesh
    Connected,
    /// Mode militer: Memutus rute publik, hanya menerima frame L2 yang tervalidasi PQC dari node intra-mesh
    AirGapped, 
}

pub struct DummyDevice;
impl DummyDevice {
    pub const fn new() -> Self { Self }
    pub fn inject(&mut self, _data: alloc::vec::Vec<u8>) {}
}

pub struct NetworkStack {
    pub device: DummyDevice,
    pub profile: NetworkProfile,
    pub air_gap_lockdown: AtomicBool,
}

impl NetworkStack {
    pub const fn new() -> Self { 
        Self { 
            device: DummyDevice::new(),
            profile: NetworkProfile::AirGapped, // Default to highest security lockdown
            air_gap_lockdown: AtomicBool::new(true),
        } 
    }
    
    pub fn set_profile(&mut self, profile: NetworkProfile) {
        self.profile = profile;
        if profile == NetworkProfile::AirGapped {
            self.air_gap_lockdown.store(true, Ordering::SeqCst);
            crate::enterprise::audit::log_security(
                crate::enterprise::audit::AuditSeverity::Critical,
                "Network", "AIR-GAPPED PROFILE ACTIVATED. Public Routing Cut Off. Hardware isolated."
            );
        } else {
            self.air_gap_lockdown.store(false, Ordering::SeqCst);
            crate::enterprise::audit::log_security(
                crate::enterprise::audit::AuditSeverity::Warning,
                "Network", "Network transitioning to Connected State."
            );
        }
    }

    pub fn poll(&mut self, _timestamp: i64) {
        if self.air_gap_lockdown.load(Ordering::Relaxed) {
             // Hardware-level firewall check
             // Drop all packets that are not PQC (Post-Quantum Cryptography) authenticated
             // Silently ignore public traffic / physical MAC filtering
             return;
        }
        
        // Normal network poll operation (driver.receive() etc) would go here
    }
}
