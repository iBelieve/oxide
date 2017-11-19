section .multiboot_header

MAGIC equ 0xe85250d6
ARCH equ 0 ; protected mode i686
LENGTH equ header_end - header_start

CHECKSUM equ 0x100000000 - (MAGIC + ARCH + LENGTH)

header_start:
    dd MAGIC
    dd ARCH
    dd LENGTH

    dd CHECKSUM

    ; insert optional multiboot tags here

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
