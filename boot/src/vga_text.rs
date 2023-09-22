use lazy_static::lazy_static;
use spin::Mutex;

use crate::io::{outb};

use core::fmt::{
    Write, 
    Arguments
};

pub static mut VGA: Vga = Vga {
    cursor_x: 0, 
    cursor_y: 0
};

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

const VGA_BUFFER: usize = 0xB8000;

const SCW: usize = 80;
const SCH: usize = 25;

const HEXCHARS: &[u8; 16] = b"0123456789abcdef";

pub struct Vga {
    cursor_x: usize,
    cursor_y: usize
} 

impl Vga {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0
        }
    }

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
                self.set_vgacol(i, j, ((Color::Black as u8) << 4) | ((Color::White as u8) & 0x0F));
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
}

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for i in s.bytes() {
            self.putc(i)
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_text::write_fmt(format_args!($($arg)*))); 
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {
    unsafe { let _ = core::fmt::write(&mut VGA, args); }
}
