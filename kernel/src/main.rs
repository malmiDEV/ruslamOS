#![no_main]
#![no_std]

#[macro_use]
pub mod sys;

pub mod arch;
pub mod utils;

use core::panic::PanicInfo;
use utils::io::*;
use core::arch::asm;
 
fn test(regs: &mut crate::arch::interrupt::Registers) {
     print!(".");
}

#[no_mangle]
pub unsafe extern "C" fn _kmain() -> ! {
     // init kernel stuff 
     arch::cpu_interrupt_set();
     arch::interrupt::clear_interrupt();
     // arch::interrupt::enable_interrupt();

     // arch::i686::interrupt::interrupts::regs_handle(0, test);

     println!("RuslamOS\n\n");
     asm!("int 1");

     loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
     unsafe {
          arch::interrupt::clear_interrupt();
          arch::interrupt::halt();
     }
     loop {}
}

