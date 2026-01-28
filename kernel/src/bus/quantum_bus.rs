//! Distributed Quantum Bus - HarmonyOS DNA++
//! Device discovery and resource sharing protocol

use core::sync::atomic::AtomicU32;

const MAX_DEVICES: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCapability {
    CPU,
    GPU,
    NPU,
    HighMemory,
    LowPower,
}

#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub id: u32,
    pub capabilities: u32, // Bitmask
    pub available_memory: usize,
    pub compute_power: u32, // In TFLOPS * 100
    pub latency_ms: u32,
}

impl Device {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            capabilities: 0,
            available_memory: 0,
            compute_power: 0,
            latency_ms: 0,
        }
    }

    pub fn has_capability(&self, cap: DeviceCapability) -> bool {
        let bit = cap as u32;
        (self.capabilities & (1 << bit)) != 0
    }

    pub fn add_capability(&mut self, cap: DeviceCapability) {
        let bit = cap as u32;
        self.capabilities |= 1 << bit;
    }

    pub fn capability_score(&self) -> u32 {
        // Higher is better
        let mut score = 0;
        score += self.compute_power;
        score += (self.available_memory / (1024 * 1024)) as u32; // MB
        score -= self.latency_ms * 10; // Penalize high latency
        score
    }
}

pub struct DeviceMesh {
    devices: [Option<Device>; MAX_DEVICES],
    device_count: usize,
    local_device_id: AtomicU32,
}

impl DeviceMesh {
    pub const fn new() -> Self {
        const NONE: Option<Device> = None;
        Self {
            devices: [NONE; MAX_DEVICES],
            device_count: 0,
            local_device_id: AtomicU32::new(0),
        }
    }

    /// Discover devices in the mesh
    pub fn discover(&mut self) -> usize {
        // In real impl: Use Bluetooth LE, WiFi Direct, etc.
        // For now: Simulate discovery
        
        // Add local device
        let mut local = Device::new(0);
        local.add_capability(DeviceCapability::CPU);
        local.available_memory = 1 << 30; // 1GB
        local.compute_power = 100; // 1 TFLOPS
        local.latency_ms = 0;
        
        self.devices[0] = Some(local);
        self.device_count = 1;
        
        self.device_count
    }

    /// Register a remote device
    pub fn register_device(&mut self, device: Device) -> Result<(), ()> {
        if self.device_count >= MAX_DEVICES {
            return Err(());
        }
        
        self.devices[self.device_count] = Some(device);
        self.device_count += 1;
        
        Ok(())
    }

    /// Find best device for a task
    pub fn find_best_device(&self, required_memory: usize, required_compute: u32) -> Option<Device> {
        let mut best: Option<Device> = None;
        let mut best_score = 0;
        
        for device in self.devices.iter().flatten() {
            if device.available_memory >= required_memory && 
               device.compute_power >= required_compute {
                let score = device.capability_score();
                if score > best_score {
                    best = Some(*device);
                    best_score = score;
                }
            }
        }
        
        best
    }

    /// Get all devices with specific capability
    pub fn devices_with_capability(&self, cap: DeviceCapability) -> [Option<Device>; MAX_DEVICES] {
        let mut result = [None; MAX_DEVICES];
        let mut count = 0;
        
        for device in self.devices.iter().flatten() {
            if device.has_capability(cap) {
                result[count] = Some(*device);
                count += 1;
            }
        }
        
        result
    }

    pub fn device_count(&self) -> usize {
        self.device_count
    }
}

/// Resource request for distributed execution
#[derive(Debug, Clone, Copy)]
pub struct ResourceRequest {
    pub memory_bytes: usize,
    pub compute_tflops: u32,
    pub max_latency_ms: u32,
    pub required_capabilities: u32,
}

impl ResourceRequest {
    pub const fn new(memory_bytes: usize) -> Self {
        Self {
            memory_bytes,
            compute_tflops: 0,
            max_latency_ms: 100,
            required_capabilities: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        let mut device = Device::new(1);
        device.add_capability(DeviceCapability::GPU);
        assert!(device.has_capability(DeviceCapability::GPU));
    }

    #[test]
    fn test_mesh_discovery() {
        let mut mesh = DeviceMesh::new();
        let count = mesh.discover();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_find_best_device() {
        let mut mesh = DeviceMesh::new();
        mesh.discover();
        
        let device = mesh.find_best_device(512 * 1024, 50);
        assert!(device.is_some());
    }
}
