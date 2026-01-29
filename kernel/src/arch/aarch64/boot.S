/*
 * AetherOS Boot Stub
 * Entry point for the kernel.
 *
 * Expectations from bootloader:
 * x0 = Physical address of Device Tree Blob (DTB)
 * x1/x2/x3 = Reserved (0)
 *
 * This code runs at EL1 (or EL2 if we need to drop, but usually RPi FW handles that or we add logic later).
 */

.section .text.boot
.global _start

_start:
    // 1. Setup Stack Pointer
    // We use a temporary stack for boot, or the final kernel stack.
    // Linker script should define __stack_top or we assume some free memory.
    // For RPi4, we can usually use memory below kernel load addr (0x80000) or high memory.
    // Let's rely on the stack allocated in .bss or fixed address.
    ldr x30, =__stack_top
    mov sp, x30

    // 2. Clear BSS (Block Started by Symbol)
    // We must zero this manually because the bootloader might not.
    ldr x1, =__bss_start
    ldr x2, =__bss_end
    sub x2, x2, x1      // Calculate size
    cbz x2, 3f          // If size is 0, skip

2:  // Loop to zero memory
    str xzr, [x1], #8
    sub x2, x2, #8
    cbnz x2, 2b

3:  // 3. Jump to kernel_main
    // x0 already contains DTB pointer (preserved from bootloader)
    // Argument 1: dtb_ptr matches x0
    bl kernel_main

    // 4. Halt if return
    b .
