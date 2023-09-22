#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
mod vga_text;

mod ata;
mod io;
mod virt;
mod phys;

use crate::ata::Ata;
use crate::virt::VirtualMem;
use crate::phys::*;
use crate::vga_text::Vga;

#[no_mangle]
pub unsafe extern "C" fn kernel_load(drive: u32) -> ! {
    // let mut virt_mem = VirtualMem::new();
    // virt_mem.init_virtual_memory();
    
    // let mut phys_addr: u32 = 0;
    // let kernel_page = phys::alloc(1000, 1, &mut phys_addr);
    // println!("{:#X}", kernel_page);
    Ata::read(7, 110, 0x100000);
    
    let kernel: fn() = core::mem::transmute(0x100000);
    kernel(); 

    loop {}
}   

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

