global long_mode_start
extern kernel_start
extern stack_top

section .text
bits 64
long_mode_start:
    ; long_mode_start gets called using its identity-mapped physical address.
    ; We need to jump to the higher half because when our kernel_start function
    ; returns, the identity-mapping will be gone.
    mov rax, higher_half_start ; NOTE: mov then jmp to do an absolute (non-PIC) jmp
    jmp rax

higher_half_start:
    ; Re-setup the stack using its proper virtual address
    mov rsp, stack_top

    ; print `OKAY` to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax

    mov rax, kernel_start
    call rax

    hlt
