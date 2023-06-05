#![no_main]
#![no_std]

#[macro_use]
pub mod sys;

pub mod arch;
pub mod utils;

use core::panic::PanicInfo;
use core::arch::asm;

use utils::io::*;
use arch::i686::interrupt; 


fn test(regs: &mut crate::arch::interrupt::Registers) {
     print!(".");
}

#[no_mangle]
pub extern "C" fn _kmain() -> ! {
     // init kernel stuff 
     unsafe {
          arch::cpu_interrupt_set();
      
          use crate::arch::pic;
          pic::remap();
          
          interrupt::enable_interrupt();

     }
     
     println!("RuslamOS\n\n");
     
     unsafe {
          interrupt::interrupts::regs_handle(0, test);
     }

     loop {
          unsafe {
               interrupt::halt();
          }
     }   
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
     unsafe {
          arch::interrupt::clear_interrupt();
          arch::interrupt::halt();
     }
     loop {}
}

