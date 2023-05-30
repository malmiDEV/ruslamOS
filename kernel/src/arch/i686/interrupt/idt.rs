use spin::Mutex;

use core::mem::size_of;
use core::arch::asm;
use bitflags::bitflags;

bitflags! {
    pub struct IdtFlags: u8 {
        const PRESENT = 1 << 7;
        const RING_0 = 0 << 5;
        const RING_1 = 1 << 5;
        const RING_2 = 2 << 5;
        const RING_3 = 3 << 5;
        const SS = 1 << 4;
        const INTERRUPT = 0xE;
        const TRAP = 0xF;
    }
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,
    offset: u32
}

impl IdtDescriptor {
    #[inline]
    const fn new(size: u16, offset: u32) -> Self {
        Self {
            size, 
            offset
        }
    }
}

pub static mut IDT: [IdtEntry; 256] = [IdtEntry::NULL; 256];

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IdtEntry {
    base_low: u16,
    seg_selector: u16,    
    always0: u8,
    flags: u8,
    base_high: u16,
}

impl IdtEntry {
    const NULL: Self = Self {
        base_low: 0x00,
        seg_selector: 0x00,    
        always0: 0x00,
        flags: 0x00,
        base_high: 0x00,
    };
    
    // set idt flags
    fn set_flags(&mut self, flags: IdtFlags) {
        self.flags = flags.bits();
    }
    
    // set offset
    fn set_offset(&mut self, segsel: u16, base: usize) {
        self.base_low = base as u16;
        self.seg_selector = segsel;
        self.base_high = (base >> 16) as u16;
    }

    // set handler function
    pub fn set_fn(&mut self, func: *const u8) {
        self.set_flags(IdtFlags::PRESENT | IdtFlags::RING_0 | IdtFlags::INTERRUPT);
        self.set_offset(0x08, func as usize)
    }
}

#[repr(C)]
pub struct Registers {
    pub ds:        u32,                                          
    pub edi:       u32, 
    pub esi:       u32, 
    pub ebp:       u32, 
    pub useless:   u32, 
    pub ebx:       u32, 
    pub edx:       u32, 
    pub ecx:       u32, 
    pub eax:       u32,   
    pub interrupt: u32, 
    pub error:     u32,                              
    pub eip:       u32, 
    pub cs:        u32, 
    pub eflags:    u32, 
    pub esp:       u32, 
    pub ss:        u32
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Handlers {
    Error(fn(&mut Registers)),
    None
}

pub static HANDLERS: Mutex<[Handlers; 256]> = 
    Mutex::new([Handlers::None; 256]);

pub fn init() {
    use super::interrupts;
    
    unsafe {
        crate::arch::interrupt::idt_gates();
    }

    HANDLERS.lock()[0] = Handlers::Error(interrupts::divide_by_zero);
    HANDLERS.lock()[1] = Handlers::Error(interrupts::debug);
    HANDLERS.lock()[2] = Handlers::Error(interrupts::non_maskable);
    HANDLERS.lock()[3] = Handlers::Error(interrupts::breakpoint);
    HANDLERS.lock()[4] = Handlers::Error(interrupts::overflow);
    HANDLERS.lock()[5] = Handlers::Error(interrupts::bound_range);
    HANDLERS.lock()[7] = Handlers::Error(interrupts::device_not_available);
    HANDLERS.lock()[8] = Handlers::Error(interrupts::double_fault);

    HANDLERS.lock()[10] = Handlers::Error(interrupts::invalid_tss);
    HANDLERS.lock()[11] = Handlers::Error(interrupts::segment_not_present);
    HANDLERS.lock()[12] = Handlers::Error(interrupts::stack_segment);
    HANDLERS.lock()[13] = Handlers::Error(interrupts::protection);

    HANDLERS.lock()[16] = Handlers::Error(interrupts::fpu_fault);
    HANDLERS.lock()[17] = Handlers::Error(interrupts::alignment_check);
    HANDLERS.lock()[18] = Handlers::Error(interrupts::machine_check);
    HANDLERS.lock()[20] = Handlers::Error(interrupts::virtualization);

    HANDLERS.lock()[30] = Handlers::Error(interrupts::security);
    
    unsafe {
        let idt_desc = IdtDescriptor::new(
           ((IDT.len() * size_of::<IdtEntry>()) - 1) as u16,
           (&IDT as *const _) as u32
        );
        load_idt(&idt_desc);
    }
}

#[inline(always)]
unsafe fn load_idt(desc: &IdtDescriptor) {
    asm!("lidt[{}]", in(reg) desc, options(nostack))
}
          

