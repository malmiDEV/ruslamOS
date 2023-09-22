#![no_main]
#![no_std]

#[macro_use]
pub mod sys;

pub mod arch;
pub mod utils;

use core::panic::PanicInfo;
use core::arch::asm;
use arch::i686::interrupt; 

use utils::*;

fn test(regs: &mut crate::arch::interrupt::Registers) {
     print!(".");
}

#[no_mangle]
pub extern "C" fn _kmain() -> ! {
     // init kernel stuff 
     unsafe {
          use crate::sys::drivers;
          drivers::vesa::vesa_console_init();
          
          // use crate::arch::mem::virt::VirtualMem;
          // let mut virt_mem = VirtualMem::new();
          // virt_mem.init_virtual_memory();

          arch::cpu_interrupt_set();

          use crate::arch::pic;
          pic::remap();

          interrupt::enable_interrupt();

          // clear ps2 status register
          while (io::inb(0x64) & 1) != 0 { 
               io::inb(0x60);
          }

          println!("RuslamOS\n\n");

          // asm!("int 14");
     }
     

     // unsafe {
     //      interrupt::interrupts::regs_handle(0, test);
     // }

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

