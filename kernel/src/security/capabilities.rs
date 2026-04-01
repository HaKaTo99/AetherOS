//! Capability-based Security Model
//! Defines permissions and access tokens for AetherOS resources

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub object_id: u32,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions {
    pub bits: u32,
}

impl Permissions {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXECUTE: u32 = 1 << 2;
    pub const SEND_IPC: u32 = 1 << 3;
    pub const RECV_IPC: u32 = 1 << 4;
    pub const MAP_MEMORY: u32 = 1 << 5;
    
    pub const fn new(bits: u32) -> Self {
        Self { bits }
    }
    
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }
    
    pub const fn all() -> Self {
        Self { bits: 0xFFFFFFFF }
    }
    
    pub fn contains(&self, other: u32) -> bool {
        (self.bits & other) == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClearanceLevel {
    Ring3Untrusted = 0, // WASM / ART Runtime Sandbox
    Confidential = 1,
    Secret = 2,
    TopSecret = 3,
    Fortress = 4, // Sovereign Kernel Space
}

pub struct SecurityContext {
    pub attributes: ClearanceLevel, // [NEW] Mandatory Access Control Level
    pub capabilities: [CapabilityToken; 16], // Fixed size for v1
    pub cap_count: usize,
}

impl SecurityContext {
    pub const fn new() -> Self {
        Self {
            attributes: ClearanceLevel::Ring3Untrusted, // Default to lowest trust (MAC Sandbox)
            capabilities: [CapabilityToken { object_id: 0, permissions: Permissions::empty() }; 16],
            cap_count: 0,
        }
    }
    
    /// Evluasi MAC (Mandatory Access Control) terpisah dari DAC/RBAC
    pub fn enforce_mac(&self, required_clearance: ClearanceLevel) -> Result<(), &'static str> {
        if self.attributes >= required_clearance {
            Ok(())
        } else {
            crate::enterprise::audit::log_security(
                crate::enterprise::audit::AuditSeverity::Critical,
                "MAC_Sandbox", 
                "Mandatory Access Violation! Untrusted payload attempted kernel access."
            );
            Err("MAC Violation: Insufficient Clearance")
        }
    }
    
    /// Hardware/HAL Boundary Protection (Quarantined MAC Sandbox)
    pub fn enforce_hal_protection(&self, object_id: u32) -> Result<(), &'static str> {
        let is_hal_device = object_id >= 0x1000 && object_id < 0x2000;
        let is_security_enclave = object_id >= 0x2000 && object_id < 0x3000;

        if self.attributes == ClearanceLevel::Ring3Untrusted {
            if is_hal_device || is_security_enclave {
                crate::enterprise::audit::log_security(
                    crate::enterprise::audit::AuditSeverity::Critical,
                    "MAC_Sandbox", 
                    "Quarantined MAC Violation: Sandboxed Container tried to map HAL or Security Space!"
                );
                return Err("Quarantined MAC Violation");
            }
        }
        Ok(())
    }
    
    pub fn has_permission(&self, object_id: u32, required: u32) -> bool {
        for i in 0..self.cap_count {
            if self.capabilities[i].object_id == object_id {
                return self.capabilities[i].permissions.contains(required);
            }
        }
        // Implicit deny
        false
    }
    
    pub fn grant(&mut self, object_id: u32, permissions: u32) -> Result<(), ()> {
        if self.cap_count >= 16 {
            return Err(());
        }
        
        self.capabilities[self.cap_count] = CapabilityToken {
            object_id,
            permissions: Permissions::new(permissions),
        };
        self.cap_count += 1;
        Ok(())
    }
}
