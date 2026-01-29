ENTRY(_start)

SECTIONS
{
    . = 0x80000; /* Load address for RPi4 Kernel (AArch64) */
    
    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    
    .rodata : { *(.rodata .rodata.*) }
    
    .data : { *(.data .data.*) }
    
    .bss : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end = .;
    }
    
    /* Allocate 64KB stack for boot */
    . = ALIGN(16);
    . += 0x10000;
    __stack_top = .;
    
    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
