use spin::Mutex;

use super::idt::{Registers, Handlers, HANDLERS};
use crate::arch::pic;

macro_rules! exceptions {
    ($(fn $name:ident() => $msg:expr,)*) => {
        $(
            pub fn $name(regs: &mut Registers) {
                println!("CPU Exeption: {}", $msg);
                println!("
                    \rREGS:\n   EAX={:#X}  EBX={:#X}  ECX={:#X}  EDX={:#X}  ESI={:#X}  EDI={:#X}
                    \r   ESP={:#X}  EBP={:#X}  EIP={:#X}  EFLAGS={:#X}  CS={:#X}  DS={:#X} SS={:#X}", 
                    regs.eax, regs.ebx, regs.ecx, regs.edx, regs.esi, regs.edi, 
                    regs.esp, regs.ebp, regs.eip, regs.eflags, regs.cs, regs.ds, regs.ss
                );
                panic!();
            }
        )*
    }
}

exceptions! {
    fn divide_by_zero()       => "Division by zero",
    fn debug()                => "Debug",
    fn non_maskable()         => "Non Maskable Interrupt",
    fn overflow()             => "Stack Overflow",
    fn bound_range()          => "Out of Bounds",
    fn device_not_available() => "Device not Available",
    fn double_fault()         => "Double Fault",
    fn invalid_tss()          => "Invalid TSS",
    fn segment_not_present()  => "Segment not Present",
    fn stack_segment()        => "Stack Segment Fault",
    fn protection()           => "Protection Fault",
    fn fpu_fault()            => "FPU floating point fault",
    fn alignment_check()      => "Alignment check fault",
    fn machine_check()        => "Machine check fault",
    fn virtualization()       => "Virtualization fault",
    fn security()             => "Security exception",
}

pub fn breakpoint(regs: &mut Registers) {
    (*regs).eip -= 1;
}

use core::arch::asm;
pub fn page_fault(regs: &mut Registers) {
    println!("PAGE FAULT");
    unsafe {
        asm!("cli;hlt");
    }
}

// ... pagefaule and more 

const IRQ_START: u8 = 0x20;

unsafe fn irq_mask(irq: u8) {
    if irq >= 8 {
        pic::SLAVE.mask_set(irq - 8);
    } else {
        pic::MASTER.mask_set(irq);
    }
}

unsafe fn irq_clear(irq: u8) {
    if irq >= 8 {
        pic::SLAVE.mask_clear(irq - 8);
    } else {
        pic::MASTER.mask_clear(irq);
    }
}

unsafe fn eoi(irq: u8) {
    if irq >= 8 {
        pic::MASTER.send_eoi();
        pic::SLAVE.send_eoi();
    } else {
        pic::MASTER.send_eoi();
    }
}

#[no_mangle]
pub unsafe extern "C" fn general_interrupt_handler(regs: &mut Registers) {
    let mut handlers = HANDLERS.lock();
    match &handlers[regs.interrupt as usize] {
        Handlers::Irq(handler) => {
            handler(regs)
        },
        Handlers::Error(handler) => {
            handler(regs);
        },
        Handlers::None => todo!()
    }
    eoi((regs.interrupt + 0x20) as u8);
}

pub unsafe fn regs_handle(int: u8, handler: fn(&mut Registers)) {
    let mut handlers = HANDLERS.lock();
    irq_mask(int + IRQ_START);
    handlers[(int + IRQ_START) as usize] = Handlers::Irq(handler);
    irq_clear(int + IRQ_START);
}