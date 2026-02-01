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

pub struct SecurityContext {
    pub capabilities: [CapabilityToken; 16], // Fixed size for v1
    pub cap_count: usize,
}

impl SecurityContext {
    pub const fn new() -> Self {
        Self {
            capabilities: [CapabilityToken { object_id: 0, permissions: Permissions::empty() }; 16],
            cap_count: 0,
        }
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
