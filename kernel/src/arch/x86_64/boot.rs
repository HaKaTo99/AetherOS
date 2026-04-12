// x86_64 Boot Stub
// Handles transition from 32-bit Protected Mode (Multiboot2) to 64-bit Long Mode

// x86_64 Boot Stub
// Handles transition from 32-bit Protected Mode (Multiboot2) to 64-bit Long Mode

use core::arch::global_asm;

global_asm!(r#"
    .section .multiboot_header, "a"
    .align 8
multiboot_header_v2:
    .long 0xe85250d6                /* magic: multiboot2 */
    .long 0                         /* architecture: i386 (protected mode) */
    .long multiboot_header_v2_end - multiboot_header_v2 /* header_length */
    /* checksum = -(magic + arch + length) mod 2^32 */
    .long -(0xe85250d6 + 0 + (multiboot_header_v2_end - multiboot_header_v2))



    .align 8
    /* Tag 5: Framebuffer Tag (Request 1920x1200x32) */
    .short 5                        /* type */
    .short 0                        /* flags */
    .long 20                        /* size */
    .long 1920                      /* width */
    .long 1200                      /* height */
    .long 32                        /* depth */

    .align 8
    /* End tag */
    .short 0                        /* type */
    .short 0                        /* flags */
    .long 8                         /* size */
multiboot_header_v2_end:

    .align 8
multiboot_header_v1:
    .long 0x1BADB002                /* magic: multiboot1 */
    .long 0x00000003                /* flags: ALIGN + MEMINFO (Standard ELF) */
    .long -(0x1BADB002 + 0x00000003) /* checksum */

    .section .note.pvh, "a"
    .align 4
pvh_note_start:
    .long 4                      /* namesz */
    .long 4                      /* descsz */
    .long 18                     /* type: XEN_ELFNOTE_PHYS32_ENTRY */
    .ascii "Xen\0"               /* name */
    .long _multiboot_entry       /* desc: physical entry point */

    .section .boot_code, "ax"
.code32
.global _multiboot_entry
_multiboot_entry:
    cli
    mov esp, offset stack_top

    /* Save Multiboot magic and info pointer */
    mov [mb2_magic], eax
    mov [mb2_info], ebx

    /* 1. Setup Paging (Identity Map first 1GB) */
    /* Map P4[0] -> P3 */
    mov eax, offset p3_table
    or eax, 0b11 /* present + writable */
    mov [p4_table], eax

    /* Map P3[0] -> P2 */
    mov eax, offset p3_table
    mov eax, offset p2_table
    or eax, 0b11 /* present + writable */
    mov [p3_table], eax

    /* Map P2[0..511] -> 2MB pages (total 1GB) */
    mov ecx, 0
.map_p2:
    mov eax, 0x200000 /* 2MB */
    mul ecx
    or eax, 0b10000011 /* present + writable + huge */
    /* WRITE 64-BIT PDE (EDX is high dword from MUL) */
    mov [p2_table + ecx * 8], eax
    mov [p2_table + ecx * 8 + 4], edx
    inc ecx
    cmp ecx, 512
    jne .map_p2

    /* 2. Enable PAE */
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    /* 3. Set Long Mode bit in EFER MSR */
    mov ecx, 0xc0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    /* 4. Enable Paging */
    mov eax, offset p4_table
    mov cr3, eax
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    /* 5. Load 64-bit GDT */
    lgdt [gdt64_pointer]

    /* 6. Long Jump to 64-bit code */
    ljmp 0x8, offset _start_64

.code64
_start_64:
    /* Jump to kernel_main_grub defined in main.rs */
    mov rax, offset kernel_main_grub
    mov rdi, [mb2_magic]
    mov rsi, [mb2_info]
    call rax
    hlt


/* --- Data Section --- */
.section .data
.align 8
mb2_magic:
    .quad 0
mb2_info:
    .quad 0

/* --- Data Section (BSS) --- */
.section .bss
.align 4096
/* Define page tables first (lower addresses) */
p4_table: .skip 4096
p3_table: .skip 4096
p2_table: .skip 4096

.align 4096
stack_bottom:
    /* Supreme Stability: 4MB stack to prevent any possible overflow during deep AI/Harmony audits */
    .skip 4096 * 1024
stack_top:


/* --- GDT (Read Only) --- */
.section .rodata
.align 8
gdt64:
    .quad 0 /* zero entry */
    .quad (1<<43) | (1<<44) | (1<<47) | (1<<53) /* code segment */
gdt64_pointer:
    .word . - gdt64 - 1
    .quad gdt64
"#);



