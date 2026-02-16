// Multiboot2 Header
use core::arch::global_asm;

global_asm!(r#"
    .section .multiboot_header
    .align 8
header_start:
    .long 0xe85250d6                /* magic number (multiboot 2) */
    .long 0                         /* architecture 0 (protected mode i386) */
    .long 24                        /* header length (16 + 8 for end tag) */
    .long 0x17adaf12                /* checksum: -(0xe85250d6 + 0 + 24) */

    /* required end tag */
    .word 0    /* type */
    .word 0    /* flags */
    .long 8    /* size */
header_end:
"#);
