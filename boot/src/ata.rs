use core::ptr;

use crate::io;

pub struct Ata;

impl Ata {
    pub unsafe fn read(lba: u32, sector: u8, address: usize) {
        io::outb(0x1F6, (0xE0 | ((lba >> 24) & 0xFF)) as u8);
        io::outb(0x1F2, sector);
        io::outb(0x1F3, lba as u8);
        io::outb(0x1F4, (lba >> 8) as u8);
        io::outb(0x1F5, (lba >> 16) as u8);
        io::outb(0x1F7, 0x20); // send read command

        let address_ptr: *mut u16 = address as *mut u16;        

        let mut ctr: isize = 0;
        for _ in 0..sector {
            while (io::inb(0x1F7) & 8) == 0 {}

            for i in 0..256 {
                *address_ptr.offset(ctr) = io::inw(0x1F0);
                ctr += 1;
            }
        
            for i in 0..4 {
                io::inb(0x3F6);
            }   
        }
    }   
}