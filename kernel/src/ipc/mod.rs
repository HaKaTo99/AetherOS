//! IPC - Inter-Process Communication Module
//! 
//! Provides:
//! - Quantum Channel (QC) for RPC-style communication
//! - Synchronization primitives (Mutex, Semaphore, RwLock)

pub mod qc;   // Quantum Channel (RPC)
pub mod sync; // Synchronization primitives
pub mod app_bindings; // [NEW] App IPC bindings (Phase 14.2)

pub use qc::{QuantumBus, QcPacket, RpcMethod, get_quantum_bus};
pub use sync::{SpinLock, Mutex, Semaphore, BinarySemaphore, RwLock, Once};
pub use app_bindings::{IpcHandle, SERVICE_REGISTRY};

