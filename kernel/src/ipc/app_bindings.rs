//! IPC Bindings for Applications (Phase 14.2)
//! High-level IPC API for third-party apps

use crate::ipc::qc::{QcPacket, RpcMethod};
use alloc::string::String;
use alloc::vec::Vec;

/// App-facing IPC handle
pub struct IpcHandle {
    pub channel_id: u16,
    pub connected: bool,
}

impl IpcHandle {
    pub fn connect(channel: u16) -> Self {
        Self { channel_id: channel, connected: true }
    }

    /// Send a typed message to another app
    pub fn send(&self, method: &str, payload: &[u8]) -> Result<(), &'static str> {
        if !self.connected { return Err("Not connected"); }
        // Would route through QuantumBus
        Ok(())
    }

    /// Receive a message (blocking-style stub)
    pub fn recv(&self) -> Result<Vec<u8>, &'static str> {
        if !self.connected { return Err("Not connected"); }
        Ok(Vec::new())
    }

    pub fn close(&mut self) {
        self.connected = false;
    }
}

/// Service registration for app IPC
pub struct ServiceRegistry {
    services: Vec<(String, u16)>, // name -> channel_id
}

impl ServiceRegistry {
    pub const fn new() -> Self {
        Self { services: Vec::new() }
    }

    pub fn register(&mut self, name: &str, channel_id: u16) {
        self.services.push((String::from(name), channel_id));
    }

    pub fn lookup(&self, name: &str) -> Option<u16> {
        self.services.iter()
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, id)| *id)
    }
}

use spin::Mutex;
pub static SERVICE_REGISTRY: Mutex<ServiceRegistry> = Mutex::new(ServiceRegistry::new());
