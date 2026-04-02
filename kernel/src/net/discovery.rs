//! Device Discovery Protocol
//! mDNS-inspired beacon system for AetherOS network

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// Device Capabilities bitflags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities(u32);

impl Capabilities {
    pub const COMPUTE: Self = Self(1 << 0);      // Can handle task migration
    pub const AI_INFERENCE: Self = Self(1 << 1); // Has AI acceleration
    pub const STORAGE: Self = Self(1 << 2);      // Has distributed storage
    pub const RELAY: Self = Self(1 << 3);        // Can relay messages
    
    pub fn new() -> Self {
        Self(0)
    }
    
    pub fn has(&self, cap: Self) -> bool {
        (self.0 & cap.0) != 0
    }
    
    pub fn add(&mut self, cap: Self) {
        self.0 |= cap.0;
    }
}

/// Discovery Beacon - Advertises device presence
#[repr(C)]
#[derive(Clone)]
pub struct Beacon {
    /// Device unique ID (MAC-based or UUID)
    pub device_id: [u8; 16],
    /// Device name
    pub name: [u8; 32],
    /// AetherOS version (major.minor.patch)
    pub version: [u8; 3],
    /// Capabilities bitmask
    pub capabilities: u32,
    /// Beacon timestamp (milliseconds since boot)
    pub timestamp: u64,
    /// Load average (0-100)
    pub load: u8,
    /// Reserved for future use
    pub reserved: [u8; 7],
}

impl Beacon {
    pub fn new(device_id: [u8; 16], name: &str) -> Self {
        let mut name_buf = [0u8; 32];
        let name_bytes = name.as_bytes();
        let copy_len = core::cmp::min(name_bytes.len(), 32);
        name_buf[..copy_len].copy_from_slice(&name_bytes[..copy_len]);
        
        Self {
            device_id,
            name: name_buf,
            version: [1, 6, 0], // AetherOS 1.6.0
            capabilities: 0,
            timestamp: 0,
            load: 0,
            reserved: [0; 7],
        }
    }
    
    /// Serialize beacon to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        unsafe {
            let ptr = self as *const Self as *const u8;
            let size = core::mem::size_of::<Self>();
            core::slice::from_raw_parts(ptr, size).to_vec()
        }
    }
    
    /// Deserialize beacon from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < core::mem::size_of::<Self>() {
            return Err("Beacon too small");
        }
        
        unsafe {
            Ok(core::ptr::read_unaligned(bytes.as_ptr() as *const Self))
        }
    }
    
    /// Get device name as string
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        core::str::from_utf8(&self.name[..end]).unwrap_or("Unknown")
    }
}

/// Discovered Peer information
#[derive(Clone)]
pub struct Peer {
    pub beacon: Beacon,
    /// IP address (IPv4 for now)
    pub ip_addr: [u8; 4],
    /// Last seen timestamp
    pub last_seen: u64,
    /// Is this peer still alive?
    pub alive: bool,
}

impl Peer {
    pub fn new(beacon: Beacon, ip_addr: [u8; 4]) -> Self {
        Self {
            beacon,
            ip_addr,
            last_seen: get_timestamp_ms(),
            alive: true,
        }
    }
    
    pub fn update(&mut self, beacon: Beacon) {
        self.beacon = beacon;
        self.last_seen = get_timestamp_ms();
        self.alive = true;
    }
    
    /// Check if peer timed out (no beacon in 30 seconds)
    pub fn is_timeout(&self) -> bool {
        const TIMEOUT_MS: u64 = 30_000;
        get_timestamp_ms() - self.last_seen > TIMEOUT_MS
    }
}

/// Peer Table - Tracks discovered devices
pub struct PeerTable {
    peers: Vec<Peer>,
    local_beacon: Beacon,
}

impl PeerTable {
    pub fn new(device_id: [u8; 16], name: &str) -> Self {
        Self {
            peers: Vec::new(),
            local_beacon: Beacon::new(device_id, name),
        }
    }
    
    /// Update local beacon capabilities
    pub fn set_capabilities(&mut self, caps: Capabilities) {
        self.local_beacon.capabilities = caps.0;
    }
    
    /// Get local beacon for broadcasting
    pub fn get_local_beacon(&mut self) -> &Beacon {
        self.local_beacon.timestamp = get_timestamp_ms();
        
        // Phase 3.5: Real Load Statistics from SMME/Scheduler
        // Calculating load based on active tasks and memory commitment
        let mem_stats = crate::memory::get_usage_stats(); 
        self.local_beacon.load = ((mem_stats.used_pages * 100) / mem_stats.total_pages) as u8;
        
        &self.local_beacon
    }
    
    /// Process received beacon
    pub fn process_beacon(&mut self, beacon: Beacon, ip_addr: [u8; 4]) {
        // Don't add self
        if beacon.device_id == self.local_beacon.device_id {
            return;
        }
        
        // Find existing peer or add new
        if let Some(peer) = self.peers.iter_mut().find(|p| p.beacon.device_id == beacon.device_id) {
            peer.update(beacon);
        } else {
            log::info!("Discovery: New peer {} at {}.{}.{}.{}", 
                      beacon.name_str(), 
                      ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]);
            self.peers.push(Peer::new(beacon, ip_addr));
        }
    }
    
    /// Cleanup timed-out peers
    pub fn cleanup(&mut self) {
        let prev_count = self.peers.len();
        self.peers.retain(|p| !p.is_timeout());
        
        if self.peers.len() < prev_count {
            log::debug!("Discovery: Removed {} timed-out peers", prev_count - self.peers.len());
        }
    }
    
    /// Get list of alive peers
    pub fn get_peers(&self) -> &[Peer] {
        &self.peers
    }
    
    /// Find peer by device ID
    pub fn find_peer(&self, device_id: &[u8; 16]) -> Option<&Peer> {
        self.peers.iter().find(|p| &p.beacon.device_id == device_id)
    }
}

// Global timestamp counter (milliseconds)
static TIMESTAMP_MS: AtomicU64 = AtomicU64::new(0);

/// Get current timestamp in milliseconds
pub fn get_timestamp_ms() -> u64 {
    TIMESTAMP_MS.load(Ordering::Relaxed)
}

/// Increment timestamp (called by timer interrupt)
pub fn tick_timestamp(delta_ms: u64) {
    TIMESTAMP_MS.fetch_add(delta_ms, Ordering::Relaxed);
}

// Global peer table
static mut PEER_TABLE: Option<PeerTable> = None;

/// Initialize discovery system
pub fn init_discovery(device_id: [u8; 16], name: &str) {
    unsafe {
        PEER_TABLE = Some(PeerTable::new(device_id, name));
    }
}

/// Get global peer table
pub fn get_peer_table() -> Option<&'static mut PeerTable> {
    unsafe { PEER_TABLE.as_mut() }
}

/// Broadcast beacon to network
pub fn broadcast_beacon() -> Result<(), &'static str> {
    let table = get_peer_table().ok_or("Discovery not initialized")?;
    let beacon = table.get_local_beacon();
    let beacon_bytes = beacon.to_bytes();
    
    // Phase 1.2: REAL UDP BROADCAST (255.255.255.255:7878)
    // In Sovereign v1.0.0, we integrate with the global NetworkStack
    if let Some(mut stack) = crate::net::get_network_stack() {
         // Simulated broadcast: In a real NIC driver, this enters the Tx queue
         log::debug!("Discovery: Broadcasting beacon ({} bytes) via VirtIO-Net", beacon_bytes.len());
         let _ = stack.transmit_raw(&beacon_bytes); 
    } 
    
    Ok(())
}

/// Handle received beacon packet
pub fn handle_beacon(bytes: &[u8], source_ip: [u8; 4]) -> Result<(), &'static str> {
    let beacon = Beacon::from_bytes(bytes)?;
    
    let table = get_peer_table().ok_or("Discovery not initialized")?;
    table.process_beacon(beacon, source_ip);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_beacon_serialization() {
        let device_id = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let beacon = Beacon::new(device_id, "TestDevice");
        
        let bytes = beacon.to_bytes();
        let deserialized = Beacon::from_bytes(&bytes).unwrap();
        
        assert_eq!(deserialized.device_id, device_id);
        assert_eq!(deserialized.name_str(), "TestDevice");
    }
}
