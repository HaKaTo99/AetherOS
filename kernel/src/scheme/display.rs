//! Display Scheme for AetherOS (v10.4 Stage-1 Factorization)
//! Provides memory-mapped access to the Framebuffer for the Orbital display server.

use crate::scheme::{Scheme, SchemeError};
// use crate::drivers::video::lfb::LFB_DRIVER; // [REMOVED] Use global driver interface


pub struct DisplayScheme;

impl DisplayScheme {
    pub const fn new() -> Self {
        Self
    }
}

impl Scheme for DisplayScheme {
    fn open(&self, _path: &str, _flags: usize) -> Result<usize, SchemeError> {
        // Return resource ID 0 (only one display for now)
        Ok(0)
    }

    fn read(&self, _id: usize, _buffer: &mut [u8]) -> Result<usize, SchemeError> {
        Err(SchemeError::NotSupported)
    }

    fn write(&self, _id: usize, _buffer: &[u8]) -> Result<usize, SchemeError> {
        Err(SchemeError::NotSupported)
    }

    fn close(&self, _id: usize) -> Result<(), SchemeError> {
        Ok(())
    }

    fn seek(&self, _id: usize, _pos: isize, _whence: usize) -> Result<usize, SchemeError> {
        Err(SchemeError::NotSupported)
    }

    fn fstat(&self, _id: usize, _stat: &mut [u8]) -> Result<(), SchemeError> {
        // Here we would return FB dimensions and pixel format
        Ok(())
    }

    fn map(&self, _id: usize, _offset: usize, _size: usize, _flags: usize) -> Result<usize, SchemeError> {
        // [MILITARY GRADE] Return physical address to be mapped into userspace
        let mut fb_ptr = 0;
        crate::drivers::video::draw(|fb| {
            fb_ptr = fb.get_fb_ptr();
        });
        
        if fb_ptr == 0 {
            Err(SchemeError::NoResource)
        } else {
            Ok(fb_ptr)
        }
    }
}
