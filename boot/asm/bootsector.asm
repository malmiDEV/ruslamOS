use16
     org 0x7C00

     jmp 0000h:Boot

Boot:
     ; store boot drive
     mov byte [drive], dl
     
     ; set bootsector segment
     xor ax, ax
     mov ds, ax 
     mov es, ax 
     mov fs, ax 
     mov gs, ax
     mov ss, ax

     ; clear direction
     cld

     ; set stack pointer
     cli                 ; clear interrupt
     mov sp, 0x7C00      ; stack grows at 0x7C00
     mov bp, sp          
     sti                 ; enable interrupt

     ;; load stage2   
     mov bx, 0x7E00
     mov cl, 0x02        ; read first available sector 0x02
     mov dh, 0x02        ; read 2 sector 
     mov dl, [drive]     ; boot drive type
     call disk_load

     ;; load kernel point 
     mov bx, 0x2000      ; ES:BX = 0x10000
     mov cl, 6           ; read at #
     mov dh, 14          ; read sector 
     mov dl, [drive]     ; boot drive type
     call disk_load

     jmp load_stage2

disk_load:
     pusha               ; push all reg 
     push dx

     mov ah, 0x02        ; read sector and put into memory
     mov al, dh          ; sector count
     mov ch, 0           ; low 8 bit of cylinder
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
     xor ax, ax          ; reset
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
     
load_stage2:  
     ; setup segment registers
     mov ax, 0
     mov ds, ax 
     mov es, ax 
     mov fs, ax 
     mov gs, ax 
     mov ss, ax 

     mov ax, 0xFFF0
     mov sp, ax

     mov dl, [drive]     
     mov [0x1000], dl    ; store bootdrive to addr 0x1000

     ; switch to stage2 
     jmp 0x0000:0x7E00

stuck:
     jmp $
     
msg_disk_err: db "BIOS Disk Read Error", 0xA, 0xD, 0
msg_sector_err: db "Sector Error", 0xA, 0xD, 0

drive: db 0

times 510-($-$$) db 0
     db 0x55
     db 0xAA