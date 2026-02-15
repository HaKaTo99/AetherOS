//! Lightweight Container Support (Phase 15.3)
//! OCI-compatible containers, resource isolation, namespaces

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// ===========================
// Container Image
// ===========================

/// OCI Image manifest (simplified)
#[derive(Debug, Clone)]
pub struct ImageManifest {
    pub name: String,
    pub tag: String,
    pub layers: Vec<ImageLayer>,
    pub entrypoint: String,
    pub env: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ImageLayer {
    pub digest: String,
    pub size: usize,
    pub data: Vec<u8>, // Compressed tar data
}

// ===========================
// Resource Isolation (cgroups-like)
// ===========================

/// Resource limits for a container
#[derive(Debug, Clone, Copy)]
pub struct ResourceLimits {
    pub memory_bytes: usize,   // Max memory
    pub cpu_shares: u32,       // CPU weight (1024 = 1 core)
    pub max_pids: u32,         // Process limit
    pub io_bandwidth: usize,   // Bytes/sec I/O limit
}

impl ResourceLimits {
    pub fn default() -> Self {
        Self {
            memory_bytes: 64 * 1024 * 1024, // 64MB
            cpu_shares: 1024,
            max_pids: 64,
            io_bandwidth: 10 * 1024 * 1024, // 10MB/s
        }
    }
}

/// Resource usage tracker
#[derive(Debug, Clone, Copy)]
pub struct ResourceUsage {
    pub memory_used: usize,
    pub cpu_time_ns: u64,
    pub pid_count: u32,
}

// ===========================
// Network Namespace
// ===========================

/// Isolated network namespace
#[derive(Debug, Clone)]
pub struct NetNamespace {
    pub id: u32,
    pub veth_ip: [u8; 4],     // Virtual ethernet IP
    pub gateway: [u8; 4],
    pub dns: [u8; 4],
}

impl NetNamespace {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            veth_ip: [10, 0, 0, (id as u8) + 2],
            gateway: [10, 0, 0, 1],
            dns: [8, 8, 8, 8],
        }
    }
}

// ===========================
// Container Runtime
// ===========================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerState {
    Created,
    Running,
    Paused,
    Stopped,
}

/// Container instance
pub struct Container {
    pub id: u32,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub limits: ResourceLimits,
    pub usage: ResourceUsage,
    pub netns: NetNamespace,
    pub pid: u32, // init process PID
}

/// Container runtime (Docker-like)
pub struct ContainerRuntime {
    containers: BTreeMap<u32, Container>,
    images: Vec<ImageManifest>,
    next_id: u32,
}

impl ContainerRuntime {
    pub const fn new() -> Self {
        Self {
            containers: BTreeMap::new(),
            images: Vec::new(),
            next_id: 1,
        }
    }

    /// Pull/register an image
    pub fn load_image(&mut self, manifest: ImageManifest) {
        self.images.push(manifest);
    }

    /// Create a container from image
    pub fn create(&mut self, name: &str, image: &str, limits: ResourceLimits) -> Result<u32, &'static str> {
        if !self.images.iter().any(|i| i.name == image) {
            return Err("Image not found");
        }
        let id = self.next_id;
        self.next_id += 1;
        let container = Container {
            id,
            name: String::from(name),
            image: String::from(image),
            state: ContainerState::Created,
            limits,
            usage: ResourceUsage { memory_used: 0, cpu_time_ns: 0, pid_count: 0 },
            netns: NetNamespace::new(id),
            pid: 0,
        };
        self.containers.insert(id, container);
        Ok(id)
    }

    /// Start a container
    pub fn start(&mut self, id: u32) -> Result<(), &'static str> {
        let c = self.containers.get_mut(&id).ok_or("Container not found")?;
        c.state = ContainerState::Running;
        c.pid = 1000 + id; // Stub PID
        Ok(())
    }

    /// Stop a container
    pub fn stop(&mut self, id: u32) -> Result<(), &'static str> {
        let c = self.containers.get_mut(&id).ok_or("Container not found")?;
        c.state = ContainerState::Stopped;
        Ok(())
    }

    /// Remove a container
    pub fn remove(&mut self, id: u32) -> Result<(), &'static str> {
        self.containers.remove(&id).ok_or("Container not found")?;
        Ok(())
    }

    /// List running containers
    pub fn list(&self) -> Vec<(u32, &str, ContainerState)> {
        self.containers.values()
            .map(|c| (c.id, c.name.as_str(), c.state))
            .collect()
    }
}

use spin::Mutex;
pub static CONTAINER_RUNTIME: Mutex<ContainerRuntime> = Mutex::new(ContainerRuntime::new());
