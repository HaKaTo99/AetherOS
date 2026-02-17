// x86_64 Boot Stub
// Handles transition from 32-bit Protected Mode (Multiboot2) to 64-bit Long Mode

use core::arch::global_asm;

global_asm!(r#"
.section .multiboot_header
.global _start
.code32

/* ---- PVH ELF Note (Modern 64-bit Direct Boot) ---- */
.section .note.pvh, "a"
.align 4
pvh_note_start:
    .long 4                      /* namesz */
    .long 4                      /* descsz */
    .long 18                     /* type: XEN_ELFNOTE_PHYS32_ENTRY */
    .ascii "Xen\0"               /* name */
    .long _start                 /* desc: physical entry point */

.section .multiboot_header
/* ---- Multiboot 1 Header (for maximum compatibility) ---- */
.align 4
mb1_header_start:
    .long 0x1BADB002                       /* magic */
    .long 0x01                             /* flags: align modules */
    .long -(0x1BADB002 + 0x01)             /* checksum */

/* ---- Multiboot 2 Header ---- */
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

.section .text

_start:
    /* Setup stack */
    mov esp, offset stack_top

    /* Zero the BSS (v7.9 Stability Fix - Full Range) */
    mov edi, offset __bss_start
    mov ecx, offset __bss_end
    sub ecx, edi
    xor eax, eax
    rep stosb

    /* Zero the start of the Heap (v7.9 Emergency Fix) */
    /* VirtualBox memory at 64MB might contain junk */
    mov edi, 0x04000000
    mov ecx, 1024 * 1024 * 32 /* Zero first 32MB of L0 pool */
    xor eax, eax
    rep stosb

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
    mov dword ptr [p4_table + 4], 0

    mov eax, offset p2_table
    or eax, 0b11
    mov [p3_table], eax
    mov dword ptr [p3_table + 4], 0

    mov ecx, 0
.map_p2_table:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax
    mov dword ptr [p2_table + ecx * 8 + 4], 0
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
    
    /* SSE/FPU initialization for improved compatibility (v7.8) */
    mov eax, cr0
    and ax, 0xFFFB      /* clear Coprocessor Monitoring (MP) and Emulation (EM) */
    or ax, 0x2          /* set MP */
    mov cr0, eax
    mov eax, cr4
    or ax, 3 << 9       /* set OSFXSR and OSXMMEXCPT */
    mov cr4, eax
    
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
/* Critical Page Tables MUST be zeroed first */
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096

/* Guard space and Stack */
.align 4096
.skip 4096 /* Bottom Guard */
stack_bottom:
    .skip 4096 * 32
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
