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

times 2048-($-$$) db 0