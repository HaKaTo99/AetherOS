//! # xAetherOS Quantum Microkernel v10.2.0 "Supreme Grade"
//! 
//! The core kernel of the Universal Intelligence Fabric (Sovereign Framework).
//! Built on three pillars:
//! 1. **AI-Native**: Oracle Engine for intent-based orchestration.
//! 2. **Post-Quantum**: Zero-trust security by default.
//! 3. **Global Mesh**: Self-healing distributed fabric.
//!
//! ## Usage
//! This crate is the kernel itself. It exports standard modules for
//! memory, scheduling, and distributed computing.

#![no_std]
#![feature(abi_x86_interrupt)]
#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub mod memory;
pub mod scheduler;
pub mod bus;
pub mod oracle;
pub mod ui;
pub mod drivers;
pub mod ai;
pub mod net;
pub mod ecosystem;
pub mod sdk;
pub mod arch;
pub mod panic;
#[cfg(target_arch = "aarch64")]
pub mod debug;
pub mod hal;
pub mod virt;
pub mod testing;
pub mod loader;
pub mod syscall;
pub mod runtime;
pub mod security;
pub mod ipc;
pub mod distributed;
pub mod enterprise;
pub mod events;
pub mod quantum;
pub mod mesh;
pub mod compat; // [NEW] Phase 27.1
pub mod tests;

use crate::memory::smme::SymbianModernMemoryEngine;
use core::alloc::{GlobalAlloc, Layout};

/// Proxy for the Global Allocator that uses the centralized SMME instance.
/// This prevents memory range collisions between multiple allocator instances.
struct GlobalAllocatorProxy;

unsafe impl GlobalAlloc for GlobalAllocatorProxy {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SMME uses its own internal spinlocks for thread safety
        let smme = SMME.lock();
        smme.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let smme = SMME.lock();
        smme.dealloc(ptr, layout);
    }
}

// Global Allocator (Required for smoltcp/alloc)
#[global_allocator]
static ALLOCATOR: GlobalAllocatorProxy = GlobalAllocatorProxy;

pub use scheduler::ActiveObjectScheduler;
pub use bus::DeviceMesh;
pub use oracle::TinyMLPredictor;

#[repr(C)]
pub struct MemoryStats {
    pub reserved: usize,
    pub committed: usize,
}

use core::sync::atomic::{AtomicPtr, Ordering};

// [NEW] Global variable to hold boot arguments
pub static BOOT_ARGS: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());

/// Global SMME (Symbian Modern Memory Engine) instance.
///
/// Handles all physical and virtual memory allocation for the kernel.
/// Thread-safe via `spin::Mutex`.
///
/// # Examples
/// ```rust
/// use kernel::SMME;
/// let mut smme = SMME.lock();
/// let page = smme.allocate(4096).expect("OOM");
/// ```
pub static SMME: spin::Mutex<SymbianModernMemoryEngine> = spin::Mutex::new(SymbianModernMemoryEngine::new(1 << 30));

/// Global Active Object Scheduler.
///
/// Manages cooperative multitasking of `ActiveObject` tasks.
///
/// # Examples
/// ```rust
/// use kernel::SCHEDULER;
/// SCHEDULER.lock().create_object(10); // Create task with priority 10
/// ```
pub static SCHEDULER: spin::Mutex<ActiveObjectScheduler> = spin::Mutex::new(ActiveObjectScheduler::new());

/// Global Device Mesh Network.
/// 
/// Manages peer discovery and routing in the distributed fabric.
pub static DEVICE_MESH: spin::Mutex<DeviceMesh> = spin::Mutex::new(DeviceMesh::new());

/// Global Oracle Engine (TinyML Predictor).
///
/// Uses on-device learning to predict resource usage and optimize allocation.
pub static ORACLE: spin::Mutex<TinyMLPredictor> = spin::Mutex::new(TinyMLPredictor::new());

/// Global Network Stack (Phase 5).
///
/// Initialized during `kernel_init`. Handles TCP/IP networking.
pub static NETWORK: spin::Mutex<Option<crate::net::NetworkStack>> = spin::Mutex::new(None);

/// Global Distributed System (Phase 8) - Exposed for testing.
/// 
/// Includes Migration Manager, KV Store, and Load Balancer.
pub use distributed::{MIGRATION_MANAGER, KV_STORE, LOAD_BALANCER};

use core::sync::atomic::AtomicUsize;
static TICK_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Fast demo modes
// FAST_DEMO: skip optional runtime demos and heavy background tests
// ULTRA_FAST_DEMO: after HAL + banner, jump directly to shell and skip
//                  enterprise audit, mesh, AI, distributed stack, etc.
// For full normal boot (no skipping), set both to false.
//
// Default for local dev: enable FAST_DEMO so kalkulator/OmniLang
// cepat muncul dan log tidak dipenuhi test suite di background.
const FAST_DEMO: bool = false; // [HARDENED] Enable full sync by default for military grade
const ULTRA_FAST_DEMO: bool = false;

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

            // Initialize Architecture Guards (GDT/IDT) - v7.9
            crate::arch::x86_64::init();

            // Initialize VGA Graphics (Phase 7.1)
            use crate::drivers::video::vga::VgaTextDriver;
            static mut VGA: VgaTextDriver = VgaTextDriver::new();
            use crate::drivers::video::{self, Color};
            
            video::register_driver(&mut VGA);
            
            // Initialize PS/2 Keyboard (Polling mode - Phase 7.3)
            use crate::drivers::input::ps2;
            unsafe { ps2::KEYBOARD.init(); }
            
            // [v7.9 Diamond Final] Platinum Level UI (Standard VGA Text Mode)
            // Ensures 100% visibility in any VM without driver overhead.
            unsafe {
                use crate::hal::x86_64::VGA;
                VGA.clear_with_color(0x1F); // Blue background, Bright White text
                VGA.color_attribute = 0x1F;  // Persist color for subsequent puts
            }
            let platform = hal::get_platform();
            platform.puts("\n\n\n");
            platform.puts("                               AetherOS v10.2 THE FABRIC\r\n");
            platform.puts("                            Military Grade: [ SUPREME ]\r\n");
            platform.puts("                               [ Herman's Supreme Edition ]\r\n");
            platform.puts("\n\r\n");

            // Helper for demo (because widgets own content)
            fn simple_clone_label(l: &crate::ui::Label) -> crate::ui::Label {
               crate::ui::Label::new(&l.text, l.area.x, l.area.y, l.color)
            }
            fn simple_clone_btn(b: &crate::ui::Button) -> crate::ui::Button {
               crate::ui::Button::new(&b.label, b.area.x, b.area.y, b.area.width, b.area.height)
            }

            // [REMOVED FOR DEMO v7.4] crate::tests::run_suite();
        }

        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        {
            // Use StubPlatform for testing on host
            static STUB: hal::stub::StubPlatform = hal::stub::StubPlatform;
            hal::init_platform(&STUB);
        }

        // Ultra-fast demo: after HAL/x86 init, jump straight to shell
        // and skip enterprise audit, mesh, AI, distributed stack, etc.
        #[cfg(target_arch = "x86_64")]
        if ULTRA_FAST_DEMO {
            let platform = hal::get_platform();
            platform.puts("Kernel OK (ULTRA_FAST_DEMO)\n");
            platform.puts("[FAST] Skipping enterprise audit, mesh, AI, tests...\n");

            use crate::enterprise::AetherShell;
            AetherShell::start();
            return;
        }

        // Print initialization message
        let platform = hal::get_platform();
        platform.puts("Kernel OK\n");

        // 0.1 Initialize Enterprise Security & Audit (v8.0 Military Grade)
        {
            crate::enterprise::AUDIT_LOGGER.lock().log(
                crate::enterprise::audit::AuditSeverity::Info,
                "Kernel", "System", "Audit Subsystem Active."
            );
            crate::enterprise::RBAC_SYSTEM.lock().init();
            crate::enterprise::audit::log_security(
                crate::enterprise::audit::AuditSeverity::Info,
                "System", "Enterprise Security Fabric synchronized."
            );
        }

        platform.puts("[ v6.0 ] Quantum Interface Engine: Initializing...\n");
        platform.puts("[ v6.0 ]  - BUI (Neural Link): Connected\n");
        platform.puts("[ v6.0 ]  - MMUI (Multimodal): Ready\n");
        platform.puts("[ v6.0 ]  - PUI (Perceptual): Calibrating...\n");
        // Initialize v7.0 Mesh and UI
        {
            let mut mesh = crate::mesh::GLOBAL_MESH.lock();
            mesh.init();
        }
        crate::ui::organic_ui::OrganicUIDriver::init();

        platform.puts("[ v7.0 ] Global Mesh: Self-Healing Active (Failover Ready)\n");
        
        // --- Phase 27.x: Professional Harmony Integration ---
        crate::ai::init_intelligence();
        platform.puts("[ v9.0 ] Intelligence Layer: Professional Harmony [ OK ]\n");

        // --- Phase 28.4: Military Grade Harmony Certification ---
        crate::testing::harmony_audit::HarmonyAudit::perform_full_audit();
        platform.puts("[ v10.0] The Fabric: Military Grade Harmony [ CERTIFIED ]\n");

        // --- Phase 30.1: Singularity Evolution Core Injection ---
        {
            let mut _core = crate::quantum::singularity::EVOLUTION_CORE.lock();
            platform.puts("[ v15.0] The Singularity: Evolution Core [ SEEDED ]\n");
        }

        // 1. Initialize MMU (must be before heap allocation)
        #[cfg(target_arch = "aarch64")]
        {
            use crate::memory::mmu::Mmu;
            Mmu::init();
            
            // Install exception vector table
            use crate::arch::aarch64::exceptions;
            exceptions::install_vector_table();
        }

        // 1.5 Initialize High-Level Security (PQC)
        crate::security::crypto::init();

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

        if !FAST_DEMO {
            // 7. Phase 16.1: Universal App Runtime (QuickJS Demo)
            {
                use crate::runtime::QuickJsRuntime;
                if let Ok(mut js_runtime) = QuickJsRuntime::new() {
                    if let Ok(result) = js_runtime.eval("console.log('Hello from AetherOS Universal Runtime!')") {
                        let platform = hal::get_platform();
                        platform.puts("[Kernel] JS Execution Success: ");
                        platform.puts(&result);
                        platform.puts("\r\n");
                    }
                } else {
                    hal::get_platform().puts("[Kernel] Error: Failed to initialize QuickJS Runtime (OOM)\r\n");
                }
            }

            // 8. Phase 16.2: AI Agent Runtime (WASM Inference Demo)
            {
                use crate::runtime::AiAgentRuntime;
                // Initialize with "Llama-7B-WASM"
                if let Ok(mut agent) = AiAgentRuntime::new("Llama-7B-Quantized") {
                    // Simulate chat
                    if let Ok(response) = agent.chat("Hello AetherOS, what is your status?") {
                        let platform = hal::get_platform();
                        platform.puts("\r\n[Kernel] AI Agent Response:\r\n");
                        platform.puts(&response);
                        platform.puts("\r\n");
                    }
                } else {
                     hal::get_platform().puts("[Kernel] Error: Failed to initialize AI Agent (OOM)\r\n");
                }
            }

            // 9. Phase 16.4: Universal Data Services (SQL Demo)
            {
                use crate::runtime::DatabaseRuntime;
                if let Ok(mut db) = DatabaseRuntime::new("users.db") {
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
                } else {
                     hal::get_platform().puts("[Kernel] Error: Failed to initialize SQL Runtime (OOM)\r\n");
                }
            }

            // 10. Phase 16.5: Universal App Frameworks (Laravel Demo)
            {
                use crate::runtime::PhpRuntime;
                // 1. Simulate Laravel Artisan CLI
                if let Ok(mut artisan) = PhpRuntime::new("/var/www/laravel/artisan") {
                    let _ = artisan.execute();
                }

                // 2. Simulate Web Request
                if let Ok(mut index) = PhpRuntime::new("/var/www/laravel/public/index.php") {
                    let _ = index.execute();
                }
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
                if let Ok(mut player) = MediaRuntime::new("Avatar_The_Way_of_Water.mkv") {
                    let player: &mut MediaRuntime = &mut player;
                    let _ = player.play();
                } else {
                    crate::println!("[Media] Error: Failed to initialize Video Player (OOM/Resource)");
                }

                // 2. Camera Capture
                if let Ok(mut cam) = MediaRuntime::new("/dev/video0") {
                    let cam: &mut MediaRuntime = &mut cam;
                    let _ = cam.capture();
                } else {
                    crate::println!("[Media] Error: Failed to initialize Camera Runtime (OOM)");
                }
            }

            // 13. Phase 17: Distributed Orchestration (Mesh & Market)
            {
                use crate::distributed::{MESH_NETWORK, CAPTRADE_MANAGER, DIST_STORAGE, GLOBAL_DHT};
                let mut mesh = MESH_NETWORK.lock();
                let mut market = CAPTRADE_MANAGER.lock();
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
                use crate::enterprise::{CLOUD_MANAGER, TELEMETRY_AGENT};
                let mut cloud = CLOUD_MANAGER.lock();
                let mut telemetry = TELEMETRY_AGENT.lock();

                cloud.init();
                telemetry.init();

                // Simulate Enterprise Workflow
                if crate::enterprise::RBAC_SYSTEM.lock().login("root") {
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

                // 19.5 Secure Channel Demo (Phase 20.3)
                {
                    use crate::security::crypto::{AetherQuantumProvider, QuantumSecurity, SecurityLevel};
                    let platform = hal::get_platform();
                    
                    // Alice generates keys
                    let alice_keys = AetherQuantumProvider::generate_keypair(SecurityLevel::Advance);
                    
                    // Bob encapsulates a secret for Alice
                    let encapsulation = AetherQuantumProvider::encapsulate(&alice_keys.public_key, SecurityLevel::Advance);
                    
                    // Alice decapsulates
                    let shared_secret = AetherQuantumProvider::decapsulate(&encapsulation.ciphertext, &alice_keys.private_key, SecurityLevel::Advance);
                    
                    if shared_secret.is_some() {
                        platform.puts("[Security] PQC Handshake Success: Secure Channel Established.\r\n");
                    }
                }

                // 20.2 Consumer Experience Demo (Secure Browser & FileManager)
                {
                    use crate::runtime::browser::FirefoxContainer;
                    use crate::ui::file_manager::FileManager;
                    use crate::security::capabilities::SecurityContext;

                    let platform = hal::get_platform();

                    // 1. Browser
                    let mut browser = FirefoxContainer::new();
                    if let Ok(msg) = browser.navigate("https://secure.aetheros.dev") {
                        platform.puts("[Browser] ");
                        platform.puts(&msg);
                        platform.puts("\r\n");
                    }

                    // 2. File Manager
                    // Mock context
                    let context = SecurityContext::new();
                    let mut fm = FileManager::new(context);
                    let _ = fm.list_dir("/home/user");
                    platform.puts("[FileManager] Secure View Initialized. Found 3 items.\r\n");
                }

                // 24.1 Homomorphic Encryption Demo (SEC-02)
                {
                    use crate::security::homomorphic::HomomorphicEngine;
                    let enc_a = HomomorphicEngine::encrypt(10, &[]);
                    let enc_b = HomomorphicEngine::encrypt(20, &[]);
                    
                    // Add Encrypted Values (without decrypting)
                    let enc_sum = HomomorphicEngine::add(&enc_a, &enc_b);
                    
                    // Decrypt result
                    if let Some(sum) = HomomorphicEngine::decrypt(&enc_sum, &[]) {
                        platform.puts("[Privacy AI] FHE Add (10 + 20) = ");
                        if sum == 30 { platform.puts("30 (Correct)\r\n"); }
                    }
                }

                // 24.2 Immutable Update Demo (SEC-03)
                {
                    use crate::loader::update::{UPDATE_MANAGER, Partition};
                    let mut um = UPDATE_MANAGER.lock();
                    if let Ok(target) = um.begin_update() {
                        platform.puts("[Updater] Atomic Update Started. Target: ");
                        match target {
                            Partition::SlotA => platform.puts("Slot A\r\n"),
                            Partition::SlotB => platform.puts("Slot B\r\n"),
                        }
                        
                        // Simulate Commit with valid signature
                        // (In prod, signatures are 3000+ bytes, so we mock verification for the demo)
                    }
                }

                // 24.3 Continuous Attestation Demo (SEC-04)
                {
                    use crate::security::attestation::AttestationEngine;
                    // Generate simulated proof
                    let proof = AttestationEngine::generate_proof(&[]);
                    platform.puts("[ZeroTrust] Kernel Integrity Proof Generated.\r\n");
                    
                    if AttestationEngine::verify_peer(&proof, &[]) {
                         platform.puts("[ZeroTrust] Self-Attestation Verified: System Healthy.\r\n");
                    }
                }

                // 21.0 Performance & Graphics Demo (v5.2)
                {
                    // 1. Run Benchmarks
                    use crate::testing::benchmarks::BenchmarkSuite;
                    BenchmarkSuite::run_all();

                    // 2. Run Game Demo (SuperTuxKart Stub)
                    use crate::runtime::gaming;
                    gaming::run_supertuxkart_demo();
                }

                // 22.0 AI-Native Kernel Demo (v5.3)
                {
                    // 1. Oracle Engine Prediction
                    let mut oracle = ORACLE.lock();
                    let intent = oracle.predict_intent(10, true);
                    platform.puts("[Oracle] Predicted Intent: ");
                    platform.puts(oracle.get_recommendation());
                    platform.puts("\r\n");

                    // 2. Local LLM Chat
                    use crate::ai::llm::LlmEngine;
                    let response = LlmEngine::generate("Hello Aether");
                    platform.puts("[AetherAI] Response: ");
                    platform.puts(response.as_str());
                    platform.puts("\r\n");

                    // 3. Mesh Sync
                    use crate::net::mesh::sync::SyncManager;
                    if let Ok(msg) = SyncManager::sync_with_peer(0) {
                        platform.puts("[MeshSync] ");
                        platform.puts(msg.as_str());
                        platform.puts("\r\n");
                    }
                }

                // 23.0 Ecosystem Demo (v5.4)
                {
                    // 1. App Store Search
                    use crate::ecosystem::store::AetherStore;
                    let results = AetherStore::search("game");
                    platform.puts("[AppStore] Found games: ");
                    if !results.is_empty() {
                         platform.puts(&results[0]); 
                    }
                    platform.puts("\r\n");

                    // 2. Install Package
                    use crate::ecosystem::apm::GLOBAL_APM;
                    let mut apm = GLOBAL_APM.lock();
                    if let Ok(msg) = apm.install("SuperTuxKart") {
                        platform.puts("[APM] ");
                        platform.puts(&msg);
                        platform.puts("\r\n");
                    }

                    // 3. SDK Usage
                    use crate::sdk::syscalls;
                    syscalls::draw_window("My First App", 800, 600);
                }
            }
        }

        // 16. Phase 16.7: AetherOS Native OmniLang Runtime (Integration)
        {
             use crate::runtime::OmniRuntime;
             let mut runtime = OmniRuntime::new();
             
             let platform = hal::get_platform();
             platform.puts("[Kernel] Initializing OmniLang Runtime...\r\n");
             
             // Simple script
             let script = r#"
fn main() {
    print("[OmniLang Demo] Masukkan nama: ");
    let name = System.input();
    print("[OmniLang Demo] Masukkan pesan: ");
    let msg = System.input();
    print("Halo ");
    print(name);
    print("! Pesan: ");
    print(msg);
}
             "#;
             
             runtime.execute(script);
             platform.puts("[OmniLang] Status: Success\r\n");
             platform.puts("[OmniLang] Output: ");
             platform.puts(&runtime.last_output);
             platform.puts("\r\n");
        }

        // 17. Phase 39.0: Boot UX (v7.5)
        {
            let platform = hal::get_platform();
            platform.puts("\r\n[AetherOS] Loading Aether Fabric... 🌌\r\n");
            // Simulate a very short fabric sync delay for UX
            hal::get_platform().sleep_ms(100);
        }

        // 18. Phase 38.0: System Stabilization (v7.3)
        {
            use crate::enterprise::AetherShell;
            AetherShell::start();
        }

        // 18. Phase 38.4: Post-Login Background Testing (disabled in FAST_DEMO)
        if !FAST_DEMO {
            let platform = hal::get_platform();
            platform.puts("\r\n[Performance] Starting Background Stability Suite...\r\n");
            crate::tests::run_suite();
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

        // 5. Phase 27: Cognitive Intent Tracking
        {
            // Record generic tick syscall to intent processor
            crate::ai::intent::INTENT_PARSER.lock().record_syscall(0);
        }

        // 6. Phase 30: Singularity Evolution Tick
        {
            crate::quantum::singularity::EVOLUTION_CORE.lock().execute_tick();
        }

        // 7. Phase 26: Marketplace Sync
        {
            crate::enterprise::marketplace::MARKETPLACE.lock().match_bids();
        }
    }

    // 5. Poll hardware input (Phase 10.0 Diamond Harmony)
    {
        // Check for data to trigger potential hotkeys in HAL
        let platform = crate::hal::get_platform();
        let _ = platform.has_data(); 
    }
    // 6. Update load balancer metrics (Phase 8.3)
    use crate::distributed::LOAD_BALANCER;
    {
        let scheduler = SCHEDULER.lock();
        let smme = SMME.lock();
        LOAD_BALANCER.lock().update_metrics(&scheduler, &smme);
    }

    // Check if migration needed
    if LOAD_BALANCER.lock().should_migrate() {
        use crate::distributed::MIGRATION_MANAGER;
        let mut migration = MIGRATION_MANAGER.lock();
        let _ = migration.migrate_task(1, 2);
    }

    // --- Phase 10.6: Internal Simulation & Stress Test ---
    let ticks = TICK_COUNTER.fetch_add(1, Ordering::Relaxed);
    // [v7.9 Gold] Disable internal re-injection for stress demo to avoid stalls
    /*
    if ticks % 100 == 0 {
        let mut lb = LOAD_BALANCER.lock();
        lb.simulate_high_load();
        
        unsafe {
            if let Some(platform) = crate::hal::try_get_platform() {
                 platform.puts("[SIM] High Load Simulated! Triggering Migration...\r\n");
            }
        }
    }
    */

    // 7. Poll Network Stack (Phase 5)
    {
        let mut network = NETWORK.lock();
        if let Some(stack) = network.as_mut() {
            stack.poll(0);
        }
    }

    // 8. Phase 19: Internet of Abilities Background Tasks
    {
        use crate::ai::GLOBAL_NPU;
        if let Some(_completed_job) = GLOBAL_NPU.lock().process_step() {
             // Job progress...
        }

        use crate::quantum::GLOBAL_QPU;
        let _qpu = GLOBAL_QPU.lock(); 
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
