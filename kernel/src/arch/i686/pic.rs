use crate::utils::io;

const PIC1:          u16 = 0x20;
const PIC1_OFFSET:   u16 = 0x20;
const PIC1_DATA:     u16 = 0x21;

const PIC2:          u16 = 0xA0;
const PIC2_OFFSET:   u16 = 0x28;
const PIC2_DATA:     u16 = 0xA1;

const PIC_EOI:       u8 = 0x20;
const PIC_MODE_8086: u8 = 0x01;
const ICW1_ICW4:     u8 = 0x01;
const ICW1_INIT:     u8 = 0x10;
const ICW1_READ:     u8 = 0x0B;

pub struct Pic;

impl Pic {
    fn new() -> Self {
        unsafe {
            let m1 = io::inb(PIC1_DATA as u16);
            io::wait();
            let m2 = io::inb(PIC2_DATA as u16); 
            io::wait();

            io::outb(PIC1, (ICW1_INIT | ICW1_ICW4) as u8);  
            io::wait();
            io::outb(PIC2, (ICW1_INIT | ICW1_ICW4) as u8);  
            io::wait();
            io::outb(PIC1_DATA, PIC1_OFFSET as u8);         
            io::wait();
            io::outb(PIC2_DATA, PIC2_OFFSET as u8);        
            io::wait();
            io::outb(PIC1_DATA, 0x04);  
            io::wait(); 
            io::outb(PIC2_DATA, 0x02);                      
            io::wait();
            io::outb(PIC1_DATA, PIC_MODE_8086 as u8);       
            io::wait();
            io::outb(PIC1_DATA, PIC_MODE_8086 as u8);       
            io::wait();
            io::outb(PIC1_DATA, m1);                        
            io::wait();
            io::outb(PIC2_DATA, m2);                       
            io::wait();
        }
        
        Self
    }

    unsafe fn pic_set_mask(&self, irq_line: u8) {
        io::outb(PIC1_DATA, irq_line & 0xFF);
        io::wait();
        // io::outb(PIC2_DATA, irq_line >> 8);
        // io::wait();
    }

    unsafe fn get_mask(&self) -> u16 {
        let master = io::inb(PIC1_DATA) as u16;
        let slave = io::inb(PIC2_DATA) as u16;
        master << 8 | slave
    }

    fn eoi(&self, irq: u8) {
        if irq >= 8 {
            unsafe { io::outb(PIC2_DATA, PIC_EOI) }
        } 
        unsafe { io::outb(PIC1_DATA, PIC_EOI) }
    }

    unsafe fn disable(&self) {
        io::outb(PIC1_DATA, 0xFF);
        io::wait();
        io::outb(PIC2_DATA, 0xFF);
        io::wait();
    }
}


// pub fn irq_set_mask(irq_line: &mut u8) {
//     let mut port: u16 = 0;

//     if *irq_line < 8 {
//         port = PIC1_DATA;
//     } else {
//         port = PIC2_DATA;
//         *irq_line -= 8;
//     }

//     let value = io::inb(port) | (1 << *irq_line);
//     io::outb(port, value);
// }

// pub fn irq_clear_mask(irq_line: &mut u8) {
//     let mut port: u16 = 0;

//     if *irq_line < 8 {
//         port = PIC1_DATA;
//     } else {
//         port = PIC2_DATA;
//         *irq_line -= 8;
//     }

//     let value = io::inb(port) & !(1 << *irq_line);
//     io::outb(port, value);
// }