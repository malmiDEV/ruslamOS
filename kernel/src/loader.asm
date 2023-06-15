[bits 32]
    extern _kmain 
    global kstart

kstart:
    cli
    call _kmain
    jmp $