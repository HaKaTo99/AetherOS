pub mod context;
pub mod boot;

use core::arch::global_asm;

// Export context switch function
extern "C" {
    pub fn __switch_context(prev: *mut context::CpuContext, next: *const context::CpuContext);
}

// Global Assembly for Context Switch (x86_64 System V ABI)
// rdi = ptr to prev CpuContext
// rsi = ptr to next CpuContext
global_asm!(
    ".global __switch_context",
    "__switch_context:",
    // Save callee-saved registers to prev (rdi)
    "   mov [rdi + 0x00], rbx",
    "   mov [rdi + 0x08], rbp",
    "   mov [rdi + 0x10], r12",
    "   mov [rdi + 0x18], r13",
    "   mov [rdi + 0x20], r14",
    "   mov [rdi + 0x28], r15",
    // Save Stack Pointer
    "   mov [rdi + 0x30], rsp",
    // Save Return Address (already on stack) - optional, 
    // but usually we just switch SP and let RET handle it.
    
    // Load registers from next (rsi)
    "   mov rbx, [rsi + 0x00]",
    "   mov rbp, [rsi + 0x08]",
    "   mov r12, [rsi + 0x10]",
    "   mov r13, [rsi + 0x18]",
    "   mov r14, [rsi + 0x20]",
    "   mov r15, [rsi + 0x28]",
    // Load Stack Pointer
    "   mov rsp, [rsi + 0x30]",
    
    // Return to new context
    "   ret"
);
