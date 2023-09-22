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

     ; load stage2
     mov ah, 0x42
     mov dl, byte [boot_disk]
     mov si, disk_dap
     int 0x13

done_loading:
     ; save drive type to stack
     mov dl, byte [boot_disk]
     mov byte [0x2000], dl 
     jmp load_bootloader

puts:
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
     ret

load_bootloader:
     mov ax, 0
     mov ds, ax 
     mov es, ax 
     mov fs, ax 
     mov gs, ax 
     mov ss, ax 
     
     mov sp, 0xFFF0
     mov bp, sp

     ; jump to stage2 bootloader
     jmp 0x00:0x7E00

.loop:
     jmp .loop

msg_load: 
     db "Loading...", 0xA, 0xD, 0
msg_err_load:
     db "Error Load Sector From Disk", 0x0A, 0x0D, 0

boot_disk:
     db 0x80
disk_dap:
   db 0x10
   db 0
   db 1
   db 0
   dw 0x7E00
   dw 0x0000
   dd 1
   dd 0

padd:
     times 510-($-$$) db 0
boot_flag:
     db 0x55, 0xAA
