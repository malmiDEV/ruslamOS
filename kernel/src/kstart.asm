[bits 32]
    extern _kmain 
    extern _bss
    extern _end
    extern vesa_console_init

    global kstart

kstart:
    cli

    ; claer bss section
    mov edi, _bss
    mov ecx, _end
    sub ecx, edi
    mov al, 0
    cld
    rep stosb   

    call vesa_console_init


    jmp _kmain
    jmp $