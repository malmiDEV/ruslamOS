[bits 16]  
    org 0x7E00

    ; clear direction
    cld 

    ; setup segment registers
    xor ax, ax
    mov ds, ax 
    mov es, ax 
    mov fs, ax 
    mov gs, ax 
    mov ss, ax 

    ; set stack pointer
    cli
    mov ax, 0xFFF0
    mov sp, ax
    sti

    ; set vesa mode
    mov ax, 0x4F00
    mov di, vbe_info_block
    int 10h

    ; if bios not support vbe
    cmp ax, 0x4F
    jne .error

    mov ax, word [vbe_info_block.video_mode_pointer]
    mov [offset], ax
    mov ax, word [vbe_info_block.video_mode_pointer+2]
    mov [_segment], ax

    mov fs, ax
    mov si, [offset]

.find_mode:
    mov dx, [fs:si]
	add si, 2
	mov [offset], si
	mov [mode], dx
    
    ; is end of vidmode array
	cmp dx, word 0xFFFF			
	je .error
 
	mov ax, 0x4F01				; get vbe mode info
	mov cx, [mode]
	mov di, mode_info_block
	int 0x10
 
	cmp ax, 0x4F
	jne .error
 
	mov ax, [width]
	cmp ax, [mode_info_block.x_res]
	jne .next_mode
 
	mov ax, [height]
	cmp ax, [mode_info_block.y_res]
	jne .next_mode
 
	mov al, [bpp]
	cmp al, [mode_info_block.bitperpixel]
	jne .next_mode
 
	; Set the mode
	mov ax, 0x4F02
	mov bx, [mode]
	or bx, 0x4000			
	xor di, di 		
	int 0x10

	cmp ax, 0x4F
	jne .error
    
    ; enable a20 gate
    call enable_a20

    ; switch cpu to 32bit protected mode
	jmp pm

.next_mode:
	mov ax, [_segment]
	mov fs, ax
	mov si, [offset]
	jmp .find_mode

.error:
    mov si, error_msg
    call puts
    jmp $          ; jmp forever

; put string 
;   param - si (char *)
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
    je .done
    inc si
    int 10h
    jmp .loop
.done:
    pop bx
    pop ax
    pop si
    ret

; set a20 line
enable_a20:
    ; disable keyboard
    call a20wait    
    mov al, 0xAD
    out 0x64, al
    
    ; read control out port
    call a20wait
    mov al, 0xD0
    out 0x64, al

    call a20wait2
    in al, 0x60
    push eax
 
    ; write control out port
    call a20wait
    mov al, 0xD1
    out 0x64, al
 
    call a20wait
    pop eax
    or al, 2
    out 0x60, al
    
    ; enable keyboard
    call a20wait
    mov al, 0xAE
    out 0x64, al

    call a20wait
    ret

a20wait:
    in al, 0x64
    test al, 2
    jnz a20wait
    ret
a20wait2:
    in al, 0x64
    test al, 1
    jz a20wait2
    ret

; switch cpu to 32bit pm
pm:
    cli                     
    lgdt [GDT_DESC]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp 0x8:.after              ; long jmp to after part

[bits 32]
.after:
    ; set 32bit segment
    mov ax, 0x10
    mov ds, ax 
    mov es, ax
    mov fs, ax
    mov gs, ax 
    mov ss, ax

    ; set stack
    mov esp, 0x90000
    mov ebp, esp

    ; store vbe mode info block
    mov esi, mode_info_block
    mov edi, 0xC000
    mov ecx, 64
    rep movsd


;; Load Kernel use ATA PIO PORTS (should change)
    ; zero out eax
    xor eax, eax

    ; destination
    mov edi, 0x100000
    ; sector to read
    mov bl, 109

    ;; CHS read
    mov edx, 0x1F6           
    mov al, [0x2000]              
    and al, 0b00001111        
    or al, 0b10100000        
    out dx, al

    mov edx, 0x1F2            
    mov al, 110          
    out dx, al
    
    mov edx, 0x1F3            
    mov al, 6               
    out dx, al
    
    mov edx, 0x1F4            
    xor eax, eax          
    out dx, al
    
    mov edx, 0x1F5            
    xor eax, eax          
    out dx, al
    
    mov edx, 0x1F7            
    mov al, 0x20        ; Read with retry           
    out dx, al
      
.kloop:
    in al, dx
    test al, 8
    jz .kloop
    mov ecx, 256
    mov edx, 0x1F0
    rep insw

    mov edx, 0x3F6
    in al, dx
    in al, dx
    in al, dx
    in al, dx    

    cmp bl, 0
    je kernel_load

    dec bl
    mov dx, 0x1F7
    jmp .kloop

kernel_load:
    jmp 0x8:0x100000

stuck:
    jmp $

; VBE Variables
width:      dw 1920
height:     dw 1080
bpp:        db 32
offset:     dw 0
_segment:   dw 0	
mode:       dw 0

GDT_START:
GDT_NULL:   
    dq 0
GDT_CODE:   
    dw 0FFFFh
    dw 0
    db 0
    db 10011010b
    db 11001111b
    db 0
GDT_DATA:   
    dw 0FFFFh
    dw 0
    db 0
    db 10010010b
    db 11001111b
    db 0
GDT_END:
GDT_DESC:   
    dw (GDT_END - GDT_START) - 1
    dd GDT_START    

error_msg: db 0x0A,0x0D,"CANNOT FIND VIDEO(VBE) MODE :<",0

    ; end stage2 bootloader
    times 1024-($-$$) db 0

    %include "boot/vbe_structure.asm"
