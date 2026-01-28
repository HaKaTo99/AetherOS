//! Exception Vector Table for AArch64
//! Handles interrupts, exceptions, and system calls

use core::arch::global_asm;

// Exception vector table
// Each entry is 128 bytes (0x80), 4 exception levels x 4 exception types = 16 entries
global_asm!(
    ".section .text.vectors",
    ".global exception_vector_table",
    ".balign 0x800",
    "exception_vector_table:",
    
    // Current EL with SP0
    ".balign 0x80",
    "b exc_sync_sp0",
    ".balign 0x80",
    "b exc_irq_sp0",
    ".balign 0x80",
    "b exc_fiq_sp0",
    ".balign 0x80",
    "b exc_serror_sp0",
    
    // Current EL with SPx
    ".balign 0x80",
    "b exc_sync",
    ".balign 0x80",
    "b exc_irq",
    ".balign 0x80",
    "b exc_fiq",
    ".balign 0x80",
    "b exc_serror",
    
    // Lower EL using AArch64
    ".balign 0x80",
    "b exc_sync_lower",
    ".balign 0x80",
    "b exc_irq_lower",
    ".balign 0x80",
    "b exc_fiq_lower",
    ".balign 0x80",
    "b exc_serror_lower",
    
    // Lower EL using AArch32
    ".balign 0x80",
    "b exc_sync_lower32",
    ".balign 0x80",
    "b exc_irq_lower32",
    ".balign 0x80",
    "b exc_fiq_lower32",
    ".balign 0x80",
    "b exc_serror_lower32",
);

// Stub exception handlers (call into Rust)
global_asm!(
    "exc_sync_sp0:",
    "   b handle_exception",
    "exc_irq_sp0:",
    "   b handle_exception",
    "exc_fiq_sp0:",
    "   b handle_exception",
    "exc_serror_sp0:",
    "   b handle_exception",
    
    "exc_sync:",
    "   b handle_exception",
    "exc_irq:",
    "   b handle_irq",      // IRQ handler (timer, etc)
    "exc_fiq:",
    "   b handle_exception",
    "exc_serror:",
    "   b handle_exception",
    
    "exc_sync_lower:",
    "   b handle_exception",
    "exc_irq_lower:",
    "   b handle_exception",
    "exc_fiq_lower:",
    "   b handle_exception",
    "exc_serror_lower:",
    "   b handle_exception",
    
    "exc_sync_lower32:",
    "   b handle_exception",
    "exc_irq_lower32:",
    "   b handle_exception",
    "exc_fiq_lower32:",
    "   b handle_exception",
    "exc_serror_lower32:",
    "   b handle_exception",
);

// IRQ handler (save context, call Rust handler, restore context)
global_asm!(
    "handle_irq:",
    // TODO: Save full context when scheduler is active
    "   stp x0, x1, [sp, #-16]!",
    "   stp x2, x3, [sp, #-16]!",
    "   stp x4, x5, [sp, #-16]!",
    "   bl irq_handler",
    "   ldp x4, x5, [sp], #16",
    "   ldp x2, x3, [sp], #16",
    "   ldp x0, x1, [sp], #16",
    "   eret",
);

// Generic exception handler
global_asm!(
    "handle_exception:",
    "   wfe",  // Halt for now
    "   b handle_exception",
);

// Global tick counter for scheduler
static mut TICK_COUNT: u64 = 0;

// Rust IRQ handler
#[no_mangle]
extern "C" fn irq_handler() {
    unsafe {
        use crate::hal::rpi::{Gic, gic};
        
        // Acknowledge interrupt
        let irq = Gic::acknowledge();
        
        if irq == gic::IRQ_TIMER {
            // Timer interrupt - increment tick counter
            TICK_COUNT += 1;
            
            // Call scheduler tick every 10ms (10 ticks)
            if TICK_COUNT % 10 == 0 {
                scheduler_tick();
            }
        }
        
        // End of interrupt
        Gic::end_of_interrupt(irq);
    }
}

/// Scheduler tick - performs context switch if needed
unsafe fn scheduler_tick() {
    use crate::SCHEDULER;
    
    let scheduler = &mut *core::ptr::addr_of_mut!(SCHEDULER);
    
    // Only switch if we have multiple tasks
    if scheduler.object_count < 2 {
        return;
    }
    
    // Get current task index
    let current_idx = scheduler.current_object.load(core::sync::atomic::Ordering::Relaxed) as usize;
    
    // Find next ready task (simple round-robin)
    let next_idx = (current_idx + 1) % scheduler.object_count;
    
    if current_idx != next_idx {
        // Safely obtain mutable references to both task slots
        if current_idx < next_idx {
            let (left, right) = scheduler.objects.split_at_mut(next_idx);
            // Unwrap Option<ActiveObject> safely
            if let (Some(current), Some(next)) = (left[current_idx].as_mut(), right[0].as_mut()) {
                // Perform context switch
                crate::arch::aarch64::__switch_context(
                    &mut current.context,
                    &mut next.context,
                );
            }
        } else {
            // current_idx > next_idx case
            let (left, right) = scheduler.objects.split_at_mut(current_idx);
            if let (Some(next), Some(current)) = (left[next_idx].as_mut(), right[0].as_mut()) {
                // Perform context switch
                crate::arch::aarch64::__switch_context(
                    &mut current.context,
                    &mut next.context,
                );
            }
        }
        // Update current task index
        scheduler.current_object.store(next_idx as u32, core::sync::atomic::Ordering::Relaxed);
    }
}



/// Get current tick count
pub fn get_tick_count() -> u64 {
    unsafe { TICK_COUNT }
}

/// Install exception vector table
pub unsafe fn install_vector_table() {
    extern "C" {
        static exception_vector_table: u8;
    }
    
    let vbar = &exception_vector_table as *const _ as u64;
    core::arch::asm!("msr vbar_el1, {}", in(reg) vbar);
}
