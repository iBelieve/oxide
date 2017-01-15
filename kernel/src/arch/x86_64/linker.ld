ENTRY(start)

SECTIONS {
	/* TODO: Move to a higher-half kernel (but for x86_64) */
	. = 1M;

    __start = .;

    .boot : {
		KEEP(*(.multiboot_header))
    }

    .text ALIGN (4K) : {
		*(.text .text.*)
	}

	.rodata ALIGN (4K) :
	{
		*(.rodata .rodata.*)
	}

    .data.rel.ro ALIGN (4K) : {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }

    .data ALIGN (4K) :
	{
		*(.data .data.*)
	}

	.bss ALIGN (4K) : {
		*(COMMON)
		*(.bss)
	}

	__end = .;
}
