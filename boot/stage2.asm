[bits 16]  
    org 0x7E00
    
    cld 

    cli
    sti
    
    call enable_a20

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
    
    ; to 32bit mode
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
    jmp 0x8:.after

[bits 32]
.after:
    ; set 32bit pm segment
    mov ax, 0x10
    mov ds, ax 
    mov es, ax
    mov fs, ax
    mov gs, ax 
    mov ss, ax

    ; set stack
    mov esp, 0x90000
    mov ebp, esp

    mov esi, mode_info_block
    mov edi, 0xC000
    mov ecx, 64
    rep movsd

    ; jump to kernel
    jmp 0x8:0x10000

    jmp $

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

; VBE Variables
width: dw 1920
height: dw 1080
bpp: db 32
offset: dw 0
_segment: dw 0	
mode: dw 0

error_msg: db 0x0A,0x0D,"CANNOT FIND VIDEO(VBE) MODE :<",0

    ; end stage2 bootloader
    times 512-($-$$) db 0

vbe_info_block:
    .vbe_signature: db 'VESA'
	.vbe_version: dw 0200h
	.oem_string_pointer: dd 0 
	.capabilities: dd 0
	.video_mode_pointer: dd 0
	.total_memory: dw 0
	.oem_software_rev: dw 0
	.oem_vendor_name_pointer: dd 0
	.oem_product_name_pointer: dd 0
	.oem_product_revision_pointer: dd 0
	.reserved: times 222 db 0
	.oem_data: times 256 db 0

mode_info_block:
    .attributes: dw 0	
	.window_a: db 0		
	.window_b: db 0		
	.granularity: dw 0	
	.window_size: dw 0
	.segment_a: dw 0
	.segment_b: dw 0
	.win_func_ptr: dd 0	
	.pitch: dw 0			

	.x_res: dw 0			 
	.y_res: dw 0		
	.w_char: db 0		
	.y_char: db 0		
	.planes: db 0
	.bitperpixel: db 0			 
	.banks: db 0			 
	.memory_model: db 0
	.bank_size: db 0		 
	.image_pages: db 0
	.reserved0: db 1
 
	.red_mask: db 0
	.red_position: db 0
	.green_mask: db 0
	.green_position: db 0
	.blue_mask: db 0
	.blue_position: db 0
	.reserved_mask: db 0
	.reserved_position: db 0
	.direct_color_attributes: db 0
 
	.framebuffer: dd 0		
	.off_screen_mem_off: dd 0
	.off_screen_mem_size: dw 0	
	.reserved1: times 206 db 0

    times 1536-($-$$) db 0