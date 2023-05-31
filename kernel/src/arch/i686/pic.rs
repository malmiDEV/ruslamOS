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

// idea from redox os kernel
pub static mut MASTER: Pic = Pic::new(PIC1);
pub static mut SLAVE: Pic = Pic::new(PIC2);

pub unsafe fn remap() {
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

pub unsafe fn disable() {
    io::outb(PIC1_DATA, 0xFF);
    io::wait();
    io::outb(PIC2_DATA, 0xFF);
    io::wait(); 
}

pub struct Pic {
    mos: u16,
    data: u16
}

impl Pic {
    pub const fn new(port: u16) -> Self {
        Self {
            mos: port,
            data: port + 1
        }
    }
    
    pub unsafe fn mask_set(&mut self, irq: u8) {
        let mut mask = unsafe { io::inb(self.data) };
        mask |= 1 << irq;
        io::outb(self.data, mask)
    }

    pub unsafe fn mask_clear(&mut self, irq: u8) {
        let mut mask = unsafe { io::inb(self.data) };
        mask &= !(1 << irq);
        io::outb(self.data, mask)
    }

    pub unsafe fn send_eoi(&self) {
        io::outb(self.data, PIC_EOI);
    }
}