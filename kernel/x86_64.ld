ENTRY(_start)

SECTIONS
{
    . = 1M;

    .boot :
    {
        /* Ensure multiboot header is at the beginning */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data :
    {
        *(.data .data.*)
    }

    .bss :
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.gnu.property)
    }
}
