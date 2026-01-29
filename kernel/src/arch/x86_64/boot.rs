use core::arch::global_asm;

// Multiboot2 header or basic entry point
global_asm!(r#"
.section .text.boot
.global _start

_start:
    // Ensure data segments are setup (if needed) but assuming valid long mode from bootloader
    
    // Setup stack
    mov rsp, OFFSET __stack_top
    
    // Call kernel_main
    // x86_64 System V ABI: rdi, rsi, rdx, rcx, r8, r9
    // We pass 0 as dummy DTB pointer for now (or handle it later)
    xor rdi, rdi 
    call kernel_main

    // Halt
    hlt
1:  jmp 1b
"#);
