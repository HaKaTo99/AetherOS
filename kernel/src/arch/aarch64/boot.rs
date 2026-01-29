// Boot stub wrapping boot.S content
use core::arch::global_asm;

global_asm!(r#"
/*
 * AetherOS Boot Stub
 * Entry point for the kernel.
 */

.section .text.boot
.global _start

_start:
    // 1. Setup Stack Pointer
    ldr x30, =__stack_top
    mov sp, x30

    // 2. Clear BSS
    ldr x1, =__bss_start
    ldr x2, =__bss_end
    sub x2, x2, x1
    cbz x2, 3f

2:  
    str xzr, [x1], #8
    sub x2, x2, #8
    cbnz x2, 2b

3:  
    // 3. Jump to kernel_main
    // x0 already contains DTB pointer
    bl kernel_main

    // 4. Halt
    b .
"#);
