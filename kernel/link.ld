ENTRY(_start)

SECTIONS
{
    . = 0x80000; /* Load address for RPi4 Kernel (AArch64) */
    
    .text : {
        . = ALIGN(4096);
        __text_start = .;
        KEEP(*(.text.boot))
        *(.text .text.*)
        . = ALIGN(4096);
        __text_end = .;
    }
    
    .rodata : {
        . = ALIGN(4096);
        __rodata_start = .;
        *(.rodata .rodata.*)
        . = ALIGN(4096);
        __rodata_end = .;
    }
    
    .data : {
        . = ALIGN(4096);
        __data_start = .;
        *(.data .data.*)
        . = ALIGN(4096);
        __data_end = .;
    }
    
    .bss : {
        . = ALIGN(4096);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4096);
        __bss_end = .;
    }
    
    /* Allocate 64KB stack for boot */
    . = ALIGN(4096);
    . += 0x10000;
    __stack_top = .;
    
    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
