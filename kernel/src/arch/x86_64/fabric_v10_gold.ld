ENTRY(_start)

PHDRS
{
    boot   PT_LOAD FLAGS(5); /* Read + Execute */
    text   PT_LOAD FLAGS(5); /* Read + Execute */
    rodata PT_LOAD FLAGS(4); /* Read Only */
    data   PT_LOAD FLAGS(6); /* Read + Write */
}

SECTIONS {
    /* Map kernel at 1MiB VA */
    . = 1M;

    .boot : ALIGN(4K) {
        KEEP(*(.multiboot_header))
        KEEP(*(.boot_code))
    } :boot

    .text : ALIGN(4K) {
        __text_start = .;
        KEEP(*(._start))
        KEEP(*(.text._start))
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
        __data_end = .;
    } :data

    .bss : ALIGN(4K) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } :data

    /* --- SMME Heap Segments (Military Grade Mapping) --- */
    /* Reserve 128MB at 16MB VA to ensure no segment overlaps in 512MB RAM */
    . = ALIGN(1M); 
    __heap_virtual_base = . < 0x01000000 ? 0x01000000 : ALIGN(4K);
    . = __heap_virtual_base;
    
    .heap (NOLOAD) : ALIGN(4K)
    {
        __heap_start = .;
        . = . + (128 * 1024 * 1024); 
        __heap_end = .;
    } :data

    /* Rigid Discard: Destroy all rogue metadata that causes misalignment */
    /DISCARD/ :
    {
        *(.note .note.*)
        *(.comment)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.rel.eh_frame)
    }
}
