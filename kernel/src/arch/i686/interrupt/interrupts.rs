use spin::Mutex;

use super::idt::{Registers, Handlers, HANDLERS};

macro_rules! exceptions {
    ($(fn $name:ident() => $msg:expr,)*) => {
        $(
            pub fn $name(regs: &mut Registers) {
                println!("\nCPU Exeption: {}", $msg);
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

// ... pagefaule and more 

#[no_mangle]
pub unsafe extern "C" fn _IsrHandler(regs: &mut Registers) {
    match &HANDLERS.lock()[regs.interrupt as usize] {
        Handlers::Error(handler) => {
            handler(regs);
        }
        Handlers::None => println!("unhandled interrupt: {:#X}", regs.interrupt)
    }
    panic!()
}