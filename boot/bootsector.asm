use16
     org 0x7C00

     jmp 0000h:Boot

Boot:
     ; clear direction
     cld

     ; store boot drive
     mov byte [drive], dl
     
     ; set bootsector segment
     xor ax, ax
     mov ds, ax 
     mov es, ax 
     mov fs, ax 
     mov gs, ax
     mov ss, ax

     ; set stack pointer
     cli
     mov sp, 0x7C00
     mov bp, sp
     sti

     ;; load stage2   
     mov bx, 0x7E00
     mov dh, 0x02        ; read 2 sector 
     mov dl, [drive]     ; boot drive type
     call disk_load

     ; switch to stage2 
     mov dl, [drive]
     mov [0x2000], dl
     jmp 0x0000:0x7E00

disk_load:
     pusha               ; push all reg 
     push dx

     mov ah, 0x02        ; read sector and put into memory
     mov al, dh          ; sector count
     mov dh, 0x02        ; read first available sector 0x02
     mov ch, 0           ; low 8 bit of cylinder
     mov cl, 0x2
     mov dh, 0           ; head number
     
     stc                 ; set carry
     int 0x13            ; read
     jc disk_error       ; BIOS read fail

     pop dx              
     cmp al, dh          ; bios also set al to # sectors read 
     jne sector_err
     popa                ; clear stack
     ret  

disk_error:
     mov si, msg_disk_err
     call puts
     xor ah, ah
     int 0x13
     jmp stuck

sector_err:
     mov si, msg_sector_err
     call puts
     jmp stuck

; print string
;    param - si (char *)
puts:
     push si
     push ax
     push bx
     mov ah, 0x0E
     mov bh, 0
     mov bl, 0x07
.loop:
     mov al, [si]
     cmp al, 0
     je .return
     int 0x10
     add si, 1
     jmp .loop
.return:     
     pop ax
     pop bx
     pop si
     ret
     
stuck:
     jmp $
     
drive: db 0

msg_disk_err: db "BIOS Disk Read Error", 0xA, 0xD, 0
msg_sector_err: db "Sector Error", 0xA, 0xD, 0

times 510-($-$$) db 0
     db 0x55
     db 0xAA