// Multiboot2 Header
use core::arch::global_asm;

global_asm!(r#"
    .section .multiboot_header
    .align 8
header_start:
    .long 0xe85250d6                /* magic number (multiboot 2) */
    .long 0                         /* architecture 0 (protected mode i386) */
    .long header_end - header_start /* header length */
    .long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) /* checksum */

    /* required end tag */
    .word 0    /* type */
    .word 0    /* flags */
    .long 8    /* size */
header_end:
"#);
