ENTRY(start)

SECTIONS {
	/* TODO: Move to a higher-half kernel (but for x86_64) */
	. = 1M;

    kernel_start = .;

    .boot : {
		KEEP(*(.multiboot_header))
    }

    .text ALIGN (4K) : {
		*(.text)
	}

	.rodata ALIGN (4K) :
	{
		*(.rodata)
	}

    .data ALIGN (4K) :
	{
		*(.data)
	}

	.bss ALIGN (4K) : {
		*(COMMON)
		*(.bss)
	}

	kernel_end = .;
}
