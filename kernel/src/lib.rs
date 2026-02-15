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
pub mod distributed; // [NEW] Distributed Computing (Phase 8 & 17)
pub mod enterprise;  // [NEW] Enterprise & Cloud (Phase 18)
pub mod events;      // [NEW] Event Queue System (Phase 12.2)
pub mod quantum;     // [NEW] Quantum Computing (Phase 19.3)

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
pub static SMME: spin::Mutex<SymbianModernMemoryEngine> = spin::Mutex::new(SymbianModernMemoryEngine::new(1 << 30));

/// Global Scheduler instance
pub static SCHEDULER: spin::Mutex<ActiveObjectScheduler> = spin::Mutex::new(ActiveObjectScheduler::new());

/// Global Device Mesh
pub static DEVICE_MESH: spin::Mutex<DeviceMesh> = spin::Mutex::new(DeviceMesh::new());

/// Global Oracle Engine
pub static ORACLE: spin::Mutex<TinyMLPredictor> = spin::Mutex::new(TinyMLPredictor::new());

/// Global Network Stack (Phase 5)
pub static NETWORK: spin::Mutex<Option<crate::net::NetworkStack>> = spin::Mutex::new(None);

/// Global Distributed System (Phase 8) - Exposed for testing
pub use distributed::{MIGRATION_MANAGER, KV_STORE, LOAD_BALANCER};

use core::sync::atomic::{AtomicUsize, Ordering};
static TICK_COUNTER: AtomicUsize = AtomicUsize::new(0);

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
        {
            let smme = SMME.lock();
            match smme.allocate(1 << 20) {
                Ok(_addr) => {
                    let mut oracle = ORACLE.lock();
                    oracle.record_allocation(1 << 20);
                }
                Err(_) => {}
            }
        }

        // 2. Initialize Scheduler
        {
            let mut scheduler = SCHEDULER.lock();
            let _ = scheduler.create_object(10); // High priority system task
            let _ = scheduler.create_object(5);  // Normal priority task
        }

        // 3. Discover devices in mesh
        {
            let mut device_mesh = DEVICE_MESH.lock();
            device_mesh.discover();
        }

        // 4. Initialize Oracle predictions
        {
            let oracle = ORACLE.lock();
            let predicted = oracle.predict_next_size();
            let smme = SMME.lock();
            let _ = smme.allocate(predicted);
        }

        // 5. Initialize Distributed Computing (Phase 8)
        use crate::distributed::{MIGRATION_MANAGER, KV_STORE, LOAD_BALANCER};
        MIGRATION_MANAGER.lock().init();
        KV_STORE.lock().init();
        LOAD_BALANCER.lock().init();

        // 6. Initialize Network Stack (Phase 5)
        {
            let mut network = NETWORK.lock();
            *network = Some(crate::net::NetworkStack::new());
        }
        
        // 6. User Mode Demo (Phase 6.4)
        #[cfg(target_arch = "aarch64")]
        {
            crate::loader::user_demo::run_user_demo();
        }

        // 7. Phase 16.1: Universal App Runtime (QuickJS Demo)
        {
            use crate::runtime::QuickJsRuntime;
            let mut js_runtime = QuickJsRuntime::new();
            // This simulates loading "vscode-web-core.js" or "antigravity-agent.js"
            if let Ok(result) = js_runtime.eval("console.log('Hello from AetherOS Universal Runtime!')") {
                let platform = hal::get_platform();
                platform.puts("[Kernel] JS Execution Success: ");
                platform.puts(&result);
                platform.puts("\r\n");
            }
        }

        // 8. Phase 16.2: AI Agent Runtime (WASM Inference Demo)
        {
            use crate::runtime::AiAgentRuntime;
            // Initialize with "Llama-7B-WASM"
            let mut agent = AiAgentRuntime::new("Llama-7B-Quantized");
            
            // Simulate chat
            if let Ok(response) = agent.chat("Hello AetherOS, what is your status?") {
                let platform = hal::get_platform();
                platform.puts("\r\n[Kernel] AI Agent Response:\r\n");
                platform.puts(&response);
                platform.puts("\r\n");
            }
        }

        // 9. Phase 16.4: Universal Data Services (SQL Demo)
        {
            use crate::runtime::DatabaseRuntime;
            let mut db = DatabaseRuntime::new("users.db");
            
            // Simulate SQL Workflow
            let _ = db.query("CREATE TABLE users (id INT, name TEXT)");
            let _ = db.query("INSERT INTO users VALUES (1, 'Alice')");
            
            if let Ok(results) = db.query("SELECT * FROM users") {
                let platform = hal::get_platform();
                platform.puts("\r\n[Kernel] SQL Query Results:\r\n");
                for row in results {
                    platform.puts(" - ");
                    platform.puts(&row);
                    platform.puts("\r\n");
                }
            }
        }

        // 10. Phase 16.5: Universal App Frameworks (Laravel Demo)
        {
            use crate::runtime::PhpRuntime;
            // 1. Simulate Laravel Artisan CLI
            let mut artisan = PhpRuntime::new("/var/www/laravel/artisan");
            let _ = artisan.execute();

            // 2. Simulate Web Request
            let mut index = PhpRuntime::new("/var/www/laravel/public/index.php");
            let _ = index.execute();
        }

        // 11. Phase 16.2: Universal Terminal Tools (PTY/Shell)
        {
            use crate::runtime::TerminalRuntime;
            let mut term = TerminalRuntime::new();
            let _pty_id = term.open_terminal();
            term.run_command("vim");
        }

        // 12. Phase 16.3: Self-Hosting Capabilities (Rustc/Git)
        {
            use crate::runtime::DevTools;
            let mut dev = DevTools::new();
            let _ = dev.git_clone("https://github.com/HaKaTo99/AetherOS");
            dev.cargo_build();
        }

        // 13. Phase 16.6: Universal Multimedia (Movie & Camera)
        {
            use crate::runtime::MediaRuntime;
            
            // 1. Play Movie
            let mut player = MediaRuntime::new("Avatar_The_Way_of_Water.mkv");
            let _ = player.play();

            // 2. Camera Capture
            let mut cam = MediaRuntime::new("/dev/video0");
            let _ = cam.capture();
        }

        // 13. Phase 17: Distributed Orchestration (Mesh & Market)
        {
            use crate::distributed::{MESH_NETWORK, CAPABILITY_MARKET, DIST_STORAGE, GLOBAL_DHT};
            let mut mesh = MESH_NETWORK.lock();
            let mut market = CAPABILITY_MARKET.lock();
            let mut storage = DIST_STORAGE.lock();
            let mut dht = GLOBAL_DHT.lock(); // DHT is thread-safe internally or we lock it here

            mesh.init();
            storage.init();
            market.init();
            
            // Phase 19.1: Join Global DHT
            dht.bootstrap([8, 8, 8, 8]); // Bootstrap via Google DNS IP (hypothetical bootnode)

            // Simulate Discovery & Trading
            let neighbors = mesh.discover();
            if neighbors > 0 {
                use crate::distributed::market::ResourceType;
                market.place_bid(1, ResourceType::Compute(10), 50); // Bid 50 AT for 10 TFLOPS
                market.place_ask(2, ResourceType::Compute(10), 45); // Ask 45 AT
            }
        }

        // 13. Phase 18: Enterprise & Cloud (RBAC, Cloud-Init, Telemetry)
        {
            use crate::enterprise::{CLOUD_MANAGER, RBAC_SYSTEM, TELEMETRY_AGENT};
            let mut cloud = CLOUD_MANAGER.lock();
            let mut rbac = RBAC_SYSTEM.lock();
            let mut telemetry = TELEMETRY_AGENT.lock();

            cloud.init();
            rbac.init();
            telemetry.init();

            // Simulate Enterprise Workflow
            if rbac.login("root") {
                telemetry.collect_metrics();
                telemetry.push_heartbeat();
            }
        }

        // 14. Phase 19: Internet of Abilities (v5.0)
        {
            // 19.2 AI-Native OS
            use crate::ai::{GLOBAL_NPU, NpuDriver};
            let mut npu = GLOBAL_NPU.lock();
            let _ = npu.init();

            // 19.3 Quantum Computing
            use crate::quantum::{GLOBAL_QPU, Complex};
            let mut qpu = GLOBAL_QPU.lock();
            let q_idx = qpu.allocate_qubit();
            if let Some(qubit) = qpu.qubits.get_mut(q_idx) {
                qubit.h_gate(); // Create superposition
            }

            // 19.4 Brain-Computer Interface
            use crate::drivers::bci::NeuralLink;
            use crate::drivers::Driver;
            let mut neural = NeuralLink::new(0xABC00000);
            unsafe { let _ = neural.init(); }

            // "The Singularity" Demo
            if let Some(signal) = neural.read_signal() {
                if signal.beta_wave > 0.7 {
                    let platform = hal::get_platform();
                    platform.puts("\r\n[AetherOS] Thought Detected! Collapsing Quantum State...\r\n");
                    let result = qpu.run_measure(q_idx);
                     platform.puts(if result { " -> State |1>\r\n" } else { " -> State |0>\r\n" });
                }
            }
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!(crate::hal::ConsoleWriter, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn kernel_tick() {
    // 1. Schedule active objects
    {
        let mut scheduler = SCHEDULER.lock();
        scheduler.schedule();
    }

    // 2. Check memory pressure and cleanup if needed
    {
        let smme = SMME.lock();
        let stats = smme.stats();
        let utilization = (stats.total_committed * 100) / (1 << 30);

        if utilization > 80 {
            let _freed = smme.predictive_cleanup();
        }

        // 3. Update Oracle with current state
        let mut oracle = ORACLE.lock();
        oracle.record_allocation(stats.total_committed);

        // 4. Check for distributed opportunities
        if oracle.should_distribute(stats.total_committed) {
             let device_mesh = DEVICE_MESH.lock();
             let _ = device_mesh.find_best_device(
                 stats.total_committed / 2,
                 100 // 1 TFLOPS
             );
        }
    }

    // 5. Poll keyboard input (Phase 7.3)
    #[cfg(target_arch = "x86_64")]
    {
            use crate::drivers::input::ps2;
            unsafe {
                if let Some(_event) = ps2::KEYBOARD.poll() {
                    // TODO: Push to event queue or handle
                    // For now, keyboard input is captured but not processed visually
                }
            }
        }

        // 6. Update load balancer metrics (Phase 8.3)
        use crate::distributed::LOAD_BALANCER;
        // Scope guards for safe locking
        {
            let scheduler = SCHEDULER.lock();
            let smme = SMME.lock();
            LOAD_BALANCER.lock().update_metrics(&scheduler, &smme);
        }

        // Check if migration needed
        if LOAD_BALANCER.lock().should_migrate() {
            use crate::distributed::MIGRATION_MANAGER;
            let mut migration = MIGRATION_MANAGER.lock();
            let _ = migration.migrate_task(1, 2); // Fake task ID 1 to Fake Device 2
        }

        // --- Phase 10.6: Internal Simulation & Stress Test ---
        let ticks = TICK_COUNTER.fetch_add(1, Ordering::Relaxed);
        if ticks % 100 == 0 {
            // Every 100 ticks, simulate high load
            let mut lb = LOAD_BALANCER.lock();
            lb.simulate_high_load();
            
            // Log simulation
            unsafe {
                if let Some(platform) = crate::hal::try_get_platform() {
                     platform.puts("[SIM] High Load Simulated! Triggering Migration...\r\n");
                }
            }
        }
        // -----------------------------------------------------

        // 7. Poll Network Stack (Phase 5)
        {
            let mut network = NETWORK.lock();
            if let Some(stack) = network.as_mut() {
                // TODO: Get real timestamp
                stack.poll(0i64);
            }
        }
    }

/// Reset kernel state for testing
pub fn kernel_reset() {
    *SMME.lock() = SymbianModernMemoryEngine::new(1 << 30);
    *SCHEDULER.lock() = ActiveObjectScheduler::new();
    *DEVICE_MESH.lock() = DeviceMesh::new();
    *ORACLE.lock() = TinyMLPredictor::new();
}

// Kernel API exports
#[no_mangle]
pub extern "C" fn aether_allocate(size: usize) -> usize {
    let smme = SMME.lock();
    match smme.allocate(size) {
        Ok(addr) => addr,
        Err(_) => 0,
    }
}

#[repr(C)]
pub struct MemoryStatsFFI {
    pub reserved: usize,
    pub committed: usize,
}

#[no_mangle]
pub extern "C" fn aether_get_memory_stats() -> MemoryStatsFFI {
    let smme = SMME.lock();
    let stats = smme.stats();
    MemoryStatsFFI {
        reserved: stats.total_reserved,
        committed: stats.total_committed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_init() {
        kernel_reset();
        kernel_init(0);
        {
            let smme = SMME.lock();
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
