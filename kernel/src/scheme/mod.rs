//! Scheme system for AetherOS (v10.4 Stage-1 Factorization)
//! Based on the "Everything is a URL" philosophy of Redox OS.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::AtomicUsize;
use spin::Mutex;

// pub mod memory; // [ERROR] File missing in v10.4 Stage-1
pub mod ai;
pub mod display;
pub mod net;
pub mod debug;
pub mod fabric; // [FABRIC] New scheme for distributed fabric access

/// Standard Error for Scheme operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemeError {
    NotSupported,
    NoResource,
    PermissionDenied,
    InvalidArguments,
    Busy,
    Internal,
}

/// The base trait for all system resource providers (schemes).
/// This is the core of the "Military Grade" Structural Factorization.
pub trait Scheme {
    /// Open a resource at the given path with flags.
    fn open(&self, path: &str, flags: usize) -> Result<usize, SchemeError>;

    /// Read data from a resource.
    fn read(&self, id: usize, buffer: &mut [u8]) -> Result<usize, SchemeError>;

    /// Write data to a resource.
    fn write(&self, id: usize, buffer: &[u8]) -> Result<usize, SchemeError>;

    /// Close a resource.
    fn close(&self, id: usize) -> Result<(), SchemeError>;

    /// Seek in a resource.
    fn seek(&self, id: usize, pos: isize, whence: usize) -> Result<usize, SchemeError>;
    
    /// Fstat a resource.
    fn fstat(&self, id: usize, stat: &mut [u8]) -> Result<(), SchemeError>;

    /// Map a resource into memory.
    fn map(&self, id: usize, offset: usize, size: usize, flags: usize) -> Result<usize, SchemeError>;
}

/// Manager for all registered schemes.
pub struct SchemeManager {
    schemes: Vec<(Box<str>, Box<dyn Scheme + Send + Sync>)>,
    _next_fd: AtomicUsize,
}

impl SchemeManager {
    pub const fn new() -> Self {
        Self {
            schemes: Vec::new(),
            _next_fd: AtomicUsize::new(100), // Start FDs at 100
        }
    }

    /// Register a new scheme with a name (e.g., "fabric", "file", "ai").
    pub fn register(&mut self, name: &str, scheme: Box<dyn Scheme + Send + Sync>) {
        self.schemes.push((Box::from(name), scheme));
    }

    /// Find a scheme by name.
    pub fn get(&self, name: &str) -> Option<&(dyn Scheme + Send + Sync)> {
        for (n, s) in &self.schemes {
            if n.as_ref() == name {
                return Some(s.as_ref());
            }
        }
        None
    }
}

pub static SCHEME_MANAGER: Mutex<SchemeManager> = Mutex::new(SchemeManager::new());

/// Global entry point for opening any resource via URI (e.g., "fabric:heartbeat").
pub fn open(uri: &str, flags: usize) -> Result<(usize, usize), SchemeError> {
    let mut parts = uri.splitn(2, ':');
    let scheme_name = parts.next().ok_or(SchemeError::InvalidArguments)?;
    let path = parts.next().unwrap_or("");

    let mgr = SCHEME_MANAGER.lock();
    if let Some(scheme) = mgr.get(scheme_name) {
        let id = scheme.open(path, flags)?;
        // Return (scheme_index, resource_id) - simplified for demo
        Ok((0, id)) 
    } else {
        Err(SchemeError::NoResource)
    }
}
