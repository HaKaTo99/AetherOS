// x86_64 Boot Stub
// Handles transition from 32-bit Protected Mode (Multiboot) to 64-bit Long Mode

use core::arch::global_asm;

global_asm!(r#"
.section .text
.global _start
.code32
_start:
    /* Setup stack */
    mov esp, offset stack_top

    /* Check for Multiboot2 magic at EAX */
    /* cmp eax, 0x36d76289 */
    /* jne .no_multiboot */

    /* 1. Check if CPU supports Long Mode */
    call check_cpuid
    call check_long_mode

    /* 2. Setup Paging (Identity Map first 1GB) */
    call setup_page_tables
    call enable_paging

    /* 3. Load 64-bit GDT */
    /* lgdt [gdt64_pointer] - Manual encoding to force 32-bit displacement */
    .byte 0x0f, 0x01, 0x15
    .long gdt64_pointer

    /* 4. Jump to Long Mode using Far Return */
    /* Stack: [CS, EIP] */
    
    /* push 0x08 (Code Selector) */
    .byte 0x6A, 0x08
    
    /* push offset long_mode_start */
    .byte 0x68
    .long long_mode_start
    
    /* retf */
    .byte 0xCB

.no_multiboot:
    hlt

/* --- Helper Functions (32-bit) --- */
check_cpuid:
    /* Check if CPUID is supported by flipping ID bit (21) in EFLAGS */
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    hlt

check_long_mode:
    /* Check for extended processor info */
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode

    /* Check for Log Mode available bit */
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret
.no_long_mode:
    hlt

setup_page_tables:
    /* Map P4[0] -> P3 */
    mov eax, offset p3_table
    or eax, 0b11 /* present + writable */
    mov [p4_table], eax

    /* Map P3[0] -> P2 */
    mov eax, offset p2_table
    or eax, 0b11 /* present + writable */
    mov [p3_table], eax

    /* Map P2[0..512] -> 2MB huge pages */
    mov ecx, 0 /* counter */

.map_p2_table:
    mov eax, 0x200000 /* 2MB size */
    mul ecx
    or eax, 0b10000011 /* present + writable + huge */
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ret

enable_paging:
    /* Load P4 to CR3 */
    mov eax, offset p4_table
    mov cr3, eax

    /* Enable PAE-flag in CR4 */
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    /* Set long mode bit in EFER MSR */
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    /* Enable paging in CR0 */
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

/* --- 64-bit Long Mode Entry --- */
.code64
long_mode_start:
    /* Clear segment registers */
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    /* Jump to Kernel Main */
    /* RDI = 1st arg, RSI = 2nd arg... */
    mov rdi, rbx /* Multiboot info pointer */
    mov rsi, 0
    mov rdx, 0
    mov rcx, 0
    
    call kernel_main
    hlt

/* --- Data Section (BSS) --- */
.section .bss
.align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 4096 * 4
stack_top:

/* --- GDT (Read Only) --- */
.section .rodata
.align 8
gdt64:
    .quad 0 /* zero entry */
gdt64_code:
    .quad (1<<43) | (1<<44) | (1<<47) | (1<<53) /* code segment */
gdt64_pointer:
    .word gdt64_pointer - gdt64 - 1
    .long gdt64 /* 32-bit pointer for 32-bit lgdt */
"#);
