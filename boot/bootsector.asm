use16
     org 0x7C00

     jmp 0000h:_bootsector

_bootsector:
     ; store boot drive
     mov byte [boot_disk], dl
     
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
     cli
     mov sp, 0x7C00
     mov bp, sp
     sti

     ; claer screen (80*25) VGA text
     mov ax, 0x0003
     int 10h

     mov si, msg_load
     call puts

     ; load stage2 bootloader
     mov bx, 0x7E00

     ; int 13h params
     mov cl, 2
     mov al, 2
     mov dl, [boot_disk]
     call read_sector

     ; load kernel
     push es
     mov ax, 0x1000
     mov es, ax
     
     ; es:bx = 0x5000:0x0000 (0x5000 << 4 + 0x0000) = 0x50000
     mov bx, 0x0000

     ; int 13h params
     mov cl, 5
     mov al, 110
     mov dl, [boot_disk]
     call read_sector
     pop es
     
     ; load bootloader
     jmp load_bootloader

; read sector into memory
read_sector:
     mov ch, 0
     mov dh, 0

     mov ah, 0x02
     int 0x13
     jc .error
     ret
.error:
     mov si, msg_load
     call puts
     xor ah, ah
     int 13h
     jmp read_sector

; print string
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

load_bootloader:
     mov ax, 0
     mov ds, ax 
     mov es, ax 
     mov fs, ax 
     mov gs, ax 
     mov ss, ax 
     
     mov ax, 0xFFF0
     mov sp, ax

     ; jump to stage2 bootloader
     jmp 0x0000:0x7E00

.loop:
     jmp .loop

msg_load: 
     db "Loading...", 0xA, 0xD, 0

boot_disk:
     db 0x00

padd:
     times 510-($-$$) db 0
boot_flag:
     db 0x55, 0xAA
