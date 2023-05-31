#![no_main]
#![no_std]

#[macro_use]
pub mod sys;

pub mod arch;
pub mod utils;

use core::panic::PanicInfo;
use utils::io::*;
 
#[no_mangle]
pub extern "C" fn _kmain() -> ! {
     unsafe {
          arch::cpu_interrupt_set();
          
          use core::arch::asm;
          asm!("int $0x1");
     }
     println!("Kernel Loaded: {:#X}", 0xDEADBEEF as u32);
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

