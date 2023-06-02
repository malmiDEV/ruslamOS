#![no_main]
#![no_std]

#[macro_use]
pub mod sys;

pub mod arch;
pub mod utils;

use core::panic::PanicInfo;
use utils::io::*;
// use core::arch::asm;
 
fn test(regs: &mut crate::arch::interrupt::Registers) {
     print!(".");
}

#[no_mangle]
pub extern "C" fn _kmain() -> ! {
     unsafe {
          arch::cpu_interrupt_set();
          arch::interrupt::enable_int();
     }
     println!("Kernel Loaded");

     arch::i686::interrupt::interrupts::regs_handle(0, test);
     loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
     unsafe {
          arch::interrupt::disable_int();
          arch::interrupt::halt();
     }
     loop {}
}

