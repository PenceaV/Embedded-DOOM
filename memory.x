MEMORY {
    /* 4 MiB Flash space matching the Pico 2 */
    FLASH : ORIGIN = 0x10000000, LENGTH = 4096K
    
    /* 512 KiB striped main RAM layout */
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
    
    /* Dedicated non-striped scratchpad banks */
    SRAM4 : ORIGIN = 0x20080000, LENGTH = 4K
    SRAM5 : ORIGIN = 0x20081000, LENGTH = 4K
}

SECTIONS {
    /* ### Boot ROM info
     * Places code metadata block in the first 4K of flash 
     * where the RP2350 Boot ROM expects to find it.
     */
    .start_block : ALIGN(4)
    {
        __start_block_addr = .;
        KEEP(*(.start_block));
        KEEP(*(.boot_info));
    } > FLASH
} INSERT AFTER .vector_table;

/* Adjust program start position to safely clear the boot blocks */
_stext = ADDR(.start_block) + SIZEOF(.start_block);

SECTIONS {
    /* ### Picotool 'Binary Info' Entries
     * Creates the structures that allow picotool to query program details.
     */
    .bi_entries : ALIGN(4)
    {
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .text;

SECTIONS {
    /* ### Boot ROM extra info
     * Automatically calculates and signs the end block app validation tags.
     */
    .end_block : ALIGN(4)
    {
        __end_block_addr = .;
        KEEP(*(.end_block));
    } > FLASH
} INSERT AFTER .uninit;

PROVIDE(start_to_end = __end_block_addr - __start_block_addr);
PROVIDE(end_to_start = __start_block_addr - __end_block_addr);