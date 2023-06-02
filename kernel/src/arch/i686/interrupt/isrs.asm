[bits 32]
    extern general_interrupt_handler

    %macro ISR_NOERRORCODE 1

    global isr%1:
    isr%1:
    push 0
    push %1 
    jmp isr_common
    %endmacro

    %macro ISR_ERRORCODE 1

    global isr%1:
    isr%1:
    push %1
    jmp isr_common
    %endmacro

    %include "kernel/src/arch/i686/interrupt/isrs_code.inc"    

isr_common:
    ; push all register
    pusha

    ; zero out eax for push ds
    xor eax, eax
    mov ax, ds
    push eax

    ; set kernel data segment
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; pass stack pointer to Rust code
    push esp
    call general_interrupt_handler
    add esp, 4

    ; restore previous segment
    pop eax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; pop all register 
    popa
    add esp, 8
    iret