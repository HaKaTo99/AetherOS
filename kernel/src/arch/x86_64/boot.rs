// x86_64 Boot Stub
// Handles transition from 32-bit Protected Mode (Multiboot2) to 64-bit Long Mode

// x86_64 Boot Stub
// Handles transition from 32-bit Protected Mode (Multiboot2) to 64-bit Long Mode

use core::arch::global_asm;

global_asm!(r#"
    .section .text
.global _multiboot_entry
.code32

/* ---- PVH ELF Note (Modern 64-bit Direct Boot) ---- */
.section .note.pvh, "a"
.align 4
pvh_note_start:
    .long 4                      /* namesz */
    .long 4                      /* descsz */
    .long 18                     /* type: XEN_ELFNOTE_PHYS32_ENTRY */
    .ascii "Xen\0"               /* name */
    .long _multiboot_entry       /* desc: physical entry point */

.section .text
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
    .long _multiboot_entry

    /* end tag */
    .align 8
    .word 0
    .word 0
    .long 8
mb2_header_end:

.section .text
.code64

/* Legacy boot entry point handed by GRUB multiboot: jump to Rust _start */
.global _start
_multiboot_entry:
    jmp _start

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
