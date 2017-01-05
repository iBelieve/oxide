section .multiboot_header
header_start:
    dd 0xE85250D6                   ; Magic number (multiboot 2)
    dd 0                            ; Architecture 0 (protected mode i386)
    dd header_end - header_start    ; Header length
    ; Checksum, which when added to the other fields, equals 0
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; Optional multiboot tags go here...

    ; Required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
