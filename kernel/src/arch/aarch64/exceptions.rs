//! Exception Vector Table for AArch64
//! Handles interrupts, exceptions, and system calls

use core::arch::global_asm;
use crate::arch::context::TrapFrame;

// Exception vector table
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
    "b exc_sync_el1",
    ".balign 0x80",
    "b exc_irq_el1",
    ".balign 0x80",
    "b exc_fiq_el1",
    ".balign 0x80",
    "b exc_serror_el1",
    
    // Lower EL using AArch64
    ".balign 0x80",
    "b exc_sync_el0_64",
    ".balign 0x80",
    "b exc_irq_el0_64",
    ".balign 0x80",
    "b exc_fiq_el0_64",
    ".balign 0x80",
    "b exc_serror_el0_64",
    
    // Lower EL using AArch32
    ".balign 0x80",
    "b exc_sync_el0_32",
    ".balign 0x80",
    "b exc_irq_el0_32",
    ".balign 0x80",
    "b exc_fiq_el0_32",
    ".balign 0x80",
    "b exc_serror_el0_32",
);

// Macro to save context (TrapFrame)
// Stack alignment must be maintained (16 bytes)
// TrapFrame size = 30*8 (regs) + 8 (x30) + 8 (elr) + 8 (spsr) + 8 (sp) = 272 bytes
global_asm!(
    ".macro SAVE_CONTEXT",
    "sub sp, sp, #272",
    
    // Save general purpose registers
    "stp x0, x1, [sp, #0]",
    "stp x2, x3, [sp, #16]",
    "stp x4, x5, [sp, #32]",
    "stp x6, x7, [sp, #48]",
    "stp x8, x9, [sp, #64]",
    "stp x10, x11, [sp, #80]",
    "stp x12, x13, [sp, #96]",
    "stp x14, x15, [sp, #112]",
    "stp x16, x17, [sp, #128]",
    "stp x18, x19, [sp, #144]",
    "stp x20, x21, [sp, #160]",
    "stp x22, x23, [sp, #176]",
    "stp x24, x25, [sp, #192]",
    "stp x26, x27, [sp, #208]",
    "stp x28, x29, [sp, #224]",
    
    // Save LR (x30)
    "str x30, [sp, #240]",
    
    // Save ELR_EL1 and SPSR_EL1
    "mrs x0, elr_el1",
    "str x0, [sp, #248]",
    "mrs x1, spsr_el1",
    "str x1, [sp, #256]",
    
    // Save original SP (pre-sub)
    "add x2, sp, #272",
    "str x2, [sp, #264]",
    ".endm",

    ".macro RESTORE_CONTEXT",
    // Restore ELR_EL1 and SPSR_EL1
    // Note: If GDB modified them in TrapFrame, this updates them
    "ldr x0, [sp, #248]",
    "msr elr_el1, x0",
    "ldr x1, [sp, #256]",
    "msr spsr_el1, x1",
    
    // Restore LR
    "ldr x30, [sp, #240]",
    
    // Restore general purpose registers
    "ldp x28, x29, [sp, #224]",
    "ldp x26, x27, [sp, #208]",
    "ldp x24, x25, [sp, #192]",
    "ldp x22, x23, [sp, #176]",
    "ldp x20, x21, [sp, #160]",
    "ldp x18, x19, [sp, #144]",
    "ldp x16, x17, [sp, #128]",
    "ldp x14, x15, [sp, #112]",
    "ldp x12, x13, [sp, #96]",
    "ldp x10, x11, [sp, #80]",
    "ldp x8, x9, [sp, #64]",
    "ldp x6, x7, [sp, #48]",
    "ldp x4, x5, [sp, #32]",
    "ldp x2, x3, [sp, #16]",
    "ldp x0, x1, [sp, #0]",
    
    "add sp, sp, #272",
    ".endm"
);

// Exception Handlers
global_asm!(
    // SYNC EL1 (Kernel Sync Exception)
    "exc_sync_el1:",
    "   SAVE_CONTEXT",
    "   mov x0, sp", // Pass TrapFrame pointer
    "   bl handle_sync_exception",
    "   RESTORE_CONTEXT",
    "   eret",

    // IRQ EL1 (Kernel IRQ)
    "exc_irq_el1:",
    "   SAVE_CONTEXT",
    "   mov x0, sp",
    "   bl handle_irq_exception",
    "   RESTORE_CONTEXT",
    "   eret",

    // Stubs for others (panic/halt)
    "exc_sync_sp0:",
    "exc_irq_sp0:",
    "exc_fiq_sp0:",
    "exc_serror_sp0:",
    "exc_fiq_el1:",
    "exc_serror_el1:",
    "exc_sync_el0_64:",
    "exc_irq_el0_64:",
    "exc_fiq_el0_64:",
    "exc_serror_el0_64:",
    "exc_sync_el0_32:",
    "exc_irq_el0_32:",
    "exc_fiq_el0_32:",
    "exc_serror_el0_32:",
    "   b unhandled_exception_parsing"
);

// Unhandled exception stub
global_asm!(
    "unhandled_exception_parsing:",
    "   wfe",
    "   b unhandled_exception_parsing"
);

// --- Rust Handlers ---

#[no_mangle]
extern "C" fn handle_sync_exception(trap_frame: &mut TrapFrame) {
    let esr: u64;
    unsafe { core::arch::asm!("mrs {}, esr_el1", out(reg) esr); }
    
    let ec = (esr >> 26) & 0x3F; // Exception Class
    let iss = esr & 0x1FFFFFF;   // Instruction Specific Syndrome
    
    // Check for Breakpoint Instruction (EC = 0x3C)
    // BRK instruction payload is in ISS.
    // GDB usually uses BRK #0 (ISS=0) or similar.
    if ec == 0x3C {
        // Breakpoint Exception!
        // Enter GDB Stub
        let uart = crate::hal::rpi::Uart::new();
        let mut gdb = crate::debug::gdb::GdbStub::new(&uart);
        
        // Signal 5 = SIGTRAP
        gdb.loop_wrapper(trap_frame, 5);
        return;
    }
    
    // Check for Data Abort (0x24, 0x25) or Instruction Abort (0x20, 0x21)
    if ec == 0x24 || ec == 0x25 || ec == 0x20 || ec == 0x21 {
         panic!("Data/Instruction Abort at EL1: PC={:#x}, ESR={:#x}", trap_frame.elr_el1, esr);
    }
    
    panic!("Unhandled Sync Exception: PC={:#x}, ESR={:#x}", trap_frame.elr_el1, esr);
}

#[no_mangle]
extern "C" fn handle_irq_exception(trap_frame: &mut TrapFrame) {
    use crate::hal::rpi::{Gic, gic};
    
    unsafe {
        // Acknowledge interrupt
        let irq = Gic::acknowledge();
        
        // TODO: Pass irq to a proper dispatcher
        
        if irq == gic::IRQ_TIMER {
            // Timer logic
            crate::SCHEDULER.tick();
            
            // Check if we need to switch tasks
            // Preemption is handled by the scheduler returning true/false
            // If switch needed, the scheduler would have updated current_object
            // Context switch happens in scheduler::schedule() called from here?
            // Wait, the current simple tick() just updates time.
            // Real preemption needs to happen here or be triggered here.
            
            use crate::scheduler::active_objects::ObjectState;
            // Access scheduler safely? We are in IRQ handler...
            let scheduler = &mut *core::ptr::addr_of_mut!(crate::SCHEDULER);
            
            // Tick the scheduler (updates time metrics)
            // scheduler.tick(); // Already called above if we make tick() public/static?
            // Actually tick() is a method on ActiveObjectScheduler.
            
            if scheduler.schedule() {
                // If schedule() returned true, it means it switched context internally?
                // Wait, do_context_switch switches Callee-saved regs.
                // But we are in an IRQ handler with a TrapFrame on stack!
                // If we switch context here using `__switch_context`, we switch stacks.
                // The new task will resume.
                // When the old task resumes, it will return here, restore TrapFrame, and eret.
                // So this logic is actually fine for preemption assuming __switch_context handles stack swizzle correctly.
            }
        } else if irq == gic::IRQ_UART {
            // Check for GDB break via UART?
            // Or just normal UART handling
        } else if irq < 1022 {
             // Other IRQs
             // log::info!("IRQ {}", irq);
        }
        
        Gic::end_of_interrupt(irq);
    }
}

/// Install exception vector table
pub unsafe fn install_vector_table() {
    extern "C" {
        static exception_vector_table: u8;
    }
    
    let vbar = &exception_vector_table as *const _ as u64;
    core::arch::asm!("msr vbar_el1, {}", in(reg) vbar);
}
