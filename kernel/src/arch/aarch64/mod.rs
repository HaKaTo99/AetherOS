//! Architecture specific code for AArch64

pub mod context;
pub mod exceptions;  // [NEW] Exception handling

use core::arch::global_asm;

// Export context switch function
extern "C" {
    pub fn __switch_context(prev: *mut context::CpuContext, next: *const context::CpuContext);
}

// Global Assembly for Context Switch
// x0 = ptr to prev CpuContext
// x1 = ptr to next CpuContext
global_asm!(
    ".global __switch_context",
    "__switch_context:",
    // Save Context to prev (x0)
    "   str x19, [x0, #0]",
    "   str x20, [x0, #8]",
    "   str x21, [x0, #16]",
    "   str x22, [x0, #24]",
    "   str x23, [x0, #32]",
    "   str x24, [x0, #40]",
    "   str x25, [x0, #48]",
    "   str x26, [x0, #56]",
    "   str x27, [x0, #64]",
    "   str x28, [x0, #72]",
    "   str x29, [x0, #80]",
    "   str x30, [x0, #88]",
    "   mov x2,  sp",
    "   str x2,  [x0, #96]",

    // Load Context from next (x1)
    "   ldr x19, [x1, #0]",
    "   ldr x20, [x1, #8]",
    "   ldr x21, [x1, #16]",
    "   ldr x22, [x1, #24]",
    "   ldr x23, [x1, #32]",
    "   ldr x24, [x1, #40]",
    "   ldr x25, [x1, #48]",
    "   ldr x26, [x1, #56]",
    "   ldr x27, [x1, #64]",
    "   ldr x28, [x1, #72]",
    "   ldr x29, [x1, #80]",
    "   ldr x30, [x1, #88]",
    "   ldr x2,  [x1, #96]",
    "   mov sp,  x2",

    // Return to new context
    "   ret"
);
