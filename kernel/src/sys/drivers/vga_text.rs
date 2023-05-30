use lazy_static::lazy_static;
use spin::Mutex;

use crate::utils::io::{outb};

lazy_static! {
     pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
          cursor_x: 0, 
          cursor_y: 0
     });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
     Black = 0,
     Blue = 1,
     Green = 2,
     Cyan = 3,
     Red = 4,
     Magenta = 5,
     Brown = 6,
     LightGray = 7,
     DarkGray = 8,
     LightBlue = 9,
     LightGreen = 10,
     LightCyan = 11,
     LightRed = 12,
     Pink = 13,
     Yellow = 14,
     White = 15,
}

static VGA_BUFFER: usize = 0xB8000;

const SCW: usize = 80;
const SCH: usize = 25;

const HEXCHARS: &[u8; 16] = b"0123456789abcdef";

pub struct Writer {
     cursor_x: usize,
     cursor_y: usize
} 

impl Writer {
     fn vga_cursor(&mut self, x: usize, y: usize) {
          let pos: usize = y * SCW + x;

          unsafe {
               outb(0x3D4, 14);
               outb(0x3D5, (pos >> 8) as u8);
               outb(0x3D4, 15);
               outb(0x3D5, (pos & 0xFF) as u8);
          }
     }

     pub fn clear_screen(&mut self) {
          for j in 0..SCH {
               for i in 0..SCW {   
                    self.set_vgacol(i, j, ((Color::Blue as u8) << 4) | ((Color::Yellow as u8) & 0x0F));
                    self.set_vgachar(i, j, b'\0');
               }
          }
          self.cursor_x = 0;
          self.cursor_y = 0;
          self.vga_cursor(self.cursor_x, self.cursor_y)
     }

     pub fn set_vgacol(&mut self, x: usize, y: usize, cl: u8) {
          unsafe { *((VGA_BUFFER + 2 * (y * SCW + x) + 1) as *mut u8) = cl }
     }

     pub fn set_vgachar(&mut self, x: usize, y: usize, chr: u8) {
          unsafe { *((VGA_BUFFER + 2 * (y * SCW + x)) as *mut u8) = chr }
     }

     pub fn putc(&mut self, chr: u8) {
          match chr {
               b'\n' => {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
               }
               b'\r' => {
                    self.cursor_x = 0;
               }
               _ => {
                    self.set_vgachar(self.cursor_x, self.cursor_y, chr);
                    self.cursor_x += 1;
               }
          }

          if self.cursor_x >= SCW {
               self.cursor_x = 0;
               self.cursor_y += 1;
          }
          self.vga_cursor(self.cursor_x, self.cursor_y)
     }    

     pub fn puts(&mut self, string: &str) {
          for i in string.bytes() {
               self.putc(i);
          }
     }

     pub fn n2a(&mut self, mut num: u32, mode: u32) {
          let mut buffer: [u8; 32] = [b'\0'; 32];
          let mut pos: usize = 0;

          loop {
               let rem: u32 = num % mode;
               num /= mode;
               buffer[pos] = HEXCHARS[rem as usize];
               pos += 1;
               if num == 0 {
                    break;
               }
          }

          while pos > 0 {
               pos -= 1;
               self.putc(buffer[pos]);
          }
     }
}
