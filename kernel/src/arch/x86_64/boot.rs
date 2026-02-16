// x86_64 Boot Stub
// Handles transition from 32-bit Protected Mode (Multiboot2) to 64-bit Long Mode

use core::arch::global_asm;

global_asm!(r#"
.section .text
.global _start
.code32

/* ---- Multiboot 2 Header (supports 64-bit kernels) ---- */
.align 8
mb2_header_start:
    .long 0xe85250d6                              /* magic */
    .long 0                                       /* architecture: i386 */
    .long mb2_header_end - mb2_header_start       /* header length */
    .long -(0xe85250d6 + 0 + (mb2_header_end - mb2_header_start)) /* checksum */

    /* entry address tag */
    .align 8
    .word 3    /* type: entry address */
    .word 0    /* flags */
    .long 12   /* size */
    .long _start

    /* end tag */
    .align 8
    .word 0
    .word 0
    .long 8
mb2_header_end:

_start:
    /* Setup stack */
    mov esp, offset stack_top

    /* 1. Check if CPU supports Long Mode */
    call check_cpuid
    call check_long_mode

    /* 2. Setup Paging (Identity Map first 1GB) */
    call setup_page_tables
    call enable_paging

    /* 3. Load 64-bit GDT */
    .byte 0x0f, 0x01, 0x15
    .long gdt64_pointer

    /* 4. Jump to Long Mode using Far Return */
    .byte 0x6A, 0x08
    .byte 0x68
    .long long_mode_start
    .byte 0xCB

.no_multiboot:
    hlt

/* --- Helper Functions (32-bit) --- */
check_cpuid:
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
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret
.no_long_mode:
    hlt

setup_page_tables:
    mov eax, offset p3_table
    or eax, 0b11
    mov [p4_table], eax
    mov eax, offset p2_table
    or eax, 0b11
    mov [p3_table], eax
    mov ecx, 0
.map_p2_table:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .map_p2_table
    ret

enable_paging:
    mov eax, offset p4_table
    mov cr3, eax
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    ret

/* --- 64-bit Long Mode Entry --- */
.code64
long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov rdi, rbx
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
    .quad 0
gdt64_code:
    .quad (1<<43) | (1<<44) | (1<<47) | (1<<53)
gdt64_pointer:
    .word gdt64_pointer - gdt64 - 1
    .long gdt64
"#);
