//! AetherOS Quantum Microkernel v1.0
//! Complete implementation with all subsystems

#![no_std]
#![allow(static_mut_refs)] // Phase 3 stability: Allow until Phase 4 sync implemented
#[macro_use]
extern crate alloc;

pub mod memory;
pub mod scheduler;
pub mod bus;
pub mod oracle;
pub mod ui;
pub mod hal;

pub mod virt; // [NEW] Virtualization module
pub mod arch; // [NEW] Architecture module
pub mod panic; // [NEW] Panic handler
#[cfg(target_arch = "aarch64")]
pub mod debug; // [NEW] Debug utilities (GDB stub)

pub mod testing; // [NEW] Test framework
pub mod drivers; // [NEW] Driver framework
pub mod loader; // [NEW] Binary loader (ELF)
pub mod syscall; // [NEW] POSIX Syscall Layer
pub mod runtime; // [NEW] High-level runtimes (WASM, ART)
pub mod security; // [NEW] Capability & Security Model
pub mod net;      // [NEW] Networking Stack (Phase 5)
pub mod ipc;      // [NEW] IPC & RPC (Phase 5.2)
pub mod ai;       // [NEW] AI Inference (Phase 5.4)
pub mod distributed; // [NEW] Distributed Computing (Phase 8)

pub mod tests;    // [NEW] Functional Test Suite (Phase 6.2)

use crate::memory::smme::SymbianModernMemoryEngine;

// Global Allocator (Required for smoltcp/alloc)
#[global_allocator]
static ALLOCATOR: memory::smme::SymbianModernMemoryEngine = memory::smme::SymbianModernMemoryEngine::new(1024 * 1024 * 128); // 128MB Heap

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
        // -1. Initialize Stack Canary
        init_stack_canary();
        
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
                platform.puts("0x");
                platform.puts("...\r\n");
                
                // Try to parse DTB header
                use crate::drivers::dtb::DeviceTree;
                if let Some(_dt) = DeviceTree::from_raw(dtb_ptr as *const u8) {
                    platform.puts("Valid DTB detected.\r\n");
                } else {
                    platform.puts("Invalid DTB Header\r\n");
                }
            } else {
                platform.puts("No DTB provided (x0 = 0)\r\n");
            }
        }

        
        #[cfg(target_arch = "x86_64")]
        {
            static X86: hal::x86_64::X86Platform = hal::x86_64::X86Platform::new();
            hal::init_platform(&X86);

            // Initialize VGA Graphics (Phase 7.1)
            use crate::drivers::video::vga::VgaTextDriver;
            static mut VGA: VgaTextDriver = VgaTextDriver::new();
            use crate::drivers::video::{self, Color};
            
            video::register_driver(&mut VGA);
            
            // Initialize PS/2 Keyboard (Polling mode - Phase 7.3)
            use crate::drivers::input::ps2;
            unsafe { ps2::KEYBOARD.init(); }
            
            // Draw UI Demo using Phase 7.2 Framework
            video::draw(|fb| {
                fb.init();
                fb.clear(Color::BLUE); // Blue background

                // Imports
                use crate::ui::{Label, Button, Rect, Widget, FlexLayout};
                use crate::ui::layout::{Direction, Alignment};

                // Define Container Area
                let container = Rect::new(0, 0, fb.width(), fb.height());

                // Define Items
                let label = Label::new("AetherOS v1.7", 0, 0, Color::WHITE);
                let btn = Button::new("Login", 0, 0, 100, 30);

                // Layout Engine
                let layout = FlexLayout {
                    direction: Direction::Column,
                    justify_content: Alignment::Start,
                    align_items: Alignment::Center,
                    padding: 50,
                    gap: 20,
                };

                // Calculate Positions
                let item_rects = layout.layout(container, &[label.area(), btn.area()]);

                // Create positioned widgets
                let mut positioned_label = simple_clone_label(&label); 
                positioned_label.area = item_rects[0];

                let mut positioned_btn = simple_clone_btn(&btn);
                positioned_btn.area = item_rects[1];

                // Draw
                positioned_label.draw(fb);
                positioned_btn.draw(fb);
            });

            // Helper for demo (because widgets own content)
            fn simple_clone_label(l: &crate::ui::Label) -> crate::ui::Label {
               crate::ui::Label::new(&l.text, l.area.x, l.area.y, l.color)
            }
            fn simple_clone_btn(b: &crate::ui::Button) -> crate::ui::Button {
               crate::ui::Button::new(&b.label, b.area.x, b.area.y, b.area.width, b.area.height)
            }

            // Run Functional Test Suite (Phase 6.2)
            crate::tests::run_suite();
        }

        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        {
            // Use StubPlatform for testing on host
            static STUB: hal::stub::StubPlatform = hal::stub::StubPlatform;
            hal::init_platform(&STUB);
        }

        // Print initialization message
        let platform = hal::get_platform();
        platform.puts("Kernel OK\n");

        // 1. Initialize MMU (must be before heap allocation)
        #[cfg(target_arch = "aarch64")]
        {
            use crate::memory::mmu::Mmu;
            Mmu::init();
            
            // Install exception vector table
            use crate::arch::aarch64::exceptions;
            exceptions::install_vector_table();
        }

        // Initialize Driver Manager using DTB
        use crate::drivers::DriverManager;
        DriverManager::init(dtb_ptr);

        // Initialize Power Management (RPi4 only)
        #[cfg(target_arch = "aarch64")]
        {
            use crate::drivers::dtb::DeviceTree;
            let dt = if dtb_ptr != 0 {
                DeviceTree::from_raw(dtb_ptr as *const u8)
            } else {
                None
            };

            // Initialize mailbox driver
            use crate::drivers::mailbox;
            mailbox::init();

            // Initialize DVFS (CPU frequency scaling)
            use crate::drivers::dvfs;
            dvfs::init(dt.as_ref());

            // Initialize power domain controller
            use crate::drivers::power;
            power::init();
        }

        // 2. Initialize SMME
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        match smme.allocate(1 << 20) {
            Ok(_addr) => {
                let oracle = &mut *core::ptr::addr_of_mut!(ORACLE);
                oracle.record_allocation(1 << 20);
            }
            Err(_) => {}
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
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        let _ = smme.allocate(predicted);

        // 5. Initialize Distributed Computing (Phase 8)
        use crate::distributed::{MIGRATION_MANAGER, KV_STORE, LOAD_BALANCER};
        MIGRATION_MANAGER.init();
        KV_STORE.init();
        LOAD_BALANCER.init();
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
        }

        // 3. Update Oracle with current state
        let oracle = &mut *core::ptr::addr_of_mut!(ORACLE);
        oracle.record_allocation(stats.total_committed);

        // 4. Check for distributed opportunities
        if oracle.should_distribute(stats.total_committed) {
            let device_mesh = &mut *core::ptr::addr_of_mut!(DEVICE_MESH);
            let _ = device_mesh.find_best_device(
                stats.total_committed / 2,
                100 // 1 TFLOPS
            );
        }

        // 5. Poll keyboard input (Phase 7.3)
        #[cfg(target_arch = "x86_64")]
        {
            use crate::drivers::input::ps2;
            if let Some(_event) = ps2::KEYBOARD.poll() {
                // TODO: Push to event queue or handle
                // For now, keyboard input is captured but not processed visually
            }
        }

        // 6. Update load balancer metrics (Phase 8.3)
        use crate::distributed::LOAD_BALANCER;
        let scheduler = &*core::ptr::addr_of!(SCHEDULER);
        let smme = &*core::ptr::addr_of!(SMME);
        LOAD_BALANCER.update_metrics(scheduler, smme);

        // Check if migration needed
        if LOAD_BALANCER.should_migrate() {
            // TODO: Trigger migration via MIGRATION_MANAGER
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

#[repr(C)]
pub struct MemoryStatsFFI {
    pub reserved: usize,
    pub committed: usize,
}

#[no_mangle]
pub extern "C" fn aether_get_memory_stats() -> MemoryStatsFFI {
    unsafe {
        let smme = &mut *core::ptr::addr_of_mut!(SMME);
        let stats = smme.stats();
        MemoryStatsFFI {
            reserved: stats.total_reserved,
            committed: stats.total_committed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_init() {
        kernel_reset();
        kernel_init(0);
        unsafe {
            let smme = &mut *core::ptr::addr_of_mut!(SMME);
            let stats = smme.stats();
            assert!(stats.total_committed > 0);
        }
    }
}

// --- Stack Canary Support ---
#[no_mangle]
static mut __stack_chk_guard: usize = 0xDEADC0DE; 

#[no_mangle]
extern "C" fn __stack_chk_fail() -> ! {
    panic!("Stack Smashing Detected! Canary corrupted.");
}

pub fn init_stack_canary() {
    unsafe {
        __stack_chk_guard = 0xDEAD_BEEF_CAFE_BABE; 
    }
}
