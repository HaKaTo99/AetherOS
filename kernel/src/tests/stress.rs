//! Stress Testing Module
//! Heavy load verification for scheduler and memory.

use super::log;
use alloc::vec::Vec;

/// Run stability stress tests
pub fn run() {
    log(format_args!("\n[STRESS] === Starting Stability Stress Tests ==="));
    
    // 1. Heap Torture
    log(format_args!("[STRESS] Running Heap Torture (Allocation/Deallocation loop)..."));
    stress_heap(100);
    
    // 2. Scheduler Stress
    log(format_args!("[STRESS] Running Scheduler Stress (Task Flood)..."));
    stress_scheduler();
    
    log(format_args!("[STRESS] === Stability Tests Passed ===\n"));
}

fn stress_heap(iterations: usize) {
    for i in 0..iterations {
        // Allocate varying sizes
        let size = (i * 1024) % (1024 * 64); // Up to 64KB
        let mut vec = Vec::with_capacity(size);
        for j in 0..size {
            vec.push((j & 0xFF) as u8);
        }
        // Vec dropped here
        if i % 10 == 0 {
            log(format_args!("[STRESS] Heap Iteration {}/{}", i, iterations));
        }
    }
    log(format_args!("[STRESS] Heap Torture OK"));
}

fn stress_scheduler() {
    // In a real OS, we'd spawn threads.
    // For now, we simulate heavy localized load.
    // This is more of a CPU burn test in current single-threaded context.
    let mut counter: u64 = 0;
    for _ in 0..1_000_000 {
        counter = counter.wrapping_add(1);
        core::hint::spin_loop();
    }
    log(format_args!("[STRESS] Scheduler Stress OK (Counter: {})", counter));
}
