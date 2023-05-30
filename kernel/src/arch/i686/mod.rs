pub mod interrupt;
pub mod pic;

pub unsafe fn cpu_interrupt_set() {
    // init idt
    interrupt::init();
}