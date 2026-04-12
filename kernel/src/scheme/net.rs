//! Network Scheme for AetherOS (v10.4 Stage-2 Factorization)
//! Provides raw packet access to the network hardware for the Netd userspace service.

use crate::scheme::{Scheme, SchemeError};
use crate::net::get_network_stack;

pub struct NetScheme;

impl NetScheme {
    pub const fn new() -> Self {
        Self
    }
}

impl Scheme for NetScheme {
    fn open(&self, _path: &str, _flags: usize) -> Result<usize, SchemeError> {
        // Return resource ID 0 (primary network interface)
        Ok(0)
    }

    fn read(&self, _id: usize, _buffer: &mut [u8]) -> Result<usize, SchemeError> {
        // [MILITARY GRADE] Receive raw ethernet frame from hardware
        if let Some(_stack) = get_network_stack() {
            // This would involve a low-level RX from the VirtIO/BCM driver
            // For now, return NO RESOURCE if no packet is waiting
            Err(SchemeError::NoResource)
        } else {
            Err(SchemeError::NoResource)
        }
    }

    fn write(&self, _id: usize, buffer: &[u8]) -> Result<usize, SchemeError> {
        // [MILITARY GRADE] Transmit raw ethernet frame directly to hardware
        if let Some(stack) = get_network_stack() {
            stack.transmit_raw(buffer);
            Ok(buffer.len())
        } else {
            Err(SchemeError::NoResource)
        }
    }

    fn close(&self, _id: usize) -> Result<(), SchemeError> {
        Ok(())
    }

    fn seek(&self, _id: usize, _pos: isize, _whence: usize) -> Result<usize, SchemeError> {
        Err(SchemeError::NotSupported)
    }

    fn fstat(&self, _id: usize, _stat: &mut [u8]) -> Result<(), SchemeError> {
        Ok(())
    }

    fn map(&self, _id: usize, _offset: usize, _size: usize, _flags: usize) -> Result<usize, SchemeError> {
        Err(SchemeError::NotSupported)
    }
}
