use core::ptr;

use crate::io;

pub struct Ata {
    disk_type: u32
}

impl Ata {
    pub fn new(disk_type: u32) -> Self {
        Self {
            disk_type: disk_type
        }
    }

    pub unsafe fn read(&self, lba: u32, sector: u8, address: usize) {
        io::outb(0x1F6, (0xE0 | ((lba >> 24) & 0xF)) as u8);
        io::outb(0x1F2, sector);
        io::outb(0x1F3, lba as u8);
        io::outb(0x1F4, (lba >> 8) as u8);
        io::outb(0x1F5, (lba >> 16) as u8);
        io::outb(0x1F7, 0x20); // send read command

        let address_ptr: *mut u32 = address as *mut u32;        

        for _ in 0..sector {
            while (io::inb(0x1F7) & 8) == 0 {}

            for i in 0..256 {
                *address_ptr.offset(i) = io::inw(0x1F0) as u32;
                print!("{:X} ", *address_ptr.offset(i));
            }
            
            for i in 0..4 {
                io::inb(0x3F6);
            }
        }
    }   
}


