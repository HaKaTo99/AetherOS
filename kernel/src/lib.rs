//! AetherOS Quantum Microkernel v1.0
//! Complete implementation with all subsystems

#![no_std]

pub mod memory;
pub mod scheduler;
pub mod bus;
pub mod oracle;
pub mod ui;

pub use memory::smme::SymbianModernMemoryEngine;
pub use scheduler::ActiveObjectScheduler;
pub use bus::DeviceMesh;
pub use oracle::TinyMLPredictor;

#[repr(C)]
pub struct MemoryStats {
    pub reserved: usize,
    pub committed: usize,
}

/// Global SMME instance
static mut SMME: SymbianModernMemoryEngine = SymbianModernMemoryEngine::new(1 << 30);

/// Global Scheduler instance
static mut SCHEDULER: ActiveObjectScheduler = ActiveObjectScheduler::new();

/// Global Device Mesh
static mut DEVICE_MESH: DeviceMesh = DeviceMesh::new();

/// Global Oracle Engine
static mut ORACLE: TinyMLPredictor = TinyMLPredictor::new();

pub fn kernel_init() {
    unsafe {
        // 1. Initialize SMME
        match SMME.allocate(1 << 20) {
            Ok(_addr) => {
                // Successfully allocated 1MB for kernel data
                ORACLE.record_allocation(1 << 20);
            }
            Err(_) => {
                // Handle allocation failure
            }
        }

        // 2. Initialize Scheduler
        let _ = SCHEDULER.create_object(10); // High priority system task
        let _ = SCHEDULER.create_object(5);  // Normal priority task

        // 3. Discover devices in mesh
        DEVICE_MESH.discover();

        // 4. Initialize Oracle predictions
        let predicted = ORACLE.predict_next_size();
        // Pre-allocate based on prediction
        let _ = SMME.allocate(predicted);
    }
}

pub fn kernel_tick() {
    unsafe {
        // 1. Schedule active objects
        SCHEDULER.schedule();

        // 2. Check memory pressure and cleanup if needed
        let stats = SMME.stats();
        let utilization = (stats.total_committed * 100) / (1 << 30);

        if utilization > 80 {
            let _freed = SMME.predictive_cleanup();
            // Log cleanup results
        }

        // 3. Update Oracle with current state
        ORACLE.record_allocation(stats.total_committed);

        // 4. Check for distributed opportunities
        if ORACLE.should_distribute(stats.total_committed) {
            // Find remote device for offloading
            let _ = DEVICE_MESH.find_best_device(
                stats.total_committed / 2,
                100 // 1 TFLOPS
            );
        }
    }
}

/// Reset kernel state for testing
pub fn kernel_reset() {
    unsafe {
        SMME = SymbianModernMemoryEngine::new(1 << 30);
        SCHEDULER = ActiveObjectScheduler::new();
        DEVICE_MESH = DeviceMesh::new();
        ORACLE = TinyMLPredictor::new();
    }
}

// Kernel API exports
#[no_mangle]
pub extern "C" fn aether_allocate(size: usize) -> usize {
    unsafe {
        match SMME.allocate(size) {
            Ok(addr) => addr,
            Err(_) => 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn aether_get_memory_stats() -> (usize, usize) {
    unsafe {
        let stats = SMME.stats();
        (stats.total_reserved, stats.total_committed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_init() {
        kernel_init();

        unsafe {
            let stats = SMME.stats();
            assert!(stats.total_committed > 0);

            let sched_stats = SCHEDULER.stats();
            assert_eq!(sched_stats.total_objects, 2);

            assert_eq!(DEVICE_MESH.device_count(), 1);
        }
    }

    #[test]
    fn test_kernel_api() {
        kernel_init();

        let addr = aether_allocate(4096);
        assert!(addr > 0);

        let (_reserved, committed) = aether_get_memory_stats();
        assert!(committed >= 4096);
    }
}
