; ===== ATA READ ======
;   eax - LBA
;   cl  - SECTOR COUNT
;   edi - destination
; =====================
ata_lba_read:
    and eax, 0x0FFFFFFF
    mov ebx, eax            

    mov edx, 0x1F6          
    shr eax, 24
    or al, 0b11100000
    out dx, al

    mov edx, 0x1F2
    mov al, cl
    out dx, al

    mov edx, 0x1F3
    mov eax, ebx
    out dx, al

    mov edx, 0x1F4
    mov eax, ebx
    shr eax, 8
    out dx, al

    mov edx, 0x1F5
    mov eax, ebx
    shr eax, 16
    out dx, al

    mov edx, 0x1F7
    mov al, 0x20    ; Read with ready
    out dx, al 
.loop:
    in al, dx
    test al, 8
    jz .loop
    mov eax, 256
    xor bx, bx
    mov bl, cl
    mul bx
    mov ecx, eax
    mov edx, 0x1F0
    rep insw
    ret

    