ENTRY(start)

SECTIONS {
	/* TODO: Move to a higher-half kernel (but for x86_64) */
	. = 1M;

    __start = .;

    .rodata ALIGN (4K) : {
        KEEP(*(.multiboot_header))
        *(.rodata .rodata.*)
        . = ALIGN(4K);
    }

    .text ALIGN (4K) : {
      *(.text .text.*)
    }

    .data ALIGN (4K) :
	{
		*(.data .data.*)
	}

	.bss ALIGN (4K) : {
		*(.bss .bss.*)
	}

	.got ALIGN (4K) : {
		*(.got)
	}

	.got.plt ALIGN (4K) : {
		*(.got.plt)
	}

    .data.rel.ro ALIGN(4K) : {
      *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }

    .gcc_except_table ALIGN(4K) : {
      *(.gcc_except_table)
    }

	__end = .;
}
