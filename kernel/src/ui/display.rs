//! Distributed Framebuffer & UI Management

use core::sync::atomic::{AtomicU32, Ordering};
use crate::bus::quantum_bus::Device;

pub const MAX_WIDTH: usize = 3840;
pub const MAX_HEIGHT: usize = 2160;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelFormat {
    RGB888,
    RGBA8888,
    VectorDelta, // macOS-inspired vector streaming
}

/// Representation of a UI change
#[derive(Debug, Clone, Copy)]
pub struct UIUpdate {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub data_ptr: usize,
    pub format: PixelFormat,
}

/// Distributed Framebuffer
pub struct DistributedFramebuffer {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub version: AtomicU32,
    pub remote_host: Option<Device>,
}

impl DistributedFramebuffer {
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            format: PixelFormat::RGBA8888,
            version: AtomicU32::new(0),
            remote_host: None,
        }
    }

    pub fn set_remote_host(&mut self, device_id: Device) {
        self.remote_host = Some(device_id);
    }

    pub fn push_update(&self, _update: UIUpdate) -> Result<(), ()> {
        self.version.fetch_add(1, Ordering::SeqCst);
        
        if let Some(_host) = self.remote_host {
            // Send update via Quantum Bus
            // In real impl: bus.send(host, update)
            Ok(())
        } else {
            // Local rendering
            Ok(())
        }
    }
}

/// Primitive Vector Renderer
pub struct VectorRenderer;

impl VectorRenderer {
    pub fn draw_rect(fb: &DistributedFramebuffer, x: u32, y: u32, w: u32, h: u32, color: u32) {
        let update = UIUpdate {
            x, y, width: w, height: h,
            data_ptr: color as usize, // For demo, use color as data
            format: PixelFormat::VectorDelta,
        };
        let _ = fb.push_update(update);
    }
}
