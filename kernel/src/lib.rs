//! AetherOS Quantum Microkernel v1.0
//! Complete implementation with all subsystems

#![no_std]

pub mod memory;
pub mod scheduler;
pub mod bus;
pub mod oracle;
pub mod ui;
pub mod hal;

use hal::Platform; // Import trait for kernel_init usage

pub mod virt; // [NEW] Virtualization module
pub mod arch; // [NEW] Architecture module
pub mod panic; // [NEW] Panic handler
pub mod debug; // [NEW] Debug utilities (GDB stub)

pub mod testing; // [NEW] Test framework
pub mod drivers; // [NEW] Driver framework

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

pub fn kernel_init(dtb_ptr: usize) {
    unsafe {
        // 0. Initialize HAL
        #[cfg(target_arch = "aarch64")]
        {
            // Use RPiPlatform for real hardware
            static RPI: hal::rpi::RPiPlatform = hal::rpi::RPiPlatform::new();
            hal::init_platform(&RPI);
            
            // Log DTB Pointer
            let platform = hal::get_platform();
            if dtb_ptr != 0 {
                platform.puts("DTB found at: ");
                // TODO: proper hex printing needed, but for now simple ack
                platform.puts("0x");
                // (Hex printing implementation omitted for brevity)
                platform.puts("...\r\n");
                
                // Try to parse DTB header
                use crate::drivers::dtb::DeviceTree;
                if let Some(dt) = DeviceTree::from_raw(dtb_ptr as *const u8) {
                    platform.puts("Valid DTB detected. Size: ");
                    // platform.put_hex(dt.total_size());
                    platform.puts(" bytes\r\n");
                    
                    // Iterate (demo)
                    let nodes = dt.nodes();
                    for item in nodes {
                        // Just iterate to verify
                    }
                    platform.puts("DTB Traversal OK\r\n");
                } else {
                    platform.puts("Invalid DTB Header\r\n");
                }
            } else {
                platform.puts("No DTB provided (x0 = 0)\r\n");
            }
        }

        
        #[cfg(not(target_arch = "aarch64"))]
        {
            // Use StubPlatform for testing on host
            static STUB: hal::stub::StubPlatform = hal::stub::StubPlatform;
            hal::init_platform(&STUB);
        }

        // Print initialization message
        let platform = hal::get_platform();
        platform.put_char(b'K');
        platform.put_char(b'e');
        platform.put_char(b'r');
        platform.put_char(b'n');
        platform.put_char(b'e');
        platform.put_char(b'l');
        platform.put_char(b' ');
        platform.put_char(b'O');
        platform.put_char(b'K');
        platform.put_char(b'\n');

        // 1. Initialize MMU (must be before heap allocation)
        #[cfg(target_arch = "aarch64")]
        {
            use crate::memory::mmu::Mmu;
            Mmu::init();
            
            if Mmu::is_enabled() {
                platform.put_char(b'M');
                platform.put_char(b'M');
                platform.put_char(b'U');
                platform.put_char(b' ');
                platform.put_char(b'O');
                platform.put_char(b'K');
                platform.put_char(b'\n');
            }
            
            // Install exception vector table
            use crate::arch::aarch64::exceptions;
            exceptions::install_vector_table();
            platform.put_char(b'I');
            platform.put_char(b'R');
            platform.put_char(b'Q');
            platform.put_char(b' ');
            platform.put_char(b'O');
            platform.put_char(b'K');
            platform.put_char(b'\n');
        }

        // 2. Initialize SMME
        // Use addr_of_mut! to avoid creating a reference to static mut which is UB/Error in 2024
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        match smme.allocate(1 << 20) {
            Ok(_addr) => {
                // Successfully allocated 1MB for kernel data
                let oracle = &mut *core::ptr::addr_of_mut!(ORACLE);
                oracle.record_allocation(1 << 20);
            }
            Err(_) => {
                // Handle allocation failure
            }
        }

        // 2. Initialize Scheduler
        let scheduler = &mut *core::ptr::addr_of_mut!(SCHEDULER);
        let _ = scheduler.create_object(10); // High priority system task
        let _ = scheduler.create_object(5);  // Normal priority task

        // 3. Discover devices in mesh
        let device_mesh = &mut *core::ptr::addr_of_mut!(DEVICE_MESH);
        device_mesh.discover();

        // 4. Initialize Oracle predictions
        let oracle = &mut *core::ptr::addr_of_mut!(ORACLE);
        let predicted = oracle.predict_next_size();
        // Pre-allocate based on prediction
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        let _ = smme.allocate(predicted);
    }
}

pub fn kernel_tick() {
    unsafe {
        // 1. Schedule active objects
        let scheduler = &mut *core::ptr::addr_of_mut!(SCHEDULER);
        scheduler.schedule();

        // 2. Check memory pressure and cleanup if needed
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        let stats = smme.stats();
        let utilization = (stats.total_committed * 100) / (1 << 30);

        if utilization > 80 {
            let _freed = smme.predictive_cleanup();
            // Log cleanup results
        }

        // 3. Update Oracle with current state
        let oracle = &mut *core::ptr::addr_of_mut!(ORACLE);
        oracle.record_allocation(stats.total_committed);

        // 4. Check for distributed opportunities
        if oracle.should_distribute(stats.total_committed) {
            // Find remote device for offloading
            let device_mesh = &mut *core::ptr::addr_of_mut!(DEVICE_MESH);
            let _ = device_mesh.find_best_device(
                stats.total_committed / 2,
                100 // 1 TFLOPS
            );
        }
    }
}

/// Reset kernel state for testing
pub fn kernel_reset() {
    unsafe {
        *core::ptr::addr_of_mut!(SMME) = SymbianModernMemoryEngine::new(1 << 30);
        *core::ptr::addr_of_mut!(SCHEDULER) = ActiveObjectScheduler::new();
        *core::ptr::addr_of_mut!(DEVICE_MESH) = DeviceMesh::new();
        *core::ptr::addr_of_mut!(ORACLE) = TinyMLPredictor::new();
    }
}

// Kernel API exports
#[no_mangle]
pub extern "C" fn aether_allocate(size: usize) -> usize {
    unsafe {
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        match smme.allocate(size) {
            Ok(addr) => addr,
            Err(_) => 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn aether_get_memory_stats() -> (usize, usize) {
    unsafe {
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        let stats = smme.stats();
        (stats.total_reserved, stats.total_committed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_init() {
        // Reset state first to ensure clean slate
        kernel_reset();
        kernel_init(0); // Pass 0 for testing

        unsafe {
            let smme = &mut *core::ptr::addr_of_mut!(SMME);
            let stats = smme.stats();
            assert!(stats.total_committed > 0);

            let scheduler = &mut *core::ptr::addr_of_mut!(SCHEDULER);
            let sched_stats = scheduler.stats();
            assert_eq!(sched_stats.total_objects, 2);

            let device_mesh = &mut *core::ptr::addr_of_mut!(DEVICE_MESH);
            assert_eq!(device_mesh.device_count(), 1);
        }
    }

    #[test]
    fn test_kernel_api() {
        kernel_reset();
        kernel_init(0); // Pass 0 for testing

        let addr = aether_allocate(4096);
        assert!(addr > 0);
        
        // stats variable undefined unless committed is checked via api
        let (reserved, committed) = aether_get_memory_stats();
        assert!(committed >= 4096);
    }
}
