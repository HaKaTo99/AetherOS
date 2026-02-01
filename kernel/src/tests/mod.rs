//! Kernel Test Suite
//! Functional verification for Scheduler, Memory, and IPC.

pub mod scheduler;
pub mod memory;
pub mod ipc;
pub mod stress;

use crate::hal;

/// Simple test logger
pub fn log(args: core::fmt::Arguments) {
    use core::fmt::Write;
    struct SerialWriter;
    impl Write for SerialWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            hal::get_platform().puts(s);
            Ok(())
        }
    }
    let _ = SerialWriter.write_fmt(args);
    hal::get_platform().puts("\n");
}

/// Run all kernel functional tests
pub fn run_suite() {
    log(format_args!("\n[TEST] === Starting Kernel Test Suite ==="));
    
    // 1. Scheduler Tests
    log(format_args!("[TEST] Running Scheduler Tests..."));
    scheduler::test_preemption();
    
    // 2. Memory Tests
    log(format_args!("[TEST] Running Memory Tests..."));
    memory::test_allocation();
    
    // 3. IPC Tests
    log(format_args!("[TEST] Running IPC Tests..."));
    ipc::test_loopback();
    
    // 4. Stress Tests
    stress::run();

    log(format_args!("[TEST] === All Tests Completed ===\n"));
}


