ENTRY(start)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_OFFSET = 0xFFFFFF0000000000;

SECTIONS {
	/* TODO: Move to a higher-half kernel (but for x86_64) */
	. = 1M + KERNEL_OFFSET;

    __start = .;

    .rodata ALIGN (4K) : AT (ADDR (.rodata) - KERNEL_OFFSET) {
        KEEP(*(.multiboot_header))
        *(.rodata .rodata.*)
    }

    .text ALIGN (4K) : AT (ADDR (.text) - KERNEL_OFFSET) {
      *(.text .text.*)
    }

    .data ALIGN (4K) : AT (ADDR (.data) - KERNEL_OFFSET) {
		*(.data .data.*)
	}

	.bss ALIGN (4K) : AT (ADDR (.bss) - KERNEL_OFFSET) {
		*(.bss .bss.*)
	}

	.got ALIGN (4K) : AT (ADDR (.got) - KERNEL_OFFSET) {
		*(.got)
	}

	.got.plt ALIGN (4K) : AT (ADDR (.got.plt) - KERNEL_OFFSET) {
		*(.got.plt)
	}

    .data.rel.ro ALIGN(4K) : AT (ADDR (.data.rel.ro) - KERNEL_OFFSET) {
      *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }

    .gcc_except_table ALIGN(4K) : AT (ADDR (.gcc_except_table) - KERNEL_OFFSET) {
      *(.gcc_except_table)
    }

	__end = .;
}
