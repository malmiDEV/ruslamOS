[bits 32]
    section .preload
    global kloader
    extern kernel_load

kloader:
    cli

    ; pass boot drive to kernel_load
    xor edx, edx
    mov dl, [0x1000]
    push edx
    
    call kernel_load
    jmp $
    