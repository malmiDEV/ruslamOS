use core::ptr;
use core::fmt::Write;

use spin::Mutex;
use lazy_static::lazy_static;

use crate::sys::shell::*;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct RSVbeInfoMode {
    attributes: u16,	
	window_a: u8,			
	window_b: u8,		
	granularity: u16,	
	window_size: u16,
	segment_a: u16,
	segment_b: u16,
	win_func_ptr: u32,	
	pitch: u16,			

	x_res: u16,			 
	y_res: u16,		
	w_char: u8,		
	y_char: u8,		
	planes: u8,
	bitperpixel: u8,			 
	banks: u8,			 
	memory_model: u8,
	bank_size: u8,		 
	image_pages: u8,
	reserved0: u8,
 
	red_mask: u8,
	red_position: u8,
	green_mask: u8,
	green_position: u8,
	blue_mask: u8,
	blue_position: u8,
	reserved_mask: u8,
	reserved_position: u8,
	direct_color_attributes: u8,
 
	framebuffer: u32,		
	off_screen_mem_off: u32,
	off_screen_mem_size: u16,	
	reserved1: [u8; 206]
}


const VBE_MODEINFO_ADDR: usize = 0xC000;
const TEST_FONTADDR: usize = 0xD000;

const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 16;

// set in kstart.asm
#[no_mangle]
pub unsafe extern "C" fn vesa_console_init() {
	let test_font = TEST_FONTADDR as *const u8;

	let vbe_mode_info = &*(VBE_MODEINFO_ADDR as *const RSVbeInfoMode);

	let bpp = (vbe_mode_info.bitperpixel + 1) / 8;
	
	let mut vesa_gfx = VesaGraphics {
		vram: (*(VBE_MODEINFO_ADDR as *const RSVbeInfoMode)).framebuffer as *mut u8,
		font: test_font,
        width: vbe_mode_info.x_res as usize,
		height: vbe_mode_info.y_res as usize,
		pitch: vbe_mode_info.pitch as usize,
		bpp: bpp as usize,
		col: 0,
		row: 0,
	};
	
	vesa_gfx.blank();
	shell_set(Shell::VesaGraphics(vesa_gfx));
}

pub struct Point {
	x: isize, y: isize
}	

impl Point {
	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y }
    }
}

// 2d graphics && font renderer
pub struct VesaGraphics {
	vram: *mut u8,
	font: *const u8,
	width: usize,
	height: usize,
	pitch: usize,
	bpp: usize,
    col: usize,
    row: usize,
}

const GRUVBOX: u32 = 0xFF3C3836;
const CHARCOL: u32 = 0xFFFBF1C7;

impl VesaGraphics {
	pub fn move_cursor(&mut self, x_pos: usize, y_pos: usize) {
		
	}

	pub fn write_char(&mut self, c: u8) {
		match c {
			b'\n' => {
			    self.row += 1;
				self.col = 0;
			}
			b'\r' => {
				self.col = 0;
			}
			_ => {
				for y in 0..16 {
					let glyph = unsafe { *self.font.offset((c - 1) as isize * 16 + y) };
		
					for x in 0..8 {	
						if glyph & (0x80 >> x) != 0 {
							self.draw_pixel(self.col * CHAR_WIDTH + x, self.row * CHAR_HEIGHT + y as usize, CHARCOL);
						}
					}
				}
				self.col += 1;
			}
		}
		self.move_cursor(self.col, self.row);
	}

	pub fn blank(&mut self) {
		unsafe {
			for j in 0..self.height {
				let pointer = self.vram.add(j * self.pitch);
			
				for i in 0..self.width {
					ptr::write(pointer.add(i * self.bpp + 0), (GRUVBOX & 0xFF) as u8);
					ptr::write(pointer.add(i * self.bpp + 1), ((GRUVBOX >> 8) & 0xFF) as u8);
					ptr::write(pointer.add(i * self.bpp + 2), ((GRUVBOX >> 16) & 0xFF) as u8);
				}
			}
		}
	}

	// bresenham's line algorithm
	pub fn draw_line(&mut self, start: &mut Point, end: &mut Point, color: u32) {		
		let mut dx = (end.x - start.x).abs();
		let mut dy = -(end.y - start.y).abs();
		let mut sign_x = if start.x < end.x { 1 } else { -1 }; 
		let mut sign_y = if start.y < end.y { 1 } else { -1	};
		
		let mut error = dx + dy;
		let mut error2 = 0;
		
		loop {
			self.draw_pixel(start.x as usize, start.y as usize, color);
			if start.x == end.x && start.y == end.y { break };
			
			error2 = error * 2;
			if error2 >= dy {
				error += dy;
				start.x += sign_x;
			}
			if error2 <= dx {
				error += dx;
				start.y += sign_y;
			}
		}
	}
	
	pub fn draw_triangle(&mut self, vertex0: &mut Point, vertex1: &mut Point, vertex2: &mut Point, color: u32) {
		self.draw_line(vertex0, vertex1, color);
		self.draw_line(vertex1, vertex2, color);
		self.draw_line(vertex2, vertex0, color);
	}

	pub fn draw_pixel(&mut self, x_pos: usize, y_pos: usize, color: u32) {
		unsafe {
			let addr = self.vram.add(y_pos * self.pitch);
			ptr::write(addr.add(x_pos * self.bpp + 0), (color & 0xFF) as u8); 
			ptr::write(addr.add(x_pos * self.bpp + 1), ((color >> 8) & 0xFF) as u8); 
			ptr::write(addr.add(x_pos * self.bpp + 2), ((color >> 16) & 0xFF) as u8); 
		}
	}
}

impl Write for VesaGraphics {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for i in s.bytes() {
			self.write_char(i);
		}

		Ok(())
	}
}
