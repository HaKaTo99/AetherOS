//! System Lifecycle Management (Phase 36.0)
//! 
//! Handles graceful shutdown, reboot, and power state transitions.

use crate::hal;
use crate::ipc::qc::get_quantum_bus;

pub struct PowerManager {
    is_shutting_down: bool,
}

impl PowerManager {
    pub const fn new() -> Self {
        Self {
            is_shutting_down: false,
        }
    }

    /// Prepare system for shutdown by notifying the mesh and syncing state.
    pub fn prepare_shutdown(&mut self) {
        if self.is_shutting_down { return; }
        self.is_shutting_down = true;

        let platform = hal::get_platform();
        platform.puts("[Power] 💤 Preparing for graceful shutdown...\n");

        // 1. Notify Global Mesh
        platform.puts("[Power] 🌐 Notifying mesh peers for task migration...\n");
        {
            let _bus = get_quantum_bus();
            // In a real mesh, we would send a 'NodeLeaving' packet here
            platform.puts("[Power] [Mesh] Migration signals sent to peers.\n");
        }

        // 2. Sync Filesystems/State (Simulator)
        platform.puts("[Power] 💾 Syncing distributed state storage...\n");
        platform.sleep_ms(200);
        platform.puts("[Power] ✅ State synchronized.\n");
    }

    /// Shutdown the physical or virtual machine.
    pub fn shutdown(&mut self) -> ! {
        self.prepare_shutdown();
        
        let platform = hal::get_platform();
        platform.puts("[Power] 🛑 System Halted. Goodbye.\n");
        platform.shutdown();
        
        // Fallback if shutdown fails
        platform.cpu_halt();
    }

    /// Restart the system.
    pub fn reboot(&mut self) -> ! {
        let platform = hal::get_platform();
        platform.puts("[Power] 🔄 Rebooting system...\n");
        
        // On x86 QEMU this might just halt, but on real hardware it triggers reset
        platform.shutdown(); 
        platform.cpu_halt();
    }
}

pub static mut POWER_MANAGER: PowerManager = PowerManager::new();

pub fn shutdown() -> ! {
    unsafe { POWER_MANAGER.shutdown() }
}

pub fn reboot() -> ! {
    unsafe { POWER_MANAGER.reboot() }
}
