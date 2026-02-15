//! Load Balancing
//! Metrics-based task placement and auto-migration

/// System metrics for load balancing
#[derive(Debug, Clone, Copy)]
pub struct SystemMetrics {
    pub cpu_utilization: u8,  // 0-100%
    pub active_tasks: usize,
    pub memory_pressure: u8,  // 0-100%
}

impl SystemMetrics {
    pub const fn new() -> Self {
        Self {
            cpu_utilization: 0,
            active_tasks: 0,
            memory_pressure: 0,
        }
    }

    /// Calculate overall load score (0-100)
    pub fn load_score(&self) -> u8 {
        // Weighted average: CPU 50%, Tasks 30%, Memory 20%
        let cpu_weight = (self.cpu_utilization as u32) * 50;
        let task_weight = (self.active_tasks.min(100) as u32) * 30;
        let mem_weight = (self.memory_pressure as u32) * 20;
        
        ((cpu_weight + task_weight + mem_weight) / 100) as u8
    }
}

/// Load Balancer
pub struct LoadBalancer {
    local_metrics: SystemMetrics,
    enabled: bool,
}

impl LoadBalancer {
    pub const fn new() -> Self {
        Self {
            local_metrics: SystemMetrics::new(),
            enabled: false,
        }
    }

    pub fn init(&mut self) {
        self.enabled = true;
    }

    /// Update local metrics from scheduler and memory stats
    pub fn update_metrics(&mut self, scheduler: &crate::scheduler::ActiveObjectScheduler, smme: &crate::memory::smme::SymbianModernMemoryEngine) {
        // Calculate CPU utilization (simplified - based on active task count)
        let task_count = scheduler.object_count;
        self.local_metrics.active_tasks = task_count;
        self.local_metrics.cpu_utilization = (task_count * 10).min(100) as u8;

        // Calculate memory pressure
        let stats = smme.stats();
        let used_pct = if stats.total_reserved > 0 {
            ((stats.total_committed * 100) / stats.total_reserved) as u8
        } else {
            0
        };
        self.local_metrics.memory_pressure = used_pct;
    }

    /// Simulate high load for testing (Phase 10.6)
    pub fn simulate_high_load(&mut self) {
        self.local_metrics.cpu_utilization = 95;
        self.local_metrics.active_tasks = 100;
        self.local_metrics.memory_pressure = 90;
    }

    /// Check if migration should be triggered
    pub fn should_migrate(&self) -> bool {
        self.enabled && self.local_metrics.load_score() > 80
    }

    /// Select target device for migration (stub - needs PeerTable integration)
    pub fn select_target_device(&self) -> Option<u32> {
        // TODO: Query PeerTable for least-loaded peer
        // For now, return None (no migration)
        None
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> SystemMetrics {
        self.local_metrics
    }
}

// Global Load Balancer
pub static LOAD_BALANCER: spin::Mutex<LoadBalancer> = spin::Mutex::new(LoadBalancer::new());
