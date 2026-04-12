//! Media Subsystem (Phase 13.3) - Sovereign Sight v2.0
//! Video codec, audio, and Military-Grade Simulated Camera Feed.

use alloc::vec::Vec;
use alloc::string::String;

pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub timestamp_ms: u64,
}

pub struct CameraDevice {
    pub id: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    streaming: bool,
    frame_count: u64,
}

impl CameraDevice {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            width: 640,
            height: 480,
            streaming: false,
            frame_count: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.streaming = true;
        Ok(())
    }

    pub fn stop(&mut self) { self.streaming = false; }

    /// [SOVEREIGN SIGHT] Generates a Military-Grade Scanner simulation
    /// Uses squared distances and linear scanlines to avoid missing f32 math in no_std.
    pub fn capture(&mut self) -> Option<VideoFrame> {
        if !self.streaming { return None; }
        
        let mut data = Vec::with_capacity((self.width * self.height * 4) as usize);
        let scan_x = (self.frame_count * 10 % self.width as u64) as i32;
        self.frame_count += 1;

        let center_x = (self.width / 2) as i32;
        let center_y = (self.height / 2) as i32;
        let radius_sq = 200 * 200;

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as i32 - center_x;
                let dy = y as i32 - center_y;
                let dist_sq = dx*dx + dy*dy;
                
                // Draw circular frame
                if dist_sq > radius_sq && dist_sq < radius_sq + 800 {
                    data.extend_from_slice(&[0, 255, 0, 255]); // Green border
                } else if dist_sq < radius_sq {
                    // Draw vertical scanline
                    if (x as i32 - scan_x).abs() < 4 {
                        data.extend_from_slice(&[0, 255, 0, 255]); // Scan beam
                    } else {
                        // Background with subtle static
                        let static_val = (self.frame_count + x as u64 + y as u64) % 20;
                        data.extend_from_slice(&[0, 20 + static_val as u8, 0, 255]);
                    }
                } else {
                    data.extend_from_slice(&[0, 0, 0, 255]); // Outer black
                }
            }
        }

        Some(VideoFrame {
            width: self.width,
            height: self.height,
            data,
            timestamp_ms: self.frame_count * 33,
        })
    }
}

pub static mut GLOBAL_CAMERA: Option<CameraDevice> = None;

pub fn init() {
    unsafe {
        GLOBAL_CAMERA = Some(CameraDevice::new(0, "SOVEREIGN_SIGHT_V1"));
    }
}
