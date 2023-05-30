use crate::utils::io;

const PIC1:          u16 = 0x20;
const PIC1_OFFSET:   u16 = 0x20;
const PIC1_DATA:     u16 = 0x21;

const PIC2:          u16 = 0xA0;
const PIC2_OFFSET:   u16 = 0x28;
const PIC2_DATA:     u16 = 0xA1;

const PIC_EOI:       u16 = 0x20;
const PIC_MODE_8086: u16 = 0x01;
const ICW1_ICW4:     u16 = 0x01;
const ICW1_INIT:     u16 = 0x10;

pub fn pic_remap() {
    let m1 = io::inb(PIC1_DATA as u16);
    let m2 = io::inb(PIC2_DATA as u16); 

    io::outb(PIC1, (ICW1_INIT | ICW1_ICW4) as u8);  io::nop();
    io::outb(PIC2, (ICW1_INIT | ICW1_ICW4) as u8);  io::nop();
    io::outb(PIC1_DATA, PIC1_OFFSET as u8);         io::nop();
    io::outb(PIC2_DATA, PIC2_OFFSET as u8);         io::nop();
    io::outb(PIC1_DATA, 0x04);                      io::nop(); 
    io::outb(PIC2_DATA, 0x02);                      io::nop();
    io::outb(PIC1_DATA, PIC_MODE_8086 as u8);       io::nop();
    io::outb(PIC1_DATA, PIC_MODE_8086 as u8);       io::nop();
    io::outb(PIC1_DATA, m1);                        io::nop();
    io::outb(PIC2_DATA, m2);                        io::nop();
}

pub fn irq_set_mask(irq_line: &mut u8) {
    let mut port: u16 = 0;

    if *irq_line < 8 {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        *irq_line -= 8;
    }

    let value = io::inb(port) | (1 << *irq_line);
    io::outb(port, value);
}

pub fn irq_clear_mask(irq_line: &mut u8) {
    let mut port: u16 = 0;

    if *irq_line < 8 {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        *irq_line -= 8;
    }

    let value = io::inb(port) & !(1 << *irq_line);
    io::outb(port, value);
}