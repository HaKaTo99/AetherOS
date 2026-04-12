ENTRY(_multiboot_entry)

SECTIONS {
    /* [V10.4.15] RIGID BOUNDARY: Kernel starts at 1MB */
    . = 1M;

    .multiboot_header : {
        KEEP(*(.multiboot_header))
    }

    .boot : ALIGN(4K) {
        *(.boot_code)
    }

    .text : ALIGN(4K) {
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
        *(.got .got.*)
    }

    .bss : ALIGN(4K) {
        *(.bss .bss.*)
        *(COMMON)
    }

    /* Start heap at 32MB to avoid any possible overlap with the kernel image */
    . = 32M;
    /DISCARD/ : {
        *(.note*)
        *(.comment*)
        *(.eh_frame*)
    }
}

