//! Quantum Channel (QC) - RPC Protocol
//!
//! A lightweight RPC framework for distributed computing across AetherOS devices.
//!
//! # Overview
//!
//! The Quantum Channel provides a type-safe RPC mechanism for cross-device communication,
//! supporting operations like:
//! - Device discovery
//! - Task migration
//! - AI inference requests
//! - General ping/pong health checks
//!
//! # Protocol
//!
//! RPC messages follow a simple binary protocol:
//! - **Header**: Method ID (u16) + Payload Length (u32)
//! - **Payload**: Variable-length binary data
//!
//! # Example
//!
//! ```no_run
//! use aetheros_kernel::ipc::qc::{RpcMethod, RpcMessage};
//!
//! // Create a ping request
//! let ping = RpcMessage {
//!     method: RpcMethod::Ping,
//!     payload: vec![],
//! };
//!
//! // Serialize for network transmission
//! let bytes = ping.serialize();
//! ```
//!
//! # Supported Methods
//!
//! - `Ping` / `Pong`: Health check and latency measurement
//! - `Discovery` / `DiscoveryReply`: Device discovery protocol
//! - `TaskMigrate` / `TaskResult`: Task migration coordination
//! - `AiInference` / `AiResult`: Distributed AI inference

use alloc::vec::Vec;

/// RPC Method identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RpcMethod {
    Ping = 0x0001,
    Pong = 0x0002,
    TaskMigrate = 0x0010,
    TaskResult = 0x0011,
    Discovery = 0x0020,
    DiscoveryReply = 0x0021,
    AiInference = 0x0030,
    AiResult = 0x0031,
}

impl RpcMethod {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x0001 => Some(Self::Ping),
            0x0002 => Some(Self::Pong),
            0x0010 => Some(Self::TaskMigrate),
            0x0011 => Some(Self::TaskResult),
            0x0020 => Some(Self::Discovery),
            0x0021 => Some(Self::DiscoveryReply),
            0x0030 => Some(Self::AiInference),
            0x0031 => Some(Self::AiResult),
            _ => None,
        }
    }
}

/// Quantum Channel Packet Header
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct QcHeader {
    /// Magic number: 0x5143 ("QC")
    pub magic: u16,
    /// Protocol version
    pub version: u8,
    /// Flags (reserved)
    pub flags: u8,
    /// RPC method
    pub method: u16,
    /// Sequence number for request/response matching
    pub sequence: u32,
    /// Payload length in bytes
    pub payload_len: u32,
}

impl QcHeader {
    const MAGIC: u16 = 0x5143; // "QC"
    const VERSION: u8 = 1;
    
    pub fn new(method: RpcMethod, sequence: u32, payload_len: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            flags: 0,
            method: method as u16,
            sequence,
            payload_len,
        }
    }
    
    pub fn validate(&self) -> bool {
        // Copy values to avoid packed field reference
        let magic = self.magic;
        let version = self.version;
        magic == Self::MAGIC && version == Self::VERSION
    }
    
    pub fn size() -> usize {
        core::mem::size_of::<Self>()
    }
}

/// Quantum Channel Packet (Header + Payload)
pub struct QcPacket {
    pub header: QcHeader,
    pub payload: Vec<u8>,
}

impl QcPacket {
    pub fn new(method: RpcMethod, sequence: u32, payload: Vec<u8>) -> Self {
        let header = QcHeader::new(method, sequence, payload.len() as u32);
        Self { header, payload }
    }
    
    /// Serialize packet to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(QcHeader::size() + self.payload.len());
        
        // Serialize header
        unsafe {
            let header_slice = core::slice::from_raw_parts(
                &self.header as *const QcHeader as *const u8,
                QcHeader::size(),
            );
            bytes.extend_from_slice(header_slice);
        }
        
        // Append payload
        bytes.extend_from_slice(&self.payload);
        bytes
    }
    
    /// Deserialize packet from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < QcHeader::size() {
            return Err("Packet too small");
        }
        
        // Deserialize header
        let header: QcHeader = unsafe {
            core::ptr::read_unaligned(bytes.as_ptr() as *const QcHeader)
        };
        
        if !header.validate() {
            return Err("Invalid header magic or version");
        }
        
        // Copy payload_len to avoid packed reference
        let payload_len = header.payload_len;
        
        if bytes.len() < QcHeader::size() + payload_len as usize {
            return Err("Payload size mismatch");
        }
        
        // Extract payload
        let payload = bytes[QcHeader::size()..QcHeader::size() + payload_len as usize].to_vec();
        
        Ok(Self { header, payload })
    }
}

/// RPC Handler trait
pub trait RpcHandler {
    fn handle(&self, packet: &QcPacket) -> Result<QcPacket, &'static str>;
}

/// Quantum Bus - RPC Manager
pub struct QuantumBus {
    sequence_counter: core::sync::atomic::AtomicU32,
}

impl QuantumBus {
    pub const fn new() -> Self {
        Self {
            sequence_counter: core::sync::atomic::AtomicU32::new(1),
        }
    }
    
    /// Send RPC request
    pub fn send_rpc(&self, method: RpcMethod, payload: Vec<u8>) -> Result<u32, &'static str> {
        let sequence = self.sequence_counter.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        let packet = QcPacket::new(method, sequence, payload);
        
        // Serialize and send via network stack
        let bytes = packet.to_bytes();
        
        // TODO: Send via UDP socket to target device
        // For now, loopback test (copy values to avoid packed reference)
        let method_val = packet.header.method;
        let seq_val = packet.header.sequence;
        log::debug!("QC: Sending RPC method=0x{:04x} seq={} len={}", method_val, seq_val, bytes.len());
        
        Ok(sequence)
    }
    
    /// Handle incoming RPC packet
    pub fn handle_rpc(&self, bytes: &[u8]) -> Result<Vec<u8>, &'static str> {
        let packet = QcPacket::from_bytes(bytes)?;
        
        // Copy to avoid packed reference
        let method_val = packet.header.method;
        let seq_val = packet.header.sequence;
        
        let method = RpcMethod::from_u16(method_val)
            .ok_or("Unknown RPC method")?;
        
        log::debug!("QC: Handling RPC method={:?} seq={}", method, seq_val);
        
        // Dispatch to handler
        let response = match method {
            RpcMethod::Ping => {
                // Reply with Pong
                QcPacket::new(RpcMethod::Pong, seq_val, packet.payload.clone())
            }
            RpcMethod::Discovery => {
                // Reply with device info
                let device_info = b"AetherOS/1.6.0";
                QcPacket::new(RpcMethod::DiscoveryReply, seq_val, device_info.to_vec())
            }
            _ => {
                return Err("RPC method not implemented");
            }
        };
        
        Ok(response.to_bytes())
    }
}

// Global Quantum Bus instance
static mut QUANTUM_BUS: QuantumBus = QuantumBus::new();

/// Get global Quantum Bus instance
pub fn get_quantum_bus() -> &'static QuantumBus {
    unsafe { &QUANTUM_BUS }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_packet_serialization() {
        let payload = b"Hello, Quantum!".to_vec();
        let packet = QcPacket::new(RpcMethod::Ping, 42, payload.clone());
        
        let bytes = packet.to_bytes();
        let deserialized = QcPacket::from_bytes(&bytes).unwrap();
        
        let method_val = deserialized.header.method;
        let seq_val = deserialized.header.sequence;
        assert_eq!(method_val, RpcMethod::Ping as u16);
        assert_eq!(seq_val, 42);
        assert_eq!(deserialized.payload, payload);
    }
}
