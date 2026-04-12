//! Debug Scheme for AetherOS (v10.4 Stage-1 Factorization)
//! Provides a standard way to write to the debug console (UART/LFB).

use super::{Scheme, SchemeError};
use crate::hal;

pub struct DebugScheme;

impl Scheme for DebugScheme {
    fn open(&self, _path: &str, _flags: usize) -> Result<usize, SchemeError> {
        Ok(0) // Only one instance
    }

    fn read(&self, _id: usize, _buffer: &mut [u8]) -> Result<usize, SchemeError> {
        Err(SchemeError::NotSupported)
    }

    fn write(&self, _id: usize, buffer: &[u8]) -> Result<usize, SchemeError> {
        let platform = hal::get_platform();
        if let Ok(s) = core::str::from_utf8(buffer) {
            platform.puts(s);
            Ok(buffer.len())
        } else {
            Err(SchemeError::InvalidArguments)
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
