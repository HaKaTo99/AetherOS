ENTRY(_multiboot_entry)

PHDRS
{
    pvh   PT_NOTE;
    boot  PT_LOAD;
    text  PT_LOAD;
    rodata PT_LOAD;
    data  PT_LOAD;
}

SECTIONS {
    /* Map kernel at 1MiB VA/PA */
    . = 1M;

    .note.pvh : {
        KEEP(*(.note.pvh))
    } :pvh :boot

    .boot : {
        __boot_start = .;
        KEEP(*(.multiboot_header))
        KEEP(*(.boot_code))
        __boot_end = .;
    } :boot




    .text : ALIGN(4K) {
        __text_start = .;
        *(._start)
        *(.text._start)
        *(.text .text.*)
        __text_end = .;
    } :text

    .rodata : ALIGN(4K) {
        __rodata_start = .;
        *(.rodata .rodata.*)
        __rodata_end = .;
    } :rodata

    .data : ALIGN(4K) {
        __data_start = .;
        *(.data .data.*)
        *(.got .got.*)
        *(.got.plt .got.plt.*)
        __data_end = .;
    } :data

    .bss : ALIGN(4K) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } :data


    /* --- SMME Heap Segments (Military Grade Mapping) --- */
    /* Use NOLOAD and do NOT assign to a PT_LOAD segment to avoid binary bloat */
    
    .heap (NOLOAD) : ALIGN(4K)
    {
        __heap_start = .;
        . = . + (16 * 1024 * 1024); /* 16MB is enough for demo stability */
        __heap_end = .;
    }



    /* Rigid Discard: Destroy all rogue metadata that causes misalignment */
    /DISCARD/ :
    {
        *(.note.GNU-stack)
        *(.comment)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.rel.eh_frame)
    }
}

