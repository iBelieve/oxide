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

	kernel_end = .;
}
