#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
mod vga_text;

mod ata;
mod io;
mod virt_mem_manager;

use crate::ata::Ata;
use crate::virt_mem_manager::VirtualMem;


const KERNEL_ADDRESS: usize = 0x100000;

#[no_mangle]
pub unsafe extern "C" fn kernel_load(drive: u32) -> ! {
    let mut ata = Ata::read(7, 100, KERNEL_ADDRESS);

    let mut virtual_mem = VirtualMem::new();
    
    virtual_mem.init_virtual_memory();
    virtual_mem.paging_enable();
    
    let kernel: fn() = core::mem::transmute(KERNEL_ADDRESS);
    kernel();

    loop {}
}   

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

